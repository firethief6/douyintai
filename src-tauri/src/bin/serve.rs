//! 浏览器走查用的本地接口服务：复用桌面端 api::dispatch，仅监听 127.0.0.1:8766。
//! 生产不跑它；Tauri 前端仍走进程内调用。
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use douyintai_lib::api::{dispatch, Backend};
use douyintai_lib::pipeline::{run_worker, Pipeline};
use serde_json::Value;

fn handle(mut s: TcpStream, backend: Backend) {
    let mut r = BufReader::new(match s.try_clone() {
        Ok(x) => x,
        Err(_) => return,
    });
    let mut line = String::new();
    if r.read_line(&mut line).is_err() {
        return;
    }
    let mut length = 0usize;
    let mut range = None;
    loop {
        let mut l = String::new();
        if r.read_line(&mut l).unwrap_or(0) == 0 || l.trim().is_empty() {
            break;
        }
        let low = l.to_ascii_lowercase();
        if let Some(v) = low.strip_prefix("content-length:") {
            length = v.trim().parse().unwrap_or(0);
        }
        if let Some(v) = low.strip_prefix("range:") {
            range = Some(v.trim().to_string());
        }
    }
    if line.starts_with("OPTIONS ") {
        let _ = write!(s, "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: GET, POST, DELETE, PATCH, OPTIONS\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        return;
    }
    let target = line.split_whitespace().nth(1).unwrap_or("/");
    let method = line.split_whitespace().next().unwrap_or("GET").to_string();
    let (path, qs) = target.split_once('?').unwrap_or((target, ""));
    let mut q = HashMap::new();
    for (k, v) in url::form_urlencoded::parse(qs.as_bytes()) {
        q.insert(k.into_owned(), v.into_owned());
    }
    let mut raw = vec![0u8; length];
    if length > 0 && r.read_exact(&mut raw).is_err() {
        return;
    }
    if path == "/media" {
        let Some(file) = q.get("u") else {
            let _ = write!(
                s,
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            return;
        };
        let Ok(mut f) = std::fs::File::open(file) else {
            let _ = write!(s, "HTTP/1.1 404 Not Found\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
            return;
        };
        let total = f.metadata().map(|m| m.len()).unwrap_or(0);
        let ext = std::path::Path::new(file)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let ctype = match ext.as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "mp4" => "video/mp4",
            "m3u8" => "application/vnd.apple.mpegurl",
            "ts" => "video/mp2t",
            "mp3" => "audio/mpeg",
            _ => "application/octet-stream",
        };
        let (start, end, code, range_head) =
            match range.as_deref().and_then(|v| v.strip_prefix("bytes=")) {
                Some(spec) if total > 0 => {
                    let (a, b) = spec.split_once('-').unwrap_or((spec, ""));
                    let start = a.trim().parse::<u64>().unwrap_or(0).min(total - 1);
                    let end = b
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(total - 1)
                        .min(total - 1)
                        .max(start);
                    (
                        start,
                        end,
                        206,
                        format!("Content-Range: bytes {start}-{end}/{total}\r\n"),
                    )
                }
                _ => (0, total.saturating_sub(1), 200, String::new()),
            };
        let count = if total == 0 { 0 } else { end - start + 1 };
        let _ = f.seek(SeekFrom::Start(start));
        let mut data = Vec::with_capacity(count as usize);
        let _ = f.take(count).read_to_end(&mut data);
        let _ = write!(s, "HTTP/1.1 {code} OK\r\nContent-Type: {ctype}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Expose-Headers: Content-Range, Content-Length\r\nAccept-Ranges: bytes\r\n{range_head}Content-Length: {}\r\nConnection: close\r\n\r\n", data.len());
        let _ = s.write_all(&data);
        return;
    }
    let body: Option<Value> = if raw.is_empty() {
        None
    } else {
        serde_json::from_slice(&raw).ok()
    };
    let out =
        tauri::async_runtime::block_on(dispatch(backend, path.to_string(), q, Some(method), body));
    let (code, body) = match out {
        Ok(v) => (200, serde_json::to_vec(&v).unwrap_or_default()),
        Err(e) => (
            e.status,
            serde_json::to_vec(&serde_json::json!({"status": e.status, "message": e.message}))
                .unwrap_or_default(),
        ),
    };
    let reason = if code == 200 { "OK" } else { "Error" };
    let _ = write!(s, "HTTP/1.1 {code} {reason}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    let _ = s.write_all(&body);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(douyintai_lib::rec::Manager::new())
        .build(tauri::generate_context!())?;
    let handle_app = app.handle().clone();
    let _ = douyintai_lib::logging::init(&handle_app);
    let api = Arc::new(dycore::api::Api::new(douyintai_lib::sign_resources(
        &handle_app,
    ))?);
    api.start_pool_maintainer();
    let pipe = Arc::new(Pipeline::new(douyintai_lib::db::path(&handle_app)?));
    pipe.attach_app(handle_app.clone());
    pipe.attach(api.clone());
    let backend = Backend {
        api,
        pipeline: pipe.clone(),
    };
    std::thread::spawn(move || run_worker(pipe));
    let l = TcpListener::bind(("127.0.0.1", 8766))?;
    eprintln!("[serve] http://127.0.0.1:8766");
    std::thread::spawn(move || {
        for c in l.incoming().flatten() {
            let b = backend.clone();
            std::thread::spawn(move || handle(c, b));
        }
    });
    app.run(|_, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
    Ok(())
}
