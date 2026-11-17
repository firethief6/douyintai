//! 回环/下载侧出网：CDN 默认直连，按设置可改走用户代理。
use std::sync::{OnceLock, RwLock};

use reqwest::Proxy;

static CDN_PROXY: OnceLock<RwLock<Option<String>>> = OnceLock::new();

fn cell() -> &'static RwLock<Option<String>> {
    CDN_PROXY.get_or_init(|| RwLock::new(None))
}

/// 设置 CDN 出口；`None` 表示直连。空串按 `None` 处理。
pub fn set_cdn_proxy(url: Option<String>) {
    let url = url.and_then(|u| {
        let u = u.trim().to_string();
        (!u.is_empty()).then_some(u)
    });
    *cell().write().unwrap_or_else(|e| e.into_inner()) = url;
}

pub fn cdn_proxy() -> Option<String> {
    cell().read().unwrap_or_else(|e| e.into_inner()).clone()
}

/// 带 CDN 出口的阻塞客户端构造器。
pub fn blocking_builder() -> reqwest::blocking::ClientBuilder {
    let b = reqwest::blocking::Client::builder();
    match cdn_proxy().and_then(|u| Proxy::all(u).ok()) {
        Some(p) => b.proxy(p),
        None => b,
    }
}

/// 带 CDN 出口的异步客户端构造器。
pub fn async_builder() -> reqwest::ClientBuilder {
    let b = reqwest::Client::builder();
    match cdn_proxy().and_then(|u| Proxy::all(u).ok()) {
        Some(p) => b.proxy(p),
        None => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_means_direct() {
        set_cdn_proxy(Some("  ".into()));
        assert_eq!(cdn_proxy(), None);
        set_cdn_proxy(Some("http://127.0.0.1:1".into()));
        assert_eq!(cdn_proxy().as_deref(), Some("http://127.0.0.1:1"));
        assert!(blocking_builder().build().is_ok());
        assert!(async_builder().build().is_ok());
        set_cdn_proxy(None);
        assert_eq!(cdn_proxy(), None);
    }
}
