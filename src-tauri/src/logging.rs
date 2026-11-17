//! 进程日志：级别过滤 + 文件轮转 + 内存环 + 推送到运行日志页 + 诊断包导出。
//! 不依赖第三方日志框架；所有入口都容忍写盘失败，日志不能反过来拖垮业务。
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

pub const MAX_BYTES: u64 = 5 * 1024 * 1024;
pub const KEEP: usize = 5;
pub const RING: usize = 2000;
pub const ISSUES: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "debug" => Some(Self::Debug),
            "info" => Some(Self::Info),
            "warn" | "warning" => Some(Self::Warn),
            "error" | "err" => Some(Self::Error),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    fn stored(self) -> String {
        match self {
            Self::Error => "err".into(),
            other => other.label().into(),
        }
    }
}

static LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);

#[derive(Clone, Serialize)]
pub struct Line {
    pub at: u64,
    pub level: String,
    pub scope: String,
    pub text: String,
}

struct Logger {
    dir: PathBuf,
    started: Instant,
    file: Mutex<Option<File>>,
    ring: Mutex<VecDeque<Line>>,
    issues: Mutex<VecDeque<Line>>,
    app: OnceLock<AppHandle>,
}

static LOG: OnceLock<Logger> = OnceLock::new();

fn logger() -> Option<&'static Logger> {
    LOG.get()
}

