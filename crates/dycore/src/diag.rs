//! 出网诊断钩子。桌面端把 debug 日志接进来；测试和其它调用方不接也能运行。
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

type Hook = fn(&str, &str);

static ENABLED: AtomicBool = AtomicBool::new(false);
static HOOK: OnceLock<Hook> = OnceLock::new();

pub fn install(hook: Hook) {
    let _ = HOOK.set(hook);
}

pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn debug(scope: &str, text: impl AsRef<str>) {
    if !enabled() {
        return;
    }
    if let Some(hook) = HOOK.get() {
        hook(scope, text.as_ref());
    }
}
