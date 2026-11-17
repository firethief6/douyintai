//! 语法层：把前端路由翻成抖音请求，回前端用的 JSON 形状。
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};

use crate::http::Http;
use crate::params::{self, common, media};
use crate::pool::{Identity, Pool, Status as PoolStatus};
use crate::register;
use crate::signer::{Backend, SignInfo, Signer};
use crate::spade;
use crate::{Error, Result};

const API_HOST: &str = "api5-normal-lq.amemv.com";
const APP_HOST: &str = "api5-core-lq.amemv.com";
const SEARCH_HOST: &str = "tsearch.amemv.com";
const LIVE_HOST: &str = "webcast-normal.amemv.com";
const USER_HOST: &str = "aweme.snssdk.com";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetKind {
    Video,
    User,
    Room,
    Series,
    Mix,
    Challenge,
    Keyword,
    Short,
}

impl TargetKind {
    fn name(self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::User => "user",
            Self::Room => "room",
            Self::Series => "series",
            Self::Mix => "mix",
            Self::Challenge => "challenge",
            Self::Keyword => "keyword",
            Self::Short => "short",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Target {
    kind: TargetKind,
    value: String,
    candidates: Vec<TargetKind>,
}

pub struct Api {
    pool: Mutex<Pool>,
    pool_cv: Condvar,
    resources: std::path::PathBuf,
    maintainer: AtomicBool,
    cdn: AtomicBool,
    http: Http,
    first_launch: String,
    details: Mutex<HashMap<String, (Instant, Value)>>,
    filtered: Mutex<HashMap<String, (Instant, String)>>,
    series_cache: Mutex<HashMap<String, (Instant, Vec<Value>)>>,
    play_stats: Mutex<PlayStats>,
    last_identity: Mutex<Option<String>>,
    buckets: Mutex<HashMap<(String, String), Bucket>>,
}

#[derive(Default)]
struct PlayStats {
    requests: u64,
    primary_hits: u64,
    fallback_used: u64,
    fallback_hits: u64,
    failures: u64,
}

struct PlayInfo {
    url: String,
    size: u64,
    bit_rate: i64,
    width: i64,
    height: i64,
    gear: String,
    key: Option<String>,
    key_err: Option<String>,
}

#[derive(Clone)]
struct Gear {
    name: String,
    quality: String,
    raw: String,
    size: u64,
    bit_rate: i64,
    width: i64,
    height: i64,
    h264: bool,
}

fn is_h264(variant: &Value) -> bool {
    // CDN URL 常是无编码标记的随机路径；Android 接口用这两个字段标记 HEVC。
    if ["is_h265", "is_bytevc1"]
        .iter()
        .any(|key| variant[*key].as_bool() == Some(true) || num_of(&variant[*key]) > 0)
    {
        return false;
    }
    let addr = &variant["play_addr"];
    [&variant["gear_name"], &addr["url_key"]]
        .into_iter()
        .chain(addr["url_list"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .all(|s| {
            let s = s.to_ascii_lowercase();
            !["h265", "hevc", "hvc1", "hev1", "bytevc1"]
                .iter()
                .any(|codec| s.contains(codec))
        })
}

fn play_gears(video: &Value, preview: bool, h264_only: bool, selected: Option<&str>) -> Vec<Gear> {
    let mut list: Vec<Gear> = video["bit_rate"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|b| {
            let raw = b["play_addr"]["url_list"][0].as_str()?.to_string();
            Some(Gear {
                name: str_of(&b["gear_name"]),
                quality: str_of(&b["quality_type"]),
                size: num_of(&b["play_addr"]["data_size"])
                    .max(num_of(&b["data_size"]))
                    .max(0) as u64,
                bit_rate: num_of(&b["bit_rate"]).max(0),
                width: num_of(&b["play_addr"]["width"]).max(0),
                height: num_of(&b["play_addr"]["height"]).max(0),
                h264: is_h264(b),
                raw,
            })
        })
        .collect();
    if preview {
        // Windows WebView2 可能只解出 HEVC 的音轨；其他平台保留 HEVC 预览。
        if h264_only {
            list.retain(|g| g.h264);
        }
        list.sort_by(|a, b| {
            (b.width * b.height)
                .cmp(&(a.width * a.height))
                .then_with(|| b.bit_rate.cmp(&a.bit_rate))
                .then_with(|| b.h264.cmp(&a.h264))
        });
        if let Some(want) = selected {
            list.retain(|g| g.name == want || g.quality == want);
        }
    } else {
        list.sort_by(|a, b| {
            (b.width * b.height)
                .cmp(&(a.width * a.height))
                .then_with(|| b.h264.cmp(&a.h264))
        });
        if let Some(want) = selected {
            if let Some(i) = list
                .iter()
                .position(|g| g.name == want || g.quality == want)
            {
                let g = list.remove(i);
                list.insert(0, g);
            }
        }
    }
    list
}

fn play_fallbacks(video: &Value, h264_only: bool) -> Vec<&str> {
    let fields: &[&str] = if h264_only {
        &["play_addr_h264", "play_addr"]
    } else {
        &["play_addr"]
    };
    fields
        .iter()
        .filter(|field| !h264_only || **field != "play_addr" || is_h264(video))
        .flat_map(|field| video[*field]["url_list"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .collect()
}

#[derive(Debug)]
struct Bucket {
    tokens: f64,
    at: Instant,
}

thread_local! {
    static CURRENT_ENDPOINT: RefCell<String> = const { RefCell::new(String::new()) };
}

/// 端点限速：敏感列表低于详情。返回值是每秒补充量与桶容量。
fn token_rate(endpoint: &str) -> (f64, f64) {
    let sensitive = endpoint.contains("/comments")
        || endpoint.contains("/replies")
        || endpoint.contains("/danmaku")
        || endpoint.starts_with("/search");
    let list = endpoint.ends_with("/posts")
        || endpoint.ends_with("/videos")
        || endpoint.ends_with("/episodes");
    let detail = endpoint.matches('/').count() == 2 && endpoint.starts_with("/video/");
    if sensitive {
        (0.4, 1.0)
    } else if list {
        (0.7, 1.0)
    } else if detail {
        (2.0, 3.0)
    } else {
        (1.0, 2.0)
    }
}

fn bucket_ready(b: &mut Bucket, endpoint: &str) -> bool {
    let (rate, cap) = token_rate(endpoint);
    let el = b.at.elapsed().as_secs_f64();
    b.tokens = (b.tokens + el * rate).min(cap);
    b.at = Instant::now();
    b.tokens >= 1.0
}

fn take_token(b: &mut Bucket, endpoint: &str) -> bool {
    if bucket_ready(b, endpoint) {
        b.tokens -= 1.0;
        true
    } else {
        false
    }
}

fn q(v: &Value, key: &str) -> String {
    v.get(key)
        .map(|x| match x {
            Value::String(s) => s.clone(),
            other => other.to_string().trim_matches('"').to_string(),
        })
        .unwrap_or_default()
}

fn count_of(v: &Value, key: &str, default: usize) -> usize {
    let n = num_of(&v[key]);
    if n > 0 {
        n as usize
    } else {
        default
    }
}

fn page(items: Vec<Value>, has_more: bool, cursor: Option<&Value>) -> Value {
    json!({"items": items, "meta": {"has_more": has_more, "cursor": cursor.cloned()}})
}

fn target_from_url(url: &str) -> Result<Target> {
    let parsed = url::Url::parse(url)?;
    let host = parsed
        .host_str()
        .unwrap_or_default()
        .trim_start_matches("www.");
    let path = parsed.path().trim_start_matches('/');
    let first = path.split('/').next().unwrap_or_default();
    let tail = path.split_once('/').map(|x| x.1).unwrap_or_default();
    let value = tail.split('/').next().unwrap_or_default();
    let target = match (host, first) {
        ("v.douyin.com", _) => Target {
            kind: TargetKind::Short,
            value: url.to_string(),
            candidates: vec![],
        },
        ("douyin.com", "video" | "note") if !value.is_empty() => Target {
            kind: TargetKind::Video,
            value: value.to_string(),
            candidates: vec![],
        },
        ("douyin.com", "user") if !value.is_empty() => Target {
            kind: TargetKind::User,
            value: value.to_string(),
            candidates: vec![],
        },
        ("douyin.com", "collection" | "mix") if !value.is_empty() => Target {
            kind: TargetKind::Mix,
            value: value.to_string(),
            candidates: vec![],
        },
        ("douyin.com", "series") if !value.is_empty() => Target {
            kind: TargetKind::Series,
            value: value.to_string(),
            candidates: vec![],
        },
        ("douyin.com", "challenge") if !value.is_empty() => Target {
            kind: TargetKind::Challenge,
            value: url::form_urlencoded::parse(value.as_bytes())
                .next()
                .map(|(v, _)| v.into_owned())
                .unwrap_or_else(|| value.to_string()),
            candidates: vec![],
        },
        ("live.douyin.com", _) if !first.is_empty() => Target {
            kind: TargetKind::Room,
            value: first.to_string(),
            candidates: vec![],
        },
        ("webcast.amemv.com", _) => parsed
            .path_segments()
            .and_then(|mut s| s.find(|x| *x == "reflow").and_then(|_| s.next()))
            .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
            .map(|x| Target {
                kind: TargetKind::Room,
                value: x.to_string(),
                candidates: vec![],
            })
            .ok_or_else(|| Error::new(400, format!("直播链接里没有房间号：{url}")))?,
        _ => return Err(Error::new(400, format!("认不出这条链接的类型：{url}"))),
    };
    Ok(target)
}

fn extract_url(input: &str) -> Option<&str> {
    let starts = [
        "https://",
        "http://",
        "v.douyin.com/",
        "www.douyin.com/",
        "live.douyin.com/",
    ];
    let mut offset = 0;
    while offset < input.len() {
        let rest = &input[offset..];
        if let Some(found) = starts
            .iter()
            .filter_map(|p| rest.find(p).map(|i| (i, *p)))
            .min_by_key(|(i, _)| *i)
        {
            let start = offset + found.0;
            let tail = &input[start..];
            let end = tail
                .find(|c: char| c.is_whitespace() || "，。、）)]】".contains(c))
                .unwrap_or(tail.len());
            return Some(tail[..end].trim_end_matches(['.', ',', ';']));
        }
        let next = rest
            .char_indices()
            .nth(1)
            .map(|(i, _)| i)
            .unwrap_or(rest.len());
        offset += next.max(1);
    }
    None
}

fn parse_target(input: &str) -> Result<Target> {
    let raw = input.trim();
    if raw.is_empty() {
        return Err(Error::new(400, "目标为空"));
    }
    if let Some((head, tail)) = raw.split_once(':') {
        let tail = tail.trim();
        if !tail.is_empty() {
            let prefix = match head.to_ascii_lowercase().as_str() {
                "video" | "note" => Some(TargetKind::Video),
                "user" | "uid" | "sec" => Some(TargetKind::User),
                "room" => Some(TargetKind::Room),
                "series" => Some(TargetKind::Series),
                "mix" | "collection" => Some(TargetKind::Mix),
                "challenge" | "cha" => Some(TargetKind::Challenge),
                "kw" | "q" => Some(TargetKind::Keyword),
                _ => None,
            };
            if let Some(kind) = prefix {
                return Ok(Target {
                    kind,
                    value: tail.to_string(),
                    candidates: vec![],
                });
            }
        }
    }
    if let Some(url) = extract_url(raw) {
        let absolute = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("https://{url}")
        };
        return target_from_url(&absolute);
    }
    let is_sec = raw.starts_with("MS4wLjAB") && raw.len() >= 24;
    if is_sec {
        return Ok(Target {
            kind: TargetKind::User,
            value: raw.to_string(),
            candidates: vec![],
        });
    }
    if raw.len() >= 4 && raw.len() <= 20 && raw.chars().all(|c| c.is_ascii_digit()) {
        let (kind, candidates) = match raw.len() {
            19..=20 => (
                TargetKind::Video,
                vec![
                    TargetKind::Series,
                    TargetKind::Video,
                    TargetKind::Room,
                    TargetKind::Mix,
                ],
            ),
            4..=18 => (TargetKind::User, vec![]),
            _ => (TargetKind::User, vec![]),
        };
        return Ok(Target {
            kind,
            value: raw.to_string(),
            candidates,
        });
    }
    Ok(Target {
        kind: TargetKind::Keyword,
        value: raw.to_string(),
        candidates: vec![],
    })
}

fn str_of(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
        Value::String(s) => s == "1" || s.eq_ignore_ascii_case("true"),
        _ => false,
    }
}

fn num_of(v: &Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .unwrap_or(0)
}

fn url0(v: &Value) -> String {
    v.get(0)
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string()
}

fn author(a: &Value) -> Value {
    json!({
        "uid": str_of(&a["uid"]), "sec_uid": str_of(&a["sec_uid"]),
        "unique_id": str_of(&a["unique_id"]),
        "nickname": str_of(&a["nickname"]), "avatar": url0(&a["avatar_thumb"]["url_list"]),
        "signature": str_of(&a["signature"]), "follower_count": num_of(&a["follower_count"]),
    })
}

fn item(raw: &Value) -> Value {
    let video = &raw["video"];
    // 图集/图片型的 play_addr 指向 `/aweme/v1/play/music/`（音轨），别当视频播。
    let urls: Vec<&str> = video["play_addr"]["url_list"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|u| u.as_str())
        .filter(|u| !u.contains("/play/music/"))
        .collect();
    let imgs = raw["images"].as_array().map(|a| a.len()).unwrap_or(0);
    let cover = if imgs > 0 {
        let u = {
            let d = url0(&raw["images"][0]["download_url_list"]);
            if d.is_empty() {
                url0(&raw["images"][0]["url_list"])
            } else {
                d
            }
        };
        if u.is_empty() {
            url0(&video["cover"]["url_list"])
        } else {
            u
        }
    } else {
        url0(&video["cover"]["url_list"])
    };
    let gallery = imgs > 0 || urls.is_empty();
    json!({
        "aweme_id": str_of(&raw["aweme_id"]), "desc": str_of(&raw["desc"]),
        "create_time": num_of(&raw["create_time"]), "duration_ms": num_of(&video["duration"]),
        "author": author(&raw["author"]),
        "statistics": raw["statistics"].clone(),
        "kind": if gallery { "gallery" } else { "video" },
        "cover": cover,
        "share_url": str_of(&raw["share_url"]), "raw": raw,
    })
}

fn page_of(list: &Value, has_more: bool, cursor: Option<&Value>) -> Value {
    let items = list
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(item)
        .collect();
    page(items, has_more, cursor)
}

impl Api {
    pub fn new(res: std::path::PathBuf) -> Result<Self> {
        Self::new_with(res, Backend::Native)
    }

    pub fn new_with(res: std::path::PathBuf, backend: Backend) -> Result<Self> {
        Self::new_stored(res, backend, crate::pool::default_path(), 16)
    }

    pub fn new_stored(
        res: std::path::PathBuf,
        backend: Backend,
        pool_path: std::path::PathBuf,
        target: usize,
    ) -> Result<Self> {
        if !pool_path.exists() {
            crate::pool::cleanup_legacy_state();
        }
        std::env::set_var("DYSIGN_POOL", &pool_path);
        for (key, name) in [("MEDUSA_ALLOC", "alloc.tsv"), ("MEDUSA_JNI", "jni.tsv")] {
            let p = res.join(name);
            if p.is_file() {
                std::env::set_var(key, p);
            }
        }
        let signer = Signer::spawn(res.clone(), backend)?;
        let http = Http::new(signer)?;
        Ok(Self {
            pool: Mutex::new(Pool::load(pool_path.clone(), target)),
            pool_cv: Condvar::new(),
            resources: res,
            maintainer: AtomicBool::new(false),
            cdn: AtomicBool::new(false),
            http,
            first_launch: (now_secs().saturating_sub(3600)).to_string(),
            details: Mutex::new(HashMap::new()),
            filtered: Mutex::new(HashMap::new()),
            series_cache: Mutex::new(HashMap::new()),
            play_stats: Mutex::new(PlayStats::default()),
            last_identity: Mutex::new(None),
            buckets: Mutex::new(HashMap::new()),
        })
    }

    pub fn sign_backend(&self) -> Backend {
        self.http.sign_info().backend
    }

    pub fn sign_info(&self) -> SignInfo {
        self.http.sign_info()
    }

    /// 设置出口代理列表（一行一个）与开关，立即作用于后续请求。
    pub fn set_proxies(&self, text: &str, enabled: bool) {
        self.http.proxies().set(text, enabled);
    }

    /// CDN（视频/图集字节）是否也走代理；默认否。
    pub fn set_cdn_via_proxy(&self, on: bool) {
        self.cdn.store(on, Ordering::Relaxed);
    }

    /// 当前给 CDN 用的出口；未启用直连或列表为空时为 None。
    pub fn cdn_proxy(&self) -> Option<String> {
        if !self.cdn.load(Ordering::Relaxed) {
            return None;
        }
        self.http.proxies().pick_for("cdn")
    }

    /// 当前代理配置原文、开关、CDN 开关。
    pub fn proxy_config(&self) -> (String, bool, bool) {
        (
            self.http.proxies().text(),
            self.http.proxies().enabled(),
            self.cdn.load(Ordering::Relaxed),
        )
    }

    pub fn proxy_status(&self) -> (usize, bool) {
        (self.http.proxies().len(), self.http.proxies().enabled())
    }

    /// 逐条实测出口：能拿到 2xx/3xx 即算通，返回 (地址, 是否可用, 毫秒)。
    pub fn proxy_test(text: &str) -> Vec<(String, bool, u64)> {
        let list = crate::proxy::parse(text);
        let mut out = Vec::with_capacity(list.len());
        // 分批并发，避免用户粘几百上千条时一次开满线程。
        for batch in list.chunks(16) {
            std::thread::scope(|s| {
                let handles: Vec<_> = batch
                    .iter()
                    .map(|url| {
                        s.spawn(move || {
                            let at = std::time::Instant::now();
                            let ok = crate::http::client_with(Some(url))
                                .ok()
                                .and_then(|c| {
                                    c.get("https://www.baidu.com/")
                                        .timeout(std::time::Duration::from_secs(8))
                                        .send()
                                        .ok()
                                })
                                .is_some_and(|r| r.status().is_success());
                            (url.clone(), ok, at.elapsed().as_millis() as u64)
                        })
                    })
                    .collect();
                for h in handles {
                    if let Ok(v) = h.join() {
                        out.push(v);
                    }
                }
            });
        }
        out
    }

    /// 切签名实现，立即作用于后续请求。
    pub fn set_sign_backend(&self, backend: Backend) {
        self.http.set_sign_backend(backend);
    }

    pub fn last_request_id(&self) -> Option<String> {
        self.http.last_request_id()
    }

    pub fn last_upstream_url(&self) -> Option<String> {
        self.http.last_url()
    }

    pub fn cool_identity(&self, device_id: &str, secs: u64) {
        self.pool().cool(device_id, secs);
    }

    pub fn pool_available(&self) -> usize {
        self.pool().usable_count()
    }

    pub fn identity_status(&self) -> PoolStatus {
        self.pool().status()
    }

    pub fn identity_set_target(&self, target: usize) {
        self.pool().set_target(target);
        self.pool_cv.notify_all();
    }

    pub fn identity_pause(&self, paused: bool) {
        self.pool().set_paused(paused);
        self.pool_cv.notify_all();
    }

    pub fn identity_clear(&self) {
        self.pool().clear();
        self.pool_cv.notify_all();
    }

    /// 立即唤醒补池线程；目标数由设置决定。
    pub fn identity_prepare(&self) {
        self.pool().set_paused(false);
        self.pool_cv.notify_all();
    }

    pub fn start_pool_maintainer(self: &Arc<Self>) {
        if self.maintainer.swap(true, Ordering::SeqCst) {
            return;
        }
        let api = self.clone();
        std::thread::Builder::new()
            .name("identity-pool".into())
            .spawn(move || {
                let mut last_try = Instant::now() - Duration::from_secs(10);
                let mut failures = 0u32;
                loop {
                    let claimed = {
                        let mut pool = api.pool();
                        if !pool.needs_identity() {
                            let _ = api
                                .pool_cv
                                .wait_timeout(pool, Duration::from_secs(15))
                                .unwrap_or_else(|e| e.into_inner());
                            continue;
                        }
                        pool.begin_create()
                    };
                    if !claimed {
                        continue;
                    }
                    let gap = if failures == 0 {
                        Duration::from_secs(2)
                    } else {
                        Duration::from_secs(5 * (1 << failures.min(4)))
                    };
                    if let Some(left) = gap.checked_sub(last_try.elapsed()) {
                        std::thread::sleep(left);
                    }
                    last_try = Instant::now();
                    let checked = register::register(&api.http, &api.resources)
                        .and_then(|id| api.verify_candidate(&id).map(|_| id));
                    let mut pool = api.pool();
                    match checked {
                        Ok(id) => {
                            eprintln!("[identity] 已创建并验收 {}", id.device_id);
                            pool.finish_create(id);
                            failures = 0;
                        }
                        Err(e) => {
                            failures += 1;
                            eprintln!("[identity] 创建失败：{e}");
                            pool.fail_create(e.to_string());
                            if failures >= 3 {
                                std::thread::sleep(Duration::from_secs(
                                    30 * failures.min(4) as u64,
                                ));
                            }
                        }
                    }
                    drop(pool);
                    api.pool_cv.notify_all();
                }
            })
            .ok();
    }

    /// 新号入池前的硬验收：feed 有条目，且该条目详情可读。
    fn verify_candidate(&self, id: &Identity) -> Result<()> {
        let fl = &id.first_launch;
        let params = feed_params(id, fl, 6);
        let body = feed_body();
        let raw = self.http.request(
            "POST",
            APP_HOST,
            "/aweme/v2/feed/",
            &params,
            Some(body.as_bytes()),
            "application/json; charset=UTF-8",
            "",
            None,
            id,
            fl,
            1,
        )?;
        let top = crate::pb::decode(&raw);
        if top
            .iter()
            .find(|(f, _)| *f == 1)
            .and_then(|(_, v)| crate::pb::as_u64(v))
            != Some(0)
        {
            return Err(Error::msg("新号 feed 状态异常"));
        }
        let first = top
            .iter()
            .filter(|(f, _)| *f == 5)
            .find_map(|(_, v)| match v {
                crate::pb::Value::Message(m) => crate::pb::one(m, 1).map(crate::pb::text),
                _ => None,
            })
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Error::msg("新号 feed 为空"))?;
        std::thread::sleep(Duration::from_secs(1));
        let mut p = media(id, fl);
        for (k, v) in [
            ("aweme_id", first.as_str()),
            ("origin_type", "forward_continue_play"),
            ("request_source", "230"),
            ("is_story", "0"),
            ("location_permission", "1"),
            ("aweme_type", "1"),
            ("recommend_collect_feedback", "-1"),
            ("from_push", "0"),
            ("item_level", "0"),
            ("group_id", ""),
            ("meta_params", ""),
            ("video_quality", "28"),
        ] {
            p.insert(k.into(), v.into());
        }
        let raw = self.http.request(
            "GET",
            API_HOST,
            "/aweme/v1/aweme/detail/",
            &p,
            None,
            "",
            "",
            None,
            id,
            fl,
            1,
        )?;
        let v: Value = json_of(&raw, &[])?;
        if v.get("aweme_detail").is_none_or(Value::is_null) {
            return Err(Error::msg("新号详情为空"));
        }
        Ok(())
    }

    /// 最近一次租用的身份，供任务层记录结果。
    pub fn last_identity(&self) -> Option<String> {
        self.last_identity
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// 预热：每条签名线程先用真实 feed 请求打穿热路径，再给常用接口预生成签名。
    pub fn warm(&self) -> Result<()> {
        let id = self.pool().lease()?;
        let fl = self.first_launch.clone();
        let body = feed_body();
        let p = feed_params(&id, &fl, 20);
        self.http
            .presign_each(&id, APP_HOST, "/aweme/v2/feed/", &p, body.as_bytes());
        for cursor in 0..30 {
            let mut p = common(&id, "664226", &fl);
            p.insert("count".into(), "6".into());
            p.insert("feed_type".into(), "1".into());
            p.insert("cursor".into(), cursor.to_string());
            let _ = self.http.presign(&id, LIVE_HOST, "/webcast/feed/", &p, &[]);
        }
        let mut p = common(&id, "664226", &fl);
        for (k, v) in [
            ("detail_list", "1"),
            ("source", "4"),
            ("disable_ad", "0"),
            ("main_billboard_count", "50"),
            ("is_vcd", "0"),
            ("request_tag_from", "h5"),
        ] {
            p.insert(k.into(), v.into());
        }
        let _ = self
            .http
            .presign(&id, API_HOST, "/aweme/v1/hot/search/list/", &p, &[]);
        Ok(())
    }

    /// 启动自检：用一条真实 feed 请求签一次，只验证签名实现本身，不发网络。
    pub fn self_check(&self) -> Result<()> {
        let id = self.pool().lease().unwrap_or_else(|_| Identity {
            device_id: "3524403579608627".into(),
            iid: "3524403579612723".into(),
            brand: "xiaomi".into(),
            model: "2201123C".into(),
            resolution: "1080x2400".into(),
            dpi: "419".into(),
            ..Identity::default()
        });
        let fl = if id.first_launch.is_empty() {
            self.first_launch.clone()
        } else {
            id.first_launch.clone()
        };
        let p = feed_params(&id, &fl, 1);
        let body = feed_body();
        self.http
            .presign(&id, APP_HOST, "/aweme/v2/feed/", &p, body.as_bytes())
    }

    fn pool(&self) -> MutexGuard<'_, Pool> {
        self.pool.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn with_identity<T>(&self, f: impl FnMut(&Identity, &str) -> Result<T>) -> Result<T> {
        self.with_identity_opt(true, f)
    }

    fn with_identity_opt<T>(
        &self,
        spaced: bool,
        mut f: impl FnMut(&Identity, &str) -> Result<T>,
    ) -> Result<T> {
        let mut last = Error::msg("身份池为空");
        for attempt in 0..6 {
            let id = match self.lease_with_token(spaced) {
                Ok(id) => id,
                // 池子先耗尽时不要用“已耗尽”覆盖真实的接口错误。
                Err(_) if last.message != "身份池为空" => return Err(last),
                Err(e) => return Err(e),
            };
            *self.last_identity.lock().unwrap_or_else(|e| e.into_inner()) =
                Some(id.device_id.clone());
            let out = f(&id, &self.first_launch);
            self.pool().release(&id.device_id);
            self.pool_cv.notify_all();
            match out {
                Ok(v) => {
                    self.pool().success(&id.device_id);
                    return Ok(v);
                }
                Err(e) => {
                    if e.status == 2483 {
                        self.pool().retire(&id.device_id);
                    } else if is_warming(&e) {
                        // 新设备风控不是坏号：只降优先级，仍保留给其他接口用。
                        self.pool().mark_miss(&id.device_id);
                    } else if e.retryable()
                        && (e.status == 403
                            || e.status == 429
                            || e.message.contains("2484")
                            || e.message.contains("antispam"))
                    {
                        self.pool().fail(&id.device_id);
                    } else {
                        self.pool().drop_current();
                    }
                    if !e.retryable() {
                        return Err(e);
                    }
                    last = e;
                    if attempt < 5 {
                        std::thread::sleep(Duration::from_millis(250 * (attempt as u64 + 1)));
                    }
                }
            }
        }
        Err(last)
    }

    #[allow(dead_code)]
    fn get_json(&self, host: &str, path: &str, params: &BTreeMap<String, String>) -> Result<Value> {
        self.with_identity(|id, fl| {
            let cookie = String::new();
            let b = self.http.request(
                "GET", host, path, params, None, "", &cookie, None, id, fl, 1,
            )?;
            let v: Value = json_of(&b, &[])?;
            check_status(&v)?;
            Ok(v)
        })
    }

    #[allow(dead_code)]
    fn post_json(
        &self,
        host: &str,
        path: &str,
        params: &BTreeMap<String, String>,
        body: &[u8],
        ct: &str,
    ) -> Result<Value> {
        self.with_identity(|id, fl| {
            let b = self.http.request(
                "POST",
                host,
                path,
                params,
                Some(body),
                ct,
                "",
                None,
                id,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &[])?;
            check_status(&v)?;
            Ok(v)
        })
    }

    pub fn call(&self, path: &str, query: &Value) -> Result<Value> {
        let prev = CURRENT_ENDPOINT.with(|e| {
            let mut e = e.borrow_mut();
            let prev = e.clone();
            *e = path.to_string();
            prev
        });
        let out = self.call_inner(path, query);
        CURRENT_ENDPOINT.with(|e| *e.borrow_mut() = prev);
        let v = out?;
        self.remember_details(&v);
        Ok(v)
    }

    fn lease_with_token(&self, spaced: bool) -> Result<Identity> {
        let endpoint = CURRENT_ENDPOINT.with(|e| e.borrow().clone());
        let started = Instant::now();
        loop {
            let leased = {
                let mut pool = self.pool();
                let allow = |id: &str| {
                    let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
                    let key = (id.to_string(), endpoint.clone());
                    buckets.entry(key).or_insert(Bucket {
                        tokens: token_rate(&endpoint).1,
                        at: Instant::now(),
                    });
                    bucket_ready(
                        buckets
                            .get_mut(&(id.to_string(), endpoint.clone()))
                            .expect("bucket"),
                        &endpoint,
                    )
                };
                pool.lease_filtered(spaced, &allow)
            };
            match leased {
                Ok(id) => {
                    let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
                    let key = (id.device_id.clone(), endpoint.clone());
                    let b = buckets.entry(key).or_insert(Bucket {
                        tokens: token_rate(&endpoint).1,
                        at: Instant::now(),
                    });
                    if take_token(b, &endpoint) {
                        return Ok(id);
                    }
                    drop(buckets);
                    std::thread::sleep(Duration::from_millis(80));
                }
                Err(e) => {
                    if started.elapsed() >= Duration::from_secs(60) {
                        return Err(Error::kinded(
                            "identity_wait",
                            format!("身份正在准备，请稍后再试（{}）", e.message),
                        ));
                    }
                    let guard = self.pool();
                    let _ = self
                        .pool_cv
                        .wait_timeout(guard, Duration::from_millis(200))
                        .unwrap_or_else(|x| x.into_inner());
                }
            }
        }
    }

    fn remember_details(&self, v: &Value) {
        let put = |x: &Value| {
            let id = x
                .get("aweme_id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if id.is_empty() {
                return;
            }
            // 评论也带 aweme_id，必须有 video/images 才算作品详情，避免污染详情缓存。
            if x.get("video").is_none() && x.get("images").is_none() {
                return;
            }
            if let Ok(mut m) = self.details.lock() {
                if m.len() > 300 {
                    m.clear();
                }
                m.insert(id.to_string(), (Instant::now(), x.clone()));
            }
        };
        if let Some(raw) = v.get("raw") {
            put(raw);
        } else if v.get("aweme_id").is_some() {
            put(v);
        }
        for key in ["items", "aweme_list"] {
            if let Some(a) = v.get(key).and_then(Value::as_array) {
                for x in a {
                    if let Some(raw) = x.get("raw") {
                        put(raw);
                    } else {
                        put(x);
                    }
                }
            }
        }
    }

    fn call_inner(&self, path: &str, query: &Value) -> Result<Value> {
        let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        match parts.as_slice() {
            ["health"] => Ok(json!({"ok": true, "sign": self.sign_info().json()})),
            ["version"] => Ok(json!({"backend": "dycore", "version": env!("CARGO_PKG_VERSION")})),
            ["probe"] => {
                let s = self.play_stats.lock().unwrap_or_else(|e| e.into_inner());
                let availability =
                    (s.requests > 0).then(|| (s.requests - s.failures) as f64 / s.requests as f64);
                let fallback_hit_rate =
                    (s.fallback_used > 0).then(|| s.fallback_hits as f64 / s.fallback_used as f64);
                Ok(json!({
                    "ok": true,
                    "backend": "dycore",
                    "sign": self.sign_info().json(),
                    "play": {
                        "requests": s.requests,
                        "primary_hits": s.primary_hits,
                        "fallback_used": s.fallback_used,
                        "fallback_hits": s.fallback_hits,
                        "failures": s.failures,
                        "availability": availability,
                        "fallback_hit_rate": fallback_hit_rate,
                    }
                }))
            }
            ["caps"] => Ok(json!([
                {"name": "feed", "route": "/feed", "help": "推荐流（protobuf 解出）"},
                {"name": "video", "route": "/video/{id}", "help": "作品详情"},
                {"name": "comments", "route": "/video/{id}/comments", "help": "一级评论"},
                {"name": "replies", "route": "/video/{id}/replies", "help": "楼中楼"},
                {"name": "video.tags", "route": "/video/{id}/tags", "help": "视频标签"},
                {"name": "search", "route": "/search", "help": "综合搜索"},
                {"name": "search.series", "route": "/search/series", "help": "短剧搜索"},
                {"name": "search.users", "route": "/search/users", "help": "用户搜索（真粉丝数）"},
                {"name": "search.sug", "route": "/search/sug", "help": "搜索联想词"},
                {"name": "search.challenges", "route": "/search/challenges", "help": "话题搜索"},
                {"name": "user", "route": "/user/{sec_uid}", "help": "用户资料"},
                {"name": "user.posts", "route": "/user/{sec_uid}/posts", "help": "主页作品"},
                {"name": "user.live", "route": "/user/{uid}/live", "help": "主播当前直播间"},
                {"name": "hot", "route": "/hot", "help": "热搜榜"},
                {"name": "hot.app", "route": "/hot/app", "help": "APP 热搜榜"},
                {"name": "challenge", "route": "/challenge/{name}", "help": "话题详情"},
                {"name": "series.wall", "route": "/series/wall", "help": "短剧墙"},
                {"name": "series.tab", "route": "/series/tab", "help": "短剧 tab"},
                {"name": "series.episodes", "route": "/series/{id}/episodes", "help": "分集"},
                {"name": "series.paid", "route": "/series/{id}/paid", "help": "付费状态"},
                {"name": "live.feed", "route": "/live/feed", "help": "直播推荐（每批 6 个，cursor 递增）"},
                {"name": "live.probe", "route": "/live/probe/{path}", "help": "直播原始接口探针"},
                {"name": "live.resolve", "route": "/live/resolve", "help": "房间与拉流地址"},
                {"name": "live.pull", "route": "/live/{room}/pull", "help": "直播拉流地址"},
                {"name": "video.gears", "route": "/video/{id}/gears", "help": "可用画质档位"},
                {"name": "video.music", "route": "/video/{id}/music", "help": "原声地址"},
                {"name": "video.cover", "route": "/video/{id}/cover", "help": "封面地址"},
                {"name": "download", "route": "/video/{id}/download", "help": "直链与字节数"},
            ])),
            ["feed"] => self.feed(num_of(&query["count"]).clamp(1, 20) as usize),
            ["hot"] => self.hot(),
            ["hot", "app"] => self.hot(),
            ["video", id] => self.detail(id).map(|d| item(&d)),
            ["video", id, "play"] => {
                let gear = q(query, "gear");
                let h264_only = q(query, "hevc") != "1";
                self.play_with_codecs(id, true, Some(&gear), h264_only)
                    .map(|p| {
                        json!({"kind": "video", "url": p.url, "gear": p.gear,
                        "width": p.width, "height": p.height, "bit_rate": p.bit_rate,
                        "key": p.key, "key_err": p.key_err})
                    })
            }
            ["video", id, "gears"] => self.gears(id),
            ["video", id, "key"] => self.cenc_key(id).map(|k| json!({"key": k})),
            ["video", id, "music"] => self.music(id),
            ["video", id, "cover"] => self.cover(id),
            ["video", id, "download"] => {
                let gear = q(query, "gear");
                self.play(id, false, Some(&gear)).map(|p| {
                    json!({
                        "url": p.url,
                        "expect_bytes": p.size,
                        "ext": "mp4",
                        "bit_rate": p.bit_rate,
                        "width": p.width,
                        "height": p.height,
                        "gear": p.gear,
                    })
                })
            }
            ["video", id, "tags"] => self.video_tags(id),
            ["video", id, "comments"] => {
                let cursor = q(query, "cursor");
                let count = count_of(query, "count", 20).clamp(1, 50);
                let mut seen: HashSet<String> = q(query, "seen")
                    .split(',')
                    .map(str::trim)
                    .filter(|x| !x.is_empty())
                    .map(str::to_string)
                    .collect();
                if !cursor.is_empty() {
                    self.comments(id, cursor.parse().unwrap_or(0), count, &seen)
                } else {
                    // 兼容旧的 pages 参数：逐页走到目标页，但返回时只保留最后一页。
                    let pages = num_of(&query["pages"]).max(1);
                    let mut next_cursor = 0i64;
                    for _ in 1..pages {
                        let p = self.comments(id, next_cursor, count, &seen)?;
                        for item in p["items"].as_array().cloned().unwrap_or_default() {
                            let cid = str_of(&item["cid"]);
                            if !cid.is_empty() {
                                seen.insert(cid);
                            }
                        }
                        if !truthy(&p["meta"]["has_more"]) {
                            return Ok(p);
                        }
                        next_cursor = num_of(&p["meta"]["cursor"]);
                    }
                    self.comments(id, next_cursor, count, &seen)
                }
            }
            ["video", id, "replies"] => self.replies(
                id,
                &q(query, "comment"),
                q(query, "cursor").parse().unwrap_or(0),
                count_of(query, "count", 20).clamp(1, 50),
            ),
            ["video", id, "danmaku"] => self.danmaku(id, num_of(&query["cursor"])),
            ["search"] => self.search(
                &q(query, "target"),
                q(query, "cursor").parse().unwrap_or(0),
                &q(query, "search_id"),
                &q(query, "search_request_id"),
            ),
            ["search", "item"] => self.search_item_doc(
                &q(query, "target"),
                q(query, "cursor").parse().unwrap_or(0),
                &q(query, "search_id"),
            ),
            ["search", "users"] => self.search_users(
                &q(query, "target"),
                q(query, "cursor").parse().unwrap_or(0),
                &q(query, "search_id"),
            ),
            ["search", "challenges"] => self.challenges(
                &q(query, "target"),
                q(query, "cursor").parse().unwrap_or(0),
                &q(query, "search_id"),
            ),
            ["search", "series"] => self.search_series(&q(query, "target")),
            ["search", "sug"] => self.search_sug(&q(query, "target")),
            ["user", uid] => self.user(uid),
            ["user", uid, "posts"] => self.user_posts(uid, q(query, "cursor").parse().unwrap_or(0)),
            ["user", uid, "mixes"] => self.user_mixes(uid, q(query, "cursor").parse().unwrap_or(0)),
            ["user", uid, "series"] => {
                self.user_series(uid, q(query, "cursor").parse().unwrap_or(0))
            }
            ["user", uid, "live"] => self.user_live(uid),
            ["challenge", name] => self.challenge(&decode(name)),
            ["challenge", name, "videos"] => self.challenge_videos(
                &decode(name),
                &q(query, "sort"),
                q(query, "cursor").parse().unwrap_or(0),
            ),
            ["series", "tab"] | ["series", "wall"] => {
                let cursor = q(query, "cursor");
                let pages = num_of(&query["pages"]).max(1);
                let count = if cursor.is_empty() {
                    (10 * pages).clamp(1, 30) as usize
                } else {
                    count_of(query, "count", 10).clamp(1, 30)
                };
                self.series_wall(cursor.parse().unwrap_or(0), count)
            }
            ["series", id, "episodes"] => {
                self.series_episodes(id, q(query, "cursor").parse().unwrap_or(0))
            }
            ["series", id, "paid"] => self.series_paid(id),
            ["series", id] => self.series(id),
            ["mix", id, "videos"] => self.mix_videos(id, q(query, "cursor").parse().unwrap_or(0)),
            ["live", "feed"] => self.live_feed(
                num_of(&query["count"]).clamp(1, 30) as usize,
                &q(query, "cursor"),
            ),
            ["live", "probe", path @ ..] => self.live_probe(&path.join("/"), query),
            ["live", "resolve"] => self.live_resolve(&q(query, "target")),
            ["live", room, "pull"] => self.live_pull(room),
            ["live", room, "messages"] => {
                self.live_messages(room, &q(query, "cursor"), &q(query, "internal_ext"))
            }
            ["resolve"] => self.resolve(&q(query, "target")),
            _ => Err(Error::new(404, format!("未知路由 {path}"))),
        }
    }

    fn feed(&self, count: usize) -> Result<Value> {
        self.with_identity(|id, fl| {
            let p = feed_params(id, fl, count);
            let body = feed_body();
            let t0 = std::time::Instant::now();
            let b = self.http.request(
                "POST",
                APP_HOST,
                "/aweme/v2/feed/",
                &p,
                Some(body.as_bytes()),
                "application/json; charset=UTF-8",
                "",
                None,
                id,
                fl,
                1,
            )?;
            if std::env::var("TIM").is_ok() {
                eprintln!("[tim] http {}ms", t0.elapsed().as_millis());
            }
            if std::env::var("DUMP_FEED").is_ok() {
                let _ = std::fs::write("/tmp/feed.bin", &b);
                eprintln!("[feed] {} 字节 {:02x?}", b.len(), &b[..b.len().min(16)]);
            }
            let t1 = std::time::Instant::now();
            let top = crate::pb::decode(&b);
            if std::env::var("TIM").is_ok() {
                eprintln!("[tim] pb {}ms", t1.elapsed().as_millis());
            }
            let status = top
                .iter()
                .find(|(f, _)| *f == 1)
                .and_then(|(_, v)| crate::pb::as_u64(v));
            if status != Some(0) {
                return Err(Error::msg(format!("feed status={status:?}")));
            }
            let mut items = Vec::new();
            for (f, v) in top.iter().filter(|(f, _)| *f == 5) {
                let _ = f;
                if let crate::pb::Value::Message(m) = v {
                    items.push(feed_item(m));
                }
            }
            Ok(page(items, true, None))
        })
    }

    fn detail(&self, id: &str) -> Result<Value> {
        if let Ok(m) = self.details.lock() {
            if let Some((at, v)) = m.get(id) {
                let rich = v["video"]["bit_rate"]
                    .as_array()
                    .is_some_and(|a| !a.is_empty())
                    || v["images"].as_array().is_some_and(|a| !a.is_empty());
                if rich && at.elapsed() < Duration::from_secs(300) {
                    return Ok(v.clone());
                }
            }
        }
        if let Ok(m) = self.filtered.lock() {
            if let Some((at, msg)) = m.get(id) {
                if at.elapsed() < Duration::from_secs(300) {
                    return Err(Error::new(404, msg.clone()));
                }
            }
        }
        let mut v = Value::Null;
        for _ in 0..8 {
            let got = self.with_identity(|ident, fl| {
                let mut p = media(ident, fl);
                for (k, val) in [
                    ("aweme_id", id),
                    ("origin_type", "forward_continue_play"),
                    ("request_source", "230"),
                    ("is_story", "0"),
                    ("location_permission", "1"),
                    ("aweme_type", "1"),
                    ("recommend_collect_feedback", "-1"),
                    ("from_push", "0"),
                    ("item_level", "0"),
                    ("group_id", ""),
                    ("meta_params", ""),
                    ("video_quality", "28"),
                ] {
                    p.insert(k.into(), val.into());
                }
                let b = self.http.request(
                    "GET",
                    API_HOST,
                    "/aweme/v1/aweme/detail/",
                    &p,
                    None,
                    "",
                    "",
                    None,
                    ident,
                    fl,
                    1,
                )?;
                let v: Value = json_of(&b, &[])?;
                check_status(&v)?;
                match v.get("aweme_detail") {
                    Some(d) if !d.is_null() => Ok(d.clone()),
                    _ => Err(match v.get("filter_detail") {
                        Some(f) if !str_of(&f["filter_reason"]).is_empty() => {
                            let e = filter_error(f);
                            if let Ok(mut m) = self.filtered.lock() {
                                if m.len() > 500 {
                                    m.clear();
                                }
                                m.insert(id.to_string(), (Instant::now(), e.message.clone()));
                            }
                            e
                        }
                        _ => Error::new(404, "作品不存在或已删除"),
                    }),
                }
            })?;
            let rich = got["video"]["bit_rate"]
                .as_array()
                .is_some_and(|a| !a.is_empty())
                || got["images"].as_array().is_some_and(|a| !a.is_empty());
            v = got;
            if rich {
                break;
            }
        }
        if let Ok(mut m) = self.filtered.lock() {
            m.remove(id);
        }
        if let Ok(mut m) = self.details.lock() {
            if m.len() > 200 {
                m.clear();
            }
            m.insert(id.to_string(), (Instant::now(), v.clone()));
        }
        Ok(v)
    }

    fn play(&self, id: &str, preview: bool, gear: Option<&str>) -> Result<PlayInfo> {
        self.play_with_codecs(id, preview, gear, false)
    }

    fn play_with_codecs(
        &self,
        id: &str,
        preview: bool,
        gear: Option<&str>,
        h264_only: bool,
    ) -> Result<PlayInfo> {
        let d = self.detail(id)?;
        self.bump_stats(|s| s.requests += 1);
        let video = &d["video"];
        let selected = gear.filter(|g| !g.is_empty());
        let list = play_gears(video, preview, h264_only, selected);
        for g in &list {
            if let Ok((url, size)) = self.resolve_location(&g.raw, !preview) {
                self.bump_stats(|s| s.primary_hits += 1);
                let mut info = PlayInfo {
                    url,
                    size: size.max(g.size),
                    bit_rate: g.bit_rate,
                    width: g.width,
                    height: g.height,
                    gear: g.name.clone(),
                    key: None,
                    key_err: None,
                };
                self.attach_cenc_key(id, &mut info);
                return Ok(info);
            }
        }
        if preview && selected.is_some() {
            self.bump_stats(|s| s.failures += 1);
            return Err(Error::new(404, "指定播放档位不可用"));
        }
        let fallback = play_fallbacks(video, h264_only);
        if !fallback.is_empty() {
            self.bump_stats(|s| s.fallback_used += 1);
        }
        for entry in fallback
            .iter()
            .filter(|u| u.contains("/aweme/v1/play/") && !u.contains("/play/music/"))
        {
            if let Ok((url, size)) = self.resolve_play_entry(entry, !preview) {
                self.bump_stats(|s| s.fallback_hits += 1);
                let known = num_of(&video["play_addr"]["data_size"]).max(0) as u64;
                let mut info = PlayInfo {
                    url,
                    size: size.max(known),
                    bit_rate: num_of(&video["bit_rate"]).max(0),
                    width: num_of(&video["width"]).max(0),
                    height: num_of(&video["height"]).max(0),
                    gear: String::new(),
                    key: None,
                    key_err: None,
                };
                self.attach_cenc_key(id, &mut info);
                return Ok(info);
            }
        }
        for raw in fallback
            .iter()
            .filter(|u| !u.contains("/play/music/"))
            .chain(fallback.first())
        {
            if let Ok((url, size)) = self.resolve_location(raw, !preview) {
                self.bump_stats(|s| s.fallback_hits += 1);
                let known = num_of(&video["play_addr"]["data_size"]).max(0) as u64;
                let mut info = PlayInfo {
                    url,
                    size: size.max(known),
                    bit_rate: num_of(&video["bit_rate"]).max(0),
                    width: num_of(&video["width"]).max(0),
                    height: num_of(&video["height"]).max(0),
                    gear: String::new(),
                    key: None,
                    key_err: None,
                };
                self.attach_cenc_key(id, &mut info);
                return Ok(info);
            }
        }
        self.bump_stats(|s| s.failures += 1);
        Err(Error::msg(format!(
            "没有播放地址 kind={} bit_rate={} urls={} gear={}",
            str_of(&d["kind"]),
            video["bit_rate"].as_array().map(|a| a.len()).unwrap_or(0),
            fallback.len(),
            selected.unwrap_or("")
        )))
    }

    /// 加密集（`er=1`）的 CENC 内容密钥：`series/aweme` 的 aK 经 spade 纯算法解出。
    /// `aid=1128` 是匿名短剧行表面，只回 aK，不回明文；key 与具体 CDN 地址无关。
    fn cenc_key(&self, id: &str) -> Result<String> {
        let d = self.detail(id)?;
        let sid = str_of(&d["series_info"]["series_id"]);
        let index = num_of(&d["series_play_info"]["series_aweme_index"]);
        if sid.is_empty() || index <= 0 {
            return Err(Error::new(404, "作品不属于短剧，取不到播放密钥"));
        }
        let mut p: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in [
            ("series_id", sid.clone()),
            ("cursor", (index - 1).to_string()),
            ("count", "1".into()),
            ("pull_type", "2".into()),
            ("source", "playlet_detail".into()),
            ("version_name", "32.2.0".into()),
            ("device_platform", "android".into()),
            ("aid", "1128".into()),
        ] {
            p.insert(k.into(), v);
        }
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(4))
            .timeout(Duration::from_secs(15))
            .user_agent("com.ss.android.ugc.aweme/320200 (Linux; U; Android 13)")
            .build()?;
        let res = client
            .get(format!("https://{USER_HOST}/aweme/v1/series/aweme/"))
            .query(&p)
            .header("Accept", "*/*")
            .send()?;
        let v: Value = json_of(&res.bytes()?, &["aweme_list"])?;
        let aks = v["aweme_list"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|x| str_of(&x["aweme_id"]) == id)
            .map(|x| str_of(&x["video"]["play_addr"]["player_access_key"]))
            .find(|ak| !ak.is_empty())
            .ok_or_else(|| Error::new(404, "服务端没有下发播放密钥"))?;
        spade::decode(&aks).map_err(|e| Error::new(500, format!("播放密钥解析失败：{e}")))
    }

    /// 加密集才取 key；取不到时把原因留在响应里，便于从日志/探针定位。
    fn attach_cenc_key(&self, id: &str, info: &mut PlayInfo) {
        if !info.url.contains("er=1") {
            return;
        }
        match self.cenc_key(id) {
            Ok(k) => info.key = Some(k),
            Err(e) => info.key_err = Some(e.message),
        }
    }

    fn gears(&self, id: &str) -> Result<Value> {
        let d = self.detail(id)?;
        let mut out: Vec<Value> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        if let Some(list) = d["video"]["bit_rate"].as_array() {
            for b in list {
                let width = num_of(&b["play_addr"]["width"]).max(0);
                let height = num_of(&b["play_addr"]["height"]).max(0);
                let name = str_of(&b["gear_name"]);
                let quality = str_of(&b["quality_type"]);
                if b["play_addr"]["url_list"][0].as_str().is_none() {
                    continue;
                }
                let key = format!("{name}|{quality}|{width}x{height}|{}", is_h264(b));
                if !seen.insert(key) {
                    continue;
                }
                out.push(json!({
                    "name": name, "quality": quality, "width": width, "height": height,
                    "bit_rate": num_of(&b["bit_rate"]).max(0),
                    "size": num_of(&b["play_addr"]["data_size"]).max(num_of(&b["data_size"])).max(0),
                    "h264": is_h264(b),
                }));
            }
        }
        out.sort_by(|a, b| {
            let aa = num_of(&a["width"]) * num_of(&a["height"]);
            let bb = num_of(&b["width"]) * num_of(&b["height"]);
            bb.cmp(&aa)
        });
        Ok(json!({"items": out}))
    }

    fn music(&self, id: &str) -> Result<Value> {
        let d = self.detail(id)?;
        let m = &d["music"];
        let list: Vec<&str> = m["play_url"]["url_list"]
            .as_array()
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        for raw in list {
            if let Ok((url, size)) = self.resolve_location(raw, true) {
                let ext = url
                    .split('?')
                    .next()
                    .and_then(|u| u.rsplit('.').next())
                    .filter(|e| matches!(*e, "mp3" | "m4a" | "aac"))
                    .unwrap_or("mp3")
                    .to_string();
                return Ok(json!({
                    "url": url,
                    "expect_bytes": size,
                    "ext": ext,
                    "title": str_of(&m["title"]),
                    "author": str_of(&m["author"]),
                }));
            }
        }
        Err(Error::new(404, "作品没有可用的原声地址"))
    }

    fn cover(&self, id: &str) -> Result<Value> {
        let d = self.detail(id)?;
        let images = d["images"].as_array();
        let raw = images
            .and_then(|a| a.first())
            .map(|i| {
                let d = url0(&i["download_url_list"]);
                if d.is_empty() {
                    url0(&i["url_list"])
                } else {
                    d
                }
            })
            .filter(|u| !u.is_empty())
            .unwrap_or_else(|| url0(&d["video"]["cover"]["url_list"]));
        if raw.is_empty() {
            return Err(Error::new(404, "作品没有封面"));
        }
        let (url, size) = self.resolve_location(&raw, true)?;
        Ok(json!({"url": url, "expect_bytes": size, "ext": "jpg"}))
    }

    fn bump_stats(&self, f: impl FnOnce(&mut PlayStats)) {
        if let Ok(mut s) = self.play_stats.lock() {
            f(&mut s);
        }
    }

    fn resolve_play_entry(&self, raw: &str, head: bool) -> Result<(String, u64)> {
        let parsed = url::Url::parse(raw)?;
        let query: BTreeMap<String, String> = parsed.query_pairs().into_owned().collect();
        let video_id = query.get("video_id").cloned().unwrap_or_default();
        let file_id = query.get("file_id").cloned().unwrap_or_default();
        let sign = query.get("sign").cloned().unwrap_or_default();
        if video_id.is_empty() || file_id.is_empty() || sign.is_empty() {
            return Err(Error::new(400, "播放入口缺少 video_id/file_id/sign"));
        }
        let path = if parsed.path().contains("/play/music/") {
            "/aweme/v1/play/music/"
        } else {
            "/aweme/v1/play/"
        };
        self.with_identity(|ident, fl| {
            let mut p = media(ident, fl);
            p.remove("device_id");
            p.remove("iid");
            for (k, v) in [
                ("video_id", video_id.clone()),
                ("line", "0".into()),
                ("is_play_url", "1".into()),
                ("source", "PackSourceEnum_FEED".into()),
                ("md", "1".into()),
                ("file_id", file_id.clone()),
                ("sign", sign.clone()),
            ] {
                p.insert(k.into(), v);
            }
            let mut url = format!("https://{API_HOST}{path}?");
            url.push_str(
                &url::form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(&p)
                    .finish(),
            );
            let client = reqwest::blocking::Client::builder()
                .redirect(reqwest::redirect::Policy::limited(8))
                .timeout(Duration::from_secs(20))
                .user_agent("ttplayer(version:2.10.202.161-dy,appId:6383,os:Android)")
                .build()?;
            let res = client
                .get(&url)
                .header("Accept", "*/*")
                .header("Referer", "https://www.douyin.com/")
                .header("X-ReqType", "play")
                .send()?;
            if !res.status().is_success() && !res.status().is_redirection() {
                return Err(Error::new(res.status().as_u16(), "播放入口失败"));
            }
            let final_url = res.url().to_string();
            let size = if head {
                client
                    .head(&final_url)
                    .header("Referer", "https://www.douyin.com/")
                    .send()
                    .ok()
                    .and_then(|r| r.content_length())
                    .unwrap_or(0)
            } else {
                0
            };
            Ok((final_url, size))
        })
    }

    fn resolve_location(&self, raw: &str, head: bool) -> Result<(String, u64)> {
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(8))
            .timeout(std::time::Duration::from_secs(20))
            .user_agent("ttplayer(version:2.10.202.161-dy,appId:6383,os:Android)")
            .build()?;
        let res = client
            .get(raw)
            .header("Referer", "https://www.douyin.com/")
            .send()?;
        if !res.status().is_success() {
            return Err(Error::new(res.status().as_u16(), "地址不可用"));
        }
        let url = res.url().to_string();
        let size = if head {
            client
                .head(&url)
                .header("Referer", "https://www.douyin.com/")
                .send()
                .ok()
                .and_then(|r| r.content_length())
                .unwrap_or(0)
        } else {
            0
        };
        Ok((url, size))
    }

    /// 取一页评论；空包常见（服务端偶发），换人重试。
    /// `seen` 是调用方已展示的 cid：身份轮换会改变热门排序，只按 cursor 翻页仍会重复。
    fn comment_batch(
        &self,
        id: &str,
        mut cur: i64,
        count: usize,
        seen: &HashSet<String>,
    ) -> Result<(Vec<Value>, bool, i64)> {
        let mut last = (Vec::new(), false, cur);
        for _ in 0..4 {
            let got: (Vec<Value>, bool, i64) = self.with_identity(|ident, fl| {
                let (token, author, ccount, _, _) = self.detail_meta(id).unwrap_or_default();
                let mut p = media(ident, fl);
                for (k, v) in [
                    ("aweme_id", id),
                    ("insert_ids", ""),
                    ("address_book_access", "2"),
                    ("gps_access", "1"),
                    ("item_type", "0"),
                    ("comment_aggregation", "0"),
                    ("is_fold_list", "false"),
                    ("forward_page_type", "1"),
                    ("channel_id", "0"),
                    ("city", "500000"),
                    ("hotsoon_filtered_count", "0"),
                    ("hotsoon_has_more", "0"),
                    ("follower_count", "0"),
                    ("is_familiar", "0"),
                    ("page_source", "0"),
                    ("user_avatar_shrink", "64_64"),
                    ("top_query_word", ""),
                    ("is_preload", "0"),
                    ("use_url_optimize", "0"),
                    ("current_l1_comment_count", "0"),
                    ("service_id", "0"),
                    ("group_id", "0"),
                    ("comment_scene", "0"),
                    ("hotspot_id", ""),
                    ("medium_shrink", "220_294"),
                    ("use_light_optimize", "0"),
                    ("friend_interaction", "0"),
                    ("need_whale_async", "0"),
                ] {
                    p.insert(k.into(), v.into());
                }
                p.insert("count".into(), count.to_string());
                p.insert("cursor".into(), cur.to_string());
                p.insert("aweme_author".into(), author.clone());
                p.insert("authentication_token".into(), token.clone());
                p.insert("comment_count".into(), ccount.to_string());
                let now = now_ms();
                let body = format!(
                    "session_id={}&session_show_cids=&ai_cmt_exposure=0",
                    url::form_urlencoded::byte_serialize(
                        format!("{id}:0:{}:{now}", ident.device_id).as_bytes()
                    )
                    .collect::<String>()
                );
                let b = self.http.request(
                    "POST",
                    API_HOST,
                    "/aweme/v2/comment/list/",
                    &p,
                    Some(body.as_bytes()),
                    "application/x-www-form-urlencoded; charset=UTF-8",
                    "",
                    None,
                    ident,
                    fl,
                    1,
                )?;
                if std::env::var("DUMP_COMMENTS").is_ok() {
                    let _ = std::fs::write("/tmp/comments.bin", &b);
                }
                let v: Value = json_of(&b, &["comments"])?;
                check_status(&v)?;
                let raw: Vec<Value> = v["comments"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .map(comment)
                    .collect();
                let mut page_seen = HashSet::new();
                let list = raw
                    .into_iter()
                    .filter(|c| {
                        let cid = str_of(&c["cid"]);
                        cid.is_empty() || (!seen.contains(&cid) && page_seen.insert(cid))
                    })
                    .collect();
                Ok((list, truthy(&v["has_more"]), num_of(&v["cursor"]).max(cur)))
            })?;
            let empty = got.0.is_empty();
            let (_, has_more, next) = got;
            last = got;
            if !empty {
                return Ok(last);
            }
            // 整页都是已展示过的内容时，按新 cursor 继续取，别把重复页交给界面。
            if has_more && next > cur {
                cur = next;
            }
        }
        Ok(last)
    }

    /// 一级评论：直接使用服务端 cursor，不在本地重抓前置页。
    fn comments(
        &self,
        id: &str,
        cursor: i64,
        count: usize,
        seen: &HashSet<String>,
    ) -> Result<Value> {
        let (items, has_more, next) = self.comment_batch(id, cursor, count, seen)?;
        Ok(page(items, has_more, Some(&Value::from(next))))
    }

    /// 视频弹幕：一次一段，前端按 cursor 续取。
    fn danmaku(&self, id: &str, cursor: i64) -> Result<Value> {
        let (token, _, _, _, duration) = self.detail_meta(id).unwrap_or_default();
        self.with_identity(|ident, fl| {
            let mut p = media(ident, fl);
            for (k, v) in [
                ("item_id", id.to_string()),
                ("start_time", cursor.max(0).to_string()),
                // total 是开关而不是上限，0 会稳定返回空包。
                ("has_total", "true".into()),
                ("total", "1".into()),
                ("danmaku_density", "-1".into()),
                ("authentication_token", token.clone()),
                ("duration", duration.to_string()),
                ("is_lvideo", "false".into()),
                ("highlight_type", "0".into()),
                (
                    "pass_through_params",
                    r#"{"first_danmaku_offset":0,"has_danmaku":1}"#.into(),
                ),
                ("use_smart_mode", "true".into()),
                ("md", "0".into()),
            ] {
                p.insert(k.into(), v);
            }
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/danmaku/get_v2/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            if std::env::var("DUMP_DANMAKU").is_ok() {
                let _ = std::fs::write("/tmp/danmaku.bin", &b);
            }
            let v: Value = json_of(&b, &["danmaku_list"])?;
            check_status(&v)?;
            let list = v["danmaku_list"].as_array().cloned().unwrap_or_default();
            let end = num_of(&v["end_time"]);
            let items: Vec<Value> = list
                .iter()
                .filter_map(|d| {
                    let text = str_of(&d["text"]);
                    let text = if text.is_empty() {
                        str_of(&d["content"])
                    } else {
                        text
                    };
                    let id = str_of(&d["danmaku_id"]);
                    let id = if id.is_empty() {
                        str_of(&d["msg_id"])
                    } else {
                        id
                    };
                    (!text.is_empty()).then(|| {
                        json!({
                            "id": id,
                            "text": text,
                            "offset_ms": num_of(&d["offset_time"])
                                .max(num_of(&d["offset"]))
                                .max(num_of(&d["offset_ms"])),
                            "color": str_of(&d["color"]),
                        })
                    })
                })
                .collect();
            let has_more = end > cursor.max(0) && (duration <= 0 || end < duration);
            Ok(page(
                items,
                truthy(&v["has_more"]) || has_more,
                Some(&Value::from(end.max(cursor))),
            ))
        })
    }

    fn video_tags(&self, id: &str) -> Result<Value> {
        let author = self
            .detail(id)
            .ok()
            .map(|d| str_of(&d["author"]["uid"]))
            .unwrap_or_default();
        self.with_identity(|ident, fl| {
            let p = common(ident, "664226", fl);
            let mut body = BTreeMap::new();
            body.insert("aweme_id".to_string(), id.to_string());
            if !author.is_empty() {
                body.insert("author_id".to_string(), author.clone());
            }
            body.insert(
                "extra_info".to_string(),
                r#"{"vtag_delay_ts":6,"ts":-1,"vtag_req_type":"pause"}"#.to_string(),
            );
            let enc = url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(body.iter())
                .finish();
            let b = self.http.request(
                "POST",
                SEARCH_HOST,
                "/aweme/v1/search/vtag/",
                &p,
                Some(enc.as_bytes()),
                "application/x-www-form-urlencoded; charset=UTF-8",
                "",
                None,
                ident,
                fl,
                2,
            )?;
            let v: Value = json_of(&b, &["tag_infos"])?;
            check_status(&v)?;
            let mut items: Vec<Value> = v["tag_infos"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter_map(|t| {
                    let name = str_of(&t["tag_name"]);
                    (!name.is_empty()).then(|| {
                        json!({
                            "tag_name": name,
                            "tag_space": str_of(&t["tag_space"]),
                            "tag_id": str_of(&t["tag_id"]),
                            "score": t["score"].clone(),
                            "start": num_of(&t["start"]),
                            "end": num_of(&t["end"]),
                        })
                    })
                })
                .collect();
            items.sort_by_key(|x| num_of(&x["start"]));
            Ok(page(items, false, None))
        })
    }

    fn detail_meta(&self, id: &str) -> Result<(String, String, i64, String, i64)> {
        let d = self.detail(id)?;
        Ok((
            str_of(&d["authentication_token"]),
            str_of(&d["author"]["sec_uid"]),
            num_of(&d["statistics"]["comment_count"]),
            str_of(&d["suggest_word"]),
            num_of(&d["video"]["duration"]),
        ))
    }

    fn replies(&self, id: &str, cid: &str, cursor: i64, count: usize) -> Result<Value> {
        if cid.is_empty() {
            return Err(Error::new(400, "缺少 comment"));
        }
        let (token, author, _, word, _) = {
            let (t, a, c, w, d) = self.detail_meta(id).unwrap_or_default();
            (t, a, c, w, d)
        };
        let mut last = page(vec![], false, None);
        for attempt in 0..3 {
            let (list, has_more, next): (Vec<Value>, bool, Value) =
                self.with_identity(|ident, fl| {
                    let mut p = media(ident, fl);
                    for (k, v) in [
                        ("comment_id", cid.to_string()),
                        ("item_id", id.to_string()),
                        ("cursor", cursor.to_string()),
                        ("count", count.to_string()),
                        ("top_ids", String::new()),
                        ("insert_ids", String::new()),
                        ("item_type", "0".into()),
                        ("channel_id", "0".into()),
                        ("city", "500000".into()),
                        ("follower_count", "0".into()),
                        ("is_familiar", "0".into()),
                        ("user_avatar_shrink", "64_64".into()),
                        ("aweme_author", author.clone()),
                        ("top_query_word", word.clone()),
                        ("authentication_token", token.clone()),
                        ("use_url_optimize", "0".into()),
                        ("service_id", "0".into()),
                        ("group_id", "0".into()),
                        ("comment_scene", "0".into()),
                        ("hotspot_id", String::new()),
                        ("medium_shrink", "317_424".into()),
                        ("viewed_comments", String::new()),
                        ("use_light_optimize", "0".into()),
                        ("need_whale_async", "0".into()),
                    ] {
                        p.insert(k.into(), v);
                    }
                    let now = now_ms();
                    let body = format!(
                        "session_id={}&session_show_cids=&ai_cmt_exposure=0",
                        url::form_urlencoded::byte_serialize(
                            format!("{id}:{cid}:{}:{now}", ident.device_id).as_bytes()
                        )
                        .collect::<String>()
                    );
                    let b = self.http.request(
                        "POST",
                        API_HOST,
                        "/aweme/v1/comment/list/reply/",
                        &p,
                        Some(body.as_bytes()),
                        "application/x-www-form-urlencoded; charset=UTF-8",
                        "",
                        None,
                        ident,
                        fl,
                        1,
                    )?;
                    if std::env::var("DUMP_COMMENTS").is_ok() {
                        let _ = std::fs::write("/tmp/comments.bin", &b);
                    }
                    let v: Value = json_of(&b, &["comments"])?;
                    check_status(&v)?;
                    let raw: Vec<Value> = v["comments"]
                        .as_array()
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                        .map(comment)
                        .collect();
                    let mut seen = HashSet::new();
                    let list = raw
                        .into_iter()
                        .filter(|c| {
                            let cid = str_of(&c["cid"]);
                            cid.is_empty() || seen.insert(cid)
                        })
                        .collect();
                    Ok((list, truthy(&v["has_more"]), v["cursor"].clone()))
                })?;
            let empty = list.is_empty();
            last = page(list, has_more, Some(&next));
            // 空包偶发：往后翻时换人重试，首页空就是真没有。
            if !empty || attempt == 2 {
                return Ok(last);
            }
        }
        Ok(last)
    }

    fn search_doc(&self, keyword: &str, cursor: i64, next: Option<&Value>) -> Result<Value> {
        if keyword.is_empty() {
            return Ok(json!({}));
        }
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            for (k, v) in [
                ("ac", "mobile"),
                ("app_name", "douyinos"),
                ("device_platform", "android"),
                ("ssmix", "a"),
                ("cpu_support64", "true"),
                ("host_abi", "arm64-v8a"),
                ("first_launch_timestamp", fl),
                ("last_deeplink_update_version_code", "32229906"),
                ("is_guest_mode", "0"),
                ("app_type", "normal"),
                ("minor_status", "0"),
                ("appTheme", "light"),
                ("is_preinstall", "0"),
                ("need_personal_recommend", "1"),
                ("is_android_pad", "0"),
                ("is_android_fold", "0"),
            ] {
                p.insert(k.into(), v.into());
            }
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            for (k, v) in [
                ("count", "10"),
                ("search_scene", "douyin_search"),
                ("query_correct_type", "1"),
                ("from_group_id", ""),
                ("search_channel", "null"),
                ("search_source", ""),
            ] {
                body.insert(k.into(), v.into());
            }
            body.insert("keyword".into(), keyword.into());
            if cursor > 0 {
                body.insert("cursor".into(), cursor.to_string());
            }
            if let Some(n) = next {
                for k in ["search_id", "search_request_id"] {
                    if let Some(s) = n.get(k).and_then(|x| x.as_str()) {
                        body.insert(k.into(), s.into());
                    }
                }
            }
            let enc = url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(body.iter())
                .finish();
            let b = self.http.request(
                "POST",
                SEARCH_HOST,
                "/aweme/v2/search/general/stream/",
                &p,
                Some(enc.as_bytes()),
                "application/x-www-form-urlencoded; charset=UTF-8",
                "",
                None,
                ident,
                fl,
                2,
            )?;
            if std::env::var("DUMP_SEARCH").is_ok() {
                let _ = std::fs::write("/tmp/search.bin", &b);
                eprintln!("[search] {} 字节", b.len());
            }
            let mut v: Value = json_of(&b, &["business_data"])?;
            check_status(&v)?;
            Ok(std::mem::take(&mut v))
        })
    }

    fn search(
        &self,
        keyword: &str,
        cursor: i64,
        search_id: &str,
        search_request_id: &str,
    ) -> Result<Value> {
        let next = (!search_id.is_empty() || !search_request_id.is_empty()).then(|| {
            json!({
                "search_id": search_id,
                "search_request_id": search_request_id,
            })
        });
        let doc = self.search_doc(keyword, cursor, next.as_ref())?;
        let mut items = Vec::new();
        for card in doc["business_data"].as_array().cloned().unwrap_or_default() {
            if let Some(a) = card["data"]["aweme_info"].as_object() {
                items.push(item(&Value::Object(a.clone())));
            }
        }
        let bc = &doc["business_config"];
        let np = &bc["next_page"];
        let mut out = page(items, truthy(&bc["has_more"]), np.get("cursor"));
        out["meta"]["search_id"] = np["search_id"].clone();
        out["meta"]["search_request_id"] = np["search_request_id"].clone();
        Ok(out)
    }

    fn search_sug(&self, keyword: &str) -> Result<Value> {
        if keyword.is_empty() {
            return Ok(page(Vec::new(), false, None));
        }
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            p.insert("keyword".into(), keyword.into());
            p.insert("count".into(), "10".into());
            let b = self.http.request(
                "GET",
                SEARCH_HOST,
                "/aweme/v1/search/sug/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                2,
            )?;
            let v: Value = json_of(&b, &["sug_list"])?;
            check_status(&v)?;
            let mut seen = HashSet::new();
            let items = v["sug_list"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|x| {
                    let word = str_of(&x["content"]);
                    (!word.is_empty() && seen.insert(word.clone()))
                        .then(|| json!({"word": word, "raw": x}))
                })
                .collect();
            Ok(page(items, false, None))
        })
    }

