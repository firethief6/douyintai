//! 保证测试二进制带上 Common-Controls 6 清单。
//!
//! `tauri-build` 只把清单链进 `[[bin]]`，而包内没有显式 test 目标时 cargo 会拒绝
//! `cargo:rustc-link-arg-tests`。这条用例同时验证库能被测试目标链接。

#[test]
fn lib_links() {
    assert!(douyintai_lib::paths::home().is_absolute());
}