fn level() -> Level {
    match LEVEL.load(Ordering::Relaxed) {
        0 => Level::Debug,
        2 => Level::Warn,
        3 => Level::Error,
        _ => Level::Info,
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn stamp(ms: u64) -> String {
    // 东八区时间，避免为日志引入 chrono。
    let t = ms / 1000 + 8 * 3600;
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
        "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02}.{:03}",
        (t / 3600) % 24,
        (t / 60) % 60,
        t % 60,
        ms % 1000
    )
}

fn rotate(dir: &Path, file: &mut Option<File>) {
    let path = dir.join("app.log");
    let too_big = std::fs::metadata(&path)
        .map(|m| m.len() >= MAX_BYTES)
        .unwrap_or(false);
    if !too_big {
        return;
    }
    *file = None;
    for i in (1..KEEP).rev() {
        let from = dir.join(format!("app.{i}.log"));
        let to = dir.join(format!("app.{}.log", i + 1));
        let _ = std::fs::rename(from, to);
    }
    let _ = std::fs::rename(&path, dir.join("app.1.log"));
}

const SECRET_KEYS: [&str; 13] = [
    "msToken",
    "a_bogus",
    "X-Bogus",
    "x-bogus",
    "token",
    "access_token",
    "sessionid",
    "sessionid_ss",
    "ttwid",
    "odin_tt",
    "cookie",
    "Cookie",
    "x-tt-dt",
];

fn redact_param(out: &mut String, key: &str) {
    let needle = format!("{key}=");
    let mut from = 0;
    while let Some(pos) = out[from..].find(&needle) {
        let start = from + pos + needle.len();
        let end = out[start..]
            .find(['&', ' ', '\t', '"', '\'', '>', ')'])
            .map(|n| start + n)
            .unwrap_or(out.len());
        if end > start {
            out.replace_range(start..end, "***");
        }
        from = start + 3;
        if from >= out.len() {
            break;
        }
    }
}

fn redact_proxy_auth(out: &mut String) {
    let mut from = 0;
    while let Some(pos) = out[from..].find("://") {
        let start = from + pos + 3;
        let Some(at) = out[start..].find('@').map(|n| start + n) else {
            break;
        };
        let slash = out[start..at].find('/').map(|n| start + n);
        if slash.is_none() && at > start {
            out.replace_range(start..at, "***");
        }
        from = at + 1;
    }
}

/// 日志会进入公开 Issue；在写入时就抹掉身份令牌和代理账号。
pub fn redact(text: &str) -> String {
    let mut out = text.to_string();
    for key in SECRET_KEYS {
        redact_param(&mut out, key);
    }
    redact_proxy_auth(&mut out);
    out
}

fn make_line(level: Level, scope: &str, text: &str) -> Line {
    Line {
        at: now_ms(),
        level: level.stored(),
        scope: scope.to_string(),
        text: redact(&text.replace(['\r', '\n'], " ")),
    }
}

fn remember_issue(line: &Line) {
    let Some(l) = logger() else {
        return;
    };
    if let Ok(mut issues) = l.issues.lock() {
        if issues.len() >= ISSUES {
            issues.pop_front();
        }
        issues.push_back(line.clone());
    }
}

fn write_raw(level: Level, scope: &str, text: &str) {
    let Some(l) = logger() else {
        eprintln!("[{}] [{scope}] {text}", level.label());
        return;
    };
    let line = make_line(level, scope, text);
    if level >= Level::Warn {
        remember_issue(&line);
    }
    if let Ok(mut ring) = l.ring.lock() {
        if ring.len() >= RING {
            ring.pop_front();
        }
        ring.push_back(line.clone());
    }
    if let Ok(mut guard) = l.file.lock() {
        rotate(&l.dir, &mut guard);
        if guard.is_none() {
            *guard = OpenOptions::new()
                .create(true)
                .append(true)
                .open(l.dir.join("app.log"))
                .ok();
        }
        if let Some(f) = guard.as_mut() {
            let _ = writeln!(
                f,
                "{} {:<5} [{}] {}",
                stamp(line.at),
                line.level.to_uppercase(),
                line.scope,
                line.text
            );
        }
    }
    if let Some(app) = l.app.get() {
        let _ = app.emit("log://line", &line);
    }
    if level == Level::Error {
        eprintln!("[{scope}] {}", line.text);
    }
}

fn write(level: Level, scope: &str, text: &str) {
    if level < self::level() {
        // 即使当前级别屏蔽展示和落盘，也保留警告与错误供诊断包取用。
        if level >= Level::Warn {
            remember_issue(&make_line(level, scope, text));
        }
        return;
    }
    write_raw(level, scope, text);
}

pub fn debug(scope: &str, text: impl AsRef<str>) {
    write(Level::Debug, scope, text.as_ref());
}
pub fn info(scope: &str, text: impl AsRef<str>) {
    write(Level::Info, scope, text.as_ref());
}
pub fn warn(scope: &str, text: impl AsRef<str>) {
    write(Level::Warn, scope, text.as_ref());
}
pub fn error(scope: &str, text: impl AsRef<str>) {
    write(Level::Error, scope, text.as_ref());
}

fn http_debug(scope: &str, text: &str) {
    debug(scope, text);
}

fn log_dir(app: &AppHandle) -> PathBuf {
    if let Ok(p) = app.path().app_log_dir() {
        return p;
    }
    crate::paths::home().join(".douyintai").join("logs")
}

#[allow(clippy::result_large_err)]
pub fn init_with_level(app: &AppHandle, saved_level: Option<&str>) -> Result<PathBuf, String> {
    let level = saved_level.and_then(Level::parse).unwrap_or(Level::Info);
    LEVEL.store(level as u8, Ordering::Relaxed);
    dycore::diag::install(http_debug);
    dycore::diag::set_enabled(level == Level::Debug);
    let dir = log_dir(app);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("app.log");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| e.to_string())?;
    let _ = LOG.set(Logger {
        dir,
        started: Instant::now(),
        file: Mutex::new(Some(file)),
        ring: Mutex::new(VecDeque::new()),
        issues: Mutex::new(VecDeque::new()),
        app: OnceLock::new(),
    });
    let _ = logger().map(|l| l.app.set(app.clone()));
    write_raw(
        Level::Info,
        "app",
        &format!(
            "日志启动 pid={} 版本={} 级别={}",
            std::process::id(),
            env!("CARGO_PKG_VERSION"),
            level.label()
        ),
    );
    Ok(path)
}

pub fn init(app: &AppHandle) -> Result<PathBuf, String> {
    init_with_level(app, None)
}

pub fn install_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |p| {
        let loc = p
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();
        error("panic", format!("{loc} {p}"));
        prev(p);
    }));
}

fn visible(line: &Line) -> bool {
    (match line.level.as_str() {
        "debug" => Level::Debug,
        "warn" => Level::Warn,
        "err" | "error" => Level::Error,
        _ => Level::Info,
    }) >= level()
}