    fn search_item_doc(&self, keyword: &str, cursor: i64, search_id: &str) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            for (k, v) in [
                ("ac", "mobile"),
                ("app_name", "douyinos"),
                ("device_platform", "android"),
                ("ssmix", "a"),
                ("cpu_support64", "true"),
                ("host_abi", "arm64-v8a"),
                ("first_launch_timestamp", fl),
                ("last_deeplink_update_version_code", "32229906"),
                ("is_guest_mode", "0"),
                ("app_type", "normal"),
                ("minor_status", "0"),
                ("appTheme", "light"),
                ("is_preinstall", "0"),
                ("need_personal_recommend", "1"),
                ("is_android_pad", "0"),
                ("is_android_fold", "0"),
            ] {
                p.insert(k.into(), v.into());
            }
            let mut body = BTreeMap::new();
            for (k, v) in [
                ("keyword", keyword),
                ("count", "10"),
                ("search_scene", "douyin_search"),
                ("query_correct_type", "1"),
                ("search_channel", "aweme_series"),
            ] {
                body.insert(k.to_string(), v.to_string());
            }
            if let Ok(extra) = std::env::var("SERIES_EXTRA") {
                for kv in extra.split(';') {
                    if let Some((k, v)) = kv.split_once('=') {
                        body.insert(k.to_string(), v.to_string());
                    }
                }
            }
            if cursor > 0 {
                body.insert("cursor".into(), cursor.to_string());
            }
            if !search_id.is_empty() {
                body.insert("search_id".into(), search_id.to_string());
            }
            let enc = url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(body.iter())
                .finish();
            if std::env::var("DUMP_SEARCH").is_ok() {
                eprintln!("[search] ident={} kw={keyword}", ident.device_id);
            }
            let b = self.http.request(
                "POST",
                SEARCH_HOST,
                "/aweme/v1/search/item/",
                &p,
                Some(enc.as_bytes()),
                "application/x-www-form-urlencoded; charset=UTF-8",
                "",
                None,
                ident,
                fl,
                2,
            )?;
            if std::env::var("DUMP_SEARCH").is_ok() {
                let _ = std::fs::write("/tmp/search_item.bin", &b);
            }
            let v: Value = json_of(&b, &["data"])?;
            check_status(&v)?;
            Ok(v)
        })
    }

    fn search_series(&self, keyword: &str) -> Result<Value> {
        let keyword = keyword.trim();
        if keyword.is_empty() {
            return Ok(page(Vec::new(), false, None));
        }
        if let Some((at, cached)) = self.series_cache.lock().ok().and_then(|m| {
            m.get(keyword)
                .filter(|(at, v)| {
                    at.elapsed() < Duration::from_secs(if v.is_empty() { 600 } else { 3600 })
                })
                .cloned()
        }) {
            let _ = at;
            return Ok(page(cached, false, None));
        }

        // 连写“短剧”比空格分隔命中率高：空格版本把结果推向剪辑号，连写才召回剧集本身。
        let variants = [
            format!("{keyword}短剧全集"),
            format!("{keyword}短剧"),
            format!("{keyword} 短剧 全集"),
            keyword.to_string(),
        ];
        // 每个改写并发取；凭证号串行签名不影响网络重叠，比逐条串行重试快一个数量级。
        // 搜索页里只有部分作品带 related_video_info，且排序每次不同：先取首屏，
        // 命中不足再顺着各改写的 cursor 续页，全空时换一批改写短查一次。
        type PageState = Option<(i64, String)>;
        type Collected = (Vec<Value>, Vec<String>, Vec<PageState>);
        let collect = |batch: &[String], starts: &[PageState], pages: usize| -> Collected {
            let budget = std::sync::atomic::AtomicUsize::new(0);
            std::thread::scope(|scope| {
                let handles: Vec<_> = batch
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let budget = &budget;
                        let start = starts.get(i).cloned().flatten();
                        scope.spawn(move || {
                            let (mut cursor, mut sid) = start.unwrap_or((0, String::new()));
                            let mut docs: Vec<Value> = Vec::new();
                            let mut ids: Vec<String> = Vec::new();
                            let mut next_state = None;
                            for _ in 0..pages {
                                let Ok(doc) = self.search_item_doc(v, cursor, &sid) else {
                                    break;
                                };
                                let next = doc["cursor"].as_i64().unwrap_or(0);
                                let impr = str_of(&doc["log_pb"]["impr_id"]);
                                let more = truthy(&doc["has_more"]);
                                if std::env::var("DUMP_SEARCH").is_ok() {
                                    eprintln!(
                                        "[series] {} cards",
                                        doc["data"].as_array().map(|a| a.len()).unwrap_or(0)
                                    );
                                }
                                let before = ids.len();
                                for card in doc["data"].as_array().cloned().unwrap_or_default() {
                                    let raw = card["aweme_info"]["related_video_info"]["extra"]
                                        .as_str()
                                        .unwrap_or("");
                                    let sid = serde_json::from_str::<Value>(raw)
                                        .ok()
                                        .and_then(|x| x.get("related_series_id").cloned())
                                        .map(|x| str_of(&x))
                                        .unwrap_or_default();
                                    if !sid.is_empty() {
                                        ids.push(sid);
                                    }
                                }
                                budget.fetch_add(
                                    ids.len() - before,
                                    std::sync::atomic::Ordering::Relaxed,
                                );
                                docs.push(doc);
                                if !more || next <= cursor || impr.is_empty() {
                                    break;
                                }
                                cursor = next;
                                sid = impr;
                                next_state = Some((cursor, sid.clone()));
                                if budget.load(std::sync::atomic::Ordering::Relaxed) >= 10 {
                                    break;
                                }
                            }
                            (docs, ids, next_state)
                        })
                    })
                    .collect();
                let mut docs = Vec::new();
                let mut ids = Vec::new();
                let mut next: Vec<PageState> = Vec::new();
                for h in handles {
                    if let Ok((d, i, n)) = h.join() {
                        docs.extend(d);
                        ids.extend(i);
                        next.push(n);
                    }
                }
                (docs, ids, next)
            })
        };
        let mut docs = Vec::new();
        let mut ids: Vec<String> = Vec::new();
        let none: Vec<Option<(i64, String)>> = variants.iter().map(|_| None).collect();
        let (d, i, next) = collect(&variants, &none, 1);
        docs.extend(d);
        ids.extend(i);
        if ids.is_empty() {
            let extra = vec![
                format!("{keyword} 逆袭"),
                format!("{keyword} 合集"),
                format!("{keyword}短剧 推荐"),
                format!("{keyword} 全集"),
            ];
            let (d, i, _) = collect(&extra, &none, 2);
            docs.extend(d);
            ids.extend(i);
        } else if ids.len() < 8 && next.iter().any(|n| n.is_some()) {
            let (d, i, _) = collect(&variants, &next, 2);
            docs.extend(d);
            ids.extend(i);
        }
        let mut seen = HashSet::new();
        ids.retain(|id| seen.insert(id.clone()));
        if std::env::var("DUMP_SEARCH").is_ok() {
            eprintln!("[series] reqs={} ids={}", docs.len(), ids.len());
            let all: Vec<&Value> = docs.iter().collect();
            let _ = std::fs::write(
                "/tmp/docs.json",
                serde_json::to_vec_pretty(&all).unwrap_or_default(),
            );
        }
        let ids: Vec<String> = ids.into_iter().take(20).collect();
        let mut out: Vec<Value> = std::thread::scope(|scope| {
            let handles: Vec<_> = ids
                .iter()
                .map(|id| scope.spawn(|| self.series(id)))
                .collect();
            handles
                .into_iter()
                .filter_map(|h| h.join().ok().and_then(|r| r.ok()))
                .filter(|s| !str_of(&s["series_id"]).is_empty())
                .collect()
        });
        out.sort_by_key(|s| std::cmp::Reverse(num_of(&s["episode_count"])));
        if let Ok(mut m) = self.series_cache.lock() {
            if m.len() > 200 {
                m.clear();
            }
            m.insert(keyword.to_string(), (Instant::now(), out.clone()));
        }
        Ok(page(out, false, None))
    }

    fn search_users(&self, keyword: &str, cursor: i64, search_id: &str) -> Result<Value> {
        if keyword.is_empty() {
            return Ok(page(Vec::new(), false, None));
        }
        self.with_identity(|ident, fl| {
            let p = common(ident, "664226", fl);
            let pull = if cursor > 0 { 1 } else { 0 };
            let sid = if search_id.is_empty() {
                String::new()
            } else {
                format!("&search_id={search_id}")
            };
            let body = format!(
                "keyword={}&cursor={cursor}&count=10&type=2&search_source=normal_search&is_pull_down={pull}&hot_search=0{sid}",
                url::form_urlencoded::byte_serialize(keyword.as_bytes()).collect::<String>());
            let b = self.http.request("POST", USER_HOST, "/aweme/v1/discover/search/", &p, Some(body.as_bytes()), "application/x-www-form-urlencoded; charset=UTF-8", "", None, ident, fl, 2)?;
            if std::env::var("DUMP_SEARCH").is_ok() {
                let _ = std::fs::write("/tmp/discover.bin", &b);
            }
            let v: Value = json_of(&b, &["user_list"])?;
            check_status(&v)?;
            let mut out = Vec::new();
            for e in v["user_list"].as_array().cloned().unwrap_or_default() {
                let raw = e["dynamic_patch"]["raw_data"].as_str().unwrap_or("");
                let ui = serde_json::from_str::<Value>(raw).ok().and_then(|x| x.get("user_info").cloned()).unwrap_or(Value::Null);
                if ui["uid"].is_string() {
                    out.push(author(&ui));
                }
            }
            let mut out_page = page(out, truthy(&v["has_more"]), Some(&v["cursor"]));
            out_page["meta"]["search_id"] = v["log_pb"]["impr_id"].clone();
            Ok(out_page)
        })
    }

    fn challenges(&self, keyword: &str, cursor: i64, search_id: &str) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let p = common(ident, "664226", fl);
            let sid = if search_id.is_empty() {
                String::new()
            } else {
                format!("&search_id={search_id}")
            };
            let body = format!("keyword={}&count=20&cursor={cursor}{sid}",
                url::form_urlencoded::byte_serialize(keyword.as_bytes()).collect::<String>());
            let b = self.http.request("POST", API_HOST, "/aweme/v1/challenge/search/", &p, Some(body.as_bytes()), "application/x-www-form-urlencoded; charset=UTF-8", "", None, ident, fl, 2)?;
            let v: Value = json_of(&b, &["challenge_list"])?;
            check_status(&v)?;
            let list = v["challenge_list"].as_array().cloned().unwrap_or_default().iter().map(|c| {
                let x = &c["challenge_info"];
                json!({"challenge_name": str_of(&x["cha_name"]), "ch_id": str_of(&x["cid"]),
                    "view_count": num_of(&x["view_count"]), "video_count": num_of(&x["video_count"]),
                    "desc": str_of(&x["desc"]), "raw": x})
            }).collect();
            let mut out = page(list, truthy(&v["has_more"]), Some(&v["cursor"]));
            out["meta"]["search_id"] = v["log_pb"]["impr_id"].clone();
            Ok(out)
        })
    }

    fn user(&self, id: &str) -> Result<Value> {
        let sec = if id.starts_with("MS4w") {
            id.to_string()
        } else {
            self.sec_uid_of(id)?
        };
        // 主页接口冷启动会偶发 status_code=2 / 空 user，重试几次。
        let mut last = None;
        for attempt in 0..5 {
            match self.user_once(&sec) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    // 404 里“用户不存在”也可能只是这一发没签好，一并重试。
                    if !(e.retryable() || e.status == 404) {
                        return Err(e);
                    }
                    last = Some(e);
                    std::thread::sleep(std::time::Duration::from_millis(300 * (attempt + 1)));
                }
            }
        }
        Err(last.unwrap_or_else(|| Error::new(404, "用户不存在")))
    }

    fn user_once(&self, sec: &str) -> Result<Value> {
        let uid = sec;
        self.with_identity(|ident, fl| {
            let mut p = media(ident, fl);
            for (k, v) in [("sec_user_id", uid), ("publish_video_strategy_type", "2"),
                ("enter_from", "others_homepage"), ("enter_method", "click_title"),
                ("enter_aweme_id", ""), ("enter_aweme_type", "0"),
                ("enter_from_context", r#"{"enter_from":"homepage_hot","with_fusion_goods":false}"#),
                ("previous_page", "0"), ("from", "0"), ("land_to", "1"), ("hit_ab_test", "1"),
                ("is_hit_mate", "0"), ("btn_in_value", "0"), ("card_partition", "0"),
                ("card_style", "0"), ("profile_other_record_enable", "1"),
                ("social_negative_user", "0"), ("source", "UserProfileFragment_initUserData"),
                ("tpsa_restriction", r#"{"restriction_ab":0}"#),
                ("use_profile_show_identify_auth_info", "true"), ("user_avatar_shrink", "188_188"),
                ("user_cover_shrink", "750_422"), ("webcast_plugin_ready", "false"),
                ("personal_center_strategy", "1")] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request("GET", USER_HOST, "/aweme/v1/user/profile/other/", &p, None, "", "", None, ident, fl, 2)?;
            let v: Value = json_of(&b, &["user"])?;
            check_status(&v)?;
            let u = &v["user"];
            if u["uid"].is_null() {
                if std::env::var("DUMP_USER").is_ok() { eprintln!("[user] {}", String::from_utf8_lossy(&b[..b.len().min(400)])); }
                return Err(Error::new(404, "用户不存在"));
            }
            Ok(json!({"uid": str_of(&u["uid"]), "sec_uid": uid, "nickname": str_of(&u["nickname"]),
                "avatar": url0(&u["avatar_thumb"]["url_list"]), "signature": str_of(&u["signature"]),
                "follower_count": num_of(&u["follower_count"]), "raw": u}))
        })
    }

    /// 数字 uid 换 sec_uid：主页接口只认 sec_uid，借作品列表里的作者字段换。
    fn sec_uid_of(&self, uid: &str) -> Result<String> {
        let mut last = None;
        for _ in 0..3 {
            match self.sec_from_posts(uid) {
                Ok(Some(sec)) => return Ok(sec),
                Ok(None) => last = Some(Error::new(404, "这个用户没有公开作品，换不到主页")),
                Err(e) => last = Some(e),
            }
            match self.sec_from_search(uid) {
                Ok(Some(sec)) => return Ok(sec),
                Ok(None) => {}
                Err(e) => last = Some(e),
            }
            // 冷启动头几发签名不稳，等一等重试。
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Err(last.unwrap_or_else(|| Error::new(404, "换不到主页")))
    }

    fn sec_from_posts(&self, uid: &str) -> Result<Option<String>> {
        let v = self.user_posts(uid, 0)?;
        Ok(v.get("items")
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(|x| x.get("raw"))
            .and_then(|r| r.get("author"))
            .and_then(|a| a.get("sec_uid"))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string))
    }

    /// 按数字 uid 搜人，只认 uid 完全相等的结果，避免认错人。
    fn sec_from_search(&self, uid: &str) -> Result<Option<String>> {
        let v = self.search_users(uid, 0, "")?;
        Ok(v.get("items")
            .and_then(Value::as_array)
            .and_then(|a| a.iter().find(|x| x["uid"].as_str() == Some(uid)))
            .and_then(|x| x["sec_uid"].as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string))
    }

    fn uid_of(&self, id: &str) -> Result<String> {
        if std::env::var("DUMP_USER").is_ok() {
            eprintln!("[uid_of] {id}");
        }
        if !id.starts_with("MS4w") {
            return Ok(id.to_string());
        }
        Ok(self
            .user(id)?
            .get("uid")
            .and_then(|v| v.as_str())
            .unwrap_or(id)
            .to_string())
    }

    /// 列表偶发因身份差异返回 200 但为空，换身份重试；空页不是可靠结果。
    fn retry_empty(&self, mut f: impl FnMut() -> Result<Value>) -> Result<Value> {
        let mut last = Value::Null;
        for attempt in 0..3 {
            let v = f()?;
            let empty = v
                .get("items")
                .and_then(Value::as_array)
                .is_none_or(|a| a.is_empty());
            if !empty || attempt == 2 {
                return Ok(v);
            }
            last = v;
            std::thread::sleep(std::time::Duration::from_millis(350 * (attempt as u64 + 1)));
        }
        Ok(last)
    }

    /// 用户相关列表偶发因身份差异返回 404，换身份重试。
    fn retry_404<T>(&self, mut f: impl FnMut() -> Result<T>) -> Result<T> {
        let mut last = Error::new(404, "内容不存在或已删除");
        for attempt in 0..3 {
            match f() {
                Ok(v) => return Ok(v),
                Err(e) if e.status == 404 => {
                    last = e;
                    std::thread::sleep(Duration::from_millis(300 * (attempt as u64 + 1)));
                }
                Err(e) => return Err(e),
            }
        }
        Err(last)
    }

    fn user_posts(&self, uid: &str, cursor: i64) -> Result<Value> {
        if std::env::var("DUMP_USER").is_ok() {
            eprintln!("[posts] {uid}");
        }
        let uid = self.uid_of(uid)?;
        let cur = cursor.to_string();
        self.retry_empty(|| {
            self.retry_404(|| {
                self.with_identity(|ident, fl| {
                    let mut p = media(ident, fl);
                    for (k, v) in [
                        ("user_id", uid.as_str()),
                        ("max_cursor", cur.as_str()),
                        ("count", "20"),
                        ("source", "0"),
                        ("publish_video_strategy_type", "2"),
                        ("is_other", "1"),
                        ("need_unread_notice_info", "1"),
                        ("need_time_list", "0"),
                        ("show_live_replay_strategy", "1"),
                        ("is_order_flow", "1"),
                        ("page_from", "2"),
                        ("location_permission", "1"),
                        ("collects_id", ""),
                        ("familiar_collects", "0"),
                        ("page_scene", "1"),
                        ("post_serial_strategy", "0"),
                        ("user_avatar_shrink", "96_96"),
                        ("video_cover_shrink", "248_330"),
                    ] {
                        p.insert(k.into(), v.into());
                    }
                    let b = self.http.request(
                        "GET",
                        API_HOST,
                        "/aweme/v1/aweme/post/",
                        &p,
                        None,
                        "",
                        "",
                        None,
                        ident,
                        fl,
                        2,
                    )?;
                    if std::env::var("DUMP_USER").is_ok() && b.len() < 600 {
                        eprintln!(
                            "[posts-empty] {}",
                            String::from_utf8_lossy(&b[..b.len().min(400)])
                        );
                    }
                    let v: Value = json_of(&b, &["aweme_list"])?;
                    check_status(&v)?;
                    Ok(page_of(
                        &v["aweme_list"],
                        truthy(&v["has_more"]),
                        Some(&v["max_cursor"]),
                    ))
                })
            })
        })
    }

    fn user_mixes(&self, uid: &str, cursor: i64) -> Result<Value> {
        let uid = if uid.starts_with("MS4w") {
            uid.to_string()
        } else {
            self.sec_uid_of(uid)?
        };
        self.retry_empty(|| self.retry_404(|| self.with_identity(|ident, fl| {
            let mut p = media(ident, fl);
            p.insert("sec_user_id".into(), uid.clone());
            p.insert("cursor".into(), cursor.to_string());
            p.insert("count".into(), "20".into());
            let b = self.http.request("GET", API_HOST, "/aweme/v1/mix/list/", &p, None, "", "", None, ident, fl, 1)?;
            let v: Value = json_of(&b, &["mix_infos"])?;
            check_status(&v)?;
            let list = v["mix_infos"].as_array().cloned().unwrap_or_default().iter().map(|m| {
                json!({"mix_id": str_of(&m["mix_id"]), "mix_name": str_of(&m["mix_name"]),
                    "cover": url0(&m["cover_url"]["url_list"]), "aweme_count": num_of(&m["statis"]["updated_to_episode"]),
                    "author": {"nickname": ""}, "raw": m})
            }).collect();
            Ok(page(list, truthy(&v["has_more"]), Some(&v["cursor"])))
        })))
    }

    fn user_series(&self, uid: &str, cursor: i64) -> Result<Value> {
        let uid = self.uid_of(uid)?;
        self.retry_empty(|| {
            self.retry_404(|| {
                self.with_identity(|ident, fl| {
                    let mut p = common(ident, "6383", fl);
                    p.insert("user_id".into(), uid.clone());
                    p.insert("cursor".into(), cursor.to_string());
                    p.insert("count".into(), "20".into());
                    let b = self.http.request(
                        "GET",
                        API_HOST,
                        "/aweme/v1/series/list/",
                        &p,
                        None,
                        "",
                        "",
                        None,
                        ident,
                        fl,
                        1,
                    )?;
                    let v: Value = json_of(&b, &["series_infos"])?;
                    check_status(&v)?;
                    let list = v["series_infos"]
                        .as_array()
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                        .map(series_item)
                        .collect();
                    Ok(page(list, truthy(&v["has_more"]), Some(&v["cursor"])))
                })
            })
        })
    }

    fn user_live(&self, uid: &str) -> Result<Value> {
        let uid = self.uid_of(uid)?;
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            p.insert("user_id".into(), uid.clone());
            let b = self.http.request(
                "GET",
                LIVE_HOST,
                "/webcast/room/info_by_user/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["data"])?;
            check_status(&v)?;
            if v["data"].is_null() || v["data"].as_object().is_some_and(Map::is_empty) {
                return Err(Error::new(404, "主播当前未开播"));
            }
            Ok(room(&v["data"]))
        })
    }

    fn challenge(&self, name: &str) -> Result<Value> {
        let ch_id = if name.chars().all(|c| c.is_ascii_digit()) {
            name.to_string()
        } else {
            let found = self.challenges(name, 0, "")?;
            found["items"][0]["ch_id"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        };
        if ch_id.is_empty() {
            return Err(Error::new(404, "搜不到话题"));
        }
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            p.insert("ch_id".into(), ch_id.clone());
            let b = self.http.request("GET", API_HOST, "/aweme/v1/challenge/detail/", &p, None, "", "", None, ident, fl, 1)?;
            let v: Value = json_of(&b, &["challenge_info|ch_info"])?;
            check_status(&v)?;
            if std::env::var("DUMP_CH").is_ok() { eprintln!("[ch] {}", String::from_utf8_lossy(&b[..b.len().min(500)])); }
            let c = v.get("challenge_info").or_else(|| v.get("ch_info")).cloned().unwrap_or(Value::Null);
            let c = &c;
            Ok(json!({"challenge_name": str_of(&c["cha_name"]), "ch_id": str_of(&c["cid"]), "view_count": num_of(&c["view_count"]),
                "video_count": num_of(&c["video_count"]), "desc": str_of(&c["desc"]), "raw": c}))
        })
    }

    fn challenge_videos(&self, name: &str, sort: &str, cursor: i64) -> Result<Value> {
        let ch_id = if name.chars().all(|c| c.is_ascii_digit()) {
            name.to_string()
        } else {
            let found = self.challenges(name, 0, "")?;
            found["items"][0]["ch_id"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        };
        // 服务端对部分身份回 `status=0 + 空列表`；换人重试，空过的身份降权。
        let mut last = Value::Null;
        for attempt in 0..6 {
            if attempt > 0 {
                self.pool().drop_current();
            }
            let cur = cursor.to_string();
            let out = self.with_identity_opt(attempt == 0, |ident, fl| {
                let mut p = media(ident, fl);
                for (k, v) in [
                    ("ch_id", ch_id.as_str()),
                    ("cursor", cur.as_str()),
                    ("count", "20"),
                    ("sort_type", if sort.is_empty() { "0" } else { sort }),
                    ("type", "5"),
                    ("query_type", "0"),
                    ("source", "challenge_video"),
                    ("show_type", "0"),
                    ("enter_from", "deeplink"),
                    ("from_group_id", ""),
                    ("is_order_flow", "1"),
                    ("video_cover_shrink", "372_496"),
                    ("location_permission", "1"),
                ] {
                    p.insert(k.into(), v.into());
                }
                let b = self.http.request(
                    "GET",
                    API_HOST,
                    "/aweme/v1/challenge/aweme/",
                    &p,
                    None,
                    "",
                    "",
                    None,
                    ident,
                    fl,
                    1,
                )?;
                let v: Value = json_of(&b, &["aweme_list"])?;
                check_status(&v)?;
                let empty = v["aweme_list"]
                    .as_array()
                    .map(|a| a.is_empty())
                    .unwrap_or(true);
                if std::env::var("DUMP_CHV").is_ok() {
                    eprintln!("[chv] {} empty={empty} {}B", ident.device_id, b.len());
                }
                if empty {
                    self.pool().mark_miss(&ident.device_id);
                } else {
                    self.pool().mark_hit(&ident.device_id);
                }
                Ok((v, empty))
            });
            match out {
                Ok((v, false)) => {
                    return Ok(page_of(
                        &v["aweme_list"],
                        truthy(&v["has_more"]),
                        Some(&v["cursor"]),
                    ))
                }
                Ok((v, true)) => last = v,
                Err(e) => {
                    if attempt == 5 {
                        return Err(e);
                    }
                }
            }
        }
        Ok(page_of(
            &last["aweme_list"],
            truthy(&last["has_more"]),
            Some(&last["cursor"]),
        ))
    }

    fn hot(&self) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            for (k, v) in [
                ("detail_list", "1"),
                ("source", "4"),
                ("disable_ad", "0"),
                ("main_billboard_count", "50"),
                ("is_vcd", "0"),
                ("request_tag_from", "h5"),
            ] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/hot/search/list/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["data.word_list|word_list"])?;
            check_status(&v)?;
            let words = v["data"]["word_list"]
                .as_array()
                .or(v["word_list"].as_array())
                .cloned()
                .unwrap_or_default();
            let list = words
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    json!({"word": str_of(&w["word"]),
                "hot_value": num_of(&w["hot_value"]), "position": i + 1})
                })
                .collect();
            Ok(page(list, false, None))
        })
    }

    fn series_wall(&self, cursor: i64, count: usize) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "6383", fl);
            p.insert("cursor".into(), cursor.to_string());
            p.insert("count".into(), count.to_string());
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/series/desktop/list/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["desktop_series_list"])?;
            check_status(&v)?;
            let list: Vec<Value> = v["desktop_series_list"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter_map(|x| x.get("native_series").or_else(|| x.get("series_info")))
                .map(series_item)
                .collect();
            if std::env::var("DY_DEBUG").is_ok() {
                eprintln!(
                    "[series_wall] cursor={cursor} n={} has_more={} srv_cursor={}",
                    list.len(),
                    v["has_more"],
                    v["cursor"]
                );
            }
            let next = v["cursor"]
                .as_i64()
                .filter(|n| *n > cursor)
                .unwrap_or(cursor + list.len() as i64);
            let has_more = truthy(&v["has_more"]) || (!list.is_empty() && next > cursor);
            Ok(page(list, has_more, Some(&Value::from(next))))
        })
    }

    fn series(&self, id: &str) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "6383", fl);
            p.insert("series_id".into(), id.into());
            p.insert("need_cover_color".into(), "0".into());
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/series/detail/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["series_info"])?;
            check_status(&v)?;
            Ok(series_item(&v["series_info"]))
        })
    }

    fn series_episodes(&self, id: &str, cursor: i64) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "6383", fl);
            let cur = cursor.to_string();
            for (k, v) in [
                ("series_id", id),
                ("cursor", cur.as_str()),
                ("count", "20"),
                ("pull_type", "2"),
                ("scene", "default"),
                ("source", "playlet_detail"),
                ("disable_insert_ad", "0"),
                ("pre_current_episode", "0"),
                ("is_related_bar", "0"),
                ("need_history", "0"),
                ("continue_broadcast_episode", "0"),
                ("is_free_watch_btn", "0"),
                ("has_show_free_watch_btn", "0"),
            ] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/series/aweme/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["aweme_list"])?;
            check_status(&v)?;
            let len = v["aweme_list"].as_array().map(|a| a.len()).unwrap_or(0);
            let next = v["cursor"]
                .as_i64()
                .filter(|n| *n > cursor)
                .unwrap_or(cursor + len as i64);
            let has_more = truthy(&v["has_more"]) || (len > 0 && next > cursor);
            Ok(page_of(
                &v["aweme_list"],
                has_more,
                Some(&Value::from(next)),
            ))
        })
    }

    fn series_paid(&self, id: &str) -> Result<Value> {
        self.with_identity(|ident, fl| {
            // 付费段用最小参数面；叠公共参数会让部分剧集回空包。
            let mut p = BTreeMap::new();
            for (k, v) in [
                ("series_id", id),
                ("cursor", "0"),
                ("count", "5"),
                ("pull_type", "2"),
                ("scene", "default"),
                ("source", "playlet_detail"),
                ("aid", "1128"),
            ] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/series/aweme/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["aweme_list"])?;
            check_status(&v)?;
            let items = v["aweme_list"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let info = if x["series_paid_info"].is_object() {
                        &x["series_paid_info"]
                    } else {
                        &x["paid"]
                    };
                    json!({
                        "series_id": id,
                        "aweme_id": str_of(&x["aweme_id"]),
                        "index": i + 1,
                        "item_price": num_of(&info["item_price"]),
                        "series_paid_status": num_of(&info["series_paid_status"]),
                    })
                })
                .collect();
            Ok(page(items, false, None))
        })
    }

    fn mix_videos(&self, id: &str, cursor: i64) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = media(ident, fl);
            let cur = cursor.to_string();
            for (k, v) in [
                ("mix_id", id),
                ("cursor", cur.as_str()),
                ("count", "20"),
                ("pull_type", "2"),
                ("skip_local_cache", "false"),
                ("req_from", "app"),
            ] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request(
                "GET",
                API_HOST,
                "/aweme/v1/mix/aweme/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["aweme_list"])?;
            check_status(&v)?;
            let mut out = page_of(&v["aweme_list"], truthy(&v["has_more"]), Some(&v["cursor"]));
            if let Some(m) = v.get("mix_info") {
                out["meta"]["mix_info"] = m.clone();
            }
            Ok(out)
        })
    }

    /// 上游一批固定回 6 个且不带游标，游标本机按批递增。
    fn live_feed_batch(&self, cursor: u64) -> Result<Vec<Value>> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            p.insert("count".into(), "6".into());
            p.insert("feed_type".into(), "1".into());
            p.insert("cursor".into(), cursor.to_string());
            let b = self.http.request(
                "GET",
                LIVE_HOST,
                "/webcast/feed/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            if b.is_empty() {
                return Err(Error::new(429, "账号仍在服务端风控期（直播空包）"));
            }
            let v: Value = json_of(&b, &["data"])?;
            check_status(&v)?;
            let out: Vec<Value> = v["data"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|it| room(&it["data"]))
                .filter(|r| !str_of(&r["room_id"]).is_empty())
                .collect();
            if out.is_empty() {
                return Err(Error::new(429, "账号仍在服务端风控期（直播空包）"));
            }
            Ok(out)
        })
    }

    /// 上游忽略 `count`（恒回 6 个）：连续拉多批合成一页，批内去重。
    fn live_feed(&self, count: usize, cursor: &str) -> Result<Value> {
        let target = count.clamp(6, 30);
        let mut page_no = cursor.parse::<u64>().unwrap_or(0);
        let mut out: Vec<Value> = Vec::new();
        let mut seen = HashSet::new();
        let mut empties = 0u32;
        let mut tried = 0u32;
        while out.len() < target && tried < 8 && empties < 2 {
            tried += 1;
            let batch = self.live_feed_batch(page_no)?;
            page_no += 1;
            if batch.is_empty() {
                empties += 1;
                continue;
            }
            for r in batch {
                if seen.insert(str_of(&r["room_id"])) {
                    out.push(r);
                }
            }
        }
        let more = !out.is_empty() && empties < 2;
        Ok(page(out, more, Some(&json!(page_no))))
    }

    fn live_probe(&self, path: &str, query: &Value) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            if let Some(m) = query.as_object() {
                for (k, v) in m {
                    if k != "path" {
                        p.insert(k.clone(), str_of(v));
                    }
                }
            }
            let b = self.http.request(
                "GET",
                LIVE_HOST,
                &format!(
                    "/webcast/{path}{}",
                    if path.ends_with('!') { "" } else { "/" }
                ),
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let _ = std::fs::write("/tmp/live.bin", &b);
            let v: Value = json_of(&b, &[])?;
            Ok(v)
        })
    }

    fn live_resolve(&self, target: &str) -> Result<Value> {
        let room_id = target
            .rsplit('/')
            .find(|x| !x.is_empty() && x.chars().all(|c| c.is_ascii_digit()))
            .unwrap_or(target);
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            p.insert("room_id".into(), room_id.to_string());
            let b = self.http.request(
                "GET",
                LIVE_HOST,
                "/webcast/room/info/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["data"])?;
            check_status(&v)?;
            Ok(room(&v["data"]))
        })
    }

    fn live_pull(&self, room_id: &str) -> Result<Value> {
        let room = self.live_resolve(room_id)?;
        let url = {
            let hls = str_of(&room["hls"]);
            if !hls.is_empty() {
                hls
            } else {
                str_of(&room["flv"])
            }
        };
        if url.is_empty() {
            return Err(Error::new(404, "直播间没有可用拉流地址"));
        }
        Ok(json!({"url": url, "out": "", "room_id": str_of(&room["room_id"])}))
    }

    fn live_messages(&self, room: &str, cursor: &str, internal_ext: &str) -> Result<Value> {
        self.with_identity(|ident, fl| {
            let mut p = common(ident, "664226", fl);
            for (k, v) in [
                ("room_id", room),
                ("cursor", cursor),
                ("internal_ext", internal_ext),
                ("last_rtt_ms", "0"),
                ("fetch_rule", "1"),
                ("sub_type", "1"),
                ("live_id", "1"),
            ] {
                p.insert(k.into(), v.into());
            }
            let b = self.http.request(
                "GET",
                LIVE_HOST,
                "/webcast/im/fetch/",
                &p,
                None,
                "",
                "",
                None,
                ident,
                fl,
                1,
            )?;
            let v: Value = json_of(&b, &["data"])?;
            check_status(&v)?;
            let mut items = Vec::new();
            for m in v["data"].as_array().cloned().unwrap_or_default() {
                let u = if m["user"]["user"].is_object() {
                    m["user"]["user"].clone()
                } else {
                    m["user"].clone()
                };
                let text = m["content"].as_str().unwrap_or_default().to_string();
                if text.is_empty() {
                    continue;
                }
                items.push(json!({"method": str_of(&m["method"]), "content": text,
                    "user": {"nickname": str_of(&u["nickname"])}}));
            }
            Ok(json!({"items": items,
                "cursor": str_of(&v["extra"]["cursor"]),
                "internal_ext": str_of(&v["internal_ext"]),
                "interval_ms": v["extra"]["fetch_interval"].as_u64().unwrap_or(1200)}))
        })
    }

    fn target_exists(&self, kind: TargetKind, value: &str) -> bool {
        match kind {
            TargetKind::Video => self.detail(value).is_ok(),
            TargetKind::User => self.user(value).is_ok(),
            TargetKind::Room => self.live_resolve(value).is_ok(),
            TargetKind::Series => self.series(value).is_ok(),
            TargetKind::Mix => self.mix_videos(value, 0).is_ok(),
            _ => false,
        }
    }

    fn resolve(&self, target: &str) -> Result<Value> {
        let mut current = target.trim().to_string();
        for _ in 0..5 {
            let parsed = parse_target(&current)?;
            if parsed.kind == TargetKind::Short {
                let client = reqwest::blocking::Client::builder()
                    .redirect(reqwest::redirect::Policy::limited(5))
                    .build()?;
                let res = client.get(&parsed.value).send()?;
                let expanded = res.url().to_string();
                if expanded == current {
                    return Err(Error::new(400, "短链没有展开"));
                }
                current = expanded;
                continue;
            }
            let kind = if parsed.candidates.is_empty() {
                parsed.kind
            } else {
                let mut kinds = parsed.candidates.clone();
                if !kinds.contains(&parsed.kind) {
                    kinds.push(parsed.kind);
                }
                kinds
                    .into_iter()
                    .find(|k| self.target_exists(*k, &parsed.value))
                    .unwrap_or(parsed.kind)
            };
            let mut out = json!({
                "type": kind.name(),
                "id": parsed.value,
                "target": target,
                "candidates": parsed.candidates.iter().map(|k| k.name()).collect::<Vec<_>>(),
            });
            match kind {
                TargetKind::Video => out["aweme_id"] = Value::String(parsed.value),
                TargetKind::User => out["uid"] = Value::String(parsed.value),
                TargetKind::Room => out["room_id"] = Value::String(parsed.value),
                TargetKind::Series => out["series_id"] = Value::String(parsed.value),
                TargetKind::Mix => out["mix_id"] = Value::String(parsed.value),
                TargetKind::Challenge | TargetKind::Keyword => {
                    out["name"] = Value::String(parsed.value)
                }
                TargetKind::Short => {}
            }
            return Ok(out);
        }
        Err(Error::new(400, "短链跳转次数过多"))
    }
}

