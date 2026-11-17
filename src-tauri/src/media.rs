//! 回环媒体代理：CDN 按 Referer/UA 校验，且 WebKit 对自定义 scheme 的 <video> 不可靠。
//! 起一个只监听 127.0.0.1 的最小 HTTP/1.1 服务，转发 Range 并补头。
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

use md5::{Digest, Md5};

pub struct Server {
    pub port: u16,
}

type Cached = Option<(Option<String>, reqwest::blocking::Client)>;
static CLIENT: OnceLock<Mutex<Cached>> = OnceLock::new();
const MAX_CONNECTIONS: usize = 24;

/// CDN 出口变化时重建；Client 克隆开销很小，按调用返回。
fn client() -> Option<reqwest::blocking::Client> {
    let proxy = crate::net::cdn_proxy();
    let cached = CLIENT.get_or_init(|| Mutex::new(None));
    let mut guard = cached.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, c)) = guard.as_ref() {
        if *at == proxy {
            return Some(c.clone());
        }
    }
    let c = crate::net::blocking_builder()
        .redirect(reqwest::redirect::Policy::limited(8))
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()
        .ok()?;
    *guard = Some((proxy, c.clone()));
    Some(c)
}

struct Slots {
    active: Mutex<usize>,
    ready: Condvar,
}

struct SlotGuard(Arc<Slots>);

impl SlotGuard {
    fn acquire(slots: &Arc<Slots>) -> Self {
        let mut n = slots.active.lock().unwrap_or_else(|e| e.into_inner());
        while *n >= MAX_CONNECTIONS {
            n = slots.ready.wait(n).unwrap_or_else(|e| e.into_inner());
        }
        *n += 1;
        Self(slots.clone())
    }
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        if let Ok(mut n) = self.0.active.lock() {
            *n = n.saturating_sub(1);
        }
        self.0.ready.notify_one();
    }
}

type Head = (String, String, Vec<(String, String)>);

fn head(s: &mut TcpStream) -> Option<Head> {
    let mut r = BufReader::new(s.try_clone().ok()?);
    let mut line = String::new();
    r.read_line(&mut line).ok()?;
    let mut it = line.split_whitespace();
    let method = it.next()?.to_string();
    let target = it.next()?.to_string();
    let mut headers = Vec::new();
    loop {
        let mut l = String::new();
        if r.read_line(&mut l).ok()? == 0 {
            break;
        }
        let l = l.trim_end();
        if l.is_empty() {
            break;
        }
        if let Some((k, v)) = l.split_once(':') {
            headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    if method == "OPTIONS" {
        let _ = s.write_all(b"HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: GET,OPTIONS\r\nContent-Length: 0\r\n\r\n");
        return None;
    }
    Some((method, target, headers))
}

fn proxied_key(port: u16, raw: &str, key: &str) -> String {
    let mut url = proxied(port, raw);
    url.push_str("&k=");
    url.push_str(key);
    url
}

fn proxied(port: u16, raw: &str) -> String {
    // 保留扩展名：WebKit/GStreamer 会按 URL 后缀判断容器格式。
    // 抖音直链常以 `/` 结尾，扩展名只能从 mime_type 查询参数取。
    let (path, query) = raw.split_once('?').unwrap_or((raw, ""));
    let mime = url::form_urlencoded::parse(query.as_bytes())
        .find(|(k, _)| k == "mime_type")
        .map(|(_, v)| v.into_owned())
        .unwrap_or_default();
    let from_mime = if mime.contains("mpegurl") || mime.contains("m3u8") {
        "m3u8"
    } else if mime.contains("mp4") {
        "mp4"
    } else if mime.contains("webm") {
        "webm"
    } else {
        ""
    };
    let ext = if !from_mime.is_empty() {
        from_mime.to_string()
    } else {
        path.rsplit('/')
            .next()
            .and_then(|name| name.rsplit_once('.').map(|(_, e)| e))
            .filter(|e| (1..=5).contains(&e.len()) && e.chars().all(|c| c.is_ascii_alphanumeric()))
            .map(str::to_string)
            .unwrap_or_else(|| "bin".into())
    };
    format!(
        "http://127.0.0.1:{port}/m.{ext}?u={}",
        url::form_urlencoded::byte_serialize(raw.as_bytes()).collect::<String>()
    )
}

/// HLS 清单里的分片/密钥地址也走本代理，否则 XHR 直连 CDN 会被 CORS 拦。
fn rewrite_manifest(body: &str, base: &reqwest::Url, port: u16) -> String {
    let mut out = String::new();
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() {
            out.push('\n');
            continue;
        }
        if let Some(rest) = t.strip_prefix('#') {
            if let Some(i) = rest.find("URI=\"") {
                let vstart = i + 5;
                if let Some(vend) = rest[vstart..].find('"') {
                    let uri = &rest[vstart..vstart + vend];
                    let abs = base
                        .join(uri)
                        .map(|u| u.to_string())
                        .unwrap_or_else(|_| uri.to_string());
                    out.push_str(&format!(
                        "#{}URI=\"{}\"{}\n",
                        &rest[..i],
                        proxied(port, &abs),
                        &rest[vstart + vend + 1..]
                    ));
                    continue;
                }
            }
            out.push_str(t);
            out.push('\n');
        } else {
            let abs = base
                .join(t)
                .map(|u| u.to_string())
                .unwrap_or_else(|_| t.to_string());
            out.push_str(&proxied(port, &abs));
            out.push('\n');
        }
    }
    out
}

fn rewrite_local_manifest(body: &str, base: &std::path::Path, port: u16) -> String {
    let mut out = String::new();
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() {
            out.push('\n');
            continue;
        }
        if let Some(rest) = t.strip_prefix('#') {
            if let Some(i) = rest.find("URI=\"") {
                let vstart = i + 5;
                if let Some(vend) = rest[vstart..].find('"') {
                    let uri = &rest[vstart..vstart + vend];
                    let p = base.join(uri);
                    out.push_str(&format!(
                        "#{}URI=\"{}\"{}\n",
                        &rest[..i],
                        proxied(port, &p.to_string_lossy()),
                        &rest[vstart + vend + 1..]
                    ));
                    continue;
                }
            }
            out.push_str(t);
            out.push('\n');
        } else {
            let p = base.join(t);
            out.push_str(&proxied(port, &p.to_string_lossy()));
            out.push('\n');
        }
    }
    out
}

