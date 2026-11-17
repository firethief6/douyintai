pub mod api;
pub mod cenc;
pub mod db;
pub mod gallery;
pub mod logging;
mod media;
mod net;
pub mod notify;
pub mod paths;
pub mod pipeline;
pub mod rec;
pub mod trash;

use tauri::{AppHandle, Manager, Window};

#[tauri::command]
async fn gallery_zip(urls: Vec<String>, dir: String, name: String) -> Result<String, String> {
    gallery::zip(urls, dir, name).await
}

#[tauri::command]
fn rec_start(
    app: AppHandle,
    url: String,
    dir: String,
    room: String,
) -> Result<rec::Session, String> {
    rec::start(app, url, dir, room)
}

#[tauri::command]
fn rec_stop(app: AppHandle, id: String) -> bool {
    app.state::<rec::Manager>().stop(&id)
}

#[tauri::command]
fn clipboard_text(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().read_text().map_err(|e| e.to_string())
}

#[tauri::command]
fn clipboard_write(app: AppHandle, text: String) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn rec_list(app: AppHandle) -> Vec<rec::Session> {
    app.state::<rec::Manager>().list()
}

#[tauri::command]
fn rec_delete(path: String) -> Result<(), String> {
    rec::delete(std::path::Path::new(&path))
}

#[tauri::command]
fn save_text(path: String, text: String) -> Result<String, String> {
    let p = paths::expand(&path);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&p, text).map_err(|e| e.to_string())?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
fn read_text(path: String) -> Result<String, String> {
    const MAX: u64 = 8 << 20;
    let p = paths::expand(&path);
    let size = std::fs::metadata(&p).map_err(|e| e.to_string())?.len();
    if size > MAX {
        return Err("文件超过 8MB".into());
    }
    std::fs::read_to_string(&p).map_err(|e| e.to_string())
}