/// 服务端偶发把 chunk 帧原样留在 body 里（无 Transfer-Encoding），按帧剥一层。
fn dechunk(b: &[u8]) -> Vec<u8> {
    if b.first().is_some_and(|c| c.is_ascii_hexdigit()) {
        let mut out = Vec::new();
        let mut i = 0;
        while i < b.len() {
            let Some(rel) = b[i..].windows(2).position(|w| w == b"\r\n") else {
                break;
            };
            let line = &b[i..i + rel];
            let Ok(n) = usize::from_str_radix(
                String::from_utf8_lossy(line)
                    .split(';')
                    .next()
                    .unwrap_or(""),
                16,
            ) else {
                break;
            };
            if n == 0 {
                break;
            }
            let s = i + rel + 2;
            if s + n > b.len() {
                break;
            }
            out.extend_from_slice(&b[s..s + n]);
            i = s + n + 2;
        }
        if !out.is_empty() {
            return out;
        }
    }
    b.to_vec()
}

/// 响应可能是多份 JSON 文档拼接（服务端偶发），取 status=0 且关键字段齐全的那份。
/// `required` 用 `|` 分隔同一字段的别名，用 `.` 表示嵌套；缺字段说明不是可用的成功包。
fn json_of(b: &[u8], required: &[&str]) -> Result<Value> {
    let d = dechunk(b);
    let mut de = serde_json::Deserializer::from_slice(&d).into_iter::<Value>();
    let mut first: Option<Value> = None;
    let mut best: Option<Value> = None;
    let mut bad: Option<Value> = None;
    while let Some(Ok(v)) = de.next() {
        let code = v.get("status_code").and_then(|x| x.as_i64()).unwrap_or(0);
        let message = v.get("status_msg").and_then(Value::as_str).unwrap_or("");
        let ok = code == 0 && !status_message_is_error(message);
        let valid = ok && required.iter().all(|path| path_exists(&v, path));
        if valid && !required.iter().all(|path| path_is_empty(&v, path)) {
            best = Some(v.clone());
        }
        if !ok && bad.is_none() {
            bad = Some(v.clone());
        }
        if first.is_none() && valid {
            first = Some(v);
        }
    }
    if let Some(v) = best.or(first) {
        if let Some(b) = bad.filter(|_| required.iter().all(|path| path_is_empty(&v, path))) {
            let code = b.get("status_code").and_then(|x| x.as_i64()).unwrap_or(0);
            let message = b.get("status_msg").and_then(Value::as_str).unwrap_or("");
            return Err(Error::new(
                status_http_code(code),
                format!("status_code={code} {message}"),
            ));
        }
        Ok(v)
    } else if let Some(b) = bad {
        let code = b.get("status_code").and_then(|x| x.as_i64()).unwrap_or(0);
        let message = b.get("status_msg").and_then(Value::as_str).unwrap_or("");
        Err(Error::new(
            status_http_code(code),
            format!("status_code={code} {message}"),
        ))
    } else {
        let snip: String =
            String::from_utf8_lossy(&d[..d.len().min(120)]).replace(['\r', '\n'], " ");
        Err(Error::new(
            502,
            format!(
                "响应缺少关键字段：{}（{}B: {}）",
                required.join(","),
                d.len(),
                snip
            ),
        ))
    }
}