fn serve_local(
    s: &mut TcpStream,
    method: &str,
    path: &std::path::Path,
    headers: &[(String, String)],
    port: u16,
) {
    use std::io::{Read, Seek, SeekFrom};
    let Ok(meta) = std::fs::metadata(path) else {
        let _ = s
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        return;
    };
    let total = meta.len();
    let range = headers.iter().find(|(k, _)| k == "range").map(|(_, v)| v);
    let (start, end, code) = if let Some(v) = range.and_then(|v| v.strip_prefix("bytes=")) {
        let mut it = v.split('-');
        let a = it
            .next()
            .and_then(|x| x.parse::<u64>().ok())
            .unwrap_or(0)
            .min(total.saturating_sub(1));
        let b = it
            .next()
            .and_then(|x| x.parse::<u64>().ok())
            .unwrap_or(total.saturating_sub(1))
            .min(total.saturating_sub(1));
        (a, b, 206u16)
    } else {
        (0, total.saturating_sub(1), 200u16)
    };
    let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
    let ctype = match ext {
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        "mp3" => "audio/mpeg",
        "m4a" | "aac" => "audio/mp4",
        "wav" => "audio/wav",
        "webp" => "image/webp",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "m3u8" | "m3u" => "application/vnd.apple.mpegurl",
        _ => "application/octet-stream",
    };
    if ext == "m3u8" || ext == "m3u" {
        let mut text = String::new();
        let _ = std::fs::File::open(path).and_then(|mut f| f.read_to_string(&mut text));
        let base = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let out = rewrite_local_manifest(&text, base, port);
        let body = out.as_bytes();
        let head = format!("HTTP/1.1 200 OK\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {ctype}\r\nCache-Control: no-cache\r\nContent-Length: {}\r\n\r\n", body.len());
        let _ = s.write_all(head.as_bytes());
        let _ = s.write_all(body);
        return;
    }
    let len = end.saturating_sub(start) + 1;
    let head = if code == 206 {
        format!("HTTP/1.1 206 Partial Content\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nAccept-Ranges: bytes\r\nContent-Type: {ctype}\r\nContent-Range: bytes {start}-{end}/{total}\r\nContent-Length: {len}\r\n\r\n")
    } else {
        format!("HTTP/1.1 200 OK\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nAccept-Ranges: bytes\r\nContent-Type: {ctype}\r\nContent-Length: {len}\r\n\r\n")
    };
    if s.write_all(head.as_bytes()).is_err() || method == "HEAD" {
        return;
    }
    let Ok(mut f) = std::fs::File::open(path) else {
        return;
    };
    if f.seek(SeekFrom::Start(start)).is_err() {
        return;
    }
    let mut left = len;
    let mut buf = vec![0u8; 64 * 1024];
    while left > 0 {
        let want = left.min(buf.len() as u64) as usize;
        match f.read(&mut buf[..want]) {
            Ok(0) => break,
            Ok(n) => {
                if s.write_all(&buf[..n]).is_err() {
                    break;
                }
                left -= n as u64;
            }
            Err(_) => break,
        }
    }
}

