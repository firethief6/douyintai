//! 浏览器开发用的本地接口服务：把 `Api::call` 暴露成 HTTP/1.1（仅 127.0.0.1）。
//! 生产不跑它；Tauri 直接走进程内调用。
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use serde_json::Value;

fn handle(mut s: TcpStream, api: Arc<dycore::api::Api>) {
    let mut r = BufReader::new(match s.try_clone() {
        Ok(x) => x,
        Err(_) => return,
    });
    let mut line = String::new();
    if r.read_line(&mut line).is_err() {
        return;
    }
    loop {
        let mut l = String::new();
        match r.read_line(&mut l) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                if l.trim().is_empty() {
                    break;
                }
            }
        }
    }
    if line.starts_with("OPTIONS ") {
        let _ = write!(s, "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: GET, OPTIONS\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        return;
    }
    let target = line.split_whitespace().nth(1).unwrap_or("/");
    let (path, qs) = target.split_once('?').unwrap_or((target, ""));
    let mut q = serde_json::Map::new();
    for (k, v) in url::form_urlencoded::parse(qs.as_bytes()) {
        q.insert(k.into_owned(), Value::String(v.into_owned()));
    }
    let query = Value::Object(q);
    let path = url::form_urlencoded::parse(format!("p={path}").as_bytes())
        .next()
        .map(|(_, v)| v.into_owned())
        .unwrap_or_default();
    let out = api.call(&path, &query);
    let (code, body) = match out {
        Ok(v) => (200, serde_json::to_vec(&v).unwrap_or_default()),
        Err(e) => (
            e.status,
            serde_json::to_vec(
                &serde_json::json!({"status": e.status, "message": e.message, "kind": e.kind}),
            )
            .unwrap_or_default(),
        ),
    };
    let reason = if code == 200 { "OK" } else { "Error" };
    let _ = write!(s, "HTTP/1.1 {code} {reason}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    let _ = s.write_all(&body);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = Arc::new(dycore::api::Api::new(dysign::default_resources())?);
    api.start_pool_maintainer();
    let l = TcpListener::bind(("127.0.0.1", 8765))?;
    eprintln!("[serve] http://127.0.0.1:8765");
    for c in l.incoming().flatten() {
        let api = api.clone();
        std::thread::spawn(move || handle(c, api));
    }
    Ok(())
}