fn path_exists(v: &Value, path: &str) -> bool {
    path.split('|').any(|alt| {
        let mut cur = v;
        alt.split('.').all(|segment| {
            let Some(next) = cur.get(segment) else {
                return false;
            };
            cur = next;
            true
        })
    })
}

fn path_is_empty(v: &Value, path: &str) -> bool {
    path.split('|').all(|alt| {
        let mut cur = v;
        for segment in alt.split('.') {
            let Some(next) = cur.get(segment) else {
                return true;
            };
            cur = next;
        }
        match cur {
            Value::Array(a) => a.is_empty(),
            Value::Object(o) => o.is_empty(),
            Value::Null => true,
            _ => false,
        }
    })
}

fn check_status(v: &Value) -> Result<()> {
    let code = v
        .get("status_code")
        .or_else(|| v.get("status"))
        .and_then(|x| x.as_i64())
        .unwrap_or(0);
    let message = v.get("status_msg").and_then(Value::as_str).unwrap_or("");
    if code == 0 && !status_message_is_error(message) {
        Ok(())
    } else {
        Err(Error::new(
            status_http_code(code),
            format!("status_code={code} {}", message),
        ))
    }
}

/// 详情接口以 200 + `filter_detail` 表示作品被内容级过滤。
fn filter_error(f: &Value) -> Error {
    let reason = str_of(&f["filter_reason"]);
    let hint = ["detail_msg", "notice"]
        .iter()
        .map(|k| str_of(&f[k]))
        .find(|s| !s.is_empty());
    let mut msg = match reason.as_str() {
        "core_dep" => "作品不可访问（已删除、私密，或链接是合集 ID）".to_string(),
        r => format!("作品不可访问（{r}）"),
    };
    if let Some(h) = hint {
        msg.push('：');
        msg.push_str(&h);
    }
    Error::new(404, msg)
}

