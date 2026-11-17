//! 直连出网（rustls，无外部运行时）+ 真机头 + 信封头。
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use md5::{Digest, Md5};
use rand::Rng;

use crate::params::{build_ua, device_token};
use crate::pool::Identity;
use crate::proxy::Proxies;
use crate::signer::{Backend, SignInfo, Signer};
use crate::{Error, Result};

const NO_SIGN_SUFFIX: [&str; 6] = [
    "douyin.com",
    "douyinpic.com",
    "douyinvod.com",
    "bytednsdoc.com",
    "byteimg.com",
    "amemv.com.cn",
];

/// 同一 URL + 同一身份 + 同一请求体的签名实测可复用（并发也可），这里按 3 分钟缓存。
/// 参数时间戳由 `params::tick_ms` 对齐到 60s 桶，桶内才可能命中。
const SIGN_TTL: Duration = Duration::from_secs(180);
const SIGN_MAX: usize = 512;

/// 同一签名正在生成时，其余同键请求在这里等，避免同 URL 并发重复签名。
type Flight = Arc<(Mutex<bool>, Condvar)>;

struct CachedSign {
    block: String,
    at: Instant,
}

pub struct Http {
    proxies: Arc<Proxies>,
    signer: Signer,
    signs: Mutex<HashMap<String, CachedSign>>,
    flights: Mutex<HashMap<String, Flight>>,
    last_request_id: Mutex<Option<String>>,
    last_url: Mutex<Option<String>>,
}

fn trace_id() -> String {
    let mut rng = rand::thread_rng();
    let a: String = (0..16)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    format!("00-{a}-{}1", &a[..15])
}

fn signable(host: &str, path: &str) -> bool {
    if path.starts_with("/aweme/v1/play/") || path.starts_with("/aweme/v1/series/") {
        return false;
    }
    !NO_SIGN_SUFFIX.iter().any(|s| host.ends_with(s))
}

const REG_DID: &str = "3524403579608627";
const REG_IID: &str = "3524403579612723";
const REG_UA: &str = "com.ss.android.ugc.aweme/320201 (Linux; U; Android 13; zh_CN; 2201123C; Build/TKQ1.221114.001; Cronet/TTNetVersion:3578d129 2024-11-05 QuicVersion:d9628e3d 2024-10-11)";

fn encode_params(params: &BTreeMap<String, String>) -> String {
    let mut ser = url::form_urlencoded::Serializer::new(String::new());
    for (k, v) in params {
        ser.append_pair(k, v);
    }
    ser.finish()
        .replace("%2A", "*")
        .replace("%28", "(")
        .replace("%29", ")")
        .replace("%5B", "[")
        .replace("%5D", "]")
        .replace("%3A", ":")
}

fn builder(proxy: Option<&str>) -> Result<reqwest::blocking::Client> {
    let mut b = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(8))
        .user_agent("douyintai");
    if let Some(url) = proxy {
        b = b.proxy(reqwest::Proxy::all(url)?);
    }
    Ok(b.build()?)
}

/// 直连客户端（媒体、公网 IP 等不需要代理选择的地方）。
pub fn direct_client() -> Result<reqwest::blocking::Client> {
    builder(None)
}

/// 指定出口的客户端。
pub fn client_with(proxy: Option<&str>) -> Result<reqwest::blocking::Client> {
    builder(proxy)
}

impl Http {
    pub fn new(signer: Signer) -> Result<Self> {
        let proxies = Arc::new(Proxies::default());
        // 环境变量只用于取证与冒烟，界面上配置的列表会覆盖它。
        if let Ok(url) = std::env::var("DYCORE_PROXY") {
            proxies.set(&url, true);
        }
        Ok(Self {
            proxies,
            signer,
            signs: Mutex::new(HashMap::new()),
            flights: Mutex::new(HashMap::new()),
            last_request_id: Mutex::new(None),
            last_url: Mutex::new(None),
        })
    }

