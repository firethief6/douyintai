//! 移入系统回收站。macOS 走 NSFileManager，避免 Finder AppleScript 超时。

use std::path::Path;

#[cfg(target_os = "macos")]
pub fn delete(path: impl AsRef<Path>) -> Result<(), String> {
    use objc2_foundation::{NSFileManager, NSString, NSURL};

    let path = NSString::from_str(&path.as_ref().to_string_lossy());
    let url = NSURL::fileURLWithPath(&path);
    NSFileManager::defaultManager()
        .trashItemAtURL_resultingItemURL_error(&url, None)
        .map_err(|e| e.localizedDescription().to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn delete(path: impl AsRef<Path>) -> Result<(), String> {
    ::trash::delete(path).map_err(|e| e.to_string())
}