fn status_message_is_error(message: &str) -> bool {
    let m = message.trim().to_ascii_lowercase();
    let positive = ["success", "ok", "成功"]
        .iter()
        .any(|word| m.contains(word));
    !m.is_empty()
        && !positive
        && [
            "match",
            "invalid",
            "error",
            "fail",
            "不合法",
            "失败",
            "错误",
            "频繁",
            "限流",
            "too many",
            "签名",
            "校验",
            "forbidden",
            "unauthorized",
            "denied",
        ]
        .iter()
        .any(|needle| m.contains(needle))
}

/// 服务端按新设备风控：重试其他身份即可，不把号打死。
fn is_warming(e: &Error) -> bool {
    e.message.contains("-99999") || e.message.contains("风控")
}

fn status_http_code(code: i64) -> u16 {
    match code {
        -99999 | 2483 | 2484 => 429,
        401 => 401,
        403 => 403,
        404 | 30001 => 404,
        5 | 10001..=10999 => 400,
        c if c > 0 && c <= u16::MAX as i64 => c as u16,
        _ => 400,
    }
}

fn comment(c: &Value) -> Value {
    json!({"cid": str_of(&c["cid"]), "text": str_of(&c["text"]), "aweme_id": str_of(&c["aweme_id"]),
        "create_time": num_of(&c["create_time"]), "digg_count": num_of(&c["digg_count"]),
        "reply_comment_total": num_of(&c["reply_comment_total"]), "user": author(&c["user"]), "raw": c})
}

