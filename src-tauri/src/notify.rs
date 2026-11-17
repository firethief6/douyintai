//! 系统通知；插件不可用时只写日志，不影响主流程。
pub fn send(app: &tauri::AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app.notification().builder().title(title).body(body).show() {
        crate::logging::warn("notify", format!("{title}: {e}"));
    }
}
