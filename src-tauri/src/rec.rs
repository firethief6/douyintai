//! 直播录制：按 HLS 分片原样落盘，累积 m3u8 索引；不依赖 ffmpeg。
//! 停录/崩溃后目录仍是可播的分片序列，播放器按 index.m3u8 连续播放。
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::Manager as _;
use tauri::{AppHandle, Emitter};

pub struct Inner {
    stop: AtomicBool,
    bytes: AtomicU64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub room: String,
    pub dir: String,
    pub state: String, // run | done | fail
    pub bytes: u64,
    pub parts: u64,
    pub started: u64,
    pub err: String,
}

pub struct Manager {
    live: Mutex<HashMap<String, Arc<Inner>>>,
    meta: Mutex<HashMap<String, Session>>,
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    pub fn new() -> Self {
        Self {
            live: Mutex::new(HashMap::new()),
            meta: Mutex::new(HashMap::new()),
        }
    }
    fn update<F: FnOnce(&mut Session)>(&self, id: &str, f: F) {
        if let Ok(mut m) = self.meta.lock() {
            if let Some(s) = m.get_mut(id) {
                f(s);
            }
        }
    }
    pub fn list(&self) -> Vec<Session> {
        let mut v: Vec<Session> = self
            .meta
            .lock()
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default();
        v.sort_by_key(|a| std::cmp::Reverse(a.started));
        v
    }
    pub fn stop(&self, id: &str) -> bool {
        let live = match self.live.lock() {
            Ok(l) => l,
            Err(_) => return false,
        };
        match live.get(id) {
            Some(s) => {
                s.stop.store(true, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }
}

fn safe(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn disk_free(path: &Path) -> u64 {
    crate::paths::available(path)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn stamp(secs: u64) -> String {
    // 东八区；civil-from-days，不引 chrono。
    let t = secs + 8 * 3600;
    let z = (t / 86400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let y = yoe + era * 400 + (m <= 2) as i64;
    format!(
        "{y:04}{m:02}{d:02}-{:02}{:02}{:02}",
        (t / 3600) % 24,
        (t / 60) % 60,
        t % 60
    )
}

/// 拉流地址失效时重新解析；返回新的播放清单地址。
pub type Refresh = Arc<dyn Fn() -> Result<String, String> + Send + Sync>;

pub struct Options {
    pub id: String,
    pub dir: PathBuf,
    pub max_secs: Option<u64>,
    pub max_bytes: Option<u64>,
    pub retain_bytes: Option<u64>,
    pub refresh: Option<Refresh>,
}

pub fn start(app: AppHandle, url: String, dir: String, room: String) -> Result<Session, String> {
    let started = now_secs();
    let out = crate::paths::expand(&dir)
        .join("直播")
        .join(safe(&room))
        .join(format!("{}_{}", stamp(started), safe(&room)));
    start_opts(
        app,
        url,
        room,
        Options {
            id: format!("rec-{started}-{}", UuidLike::make()),
            dir: out,
            max_secs: None,
            max_bytes: None,
            retain_bytes: None,
            refresh: None,
        },
    )
}

struct UuidLike;
impl UuidLike {
    fn make() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

pub fn start_opts(
    app: AppHandle,
    url: String,
    room: String,
    opts: Options,
) -> Result<Session, String> {
    let mgr = app.state::<Manager>();
    if mgr
        .list()
        .iter()
        .any(|s| s.room == room && matches!(s.state.as_str(), "queued" | "running"))
    {
        return Err("这个房间已经在录".into());
    }
    let started = now_secs();
    crate::logging::info("rec", format!("开始录制 {room} -> {}", opts.dir.display()));
    std::fs::create_dir_all(&opts.dir).map_err(|e| e.to_string())?;
    let id = opts.id.clone();
    let s = Session {
        id: id.clone(),
        room: room.clone(),
        dir: opts.dir.to_string_lossy().to_string(),
        state: "running".into(),
        bytes: 0,
        parts: 0,
        started,
        err: String::new(),
    };
    if let Ok(mut m) = mgr.meta.lock() {
        m.insert(id.clone(), s.clone());
    }
    let inner = Arc::new(Inner {
        stop: AtomicBool::new(false),
        bytes: AtomicU64::new(0),
    });
    if let Ok(mut l) = mgr.live.lock() {
        l.insert(id.clone(), inner.clone());
    }
    // 录制是长任务，独占线程和单线程运行时；避免阻塞共享 tokio worker。
    std::thread::spawn(move || {
        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => {
                rt.block_on(run(app.clone(), id.clone(), url, opts, inner));
            }
            Err(e) => finish(&app, &id, "fail", e.to_string()),
        }
    });
    Ok(s)
}

pub fn get(app: &AppHandle, id: &str) -> Option<Session> {
    app.state::<Manager>().meta.lock().ok()?.get(id).cloned()
}

fn emit(app: &AppHandle, id: &str) {
    if let Ok(m) = app.state::<Manager>().meta.lock() {
        if let Some(s) = m.get(id) {
            let _ = app.emit("rec-change", s.clone());
        }
    }
}

fn finish(app: &AppHandle, id: &str, state: &str, err: String) {
    let mgr = app.state::<Manager>();
    mgr.update(id, |s| {
        s.state = state.into();
        s.err = err;
    });
    if let Ok(mut l) = mgr.live.lock() {
        l.remove(id);
    }
    if let Ok(m) = mgr.meta.lock() {
        if let Some(s) = m.get(id) {
            let dir = PathBuf::from(&s.dir);
            let _ = std::fs::OpenOptions::new()
                .append(true)
                .open(dir.join("index.m3u8"))
                .and_then(|mut f| std::io::Write::write_all(&mut f, b"#EXT-X-ENDLIST\n"));
        }
    }
    emit(app, id);
}

fn base_dir(u: &reqwest::Url) -> reqwest::Url {
    let mut b = u.clone();
    b.set_query(None);
    b.set_fragment(None);
    let path = b.path().to_string();
    match path.rfind('/') {
        Some(i) => {
            b.set_path(&path[..=i]);
            b
        }
        None => b,
    }
}

fn segments(body: &str, base: &reqwest::Url) -> Vec<reqwest::Url> {
    body.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| base.join(l).ok())
        .collect()
}

async fn fetch_text(c: &reqwest::Client, u: &str) -> Result<(String, reqwest::Url), String> {
    let r = c
        .get(u)
        .header("Referer", "https://www.douyin.com/")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !r.status().is_success() {
        return Err(format!("HTTP {}", r.status()));
    }
    let final_url = r.url().clone();
    Ok((r.text().await.map_err(|e| e.to_string())?, final_url))
}

async fn run(app: AppHandle, id: String, mut url: String, opts: Options, inner: Arc<Inner>) {
    let dir = opts.dir;
    let mgr = app.state::<Manager>();
    let c = match crate::net::async_builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/126 Safari/537.36")
        .build()
    {
        Ok(c) => c,
        Err(e) => return finish(&app, &id, "fail", e.to_string()),
    };
    let playlist = dir.join("index.m3u8");
    let _ = std::fs::write(&playlist, "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:4\n#EXT-X-PLAYLIST-TYPE:EVENT\n#EXT-X-MEDIA-SEQUENCE:0\n");
    let mut seen: std::collections::HashSet<String> = Default::default();
    let mut n: u64 = 0;
    let mut last_tick = Instant::now();
    let mut last_disk = Instant::now() - Duration::from_secs(60);
    let started = Instant::now();
    let mut pull_failures = 0u32;

    'outer: loop {
        if inner.stop.load(Ordering::SeqCst) {
            break;
        }
        if opts
            .max_secs
            .is_some_and(|v| v > 0 && started.elapsed() > Duration::from_secs(v))
        {
            crate::logging::info("rec", format!("{id} 到时长上限"));
            finish(&app, &id, "done", "达到录制时长上限，已停".into());
            return;
        }
        if last_disk.elapsed() > Duration::from_secs(30) {
            last_disk = Instant::now();
            let keep = opts.retain_bytes.unwrap_or(20 * (1 << 30));
            if disk_free(&dir) < keep {
                crate::logging::error("rec", format!("{id} 磁盘不足"));
                finish(&app, &id, "done", "磁盘保留空间不足，已停".into());
                return;
            }
        }
        let (mut text, mut base) = match fetch_text(&c, &url).await {
            Ok(v) => v,
            Err(e) => {
                pull_failures += 1;
                crate::logging::error("rec", format!("{id} 取清单失败 {pull_failures}/6：{e}"));
                if pull_failures == 1 || pull_failures == 3 {
                    if let Some(refresh) = &opts.refresh {
                        let refresh = refresh.clone();
                        // 同步 API 客户端自带阻塞运行时，必须在阻塞线程外调用。
                        match tokio::task::spawn_blocking(move || refresh()).await {
                            Ok(Ok(fresh)) => {
                                url = fresh;
                                crate::logging::info("rec", format!("{id} 已刷新拉流地址"));
                            }
                            Ok(Err(e)) => {
                                crate::logging::warn("rec", format!("{id} 刷新拉流地址失败：{e}"))
                            }
                            Err(e) => crate::logging::warn(
                                "rec",
                                format!("{id} 刷新拉流地址任务失败：{e}"),
                            ),
                        }
                    }
                }
                if pull_failures >= 6 {
                    let state = if n == 0 { "fail" } else { "done" };
                    finish(&app, &id, state, format!("开播中断：{e}"));
                    return;
                }
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }
        };
        pull_failures = 0;
        base = base_dir(&base);
        if let Some(line) = text
            .lines()
            .map(|l| l.trim())
            .find(|l| !l.is_empty() && !l.starts_with('#'))
        {
            if text.contains("#EXT-X-STREAM-INF") {
                if let Ok(abs) = base.join(line) {
                    if let Ok((t2, b2)) = fetch_text(&c, abs.as_str()).await {
                        text = t2;
                        base = base_dir(&b2);
                    }
                }
            }
        }
        for seg in segments(&text, &base) {
            if inner.stop.load(Ordering::SeqCst) {
                break 'outer;
            }
            let key = seg.to_string();
            if !seen.insert(key) {
                continue;
            }
            if n >= 200_000 {
                finish(&app, &id, "done", "分片数量到上限，已停".into());
                return;
            }
            n += 1;
            let ext = seg
                .path()
                .rsplit('.')
                .next()
                .filter(|e| e.len() <= 5)
                .unwrap_or("ts")
                .to_string();
            let name = format!("{n:05}.{ext}");
            let r = c
                .get(seg.as_str())
                .header("Referer", "https://www.douyin.com/")
                .send()
                .await;
            let Ok(r) = r else {
                n -= 1;
                continue;
            };
            if !r.status().is_success() {
                n -= 1;
                continue;
            }
            let b = match r.bytes().await {
                Ok(b) => b,
                Err(_) => {
                    n -= 1;
                    continue;
                }
            };
            if let Err(e) = std::fs::write(dir.join(&name), &b) {
                crate::logging::error("rec", format!("{id} 写分片 {name} 失败：{e}"));
                n -= 1;
                continue;
            }
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open(&playlist) {
                let _ =
                    std::io::Write::write_all(&mut f, format!("#EXTINF:4.0,\n{name}\n").as_bytes());
            }
            inner.bytes.fetch_add(b.len() as u64, Ordering::SeqCst);
            if opts
                .max_bytes
                .is_some_and(|v| v > 0 && inner.bytes.load(Ordering::SeqCst) >= v)
            {
                finish(&app, &id, "done", "达到文件大小上限，已停".into());
                return;
            }
            mgr.update(&id, |s| {
                s.bytes += b.len() as u64;
                s.parts += 1;
            });
            last_tick = Instant::now();
            if n % 4 == 0 {
                crate::logging::info(
                    "rec",
                    format!("{id} 第{n}片 累计{}B", inner.bytes.load(Ordering::SeqCst)),
                );
                emit(&app, &id);
            }
        }
        let wait = if last_tick.elapsed() > Duration::from_secs(6) {
            1
        } else {
            2
        };
        tokio::time::sleep(Duration::from_secs(wait)).await;
    }
    finish(&app, &id, "done", String::new());
}

pub fn delete(session_dir: &Path) -> Result<(), String> {
    std::fs::remove_dir_all(session_dir).map_err(|e| e.to_string())
}