fn series_item(s: &Value) -> Value {
    let episodes = num_of(&s["stats"]["total_episode"]).max(num_of(&s["episode_count"]));
    let charge = s["is_charge_series"].as_bool().unwrap_or(false)
        || num_of(&s["charge_info"]["charge_type"]) != 0;
    json!({"series_id": str_of(&s["series_id"]), "series_name": str_of(&s["series_name"]),
        "cover": url0(&s["cover_url"]["url_list"]), "episode_count": episodes,
        "is_charge": charge, "intro": if str_of(&s["desc"]).is_empty() { str_of(&s["description"]) } else { str_of(&s["desc"]) }, "raw": s})
}

fn room(r: &Value) -> Value {
    let su = &r["stream_url"];
    json!({"room_id": str_of(&r["id_str"]), "title": str_of(&r["title"]),
        "cover": url0(&r["cover"]["url_list"]), "owner": author(&r["owner"]),
        "user_count": num_of(&r["user_count"]),
        "hls": su["hls_pull_url_map"].as_object().and_then(|m| m.values().next()).and_then(|v| v.as_str()).unwrap_or_default(),
        "flv": su["flv_pull_url"].as_object().and_then(|m| m.values().next()).and_then(|v| v.as_str()).unwrap_or_default(),
        "raw": r})
}

