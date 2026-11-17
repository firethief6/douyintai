#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // WebKitGTK 的媒体进程沙箱在容器/无 /dev/shm 环境里起不来，表现是 <video> 永远 readyState=0。
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS", "1");
    douyintai_lib::run()
}
