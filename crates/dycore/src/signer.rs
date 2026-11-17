//! 签名实现放在独立线程池里（Unicorn 非 Send），多条线程并行签名。
//!
//! 单次签名约 40–90ms，全部串行时并发请求会排队；这里按 CPU 起 2–4 条
//! 工作线程，每条独占一套后端，身份粘在最近一次使用的那条线程上。
//!
//! 后端两套：`Native`（进程内 AOT 纯算）默认，`Emu`（Unicorn 装 .so 的台子）备选。
//! 运行时可切；签名报错直接返回并提示用户手动切换，不做自动兜底。
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;

use crate::{Error, Result};

fn pure_device(did: &str, iid: &str) -> dysign_pure::Device {
    let path = std::env::var("DYSIGN_POOL")
        .unwrap_or_else(|_| crate::pool::default_path().to_string_lossy().into_owned());
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            if let Ok(d) = serde_json::from_str::<dysign_pure::Device>(line) {
                if d.device_id == did {
                    return d;
                }
            }
        }
    }
    dysign_pure::Device {
        device_id: did.into(),
        iid: iid.into(),
        ..Default::default()
    }
}

fn pure_signer(did: &str, iid: &str) -> dysign_pure::PureSigner {
    let clock = std::env::var("DYSIGN_FIXED_TIME")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let seed = std::env::var("DYSIGN_RANDOM_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    dysign_pure::PureSigner::new(&pure_device(did, iid), clock, seed)
}

/// 签名实现。`Native` 是默认路径，`Emu` 是用户手动切回模拟器的备选。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Native,
    Emu,
}

impl Backend {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "native" | "pure" | "aot" => Some(Self::Native),
            "emu" | "qemu" | "unidbg" | "unidroid" | "replay" => Some(Self::Emu),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Emu => "emu",
        }
    }

    /// 中文名，用于给用户的错误提示。
    pub fn cn(self) -> &'static str {
        match self {
            Self::Native => "原生",
            Self::Emu => "兼容（模拟器）",
        }
    }

    fn idx(self) -> usize {
        match self {
            Self::Native => 0,
            Self::Emu => 1,
        }
    }
}

/// `DYSIGN_IMPL` 是开发脚本的固定覆盖项，优先于设置。
fn env_backend() -> Option<Backend> {
    std::env::var("DYSIGN_IMPL")
        .ok()
        .as_deref()
        .and_then(Backend::parse)
}

/// 最近一次签名失败。
#[derive(Clone, Debug)]
pub struct LastErr {
    pub backend: Backend,
    pub at: u64,
    pub message: String,
}

#[derive(Default)]
struct Stats {
    ok: [AtomicU64; 2],
    err: [AtomicU64; 2],
    last: std::sync::Mutex<Option<LastErr>>,
}

impl Stats {
    fn fail(&self, backend: Backend, message: String) {
        self.err[backend.idx()].fetch_add(1, Ordering::Relaxed);
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let mut last = self.last.lock().unwrap_or_else(|e| e.into_inner());
        *last = Some(LastErr {
            backend,
            at,
            message,
        });
    }
}

/// 当前后端与两边的成败计数，供 `/health` 与设置页显示。
#[derive(Clone, Debug)]
pub struct SignInfo {
    pub backend: Backend,
    pub env: Option<Backend>,
    pub ok: [u64; 2],
    pub err: [u64; 2],
    pub last: Option<LastErr>,
}

impl SignInfo {
    pub fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "backend": self.backend.label(),
            "env": self.env.map(|b| b.label()),
            "native": {"ok": self.ok[0], "err": self.err[0]},
            "emu": {"ok": self.ok[1], "err": self.err[1]},
            "last": self.last.as_ref().map(|l| serde_json::json!({
                "backend": l.backend.label(),
                "at": l.at,
                "message": l.message,
            })),
        })
    }
}

enum Cmd {
    Sign {
        did: String,
        iid: String,
        url: String,
        extra: String,
        backend: Backend,
        reply: Option<Sender<Result<String>>>,
    },
}

#[derive(Clone)]
pub struct Signer {
    workers: Arc<Vec<Sender<Cmd>>>,
    next: Arc<AtomicUsize>,
    backend: Arc<AtomicUsize>,
    env: Option<Backend>,
    stats: Arc<Stats>,
}

/// 工作线程数：默认按核数取 2–4，`DYSIGN_WORKERS` 可覆盖。
fn workers_default() -> usize {
    if let Ok(n) = std::env::var("DYSIGN_WORKERS") {
        if let Ok(n) = n.parse::<usize>() {
            if n > 0 {
                return n.min(8);
            }
        }
    }
    // 空闲工作线程本身有轮询开销，单条链路优先；并发多时用 DYSIGN_WORKERS 上调。
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .div_ceil(5)
        .clamp(2, 4)
}

