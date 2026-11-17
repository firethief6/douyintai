//! 跨平台路径展开与磁盘容量。界面只保存可移植的 `~` 前缀。
use std::path::{Path, PathBuf};

pub fn home() -> PathBuf {
    dirs::home_dir()
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default()
}

pub fn expand(raw: &str) -> PathBuf {
    if raw == "~" {
        return home();
    }
    if let Some(rest) = raw.strip_prefix("~/").or_else(|| raw.strip_prefix("~\\")) {
        return home().join(rest);
    }
    PathBuf::from(raw)
}

pub fn available(path: &Path) -> u64 {
    fs2::available_space(path).unwrap_or(u64::MAX)
}

pub fn capacity(path: &Path) -> (u64, u64) {
    let free = fs2::available_space(path).unwrap_or(0);
    let total = fs2::total_space(path).unwrap_or(0);
    (total, free)
}
