//! 用户自备出口 IP：一行一个地址，按请求轮换，失败冷却。
//!
//! 地址可写 `host:port` 或带协议（`http://`、`socks5://`），默认 `http://`。
//! 列表为空或未启用时直连；全部冷却时仍用最差的一条，避免整段断网。
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use reqwest::blocking::Client;

use crate::Result;

#[derive(Default)]
struct Entry {
    bad: u32,
    until: Option<Instant>,
    last: Option<Instant>,
}

/// 同一身份在此时间内固定同一出口，避免来回换 IP。
const BIND_TTL: Duration = Duration::from_secs(600);

#[derive(Default)]
pub struct Proxies {
    list: Mutex<Vec<String>>,
    text: Mutex<String>,
    bindings: Mutex<HashMap<String, (String, Instant)>>,
    state: Mutex<HashMap<String, Entry>>,
    enabled: Mutex<bool>,
    cursor: AtomicUsize,
    clients: Mutex<HashMap<String, Client>>,
}

fn normalize(raw: &str) -> Option<String> {
    let line = raw.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let url = if line.contains("://") {
        line.to_string()
    } else {
        format!("http://{line}")
    };
    url::Url::parse(&url).ok()?;
    Some(url)
}

/// 文本按行解析并去重；保留用户顺序。
pub fn parse(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(url) = normalize(line) {
            if seen.insert(url.clone()) {
                out.push(url);
            }
        }
    }
    out
}