#[allow(clippy::too_many_arguments)]
fn run_on(
    backend: Backend,
    res: &Path,
    emu: &mut Option<dysign::Signer>,
    pure: &mut Option<dysign_pure::PureSigner>,
    emu_ident: &mut Option<(String, String)>,
    pure_ident: &mut Option<(String, String)>,
    did: &str,
    iid: &str,
    url: &str,
    extra: &str,
) -> std::result::Result<String, String> {
    if std::env::var("DYSIGN_FORCE_FAIL")
        .ok()
        .is_some_and(|v| v == "1" || v == backend.label())
    {
        return Err(format!("强制失败（DYSIGN_FORCE_FAIL={}）", backend.label()));
    }
    match backend {
        Backend::Emu => {
            if emu.is_none() {
                *emu = Some(dysign::Signer::new(res).map_err(|e| e.to_string())?);
            }
            sign_emu(emu.as_mut().unwrap(), emu_ident, did, iid, url, extra)
        }
        Backend::Native => {
            if pure.is_none() {
                *pure = Some(pure_signer(did, iid));
                *pure_ident = Some((did.into(), iid.into()));
            }
            let p = pure.as_mut().unwrap();
            if pure_ident.as_ref().map(|(d, i)| (d.as_str(), i.as_str())) != Some((did, iid)) {
                p.set_device(pure_device(did, iid));
                *pure_ident = Some((did.into(), iid.into()));
            }
            p.sign_strict(url, extra).map_err(|e| e.to_string())
        }
    }
}

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        (*s).into()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "未知 panic".into()
    }
}

/// 实现内部 panic 也不能带走工作线程，按一次失败处理。
#[allow(clippy::too_many_arguments)]
fn attempt(
    backend: Backend,
    res: &Path,
    emu: &mut Option<dysign::Signer>,
    pure: &mut Option<dysign_pure::PureSigner>,
    emu_ident: &mut Option<(String, String)>,
    pure_ident: &mut Option<(String, String)>,
    did: &str,
    iid: &str,
    url: &str,
    extra: &str,
) -> std::result::Result<String, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_on(
            backend, res, emu, pure, emu_ident, pure_ident, did, iid, url, extra,
        )
    }))
    .unwrap_or_else(|p| {
        // 崩过的实例状态不可信，丢掉让下一发重建。
        match backend {
            Backend::Emu => {
                *emu = None;
                *emu_ident = None;
            }
            Backend::Native => {
                *pure = None;
                *pure_ident = None;
            }
        }
        Err(format!("{} 崩溃：{}", backend.label(), panic_text(p)))
    })
}

impl Signer {
    /// 用设置里的后端启动；`DYSIGN_IMPL` 存在时压过它。
    pub fn spawn(res: PathBuf, requested: Backend) -> Result<Self> {
        let env = env_backend();
        let backend = env.unwrap_or(requested);
        let n = workers_default();
        let stats = Arc::new(Stats::default());
        let current = Arc::new(AtomicUsize::new(backend.idx()));
        let mut workers = Vec::with_capacity(n);
        for i in 0..n {
            let (tx, rx) = mpsc::channel::<Cmd>();
            let res = res.clone();
            let stats = stats.clone();
            std::thread::Builder::new()
                .name(format!("dysign-{i}"))
                .stack_size(8 * 1024 * 1024)
                .spawn(move || {
                    // 后端按需加载：只切换过的那套才付冷启代价，首次请求落在带台子
                    // 冷启时界面已经起来了。
                    let mut emu: Option<dysign::Signer> = None;
                    let mut pure: Option<dysign_pure::PureSigner> = None;
                    let mut emu_ident: Option<(String, String)> = None;
                    let mut pure_ident: Option<(String, String)> = None;
                    for cmd in rx {
                        let Cmd::Sign {
                            did,
                            iid,
                            url,
                            extra,
                            backend,
                            reply,
                        } = cmd;
                        let primary = backend;
                        let out = match attempt(
                            primary,
                            &res,
                            &mut emu,
                            &mut pure,
                            &mut emu_ident,
                            &mut pure_ident,
                            &did,
                            &iid,
                            &url,
                            &extra,
                        ) {
                            Ok(v) => {
                                stats.ok[primary.idx()].fetch_add(1, Ordering::Relaxed);
                                Ok(v)
                            }
                            Err(e) => {
                                stats.fail(primary, e.clone());
                                eprintln!("[sign] 线程 {i} {} 失败：{e}", primary.label());
                                Err(Error::kinded(
                                    "sign_failed",
                                    format!(
                                        "{}签名失败：{e}。可在「设置 → 服务 → 签名」切到{}后重试",
                                        primary.cn(),
                                        if primary == Backend::Native {
                                            Backend::Emu.cn()
                                        } else {
                                            Backend::Native.cn()
                                        }
                                    ),
                                ))
                            }
                        };
                        if let Some(tx) = reply {
                            let _ = tx.send(out);
                        }
                    }
                })
                .map_err(|e| Error::msg(format!("签名线程启动失败: {e}")))?;
            workers.push(tx);
        }
        Ok(Self {
            workers: Arc::new(workers),
            next: Arc::new(AtomicUsize::new(0)),
            backend: current,
            env,
            stats,
        })
    }