#[tauri::command]
fn reveal(path: String) -> Result<(), String> {
    tauri_plugin_opener::reveal_item_in_dir(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_path(path: String) -> Result<(), String> {
    let path = paths::expand(&path);
    if !path.is_file() {
        return Err("文件不存在".into());
    }
    tauri_plugin_opener::open_path(path, None::<String>).map_err(|e| e.to_string())
}

#[tauri::command]
fn trash_path(path: String) -> Result<(), String> {
    trash::delete(path)
}

#[tauri::command]
fn open_external(url: String) -> Result<(), String> {
    tauri_plugin_opener::open_url(url, None::<String>).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct DiskInfo {
    total: u64,
    free: u64,
}

#[tauri::command]
fn disk_info(dir: String) -> DiskInfo {
    let (total, free) = paths::capacity(&paths::expand(&dir));
    DiskInfo { total, free }
}

#[tauri::command]
fn disk_free(dir: String) -> u64 {
    let value = paths::available(&paths::expand(&dir));
    if value == u64::MAX {
        0
    } else {
        value
    }
}

#[tauri::command]
fn win_action(window: Window, action: String) {
    match action.as_str() {
        "min" => {
            let _ = window.minimize();
        }
        "max" => {
            let _ = if window.is_maximized().unwrap_or(false) {
                window.unmaximize()
            } else {
                window.maximize()
            };
        }
        "close" => {
            let _ = window.close();
        }
        _ => {}
    }
}

#[tauri::command]
fn mini(window: Window, on: bool) -> Result<(), String> {
    if on {
        window
            .set_size(tauri::LogicalSize::new(430.0, 760.0))
            .map_err(|e| e.to_string())?;
        window.set_always_on_top(true).map_err(|e| e.to_string())?;
        window
            .set_min_size(Some(tauri::LogicalSize::new(360.0, 560.0)))
            .map_err(|e| e.to_string())?;
    } else {
        window.set_always_on_top(false).map_err(|e| e.to_string())?;
        window
            .set_size(tauri::LogicalSize::new(1440.0, 900.0))
            .map_err(|e| e.to_string())?;
        window
            .set_min_size(Some(tauri::LogicalSize::new(1080.0, 680.0)))
            .map_err(|e| e.to_string())?;
        let _ = window.center();
    }
    Ok(())
}

fn focus_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

/// 重复启动时聚焦已有窗口，重复进程自行退出（由插件负责）。
fn on_second_instance(app: &tauri::AppHandle) {
    logging::info("app", "检测到重复启动，聚焦已有窗口");
    focus_main(app);
}

fn toggle_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        if w.is_visible().unwrap_or(false) {
            let _ = w.hide();
        } else {
            let _ = w.show();
            let _ = w.unminimize();
            let _ = w.set_focus();
        }
    }
}

#[tauri::command]
fn media_url(app: AppHandle, url: String, key: Option<String>, key_err: Option<String>) -> String {
    let key = key.filter(|k| !k.is_empty());
    if key.is_none() && url.contains("er=1") {
        let why = key_err.unwrap_or_else(|| "未返回原因".into());
        logging::warn(
            "media",
            format!("er=1 但没取到解密 key（{why}），按密文播放"),
        );
    }
    match key {
        Some(k) => app.state::<media::Server>().url_key(&url, &k),
        None => app.state::<media::Server>().url(&url),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn sign_resources(app: &tauri::AppHandle) -> std::path::PathBuf {
    if let Ok(p) = std::env::var("DOUYINTAI_SIGN_RES") {
        return p.into();
    }
    if let Ok(dir) = app.path().resource_dir() {
        let p = dir.join("sign");
        if p.join("libmetasec_ml.so").is_file() {
            return p;
        }
    }
    let rel = std::path::PathBuf::from("crates/dysign/resources");
    if rel.join("libmetasec_ml.so").is_file() {
        return rel;
    }
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/dysign/resources")
}

pub fn run() {
    logging::install_panic_hook();
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            on_second_instance(app);
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["ctrl+shift+d"])
                .unwrap_or_default()
                .with_handler(|app, _sc, event| {
                    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        toggle_main(app);
                    }
                })
                .build(),
        )
        .manage(rec::Manager::new())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem};
            let saved = db::db_get_settings(app.handle().clone())
                .ok()
                .flatten()
                .and_then(|j| serde_json::from_str::<serde_json::Value>(&j).ok());
            let saved_level = saved
                .as_ref()
                .and_then(|v| v.get("logLevel"))
                .and_then(|v| v.as_str())
                .map(str::to_string);
            if let Err(e) = logging::init_with_level(app.handle(), saved_level.as_deref()) {
                eprintln!("日志初始化失败：{e}");
            }
            logging::info("app", "窗口创建");
            use tauri::tray::{TrayIconBuilder, TrayIconEvent};
            let show = MenuItem::with_id(app, "show", "显示 / 隐藏", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            let mut tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, ev| match ev.id().as_ref() {
                    "show" => toggle_main(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, ev| {
                    if let TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = ev
                    {
                        toggle_main(tray.app_handle());
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            let _ = tray.build(app);
            match media::Server::start() {
                Ok(srv) => {
                    logging::info("media", format!("回环代理 127.0.0.1:{}", srv.port));
                    app.manage(srv);
                }
                Err(e) => logging::error("media", format!("回环代理启动失败：{e}")),
            }
            let res = sign_resources(app.handle());
            let backend = saved
                .as_ref()
                .and_then(|v| {
                    v.get("signBackend")
                        .and_then(|s| s.as_str())
                        .and_then(dycore::signer::Backend::parse)
                })
                .unwrap_or(dycore::signer::Backend::Native);
            let target = saved
                .as_ref()
                .and_then(|v| v.get("identityPoolSize"))
                .and_then(|v| v.as_u64())
                .unwrap_or(64) as usize;
            let proxy_list = saved
                .as_ref()
                .and_then(|v| v.get("proxyList"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let proxy_cdn = saved
                .as_ref()
                .and_then(|v| v.get("proxyCdn"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let proxy_on = saved
                .as_ref()
                .and_then(|v| v.get("proxyEnabled"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let data = app.path().app_data_dir().map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&data).map_err(|e| e.to_string())?;
            logging::info("sign", format!("签名实现 {}", backend.label()));
            logging::info(
                "proxy",
                format!(
                    "出口 {} 条 · 启用={proxy_on} · CDN={proxy_cdn}",
                    proxy_list.lines().count()
                ),
            );
            let api = dycore::api::Api::new_stored(
                res,
                backend,
                data.join("identity-pool.jsonl"),
                target,
            )
            .map_err(|e| e.to_string())?;
            let api = std::sync::Arc::new(api);
            api.set_proxies(&proxy_list, proxy_on);
            logging::info(
                "proxy",
                format!("setup arc={:p}", std::sync::Arc::as_ptr(&api)),
            );
            api.set_cdn_via_proxy(proxy_cdn);
            net::set_cdn_proxy(api.cdn_proxy());
            api.start_pool_maintainer();
            {
                // 启动自检：真签一次，失败只记日志，由界面读到 last 提示用户切换。
                let probe = api.clone();
                std::thread::spawn(move || {
                    if let Err(e) = probe.self_check() {
                        logging::warn("sign", format!("启动自检失败：{e}"));
                    }
                });
            }
            let pipe = std::sync::Arc::new(pipeline::Pipeline::new(db::path(app.handle())?));
            pipe.attach_app(app.handle().clone());
            pipe.attach(api.clone());
            app.manage(api::Backend {
                api: api.clone(),
                pipeline: pipe.clone(),
            });
            std::thread::spawn(move || pipeline::run_worker(pipe));
            std::thread::spawn(move || {
                let mut first = true;
                loop {
                    match api.warm() {
                        Ok(()) => {
                            if first {
                                logging::info("sign", "签名热启完成");
                                first = false;
                            }
                        }
                        Err(e) if first => logging::warn("sign", format!("签名热启失败：{e}")),
                        Err(_) => {}
                    }
                    std::thread::sleep(std::time::Duration::from_secs(50));
                }
            });
            if let Some(w) = app.get_webview_window("main") {
                logging::info("app", format!("窗口可见={:?}", w.is_visible()));
                if !w.is_visible().unwrap_or(true) {
                    let _ = w.show();
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            let name = match event {
                tauri::WindowEvent::CloseRequested { .. } => "关闭请求",
                tauri::WindowEvent::Destroyed => "窗口销毁",
                tauri::WindowEvent::Focused(v) => {
                    if *v {
                        "获得焦点"
                    } else {
                        "失去焦点"
                    }
                }
                _ => return,
            };
            logging::info("app", name);
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            api::api_call,
            api::sign_status,
            api::set_sign_backend,
            api::identity_status,
            api::identity_set_target,
            api::identity_pause,
            api::identity_prepare,
            api::identity_clear,
            api::proxy_get,
            api::proxy_set,
            api::proxy_status,
            api::proxy_test,
            db::db_get_settings,
            db::db_set_settings,
            db::db_load_rows,
            db::db_put_row,
            db::db_del_row,
            db::db_clear_rows,
            logging::ui_log,
            logging::log_tail,
            logging::log_clear,
            logging::log_path,
            logging::log_level,
            logging::log_set_level,
            logging::log_export,
            gallery_zip,
            media_url,
            rec_start,
            rec_stop,
            clipboard_text,
            clipboard_write,
            rec_list,
            rec_delete,
            read_text,
            save_text,
            reveal,
            open_path,
            trash_path,
            open_external,
            disk_free,
            disk_info,
            win_action,
            mini
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