static DECRYPT_LOCK: Mutex<()> = Mutex::new(());

/// 直链带签名参数、CDN 前缀也随取址变化，只有路径末段的对象 id 稳定。
fn cache_id(raw: &str) -> String {
    let path = url::Url::parse(raw)
        .map(|u| u.path().to_string())
        .unwrap_or_else(|_| raw.to_string());
    match path.rsplit('/').find(|s| !s.is_empty()) {
        Some(obj) => obj.to_string(),
        None => path,
    }
}

fn cache_path(raw: &str, key: &str) -> Option<PathBuf> {
    let dir = dirs::cache_dir()?.join("douyintai").join("cenc");
    std::fs::create_dir_all(&dir).ok()?;
    let mut h = Md5::new();
    h.update(cache_id(raw).as_bytes());
    h.update(b"|");
    h.update(key.as_bytes());
    Some(dir.join(format!("{:x}.mp4", h.finalize())))
}

fn prune(dir: &std::path::Path) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    let mut files: Vec<_> = rd
        .flatten()
        .filter_map(|e| e.metadata().ok().map(|m| (m.modified().ok(), e.path())))
        .collect();
    files.sort_by_key(|f| f.0);
    for (_, p) in files.iter().rev().skip(24) {
        let _ = std::fs::remove_file(p);
    }
}

/// 整片下载后解密并落缓存；Range 由 `serve_local` 处理。首个请求会等下载完成。
fn prepare(raw: &str, key: &str) -> Option<PathBuf> {
    let path = cache_path(raw, key)?;
    if path.is_file() {
        return Some(path);
    }
    let _guard = DECRYPT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if path.is_file() {
        return Some(path);
    }
    let kb: [u8; 16] = {
        let v: Vec<u8> = (0..key.len() / 2)
            .filter_map(|i| u8::from_str_radix(key.get(i * 2..i * 2 + 2)?, 16).ok())
            .collect();
        v.try_into().ok()?
    };
    let res = client()?
        .get(raw)
        .header("Referer", "https://www.douyin.com/")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15")
        .send()
        .ok()?;
    if !res.status().is_success() {
        crate::logging::error("media", format!("cenc 下载 {}", res.status()));
        return None;
    }
    let mut body = res.bytes().ok()?.to_vec();
    match crate::cenc::decrypt(&mut body, &kb) {
        Ok(n) => crate::logging::info("media", format!("cenc 解密 {n} 样本 {}B", body.len())),
        Err(e) => {
            crate::logging::error("media", format!("cenc 解密失败：{e}"));
            return None;
        }
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, &body).ok()?;
    std::fs::rename(&tmp, &path).ok()?;
    if let Some(dir) = path.parent() {
        prune(dir);
    }
    Some(path)
}