#[allow(dead_code)]
fn users_of(doc: &Value) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::new();
    fn walk(o: &Value, out: &mut Vec<Value>) {
        match o {
            Value::Object(m) => {
                if m.get("uid").is_some()
                    && m.get("nickname").is_some()
                    && m.get("sec_uid").is_some()
                {
                    out.push(author(o));
                }
                for v in m.values() {
                    walk(v, out);
                }
            }
            Value::Array(a) => {
                for v in a {
                    walk(v, out);
                }
            }
            _ => {}
        }
    }
    for card in doc["business_data"].as_array().cloned().unwrap_or_default() {
        walk(&card["data"], &mut out);
    }
    out.sort_by(|a, b| a["uid"].as_str().cmp(&b["uid"].as_str()));
    out.dedup_by(|a, b| a["uid"] == b["uid"]);
    out
}

fn feed_item(it: &[(u32, crate::pb::Value)]) -> Value {
    let s = |no| {
        crate::pb::one(it, no)
            .map(crate::pb::text)
            .unwrap_or_default()
    };
    let n = |no| {
        crate::pb::one(it, no)
            .and_then(crate::pb::as_u64)
            .unwrap_or(0)
    };
    let msg = |no| match crate::pb::one(it, no) {
        Some(crate::pb::Value::Message(m)) => m.clone(),
        _ => Vec::new(),
    };
    let au = msg(4);
    let vi = msg(7);
    let st = msg(10);
    let all_urls: Vec<String> = crate::pb::fields(&vi, 1)
        .flat_map(|p| match p {
            crate::pb::Value::Message(m) => crate::pb::fields(m, 2)
                .map(crate::pb::text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect();
    // 图集型 feed 条目只有音轨，别当视频播。
    let urls: Vec<String> = all_urls
        .iter()
        .filter(|u| !u.contains("/play/music/"))
        .cloned()
        .collect();
    let covers: Vec<String> = match crate::pb::one(&vi, 2) {
        Some(crate::pb::Value::Message(m)) => {
            crate::pb::fields(m, 2).map(crate::pb::text).collect()
        }
        _ => Vec::new(),
    };
    let stat = |no| crate::pb::one(&st, no).and_then(crate::pb::as_u64);
    let raw = json!({
        "aweme_id": s(1), "desc": s(2), "create_time": n(3), "share_url": s(8),
        "author": {"uid": crate::pb::one(&au,1).map(crate::pb::text).unwrap_or_default(),
                   "nickname": crate::pb::one(&au,3).map(crate::pb::text).unwrap_or_default(),
                   "avatar_thumb": {"url_list": match crate::pb::one(&au,6) { Some(crate::pb::Value::Message(m)) => crate::pb::fields(m,2).map(crate::pb::text).collect::<Vec<_>>(), _ => Vec::new() }}},
        "video": {"cover": {"url_list": covers}, "play_addr": {"url_list": urls},
                  "width": crate::pb::one(&vi,1).and_then(|p| match p { crate::pb::Value::Message(m) => crate::pb::one(m,3).and_then(crate::pb::as_u64), _ => None }),
                  "height": crate::pb::one(&vi,1).and_then(|p| match p { crate::pb::Value::Message(m) => crate::pb::one(m,4).and_then(crate::pb::as_u64), _ => None })},
        "statistics": {"comment_count": stat(2), "digg_count": stat(3), "share_count": stat(4), "play_count": stat(6)},
        "kind": if urls.is_empty() { "gallery" } else { "video" },
    });
    item(&raw)
}

fn feed_params(id: &Identity, fl: &str, count: usize) -> BTreeMap<String, String> {
    let mut p = media(id, fl);
    let fixed = [
        ("type", "0"),
        ("max_cursor", "0"),
        ("min_cursor", "0"),
        ("volume", "0.33"),
        ("pull_type", "0"),
        ("is_cold_start", "0"),
        ("address_book_access", "2"),
        ("gps_access", "1"),
        ("user_avatar_shrink", "96_96"),
        ("screen_type", "1"),
        ("need_relieve_aweme", "0"),
        ("filter_warn", "0"),
        ("req_from", "enter_auto"),
        ("action_mask", "-1"),
        ("action_mask_detail", "{}"),
        ("action_mask_info", "{}"),
        ("ad_extra", ""),
        ("teen_protector_vote_aweme_count", "0"),
        ("last_teen_protector_vote_aweme_interval", "0"),
        ("sp", "-1"),
        ("is_order_flow", "1"),
        ("pitaya_type", "0"),
        ("location_permission", "true"),
        ("font_category", "2"),
        ("feed_recommend_req_index", "1"),
        ("audio_value", "33"),
        ("launch_times", "1"),
        ("recent_items", ""),
        ("play_style", "0"),
        ("filter_live_cell", "true"),
        ("addiction_info", ""),
        ("client_extra", ""),
        ("from_push", "0"),
        ("show_unread_entrance", "0"),
        ("show_jump_tab_list", ""),
        ("danmaku_switch_status", "0"),
        ("elder_mode", "0"),
        ("need_hotlist_card", "false"),
        ("mock_info", ""),
        ("life_card_accept", "0"),
        ("life_card_unexposed_cards", ""),
        (
            "nearby_extra_data",
            r#"{"nearby_tab_name":"同城","has_groupon_tab":false}"#,
        ),
        ("tab_guide", ""),
        ("error_monitor_info", "[]"),
        ("splash_cid", ""),
        ("splash_type", "-1"),
        ("download_sdk_info", "{}"),
        ("cached_item_num", "0"),
        ("last_ad_show_interval", "-1"),
        ("ug_ab_params", "{}"),
        ("preload_aweme_ids", ""),
        ("enable_url_compress", "1"),
        ("recent_gids", ""),
        ("recent_gids_first_time", "null"),
        ("md", "0"),
        ("pull_scene", "did"),
        ("replace_count", "20"),
        ("unconsumed_vids", "[]"),
    ];
    for (k, v) in fixed {
        p.insert(k.into(), v.into());
    }
    p.insert("count".into(), count.to_string());
    p
}

/// 首屏请求体；时间随 60s 桶走，同一桶内签名可复用。
fn feed_body() -> String {
    let begin = (params::tick_ms() / 1000) as i64 - 60;
    format!(
        r#"{{"client_extra":"{{\"charging\":1,\"qoe\":87,\"har_info\":{{}},\"launch_mode\":\"EXTERNAL\",\"session_begin_time\":{begin},\"feed_fetch_count\":0,\"aweme_autoplay\":0,\"pitaya_param_for_sati\":\"\",\"feed_recommend_req_index_in_life\":1,\"is_luna_install\":0,\"is_luna_download\":0}}"}}"#
    )
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
fn now_secs() -> i64 {
    now_ms() / 1000
}
fn decode(s: &str) -> String {
    url::form_urlencoded::parse(format!("x={s}").as_bytes())
        .next()
        .map(|(_, v)| v.into_owned())
        .unwrap_or_else(|| s.to_string())
}

#[allow(dead_code)]
fn _unused(_: Map<String, Value>) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn opaque_gear(name: &str, rate: i64, hevc: i64) -> Value {
        json!({
            "gear_name": name, "bit_rate": rate, "is_h265": hevc, "is_bytevc1": hevc,
            "play_addr": {"url_list": [format!("https://cdn.example/{name}/?mime_type=video_mp4")],
                "width": 1280, "height": 720}
        })
    }

    #[test]
    fn recognizes_warming_errors() {
        assert!(is_warming(&Error::new(429, "status_code=-99999 风控")));
        assert!(is_warming(&Error::new(
            429,
            "账号仍在服务端风控期（直播空包）"
        )));
        assert!(!is_warming(&Error::new(429, "身份池已耗尽")));
    }

    #[test]
    fn self_check_signs_once() {
        let api = Api::new(dysign::default_resources()).unwrap();
        api.self_check().unwrap();
    }

    #[test]
    fn default_sign_backend_is_native_and_can_switch() {
        let api = Api::new(dysign::default_resources()).unwrap();
        let env = std::env::var("DYSIGN_IMPL")
            .ok()
            .and_then(|s| Backend::parse(&s));
        assert_eq!(api.sign_backend(), env.unwrap_or(Backend::Native));
        api.set_sign_backend(Backend::Native);
        if env.is_none() {
            assert_eq!(api.sign_backend(), Backend::Native);
        }
        let info = api.sign_info();
        assert_eq!(info.ok, [0, 0]);
        assert_eq!(info.err, [0, 0]);
    }

    #[test]
    fn preview_uses_h264_even_when_opaque_hevc_url_has_lower_bitrate() {
        let video = json!({"bit_rate": [
            opaque_gear("normal_720_0", 790358, 0),
            opaque_gear("adapt_lower_540_1", 302171, 1),
            opaque_gear("adapt_low_540_0", 420500, 0),
        ]});
        let list = play_gears(&video, true, true, None);
        assert_eq!(
            list.iter().map(|g| g.name.as_str()).collect::<Vec<_>>(),
            ["normal_720_0", "adapt_low_540_0"]
        );
        // 支持 HEVC 时可精确指定；不支持时不会返回不兼容或其他档位。
        let list = play_gears(&video, true, false, None);
        assert_eq!(list[0].name, "normal_720_0");
        let list = play_gears(&video, true, false, Some("adapt_lower_540_1"));
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "adapt_lower_540_1");
        assert!(play_gears(&video, true, true, Some("adapt_lower_540_1")).is_empty());
        let list = play_gears(&video, false, false, Some("adapt_lower_540_1"));
        assert_eq!(list[0].name, "adapt_lower_540_1");
        assert!(!list[0].h264);
    }

    #[test]
    fn codec_detection_reads_flags_and_address_metadata() {
        for flags in [
            json!({"is_h265": 1}),
            json!({"is_bytevc1": 1}),
            json!({"is_h265": true}),
            json!({"is_bytevc1": "1"}),
        ] {
            assert!(!is_h264(&flags), "{flags}");
        }
        for marker in ["HVC1", "hev1", "hevc", "h265", "bytevc1"] {
            assert!(!is_h264(
                &json!({"play_addr": {"url_key": format!("id_{marker}_720p")}})
            ));
            assert!(!is_h264(
                &json!({"play_addr": {"url_list": [format!("https://cdn.example/{marker}/")]}})
            ));
        }
        assert!(is_h264(&opaque_gear("low_540_0", 624119, 0)));
    }

    #[test]
    fn preview_falls_back_to_explicit_h264_address_without_using_hevc() {
        let mut video = json!({
            "is_bytevc1": 1,
            "bit_rate": [opaque_gear("low_540_1", 302171, 1)],
            "play_addr": {"url_list": ["https://cdn.example/default/"]},
            "play_addr_h264": {"url_list": ["https://cdn.example/compatible/"]},
        });
        assert!(play_gears(&video, true, true, None).is_empty());
        assert_eq!(
            play_fallbacks(&video, true),
            ["https://cdn.example/compatible/"]
        );
        assert_eq!(
            play_fallbacks(&video, false),
            ["https://cdn.example/default/"]
        );
        video.as_object_mut().unwrap().remove("play_addr_h264");
        assert!(play_fallbacks(&video, true).is_empty());
        video["is_bytevc1"] = json!(0);
        assert_eq!(
            play_fallbacks(&video, true),
            ["https://cdn.example/default/"]
        );
    }

    #[test]
    fn download_keeps_highest_resolution_independent_of_windows_preview() {
        let mut hd = opaque_gear("high_1440_1", 4_000_000, 1);
        hd["play_addr"]["width"] = json!(2560);
        hd["play_addr"]["height"] = json!(1440);
        let video = json!({"bit_rate": [opaque_gear("normal_720_0", 790358, 0), hd]});
        assert_eq!(play_gears(&video, true, true, None)[0].name, "normal_720_0");
        assert_eq!(
            play_gears(&video, false, false, None)[0].name,
            "high_1440_1"
        );
        assert_eq!(
            play_gears(&video, false, false, Some("normal_720_0"))[0].name,
            "normal_720_0"
        );
    }

    #[test]
    fn token_bucket_is_per_identity_and_endpoint() {
        let sensitive = "/video/1/comments";
        let detail = "/video/1";
        assert!(token_rate(sensitive).1 < token_rate(detail).1);
        let mut a = Bucket {
            tokens: 1.0,
            at: Instant::now(),
        };
        let mut b = Bucket {
            tokens: 1.0,
            at: Instant::now(),
        };
        assert!(take_token(&mut a, sensitive));
        assert!(!take_token(&mut a, sensitive));
        assert!(take_token(&mut b, sensitive));
        let mut c = Bucket {
            tokens: 3.0,
            at: Instant::now(),
        };
        assert!(take_token(&mut c, detail));
        assert!(take_token(&mut c, detail));
    }

    #[test]
    fn status_message_classification() {
        assert!(!status_message_is_error(""));
        assert!(!status_message_is_error("success"));
        assert!(!status_message_is_error("ok"));
        assert!(!status_message_is_error("请求成功"));
        assert!(status_message_is_error("参数不合法"));
        assert!(status_message_is_error("请求失败"));
        assert!(status_message_is_error("too many requests"));
        assert!(status_message_is_error("url doesn't match"));
        assert!(status_message_is_error("签名校验失败"));
    }

    #[test]
    fn json_requires_key_and_prefers_non_empty_document() {
        let body = br#"{"status_code":0,"comments":[{"cid":"1"}]}
{"status_code":-99999,"status_msg":"gate"}"#;
        let v = json_of(body, &["comments"]).unwrap();
        assert_eq!(v["comments"][0]["cid"], "1");

        let body = br#"{"status_code":0,"comments":[]}
{"status_code":-99999,"status_msg":"gate"}"#;
        assert_eq!(json_of(body, &["comments"]).unwrap_err().status, 429);

        let body = br#"{"status_code":0,"status_msg":""}"#;
        assert_eq!(json_of(body, &["danmaku_list"]).unwrap_err().status, 502);
    }

    #[test]
    fn retry_classification() {
        assert!(Error::new(0, "").retryable());
        assert!(Error::new(403, "").retryable());
        assert!(Error::new(429, "").retryable());
        assert!(!Error::new(400, "").retryable());
        assert!(!Error::new(404, "").retryable());
    }

    #[test]
    fn target_parsing_accepts_share_text_and_prefixes() {
        let t = parse_target("7.89 复制打开抖音 https://v.douyin.com/iRNBho6u/").unwrap();
        assert_eq!(t.kind, TargetKind::Short);
        let t = parse_target("video:7686828011362045190").unwrap();
        assert_eq!(t.kind, TargetKind::Video);
        let t = parse_target("MS4wLjABAAAAy0VQDxVXKNnFRLf4kctvrvo5fRCFtBDo17GVLzBzcxE").unwrap();
        assert_eq!(t.kind, TargetKind::User);
    }

    #[test]
    fn target_parsing_rejects_unknown_link() {
        assert!(parse_target("https://example.com/x").is_err());
    }
}