    fn cached(&self, key: &str) -> Option<String> {
        let cache = self.signs.lock().ok()?;
        let hit = cache.get(key)?;
        (hit.at.elapsed() < SIGN_TTL).then(|| hit.block.clone())
    }

    fn sign_as(&self, identity: &Identity, url: &str, extra: &str) -> Result<(String, bool)> {
        let key = format!(
            "{}\u{1}{url}\u{1}{:x}",
            identity.device_id,
            Md5::digest(extra.as_bytes())
        );
        if let Some(hit) = self.cached(&key) {
            return Ok((hit, true));
        }
        // 同键只签一次：并发请求等首个结果，不让同一 URL 重复进模拟器。
        let (flight, leader) = {
            let mut map = self.flights.lock().unwrap_or_else(|e| e.into_inner());
            match map.get(&key) {
                Some(f) => (f.clone(), false),
                None => {
                    let f: Flight = Arc::new((Mutex::new(false), Condvar::new()));
                    map.insert(key.clone(), f.clone());
                    (f, true)
                }
            }
        };
        if !leader {
            let (lock, cv) = &*flight;
            let mut done = lock.lock().unwrap_or_else(|e| e.into_inner());
            while !*done {
                done = cv.wait(done).unwrap_or_else(|e| e.into_inner());
            }
            drop(done);
            self.flights
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&key);
            if let Some(hit) = self.cached(&key) {
                return Ok((hit, true));
            }
        }
        let block = match self
            .signer
            .sign(&identity.device_id, &identity.iid, url, extra)
        {
            Ok(b) => b,
            Err(e) => {
                self.flights
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .remove(&key);
                let (lock, cv) = &*flight;
                *lock.lock().unwrap_or_else(|e| e.into_inner()) = true;
                cv.notify_all();
                return Err(e);
            }
        };
        if let Ok(mut cache) = self.signs.lock() {
            if cache.len() >= SIGN_MAX {
                cache.retain(|_, v| v.at.elapsed() < SIGN_TTL);
                if cache.len() >= SIGN_MAX {
                    cache.clear();
                }
            }
            cache.insert(
                key.clone(),
                CachedSign {
                    block: block.clone(),
                    at: Instant::now(),
                },
            );
        }
        self.flights
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&key);
        {
            let (lock, cv) = &*flight;
            *lock.lock().unwrap_or_else(|e| e.into_inner()) = true;
            cv.notify_all();
        }
        Ok((block, false))
    }

    /// 发一条 device_register：签名固定用引导身份，URL 里不带 device_id/iid。
    /// 注册与取公网 IP 必须同一出口，先取一条再全程复用。
    pub fn pick_proxy(&self) -> Option<String> {
        self.proxies.pick()
    }

    pub fn bind_proxy(&self, device_id: &str, url: Option<&str>) {
        self.proxies.bind(device_id, url);
    }

    pub fn register_request(
        &self,
        url: &str,
        body: &[u8],
        picked: Option<&str>,
    ) -> Result<Vec<u8>> {
        let stub = hex_upper(&Md5::digest(body));
        let extra = format!("x-ss-stub\r\n{stub}");
        let block = self.signer.sign(REG_DID, REG_IID, url, &extra)?;
        let client = self.proxies.client(picked)?;
        let mut req = client
            .post(url)
            .header("Content-Type", "application/octet-stream;tt-data=a")
            .header("User-Agent", REG_UA)
            .header("log-encode-type", "gzip")
            .header("X-SS-STUB", stub)
            .header("x-tt-request-tag", "t=0;n=1")
            .header("activity_now_client", "0")
            .body(body.to_vec());
        let clean = block.replace('\r', "");
        let lines: Vec<&str> = clean.split('\n').collect();
        for pair in lines.chunks(2) {
            let (Some(name), Some(value)) = (pair.first(), pair.get(1)) else {
                continue;
            };
            if let (Ok(n), Ok(v)) = (
                reqwest::header::HeaderName::from_bytes(name.trim().as_bytes()),
                reqwest::header::HeaderValue::from_str(value.trim()),
            ) {
                req = req.header(n, v);
            }
        }
        let resp = match req.send() {
            Ok(r) => {
                if let Some(p) = picked {
                    self.proxies.report(p, r.status().is_success());
                }
                r
            }
            Err(e) => {
                if let Some(p) = picked {
                    self.proxies.report(p, false);
                }
                return Err(e.into());
            }
        };
        let status = resp.status();
        let bytes = resp.bytes()?.to_vec();
        if !status.is_success() {
            let text: String = String::from_utf8_lossy(&bytes).chars().take(200).collect();
            return Err(Error::new(status.as_u16(), text));
        }
        Ok(bytes)
    }

    /// 注册明文要填出口 IP；失败返回空串，不影响注册。
    /// 当前出口的公网 IP；配了代理时与随后的注册请求走同一出口。
    pub fn public_ip(&self) -> String {
        self.public_ip_via(None)
    }

    pub fn public_ip_via(&self, picked: Option<&str>) -> String {
        let Ok(client) = self.proxies.client(picked) else {
            return String::new();
        };
        client
            .get("http://ip-api.com/line/?fields=query")
            .timeout(Duration::from_secs(8))
            .send()
            .and_then(|r| r.text())
            .map(|s| s.trim().to_string())
            .unwrap_or_default()
    }

    /// 翻页预热：有下一页时先把 `cursor + count` 的签名算好，下次请求直接命中缓存。
    /// 只投递不等待，网络往返期间完成，不占当前请求的关键路径。
    /// 组装签名输入：URL + `x-ss-stub`/`x-tt-dt` 两行 extra。
    fn sign_parts(
        &self,
        identity: &Identity,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: &[u8],
    ) -> Option<(String, String)> {
        if !signable(host, path) {
            return None;
        }
        let mut url = format!("https://{host}{path}");
        if !params.is_empty() {
            url.push('?');
            url.push_str(&encode_params(params));
        }
        let mut extra = format!("x-ss-stub\r\n{}", hex_upper(&Md5::digest(body)));
        let token = device_token(identity);
        if !token.is_empty() {
            extra.push_str("\r\nx-tt-dt\r\n");
            extra.push_str(token);
        }
        Some((url, extra))
    }

    fn presign_next(
        &self,
        identity: &Identity,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: &[u8],
    ) {
        if !has_more(body) {
            return;
        }
        let num = |k: &str| params.get(k).and_then(|v| v.parse::<i64>().ok());
        let (Some(cursor), Some(count)) = (num("cursor"), num("count")) else {
            return;
        };
        let mut next = params.clone();
        next.insert("cursor".into(), (cursor + count).to_string());
        if let Some((url, extra)) = self.sign_parts(identity, host, path, &next, body) {
            self.signer
                .presign(&identity.device_id, &identity.iid, &url, &extra);
        }
    }

    /// 只签名不发请求，供常用接口预热。
    pub fn presign(
        &self,
        identity: &Identity,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: &[u8],
    ) -> Result<()> {
        let Some((url, extra)) = self.sign_parts(identity, host, path, params, body) else {
            return Ok(());
        };
        self.sign_as(identity, &url, &extra).map(|_| ())
    }

    /// 每条签名线程各预热一次（真实签名，不只看缓存）。
    pub fn presign_each(
        &self,
        identity: &Identity,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: &[u8],
    ) {
        let Some((url, extra)) = self.sign_parts(identity, host, path, params, body) else {
            return;
        };
        let _ = self
            .signer
            .warm_each(&identity.device_id, &identity.iid, &url, &extra);
    }

    /// 切签名实现；旧实现签出的缓存作废，下一发按新实现重签。
    pub fn set_sign_backend(&self, backend: Backend) {
        self.signer.set_backend(backend);
        self.signs.lock().unwrap_or_else(|e| e.into_inner()).clear();
    }

    pub fn proxies(&self) -> &Proxies {
        &self.proxies
    }

    pub fn sign_info(&self) -> SignInfo {
        self.signer.info()
    }

    pub fn last_request_id(&self) -> Option<String> {
        self.last_request_id
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn last_url(&self) -> Option<String> {
        self.last_url
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn request(
        &self,
        method: &str,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: Option<&[u8]>,
        content_type: &str,
        cookie: &str,
        referer: Option<&str>,
        identity: &Identity,
        first_launch: &str,
        retries: usize,
    ) -> Result<Vec<u8>> {
        let mut url = format!("https://{host}{path}");
        if !params.is_empty() {
            url.push('?');
            url.push_str(&encode_params(params));
        }
        let mut last = String::new();
        *self.last_url.lock().unwrap_or_else(|e| e.into_inner()) = Some(url.clone());
        for attempt in 0..=retries {
            let attempt_at = Instant::now();
            let picked = self.proxies.pick_for(&identity.device_id);
            let client = self.proxies.client(picked.as_deref())?;
            let trace = trace_id();
            crate::diag::debug(
                "http",
                format!(
                    "→ {method} https://{host}{path} attempt={attempt} trace={trace} identity={} proxy={} params={}",
                    short_id(&identity.device_id),
                    proxy_label(picked.as_deref()),
                    params.keys().take(24).cloned().collect::<Vec<_>>().join(",")
                ),
            );
            *self
                .last_request_id
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = Some(trace.clone());
            let mut req = client.request(
                reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
                &url,
            );
            req = req
                .header("User-Agent", build_ua(identity))
                .header("Accept-Encoding", "gzip, deflate, br")
                .header("sdk-version", "2")
                .header("passport-sdk-version", "203274")
                .header("x-vc-bdturing-sdk-version", "3.7.4.cn")
                .header("x-tt-request-tag", "t=0;n=0");
            if let Some(aid) = params.get("aid") {
                req = req.header("X-SS-DP", aid);
            }
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            req = req
                .header("X-SS-REQ-TICKET", now.to_string())
                .header("activity_now_client", now.to_string())
                .header("x-tt-trace-id", trace.clone());
            let token = device_token(identity);
            if !token.is_empty() {
                req = req.header("x-tt-dt", token);
            }
            if !cookie.is_empty() && !host.contains("webcast") {
                req = req.header("Cookie", cookie);
            }
            if let Some(r) = referer {
                req = req.header("Referer", r);
            }
            if signable(host, path) {
                let raw = body.unwrap_or(&[]);
                let mut h = format!("x-ss-stub\r\n{}", hex_upper(&Md5::digest(raw)));
                if !token.is_empty() {
                    h.push_str("\r\nx-tt-dt\r\n");
                    h.push_str(token);
                }
                let ts = std::time::Instant::now();
                let (block, hit) = self.sign_as(identity, &url, &h).map_err(|e| Error {
                    status: 503,
                    message: format!("信封签名失败: {}", e.message),
                    kind: e.kind,
                })?;
                crate::diag::debug(
                    "http",
                    format!(
                        "  签名 hit={} {}ms trace={trace}",
                        hit as u8,
                        ts.elapsed().as_millis()
                    ),
                );
                if std::env::var("DYSIGN_DIAG").is_ok() {
                    let _ = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("/tmp/sign_diag.log")
                        .and_then(|mut f| {
                            use std::io::Write;
                            writeln!(
                                f,
                                "{} {} hit={} ms={} {}",
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis(),
                                path,
                                hit as u8,
                                ts.elapsed().as_millis(),
                                identity.device_id
                            )
                        });
                }
                if std::env::var("TIM").is_ok() {
                    let names: Vec<&str> = block.split("\r\n").step_by(2).collect();
                    eprintln!(
                        "[tim] sign {}ms {} {:?}",
                        ts.elapsed().as_millis(),
                        block.len(),
                        names
                    );
                    std::fs::write("/tmp/signblock.txt", &block).ok();
                    std::fs::write("/tmp/signua.txt", build_ua(identity)).ok();
                    std::fs::write(
                        "/tmp/signinput.txt",
                        format!("{url}\n---\n{h}\n---\n{:?}", identity),
                    )
                    .ok();
                }
                for line in block.split("\r\n").collect::<Vec<_>>().chunks(2) {
                    if line.len() == 2 && !line[0].is_empty() {
                        req = req.header(line[0], line[1]);
                    }
                }
                if !raw.is_empty() {
                    req = req.header("X-SS-STUB", hex_upper(&Md5::digest(raw)));
                }
            }
            if let Some(b) = body {
                req = req.header("Content-Type", content_type).body(b.to_vec());
            }
            let _ = first_launch;
            if std::env::var("TIM").is_ok() {
                if let Some(r) = req.try_clone().and_then(|c| c.build().ok()) {
                    eprintln!("[req] {method} {url}");
                    for (k, v) in r.headers() {
                        eprintln!("[req] {k}: {}", v.to_str().unwrap_or("?"));
                    }
                }
            }
            match req.send() {
                Ok(res) => {
                    let status = res.status().as_u16();
                    let final_url = res.url().clone();
                    let bytes = res.bytes().map(|b| b.to_vec()).unwrap_or_default();
                    let shown_url = final_url.as_str();
                    let shown_url = &shown_url[..shown_url.len().min(1200)];
                    crate::diag::debug(
                        "http",
                        format!(
                            "← {method} https://{host}{path} status={status} {}B {}ms trace={trace} url={shown_url}",
                            bytes.len(),
                            attempt_at.elapsed().as_millis(),
                        ),
                    );
                    if (status == 200 || status == 302) && !bytes.is_empty() {
                        if let Some(p) = &picked {
                            self.proxies.report(p, true);
                        }
                        self.presign_next(identity, host, path, params, body.unwrap_or(&[]));
                        return Ok(bytes);
                    }
                    if let Some(p) = &picked {
                        if status == 403 || status == 429 || status >= 500 {
                            self.proxies.report(p, false);
                        }
                    }
                    last = format!("HTTP {status} {}B", bytes.len());
                    let snip = String::from_utf8_lossy(&bytes[..bytes.len().min(200)]).to_string();
                    if !snip.is_empty() {
                        last.push_str(&format!(" {snip}"));
                    }
                    if matches!(status, 400 | 401 | 404) {
                        break;
                    }
                }
                Err(e) => {
                    if let Some(p) = &picked {
                        self.proxies.report(p, false);
                    }
                    crate::diag::debug(
                        "http",
                        format!(
                            "← {method} https://{host}{path} error {}ms trace={trace} timeout={} connect={} error={e}",
                            attempt_at.elapsed().as_millis(),
                            e.is_timeout(),
                            e.is_connect()
                        ),
                    );
                    last = e.to_string();
                }
            }
            if attempt < retries {
                std::thread::sleep(Duration::from_millis(500 * (attempt as u64 + 1)));
            }
        }
        let status = last
            .strip_prefix("HTTP ")
            .and_then(|x| x.split_whitespace().next())
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        // 200 空包是服务端风控的典型表现，按可换号重试的 429 上报。
        let (status, last) = if status == 200 && last.ends_with("0B") {
            (429, format!("{last}（风控空包）"))
        } else {
            (status, last)
        };
        Err(Error::new(
            status,
            format!("{method} {host}{path} 失败: {last}"),
        ))
    }
}

fn short_id(id: &str) -> String {
    id.chars()
        .rev()
        .take(8)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

fn proxy_label(proxy: Option<&str>) -> &str {
    if proxy.is_some() {
        "proxy"
    } else {
        "direct"
    }
}

/// 响应里是否还有下一页（不做完整解析，看 `has_more` 之后的十余字节）。
fn has_more(body: &[u8]) -> bool {
    let mut i = 0;
    while let Some(pos) = body[i..].windows(9).position(|w| w == b"has_more") {
        let at = i + pos + 9;
        let tail = &body[at..body.len().min(at + 24)];
        if tail.contains(&b't') && String::from_utf8_lossy(tail).contains("true") {
            return true;
        }
        i = at;
    }
    false
}

fn hex_upper(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for x in b {
        s.push_str(&format!("{x:02X}"));
    }
    s
}