fn handle(mut s: TcpStream, port: u16) {
    let Some((method, target, headers)) = head(&mut s) else {
        return;
    };
    let started = std::time::Instant::now();
    let query = target.split_once('?').map(|(_, q)| q).unwrap_or("");
    let raw = url::form_urlencoded::parse(query.as_bytes())
        .find(|(k, _)| k == "u")
        .map(|(_, v)| v.into_owned());
    let key = url::form_urlencoded::parse(query.as_bytes())
        .find(|(k, _)| k == "k")
        .map(|(_, v)| v.into_owned());
    let Some(raw) = raw else {
        crate::logging::warn("media", format!("{method} {target} 缺少 u 参数"));
        let _ = s.write_all(
            b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        return;
    };
    if let Some(key) = key.as_deref() {
        match prepare(&raw, key) {
            Some(path) => serve_local(&mut s, &method, &path, &headers, port),
            None => {
                let _ = s.write_all(
                    b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
            }
        }
        return;
    }
    let local = std::path::PathBuf::from(&raw);
    if local.is_file() {
        serve_local(&mut s, &method, &local, &headers, port);
        return;
    }
    let label = url::Url::parse(&raw)
        .map(|u| format!("{}{}", u.host_str().unwrap_or("?"), u.path()))
        .unwrap_or_else(|_| raw.chars().take(160).collect());
    let range = headers
        .iter()
        .find(|(k, _)| k == "range")
        .map(|(_, v)| v.clone());
    crate::logging::info(
        "media",
        format!(
            "→ {method} {label} range={}",
            range.as_deref().unwrap_or("-")
        ),
    );
    let Some(client) = client() else { return };
    let mut rb = client.get(&raw)
        .header("Referer", "https://www.douyin.com/")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15");
    if let Some(r) = &range {
        rb = rb.header("Range", r.clone());
    }
    let mut resp = match rb.send() {
        Ok(r) => r,
        Err(e) => {
            crate::logging::error(
                "media",
                format!("← {label} 502 {}ms {e}", started.elapsed().as_millis()),
            );
            let _ = s.write_all(
                b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
            return;
        }
    };
    let status = resp.status();
    let mut code = status.as_u16();
    let ctype = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let is_manifest = ctype.contains("mpegurl") || resp.url().path().ends_with(".m3u8");
    let final_url = resp.url().clone();
    if is_manifest {
        let mut body = resp.text().unwrap_or_default();
        // CDN 边缘偶发 404/0 字节清单：重试一次，再失败就返回 502，避免把空清单当播放列表交给 hls.js。
        if !status.is_success() || body.trim().is_empty() {
            crate::logging::warn(
                "media",
                format!("← {label} manifest {code} {}B，重试", body.len()),
            );
            let mut rb2 = client.get(&raw)
                .header("Referer", "https://www.douyin.com/")
                .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15");
            if let Some(r) = &range {
                rb2 = rb2.header("Range", r.clone());
            }
            if let Ok(r2) = rb2.send() {
                if r2.status().is_success() {
                    body = r2.text().unwrap_or_default();
                    code = if body.trim().is_empty() { 502 } else { 200 };
                }
            }
        }
        if !(200..300).contains(&code) || body.trim().is_empty() {
            let reply =
                "HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
            let _ = s.write_all(reply.as_bytes());
            crate::logging::error(
                "media",
                format!(
                    "← {label} manifest 空/失败 {code} {}ms",
                    started.elapsed().as_millis()
                ),
            );
            return;
        }
        let body = rewrite_manifest(&body, &final_url, port);
        let out = format!("HTTP/1.1 200 OK\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/vnd.apple.mpegurl\r\nCache-Control: no-cache\r\nContent-Length: {}\r\n\r\n", body.len());
        let _ = s.write_all(out.as_bytes());
        let _ = s.write_all(body.as_bytes());
        crate::logging::info(
            "media",
            format!(
                "← {label} manifest {}B {}ms",
                body.len(),
                started.elapsed().as_millis()
            ),
        );
        return;
    }
    let reason = match code {
        200 => "OK",
        206 => "Partial Content",
        304 => "Not Modified",
        _ => status.canonical_reason().unwrap_or("Upstream"),
    };
    let mut out = format!("HTTP/1.1 {code} {reason}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nAccept-Ranges: bytes\r\nCache-Control: private, max-age=300\r\nX-Content-Type-Options: nosniff\r\n");
    for h in ["content-type", "content-range", "content-length"] {
        if let Some(v) = resp.headers().get(h).and_then(|v| v.to_str().ok()) {
            out.push_str(&format!("{h}: {v}\r\n"));
        }
    }
    out.push_str("\r\n");
    if s.write_all(out.as_bytes()).is_err() {
        crate::logging::warn(
            "media",
            format!(
                "← {label} 客户端断开（响应头）{}ms",
                started.elapsed().as_millis()
            ),
        );
        return;
    }
    if method == "HEAD" {
        crate::logging::info(
            "media",
            format!("← {label} {code} HEAD {}ms", started.elapsed().as_millis()),
        );
        return;
    }
    let mut sent = 0u64;
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        match resp.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                sent += n as u64;
                if s.write_all(&buf[..n]).is_err() {
                    crate::logging::warn(
                        "media",
                        format!(
                            "← {label} 客户端断开 {sent}B {}ms",
                            started.elapsed().as_millis()
                        ),
                    );
                    return;
                }
            }
            Err(e) => {
                crate::logging::error(
                    "media",
                    format!(
                        "← {label} 读取中断 {sent}B {}ms {e}",
                        started.elapsed().as_millis()
                    ),
                );
                return;
            }
        }
    }
    crate::logging::info(
        "media",
        format!(
            "← {label} {code} {sent}B {}ms",
            started.elapsed().as_millis()
        ),
    );
}