impl Proxies {
    pub fn set(&self, text: &str, enabled: bool) {
        *self.text.lock().unwrap_or_else(|e| e.into_inner()) = text.to_string();
        let list = parse(text);
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.retain(|k, _| list.contains(k));
        drop(state);
        self.bindings
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|_, (u, _)| list.contains(u));
        *self.list.lock().unwrap_or_else(|e| e.into_inner()) = list;
        *self.enabled.lock().unwrap_or_else(|e| e.into_inner()) = enabled;
        eprintln!("[proxy] set {} 条 enabled={enabled}", self.len());
        self.cursor.store(0, Ordering::Relaxed);
    }

    pub fn clear(&self) {
        self.set("", false);
    }

    pub fn enabled(&self) -> bool {
        *self.enabled.lock().unwrap_or_else(|e| e.into_inner())
            && !self
                .list
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
    }

    /// 用户原始输入（界面回显用，未做规范化）。
    pub fn text(&self) -> String {
        self.text.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.list.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// 取一条可用出口；全冷却时取最久没用过的一条。
    pub fn pick(&self) -> Option<String> {
        if !self.enabled() {
            return None;
        }
        let list = self.list.lock().unwrap_or_else(|e| e.into_inner()).clone();
        let now = Instant::now();
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut best: Option<(u32, Option<Instant>, String)> = None;
        let start = self.cursor.fetch_add(1, Ordering::Relaxed);
        for i in 0..list.len() {
            let url = &list[(start + i) % list.len()];
            let e = state.entry(url.clone()).or_default();
            if e.until.is_some_and(|t| t > now) {
                continue;
            }
            let rank = (e.bad, e.last);
            if best.as_ref().is_none_or(|b| (b.0, b.1) > rank) {
                best = Some((e.bad, e.last, url.clone()));
            }
        }
        let picked = best.map(|b| b.2).or_else(|| {
            list.iter()
                .min_by_key(|u| state.get(*u).map(|e| e.bad).unwrap_or(0))
                .cloned()
        })?;
        if let Some(e) = state.get_mut(&picked) {
            e.last = Some(now);
        }
        Some(picked)
    }

    fn usable(&self, url: &str) -> bool {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state
            .get(url)
            .is_none_or(|e| e.until.is_none_or(|t| t <= Instant::now()))
    }

    /// 按身份取出口：绑定期内固定复用，未绑定或绑定已失效时重新分配。
    pub fn pick_for(&self, key: &str) -> Option<String> {
        if !self.enabled() {
            eprintln!("[proxy] pick_for {key} -> none (len={})", self.len());
            return None;
        }
        if let Some((url, at)) = self
            .bindings
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(key)
            .cloned()
        {
            if at.elapsed() < BIND_TTL && self.usable(&url) {
                return Some(url);
            }
        }
        let picked = self.pick();
        self.bind(key, picked.as_deref());
        picked
    }

    pub fn bind(&self, key: &str, url: Option<&str>) {
        let mut b = self.bindings.lock().unwrap_or_else(|e| e.into_inner());
        match url {
            Some(url) => {
                b.insert(key.to_string(), (url.to_string(), Instant::now()));
            }
            None => {
                b.remove(key);
            }
        }
    }

    /// 反馈一条出口的结果：成功清零，失败按次数指数冷却（60s…10min）。
    pub fn report(&self, url: &str, ok: bool) {
        {
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            let e = state.entry(url.to_string()).or_default();
            if ok {
                e.bad = 0;
                e.until = None;
            } else {
                e.bad = (e.bad + 1).min(6);
                let secs = 60u64.saturating_mul(1 << e.bad.min(4)).min(600);
                e.until = Some(Instant::now() + Duration::from_secs(secs));
            }
        }
        // 先释放 state 再取 bindings，保持与 pick_for 相反的加锁顺序不交叉。
        if !ok {
            self.bindings
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .retain(|_, (u, _)| u != url);
        }
    }

    /// 出口对应的 reqwest 客户端，连接参数按出口缓存复用（直连用空键）。
    pub fn client(&self, url: Option<&str>) -> Result<Client> {
        let key = url.unwrap_or_default();
        if let Some(c) = self
            .clients
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(key)
        {
            return Ok(c.clone());
        }
        let client = crate::http::client_with(url)?;
        self.clients
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key.to_string(), client.clone());
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_keeps_order_and_dedups() {
        let list =
            parse("1.2.3.4:80\n\nhttp://5.6.7.8:8080\n1.2.3.4:80\n# 注释\nsocks5://9.9.9.9:1080");
        assert_eq!(
            list,
            vec![
                "http://1.2.3.4:80",
                "http://5.6.7.8:8080",
                "socks5://9.9.9.9:1080"
            ]
        );
    }

    #[test]
    fn rotates_and_cools_failures() {
        let p = Proxies::default();
        p.set("a.test:1\nb.test:2", true);
        let first = p.pick().unwrap();
        assert!(p.enabled());
        p.report(&first, false);
        let second = p.pick().unwrap();
        assert_ne!(first, second);
        p.report(&second, false);
        // 两条都冷却时仍要给出出口
        assert!(p.pick().is_some());
        p.report(&first, true);
        assert_eq!(p.pick().as_deref(), Some(first.as_str()));
    }

    #[test]
    fn binds_identity_to_one_exit() {
        let p = Proxies::default();
        p.set("a.test:1\nb.test:2", true);
        let a = p.pick_for("dev-a").unwrap();
        assert_eq!(p.pick_for("dev-a").as_deref(), Some(a.as_str()));
        p.report(&a, false);
        assert_ne!(p.pick_for("dev-a").as_deref(), Some(a.as_str()));
    }

    #[test]
    fn replacing_list_drops_stale_binding() {
        let p = Proxies::default();
        p.set("a.test:1\nb.test:2", true);
        let a = p.pick_for("dev-a").unwrap();
        p.set("b.test:2", true);
        if a == "http://a.test:1" {
            assert_eq!(p.pick_for("dev-a").as_deref(), Some("http://b.test:2"));
        }
    }

    #[test]
    fn disabled_means_direct() {
        let p = Proxies::default();
        p.set("a.test:1", false);
        assert!(!p.enabled());
        assert_eq!(p.pick(), None);
        p.set("", true);
        assert_eq!(p.pick(), None);
    }
}