pub fn tail(limit: usize) -> Vec<Line> {
    logger()
        .and_then(|l| {
            l.ring.lock().ok().map(|r| {
                r.iter()
                    .rev()
                    .filter(|line| visible(line))
                    .take(limit)
                    .cloned()
                    .collect()
            })
        })
        .unwrap_or_default()
}

pub fn clear() -> Result<(), String> {
    if let Some(l) = logger() {
        if let Ok(mut r) = l.ring.lock() {
            r.clear();
        }
        if let Ok(mut i) = l.issues.lock() {
            i.clear();
        }
        if let Ok(mut g) = l.file.lock() {
            *g = None;
            let _ = std::fs::remove_file(l.dir.join("app.log"));
            for i in 1..=KEEP {
                let _ = std::fs::remove_file(l.dir.join(format!("app.{i}.log")));
            }
            *g = OpenOptions::new()
                .create(true)
                .append(true)
                .open(l.dir.join("app.log"))
                .ok();
        }
    }
    Ok(())
}

pub fn path() -> String {
    logger()
        .map(|l| l.dir.join("app.log").to_string_lossy().to_string())
        .unwrap_or_default()
}

fn log_files() -> Vec<PathBuf> {
    let Some(l) = logger() else {
        return Vec::new();
    };
    let mut files = Vec::new();
    let main = l.dir.join("app.log");
    if main.is_file() {
        files.push(main);
    }
    for i in 1..=KEEP {
        let p = l.dir.join(format!("app.{i}.log"));
        if p.is_file() {
            files.push(p);
        }
    }
    files
}

fn human_settings(app: &AppHandle) -> String {
    let saved = crate::db::db_get_settings(app.clone())
        .ok()
        .flatten()
        .and_then(|j| serde_json::from_str::<serde_json::Value>(&j).ok())
        .unwrap_or_default();
    let text = |key: &str| {
        saved
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
    };
    let flag = |key: &str| saved.get(key).and_then(serde_json::Value::as_bool);
    let home = crate::paths::home();
    let dir = text("downloadDir").replace(&home.to_string_lossy().to_string(), "~");
    format!(
        "主题={}\n签名实现={}\n下载目录={}\n并发={}\n跳过已下载={:?}\n出口代理={:?} · CDN 代理={:?}\n日志级别={}\n",
        text("theme"),
        text("signBackend"),
        dir,
        saved
            .get("concurrency")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0),
        flag("skipExisting"),
        flag("proxyEnabled"),
        flag("proxyCdn"),
        level().label()
    )
}

fn diagnostics(app: &AppHandle, state: &crate::api::Backend) -> String {
    let window = app.get_webview_window("main");
    let webview = tauri::webview_version().unwrap_or_else(|_| "unknown".into());
    let size = window
        .as_ref()
        .and_then(|w| w.inner_size().ok())
        .map(|s| format!("{}x{}", s.width, s.height))
        .unwrap_or_else(|| "unknown".into());
    let scale = window
        .as_ref()
        .and_then(|w| w.scale_factor().ok())
        .unwrap_or(0.0);
    let (proxy_count, proxy_on) = state.api.proxy_status();
    let sign = state.api.sign_info().json();
    let pool = state.api.identity_status();
    let (warns, errors) = logger()
        .and_then(|l| {
            l.issues.lock().ok().map(|r| {
                (
                    r.iter().filter(|x| x.level == "warn").count(),
                    r.iter().filter(|x| x.level == "err").count(),
                )
            })
        })
        .unwrap_or((0, 0));
    let started = logger().map(|l| l.started.elapsed().as_secs()).unwrap_or(0);
    let files = log_files();
    let mut out = String::new();
    out.push_str("抖音台诊断包\n");
    out.push_str("================\n");
    out.push_str(&format!("生成时间：{}\n", stamp(now_ms())));
    out.push_str(&format!(
        "版本：{} · pid={} · 已运行 {}s\n",
        app.package_info().version,
        std::process::id(),
        started
    ));
    out.push_str(&format!(
        "系统：{} {} ({})\n",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::consts::FAMILY
    ));
    out.push_str(&format!(
        "WebView：{} · 窗口 {} · 缩放 {scale}\n",
        webview, size
    ));
    out.push_str(&format!(
        "后端：端内 · 导出时日志级别={} · 内存环 warn={} error={}\n",
        level().label(),
        warns,
        errors
    ));
    out.push_str(&format!(
        "代理：{proxy_count} 条 · 启用={proxy_on}\n签名：{}\n身份池：{}\n",
        serde_json::to_string(&sign).unwrap_or_default(),
        serde_json::to_string(&pool).unwrap_or_default()
    ));
    out.push_str("设置摘要：\n");
    out.push_str(&human_settings(app));
    out.push_str(&format!("\n日志文件：{} 个\n", files.len()));
    for f in &files {
        out.push_str(&format!(
            "- {} ({} B)\n",
            f.file_name().unwrap_or_default().to_string_lossy(),
            std::fs::metadata(f).map(|m| m.len()).unwrap_or(0)
        ));
    }
    out
}