impl Server {
    pub fn start() -> std::io::Result<Self> {
        let l = TcpListener::bind(("127.0.0.1", 0))?;
        let port = l.local_addr()?.port();
        std::thread::spawn(move || {
            let slots = Arc::new(Slots {
                active: Mutex::new(0),
                ready: Condvar::new(),
            });
            for c in l.incoming().flatten() {
                let guard = SlotGuard::acquire(&slots);
                std::thread::spawn(move || {
                    let _guard = guard;
                    handle(c, port);
                });
            }
        });
        Ok(Self { port })
    }

    pub fn url(&self, raw: &str) -> String {
        proxied(self.port, raw)
    }

    pub fn url_key(&self, raw: &str, key: &str) -> String {
        proxied_key(self.port, raw, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_id_ignores_signed_query() {
        let a = "https://v11-weba.douyinvod.com/abc/1/video/tos/cn/tos-cn-ve-15/oBJ1/?a=6383&er=1&l=2026";
        let b = "https://v26-web.douyinvod.com/def/2/video/tos/cn/tos-cn-ve-15/oBJ1/?a=6383&er=1&l=2027";
        assert_eq!(cache_id(a), cache_id(b));
        assert_ne!(cache_id(a), cache_id(&a.replace("oBJ1", "oBJ2")));
    }

    #[test]
    fn ext_from_mime_when_path_ends_with_slash() {
        let u = proxied(
            42,
            "https://v.example/video/tos/cn/item/?mime_type=video_mp4&a=1",
        );
        assert!(u.starts_with("http://127.0.0.1:42/m.mp4?u="), "{u}");
    }

    #[test]
    fn ext_from_path_for_manifest() {
        let u = proxied(42, "https://v.example/live/index.m3u8?x=1");
        assert!(u.starts_with("http://127.0.0.1:42/m.m3u8?u="), "{u}");
    }

    #[test]
    fn ext_falls_back_to_bin() {
        let u = proxied(42, "https://v.example/stream/");
        assert!(u.starts_with("http://127.0.0.1:42/m.bin?u="), "{u}");
    }

    #[test]
    fn manifest_segments_and_keys_are_proxied() {
        let base = reqwest::Url::parse("https://v.example/live/index.m3u8").unwrap();
        let out = rewrite_manifest(
            "#EXTM3U\n#EXT-X-KEY:METHOD=AES-128,URI=\"key.bin\"\nseg1.ts\n",
            &base,
            42,
        );
        assert!(
            out.contains(
                "URI=\"http://127.0.0.1:42/m.bin?u=https%3A%2F%2Fv.example%2Flive%2Fkey.bin\""
            ),
            "{out}"
        );
        assert!(
            out.contains("http://127.0.0.1:42/m.ts?u=https%3A%2F%2Fv.example%2Flive%2Fseg1.ts"),
            "{out}"
        );
    }

    #[test]
    fn local_manifest_uses_full_proxy_urls() {
        let base = std::path::Path::new("/tmp/rec");
        let key = base.join("key.bin");
        let seg = base.join("00001.ts");
        let out = rewrite_local_manifest(
            "#EXTM3U\n#EXT-X-KEY:METHOD=AES-128,URI=\"key.bin\"\n00001.ts\n",
            base,
            42,
        );
        assert!(
            out.contains(&format!("URI=\"{}\"", proxied(42, &key.to_string_lossy()))),
            "{out}"
        );
        assert!(out.contains(&proxied(42, &seg.to_string_lossy())), "{out}");
    }
}