    /// 没接设置入口的调用方（探针、独立二进制）用：环境覆盖，缺省台子。
    pub fn spawn_from_env(res: PathBuf) -> Result<Self> {
        Self::spawn(res, env_backend().unwrap_or(Backend::Native))
    }

    pub fn backend(&self) -> Backend {
        if self.backend.load(Ordering::Relaxed) == 1 {
            Backend::Emu
        } else {
            Backend::Native
        }
    }

    /// 运行时切换；在途请求保持发起时的后端，新请求走 `backend`。
    pub fn set_backend(&self, backend: Backend) {
        self.backend.store(backend.idx(), Ordering::Relaxed);
    }

    pub fn info(&self) -> SignInfo {
        SignInfo {
            backend: self.backend(),
            env: self.env,
            ok: [
                self.stats.ok[0].load(Ordering::Relaxed),
                self.stats.ok[1].load(Ordering::Relaxed),
            ],
            err: [
                self.stats.err[0].load(Ordering::Relaxed),
                self.stats.err[1].load(Ordering::Relaxed),
            ],
            last: self
                .stats
                .last
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone(),
        }
    }

    pub fn sign(&self, did: &str, iid: &str, url: &str, extra: &str) -> Result<String> {
        let (tx, rx) = mpsc::channel();
        self.send(did, iid, url, extra, Some(tx))?;
        rx.recv().map_err(|_| Error::msg("签名线程已退出"))?
    }

    /// 每条工作线程各打一发真实签名，把它们自己的冷译文提前跑掉。
    pub fn warm_each(&self, did: &str, iid: &str, url: &str, extra: &str) -> Result<()> {
        let mut rx = Vec::with_capacity(self.workers.len());
        for tx in self.workers.iter() {
            let (rtx, rrx) = mpsc::channel();
            tx.send(Cmd::Sign {
                did: did.into(),
                iid: iid.into(),
                url: url.into(),
                extra: extra.into(),
                backend: self.backend(),
                reply: Some(rtx),
            })
            .map_err(|_| Error::msg("签名线程已退出"))?;
            rx.push(rrx);
        }
        for r in rx {
            let _ = r.recv().map_err(|_| Error::msg("签名线程已退出"))?;
        }
        Ok(())
    }

    /// 只投递不等待：用于预热下一页。
    pub fn presign(&self, did: &str, iid: &str, url: &str, extra: &str) {
        let _ = self.send(did, iid, url, extra, None);
    }

    fn send(
        &self,
        did: &str,
        iid: &str,
        url: &str,
        extra: &str,
        reply: Option<Sender<Result<String>>>,
    ) -> Result<()> {
        let n = self.workers.len();
        if n == 0 {
            return Err(Error::msg("签名线程已退出"));
        }
        let backend = self.backend();
        let start = self.next.fetch_add(1, Ordering::Relaxed) % n;
        for k in 0..n {
            let tx = &self.workers[(start + k) % n];
            if tx
                .send(Cmd::Sign {
                    did: did.into(),
                    iid: iid.into(),
                    url: url.into(),
                    extra: extra.into(),
                    backend,
                    reply: reply.clone(),
                })
                .is_ok()
            {
                return Ok(());
            }
        }
        Err(Error::msg("签名线程已退出"))
    }
}

fn sign_emu(
    s: &mut dysign::Signer,
    ident: &mut Option<(String, String)>,
    did: &str,
    iid: &str,
    url: &str,
    extra: &str,
) -> std::result::Result<String, String> {
    if ident.as_ref().map(|(d, i)| (d.as_str(), i.as_str())) != Some((did, iid)) {
        s.set_identity(did, iid).map_err(|e| e.to_string())?;
        *ident = Some((did.to_string(), iid.to_string()));
    }
    s.sign(url, extra).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::Backend;

    #[test]
    fn sign_info_json_shape() {
        let i = super::SignInfo {
            backend: Backend::Emu,
            env: None,
            ok: [1, 2],
            err: [3, 4],
            last: None,
        };
        let v = i.json();
        assert_eq!(v["backend"], "emu");
        assert_eq!(v["native"]["ok"], 1);
        assert_eq!(v["emu"]["err"], 4);
        assert!(v["last"].is_null());
    }

    #[test]
    fn backend_labels_for_hints() {
        assert_eq!(Backend::Native.cn(), "原生");
        assert_eq!(Backend::Emu.cn(), "兼容（模拟器）");
    }

    #[test]
    fn parse_backend_aliases() {
        assert_eq!(Backend::parse("native"), Some(Backend::Native));
        assert_eq!(Backend::parse(" PURE "), Some(Backend::Native));
        assert_eq!(Backend::parse("emu"), Some(Backend::Emu));
        assert_eq!(Backend::parse("qemu"), Some(Backend::Emu));
        assert_eq!(Backend::parse("replay"), Some(Backend::Emu));
        assert_eq!(Backend::parse(""), None);
    }
}