/// 导出诊断包，返回本地路径。默认写到下载目录，文件名带东八区时间。
pub fn export(app: &AppHandle, state: &crate::api::Backend) -> Result<String, String> {
    let dir = app
        .path()
        .download_dir()
        .or_else(|_| app.path().app_log_dir())
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let digest = stamp(now_ms())
        .replace(' ', "-")
        .replace(':', "")
        .replace('.', "-");
    let base = format!("douyintai-diagnostics-{digest}");
    let mut out = dir.join(format!("{base}.zip"));
    let mut n = 2;
    while out.exists() {
        out = dir.join(format!("{base}-{n}.zip"));
        n += 1;
    }
    let part = out.with_extension("zip.part");
    let file = File::create(&part).map_err(|e| e.to_string())?;
    let mut zw = zip::ZipWriter::new(file);
    let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let result = (|| -> Result<(), String> {
        zw.start_file("README.txt", opts)
            .map_err(|e| e.to_string())?;
        zw.write_all(
            b"douyintai diagnostics\n\n1. \xe6\x8a\x8a\xe8\xbf\x99\xe4\xb8\xaa\xe5\x8e\x8b\xe7\xbc\xa9\xe5\x8c\x85\xe9\x99\x84\xe5\x88\xb0 Issue\xe3\x80\x82\n2. \xe5\x86\x99\xe6\x98\x8e\xe6\x93\x8d\xe4\xbd\x9c\xe6\xad\xa5\xe9\xaa\xa4\xe3\x80\x81\xe9\xa2\x84\xe6\x9c\x9f\xe7\xbb\x93\xe6\x9e\x9c\xe5\x92\x8c\xe5\xae\x9e\xe9\x99\x85\xe7\xbb\x93\xe6\x9e\x9c\xe3\x80\x82\n3. \xe6\x97\xa5\xe5\xbf\x97\xe5\x8f\xaf\xe8\x83\xbd\xe5\x8c\x85\xe5\x90\xab\xe4\xbd\x9c\xe5\x93\x81 ID\xe3\x80\x81\xe4\xbd\x9c\xe8\x80\x85\xe5\x90\x8d\xe5\x92\x8c\xe6\x9c\xac\xe6\x9c\xba\xe7\x9b\xae\xe5\xbd\x95\xef\xbc\x9b\xe4\xb8\x8d\xe5\x8c\x85\xe5\x90\xab\xe8\xb4\xa6\xe5\x8f\xb7\xe5\xaf\x86\xe7\xa0\x81\xe5\x92\x8c\xe4\xbb\xa3\xe7\x90\x86\xe8\xb4\xa6\xe5\x8f\xb7\xe3\x80\x82\n4. \xe6\x8e\x92\xe6\x9f\xa5\xe7\xbb\x93\xe6\x9d\x9f\xe5\x90\x8e\xe5\x88\x87\xe5\x9b\x9e info\xe3\x80\x82\n\n---\n\n1. Upload this archive to the issue.\n2. Describe what you did, what you expected, and what happened.\n3. Logs may contain video IDs, author names, and local folders. They do not contain the Douyin password or proxy credentials.\n4. Turn off Debug after troubleshooting.\n",
        )
        .map_err(|e| e.to_string())?;
        zw.start_file("diagnostics.txt", opts)
            .map_err(|e| e.to_string())?;
        zw.write_all(diagnostics(app, state).as_bytes())
            .map_err(|e| e.to_string())?;
        zw.start_file("issues.log", opts)
            .map_err(|e| e.to_string())?;
        if let Some(l) = logger() {
            if let Ok(issues) = l.issues.lock() {
                for line in issues.iter() {
                    let _ = writeln!(
                        zw,
                        "{} {:<5} [{}] {}",
                        stamp(line.at),
                        line.level.to_uppercase(),
                        line.scope,
                        line.text
                    );
                }
            }
        }
        let files = log_files();
        if files.is_empty() {
            zw.start_file("logs/ring.log", opts)
                .map_err(|e| e.to_string())?;
            for line in tail(RING) {
                let _ = writeln!(
                    zw,
                    "{} {:<5} [{}] {}",
                    stamp(line.at),
                    line.level.to_uppercase(),
                    line.scope,
                    line.text
                );
            }
        } else {
            for path in files {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                zw.start_file(format!("logs/{name}"), opts)
                    .map_err(|e| e.to_string())?;
                let mut f = File::open(&path).map_err(|e| e.to_string())?;
                std::io::copy(&mut f, &mut zw).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    })();
    let _ = zw.finish();
    if let Err(e) = result {
        let _ = std::fs::remove_file(&part);
        return Err(e);
    }
    std::fs::rename(&part, &out).map_err(|e| e.to_string())?;
    write_raw(
        Level::Info,
        "app",
        &format!("诊断包已导出 {}", redact(&out.to_string_lossy())),
    );
    Ok(out.to_string_lossy().to_string())
}

#[tauri::command]
pub fn ui_log(level: String, scope: String, text: String) {
    let level = Level::parse(&level).unwrap_or(Level::Info);
    write(level, &scope, &text);
}

#[tauri::command]
pub fn log_tail(limit: Option<usize>) -> Vec<Line> {
    tail(limit.unwrap_or(500).min(RING))
}

#[tauri::command]
pub fn log_clear() -> Result<(), String> {
    clear()
}

#[tauri::command]
pub fn log_path() -> String {
    path()
}

#[tauri::command]
pub fn log_level() -> String {
    level().label().into()
}

#[tauri::command]
pub fn log_set_level(level_name: String) -> Result<String, String> {
    let parsed = Level::parse(&level_name).ok_or_else(|| format!("未知日志级别：{level_name}"))?;
    LEVEL.store(parsed as u8, Ordering::Relaxed);
    dycore::diag::set_enabled(parsed == Level::Debug);
    write_raw(
        Level::Info,
        "app",
        &format!("日志级别 → {}", parsed.label()),
    );
    Ok(parsed.label().into())
}

#[tauri::command]
pub async fn log_export(
    app: AppHandle,
    state: tauri::State<'_, crate::api::Backend>,
) -> Result<String, String> {
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || export(&app, &state))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_known_query_keys() {
        let text = "GET /x?msToken=abc123&a_bogus=xyz&target=77&X-Bogus=Q";
        let out = redact(text);
        assert!(!out.contains("abc123"));
        assert!(!out.contains("xyz"));
        assert_eq!(out.matches("***").count(), 3);
        assert!(out.contains("target=77"));
    }

    #[test]
    fn redacts_empty_and_short_values() {
        assert_eq!(redact("token="), "token=");
        assert_eq!(redact("token=a"), "token=***");
        assert_eq!(redact("msToken=a&a_bogus=b"), "msToken=***&a_bogus=***");
    }

    #[test]
    fn redacts_proxy_credentials() {
        assert_eq!(
            redact("proxy=http://user:pass@127.0.0.1:8080"),
            "proxy=http://***@127.0.0.1:8080"
        );
    }

    #[test]
    fn levels_are_ordered_and_parsed() {
        assert!(Level::Debug < Level::Info);
        assert!(Level::Warn < Level::Error);
        assert_eq!(Level::parse("ERROR"), Some(Level::Error));
        assert_eq!(Level::parse("err"), Some(Level::Error));
        assert_eq!(Level::parse("nope"), None);
    }
}
