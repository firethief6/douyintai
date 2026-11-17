//! 采样/计数剖析（`DYSIGN_PROFILE=1` 开启）：按模块、线程归集 guest 指令数与时间样本，
//! 另记宿主 sleep、系统调用耗时与线程切换。只为定位热点，默认零开销。
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

const MAX_MODULES: usize = 96;

pub struct Profile {
    names: Vec<String>,
    starts: Vec<u64>,
    ends: Vec<u64>,
    blocks: Vec<AtomicU64>,
    instrs: Vec<AtomicU64>,
    samples: Vec<AtomicU64>,
    sample_log: Mutex<Vec<(u64, usize, u64, u64)>>,
    stack_n: AtomicU64,
    switches: AtomicU64,
    sleep_ns: AtomicU64,
    syscall_ns: AtomicU64,
    syscalls: Mutex<HashMap<u64, u64>>,
    started: Instant,
    last_sample: Mutex<Instant>,
    marks: Mutex<Vec<(String, Duration)>>,
}

static INSTANCE: OnceLock<Profile> = OnceLock::new();
static ON: OnceLock<bool> = OnceLock::new();

fn on() -> bool {
    *ON.get_or_init(|| std::env::var("DYSIGN_PROFILE").is_ok())
}

pub fn get() -> Option<&'static Profile> {
    if !on() {
        return None;
    }
    INSTANCE.get()
}

/// 由模拟器在装载完模块后调用；只登记可执行区间。
pub fn init(ranges: Vec<(u64, u64, String)>) {
    if !on() {
        return;
    }
    let mut ranges = ranges;
    ranges.truncate(MAX_MODULES);
    let n = ranges.len();
    let mut names = Vec::with_capacity(n);
    let mut starts = Vec::with_capacity(n);
    let mut ends = Vec::with_capacity(n);
    for (a, b, name) in ranges {
        starts.push(a);
        ends.push(b);
        names.push(name);
    }
    let p = Profile {
        names,
        starts,
        ends,
        blocks: (0..n).map(|_| AtomicU64::new(0)).collect(),
        instrs: (0..n).map(|_| AtomicU64::new(0)).collect(),
        samples: (0..n).map(|_| AtomicU64::new(0)).collect(),
        sample_log: Mutex::new(Vec::new()),
        stack_n: AtomicU64::new(0),
        switches: AtomicU64::new(0),
        sleep_ns: AtomicU64::new(0),
        syscall_ns: AtomicU64::new(0),
        syscalls: Mutex::new(HashMap::new()),
        started: Instant::now(),
        last_sample: Mutex::new(Instant::now()),
        marks: Mutex::new(Vec::new()),
    };
    let _ = INSTANCE.set(p);
}

impl Profile {
    pub fn on_block(&self, address: u64, size: u64, tid: u64) {
        let _ = self.on_block_lr(address, size, tid, 0);
    }
    pub fn on_block_lr(&self, address: u64, size: u64, tid: u64, lr: u64) -> bool {
        let Some(idx) = self.index_of(address) else { return false };
        self.blocks[idx].fetch_add(1, Ordering::Relaxed);
        self.instrs[idx].fetch_add(size, Ordering::Relaxed);
        let now = Instant::now();
        let mut last = self.last_sample.lock().unwrap_or_else(|e| e.into_inner());
        if now.duration_since(*last) >= Duration::from_micros(200) {
            *last = now;
            self.samples[idx].fetch_add(1, Ordering::Relaxed);
            if let Ok(mut log) = self.sample_log.lock() {
                if log.len() < 2_000_000 {
                    log.push((tid, idx, address, lr));
                }
            }
            return true;
        }
        false
    }

    fn index_of(&self, address: u64) -> Option<usize> {
        // 模块数很少，线性扫足够；区间按装载顺序基本有序。
        for i in 0..self.starts.len() {
            if address >= self.starts[i] && address < self.ends[i] {
                return Some(i);
            }
        }
        None
    }

    pub fn stack_due(&self) -> bool {
        self.stack_n.fetch_add(1, Ordering::Relaxed) % 512 == 0
    }

    pub fn on_switch(&self) {
        self.switches.fetch_add(1, Ordering::Relaxed);
    }

    pub fn on_sleep(&self, d: Duration) {
        self.sleep_ns.fetch_add(d.as_nanos() as u64, Ordering::Relaxed);
    }

    pub fn on_syscall(&self, nr: u64, d: Duration) {
        self.syscall_ns.fetch_add(d.as_nanos() as u64, Ordering::Relaxed);
        if let Ok(mut m) = self.syscalls.lock() {
            *m.entry(nr).or_insert(0) += 1;
        }
    }

    pub fn mark(&self, name: &str) {
        if let Ok(mut m) = self.marks.lock() {
            m.push((name.to_string(), self.started.elapsed()));
        }
    }
}

pub fn mark(name: &str) {
    if let Some(p) = get() {
        p.mark(name);
    }
}
pub fn on_switch() {
    if let Some(p) = get() {
        p.on_switch();
    }
}
pub fn on_sleep(d: Duration) {
    if let Some(p) = get() {
        p.on_sleep(d);
    }
}
pub fn on_syscall(nr: u64, d: Duration) {
    if let Some(p) = get() {
        p.on_syscall(nr, d);
    }
}

fn pct(part: u64, whole: u64) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 * 100.0 / whole as f64 }
}

pub fn dump() {
    let Some(p) = get() else { return };
    let wall = p.started.elapsed();
    let total_samples: u64 = p.samples.iter().map(|a| a.load(Ordering::Relaxed)).sum();
    let total_instr: u64 = p.instrs.iter().map(|a| a.load(Ordering::Relaxed)).sum();
    let sleep = p.sleep_ns.load(Ordering::Relaxed);
    let sys_ns = p.syscall_ns.load(Ordering::Relaxed);
    eprintln!("[prof] wall={}ms samples={total_samples} instr={total_instr} switches={} sleep={}ms syscall={}ms",
        wall.as_millis(), p.switches.load(Ordering::Relaxed), sleep / 1_000_000, sys_ns / 1_000_000);
    if let Ok(m) = p.marks.lock() {
        for (n, t) in m.iter() {
            eprintln!("[prof] mark {n} @{}ms", t.as_millis());
        }
    }
    eprintln!("[prof] {:<34} {:>10} {:>7} {:>7}", "module", "instr", "instr%", "time%");
    let mut order: Vec<usize> = (0..p.names.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(p.instrs[i].load(Ordering::Relaxed)));
    for &i in &order {
        let n = p.instrs[i].load(Ordering::Relaxed);
        if n == 0 { continue; }
        eprintln!("[prof] {:<34} {:>10} {:>6.1}% {:>6.1}%",
            p.names[i], n, pct(n, total_instr), pct(p.samples[i].load(Ordering::Relaxed), total_samples));
    }
    // 线程时间占比 + 每线程热点
    let log = p.sample_log.lock().map(|g| g.clone()).unwrap_or_default();
    if let Ok(path) = std::env::var("DYSIGN_PROFILE_DUMP") {
        use std::io::Write;
        if let Ok(mut f) = std::fs::File::create(&path) {
            for (tid, idx, addr, lr) in log.iter() {
                let _ = writeln!(f, "{tid} {idx} 0x{addr:x} 0x{lr:x}");
            }
        }
        for (i, name) in p.names.iter().enumerate() {
            eprintln!("[prof] base {i} {name} 0x{:x}", p.starts[i]);
        }
    }
    let mut by_tid: HashMap<u64, u64> = HashMap::new();
    let mut by_tid_mod: HashMap<(u64, usize), u64> = HashMap::new();
    let mut by_tid_off: HashMap<(u64, usize, u64), u64> = HashMap::new();
    let mut by_tid_lr: HashMap<(u64, usize, u64), u64> = HashMap::new();
    let page = 1u64;
    for (tid, idx, addr, lr) in log.iter() {
        *by_tid.entry(*tid).or_insert(0) += 1;
        *by_tid_mod.entry((*tid, *idx)).or_insert(0) += 1;
        *by_tid_off.entry((*tid, *idx, addr / page * page)).or_insert(0) += 1;
        *by_tid_lr.entry((*tid, *idx, lr / page * page)).or_insert(0) += 1;
    }
    let mut tids: Vec<(u64, u64)> = by_tid.into_iter().collect();
    tids.sort_by_key(|x| std::cmp::Reverse(x.1));
    eprintln!("[prof] threads (sample share):");
    for (tid, n) in tids.iter().take(12) {
        eprintln!("[prof]   tid {tid}: {n} ({:.1}%)", pct(*n, total_samples));
        let mut mods: Vec<((u64, usize), u64)> = by_tid_mod.iter().filter(|((t, _), _)| t == tid).map(|(k, v)| (*k, *v)).collect();
        mods.sort_by_key(|x| std::cmp::Reverse(x.1));
        for ((_, idx), c) in mods.iter().take(5) {
            eprintln!("[prof]     {:<30} {} ({:.1}%)", p.names[*idx], c, pct(*c, *n));
        }
        let mut offs: Vec<((u64, usize, u64), u64)> = by_tid_off.iter().filter(|((t, _, _), _)| t == tid).map(|(k, v)| (*k, *v)).collect();
        offs.sort_by_key(|x| std::cmp::Reverse(x.1));
        for ((_, idx, off), c) in offs.iter().take(6) {
            eprintln!("[prof]     hot {}:0x{:x} {}", p.names[*idx], off.saturating_sub(p.starts[*idx]), c);
        }
        let mut lrs: Vec<((u64, usize, u64), u64)> = by_tid_lr.iter().filter(|((t, _, _), _)| t == tid).map(|(k, v)| (*k, *v)).collect();
        lrs.sort_by_key(|x| std::cmp::Reverse(x.1));
        for ((_, idx, lr), c) in lrs.iter().take(6) {
            eprintln!("[prof]     lr {}:0x{:x} {}", p.names[*idx], lr.saturating_sub(p.starts[*idx]), c);
        }
    }
    eprintln!("[prof] syscalls:");
    if let Ok(m) = p.syscalls.lock() {
        let mut v: Vec<(u64, u64)> = m.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1));
        for (nr, c) in v.iter().take(12) {
            eprintln!("[prof]   nr {nr}: {c}");
        }
    }
}

// ---- 范围读监视（`DYSIGN_WATCH=0xLO:len`）：统计每个 PC 命中次数 ----

static WATCH: OnceLock<Option<(u64, u64)>> = OnceLock::new();
static WATCH_HITS: OnceLock<Mutex<HashMap<(u64, u64, u64), u64>>> = OnceLock::new();

pub fn watch_spec() -> Option<(u64, u64)> {
    *WATCH.get_or_init(|| {
        let s = std::env::var("DYSIGN_WATCH").ok()?;
        let (lo, len) = s.split_once(':')?;
        let lo = u64::from_str_radix(lo.trim_start_matches("0x"), 16).ok()?;
        let len = u64::from_str_radix(len.trim_start_matches("0x"), 16).unwrap_or(0x1000);
        Some((lo, len))
    })
}

pub fn watch_hit(pc: u64, tid: u64, addr: u64) {
    let m = WATCH_HITS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut m) = m.lock() {
        *m.entry((pc, tid, addr / 8)).or_insert(0) += 1;
    }
}

pub fn watch_dump() {
    if watch_spec().is_none() { return; }
    let m = WATCH_HITS.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(m) = m.lock() else { return };
    let mut v: Vec<((u64, u64, u64), u64)> = m.iter().map(|(k, c)| (*k, *c)).collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.1));
    let total: u64 = v.iter().map(|x| x.1).sum();
    eprintln!("[watch] hits={total} distinct={}", v.len());
    for ((pc, tid, off), c) in v.iter().take(12) {
        eprintln!("[watch]   pc 0x{pc:x} tid {tid} off 0x{:x}: {c}", off * 8);
    }
}

pub fn module_base(name: &str) -> Option<u64> {
    let p = INSTANCE.get()?;
    (0..p.names.len()).find(|&i| p.names[i].contains(name)).map(|i| p.starts[i])
}

// ---- 堆写监视（`DYSIGN_WWRITE=1`）：记录 guest 堆范围的每次写，可落盘 ----

static WTRACE: OnceLock<bool> = OnceLock::new();
static WALL: OnceLock<bool> = OnceLock::new();
static ALLOC_ON: OnceLock<bool> = OnceLock::new();
static WADDR: OnceLock<Option<(u64, u64)>> = OnceLock::new();
static WLOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static ALOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn wwrite_on() -> bool {
    *WTRACE.get_or_init(|| std::env::var("DYSIGN_WWRITE").is_ok())
}
fn wwrite_all() -> bool {
    *WALL.get_or_init(|| std::env::var("DYSIGN_WWRITE").ok().as_deref() == Some("all"))
}
pub fn alloc_on() -> bool {
    *ALLOC_ON.get_or_init(|| std::env::var("DYSIGN_ALLOC").is_ok())
}
static ALLOCW: OnceLock<u64> = OnceLock::new();
static WATCHED: OnceLock<Mutex<Vec<(u64, u64)>>> = OnceLock::new();
pub fn note_watched(a: u64, b: u64) {
    WATCHED.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().push((a, b));
}
pub fn is_watched(a: u64) -> bool {
    WATCHED.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap().iter().any(|(x, y)| a >= *x && a < *y)
}
static MEMLOG2: OnceLock<Option<u64>> = OnceLock::new();
static MEMLOGX: OnceLock<Option<u64>> = OnceLock::new();
pub fn memlogx_pc() -> Option<u64> {
    *MEMLOGX.get_or_init(|| {
        std::env::var("DYSIGN_MEMLOGX")
            .ok()
            .and_then(|v| u64::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok())
    })
}
pub fn memlogx_hit(lr: u64, dst: u64, len: u64, bytes: &[u8]) {
    let mut s = format!("X 0x{lr:x} 0x{dst:x} {len} ");
    for b in bytes { s.push_str(&format!("{b:02x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}
pub fn memlog2_pc() -> Option<u64> {
    *MEMLOG2.get_or_init(|| {
        std::env::var("DYSIGN_MEMLOG2")
            .ok()
            .and_then(|v| u64::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok())
    })
}
pub fn memlog2_hit(lr: u64, dst: u64, src: u64, len: u64, bytes: &[u8]) {
    let mut s = format!("M 0x{lr:x} 0x{dst:x} 0x{src:x} {len} ");
    for b in bytes { s.push_str(&format!("{b:02x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}
pub fn allocw_size() -> u64 {
    *ALLOCW.get_or_init(|| {
        std::env::var("DYSIGN_ALLOCWATCH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    })
}
fn wlog() -> &'static Mutex<Vec<String>> {
    WLOG.get_or_init(|| Mutex::new(Vec::new()))
}
fn alog() -> &'static Mutex<Vec<String>> {
    ALOG.get_or_init(|| Mutex::new(Vec::new()))
}
fn interesting(bytes: &[u8]) -> bool {
    if bytes.len() == 1 {
        return matches!(bytes[0], b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' | b'=');
    }
    if bytes.len() == 36 || bytes.len() == 48 {
        return true;
    }
    if bytes.len() >= 9 && bytes.windows(8).any(|w| w == b"X-Helios" || w == b"X-Medusa") {
        return true;
    }
    bytes.len() >= 8
        && bytes.iter().all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' | b'=' ))
}
/// 把绝对地址渲染成“模块+偏移”，便于跨次运行对位。
pub fn loc(addr: u64) -> String {
    const M: &[(u64, u64, &str)] = &[
        (0x10000000, 0x1032b000, "metasec"),
        (0x10360000, 0x10369000, "liblog"),
        (0x10389000, 0x10460000, "libc++"),
        (0x10480000, 0x10481000, "libdl"),
        (0x104a0000, 0x10563000, "libc"),
        (0x10590000, 0x105c8000, "libm"),
        (0x70000000, 0x70200000, "stack"),
        (0x50000000, 0x60000000, "heap"),
    ];
    for (lo, hi, name) in M {
        if addr >= *lo && addr < *hi {
            return format!("{name}+0x{:x}", addr - lo);
        }
    }
    format!("@0x{addr:x}")
}

pub fn write_hit(pc: u64, tid: u64, lr: u64, addr: u64, bytes: &[u8]) {
    if wlog().lock().map(|m| m.len() >= 4_000_000).unwrap_or(false) {
        return;
    }
    if let Some((lo, len)) = *WADDR.get_or_init(|| {
        let s = std::env::var("DYSIGN_WADDR").ok()?;
        let (lo, len) = s.split_once(':')?;
        Some((
            u64::from_str_radix(lo.trim().trim_start_matches("0x"), 16).ok()?,
            u64::from_str_radix(len.trim().trim_start_matches("0x"), 16).unwrap_or(0x100),
        ))
    }) {
        if addr < lo || addr >= lo + len {
            return;
        }
    } else if !wwrite_all() && !interesting(bytes) {
        return;
    }
    let mut s = String::with_capacity(16 + bytes.len() * 2);
    s.push_str(&format!("W {tid} {} {} 0x{addr:x} {} ", loc(pc), loc(lr), bytes.len()));
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    if let Ok(mut m) = wlog().lock() {
        m.push(s);
    }
}
pub fn alloc_hit(tid: u64, size: u64, lr: u64, ret: Option<u64>) {
    let mut s = format!("A {tid} 0x{lr:x} {size}");
    if let Some(ret) = ret {
        s.push_str(&format!(" 0x{ret:x}"));
    }
    if let Ok(mut m) = alog().lock() {
        m.push(s);
    }
}
pub fn write_dump() {
    let path = std::env::var("DYSIGN_TRACE_FILE").ok();
    let Ok(w) = wlog().lock() else { return };
    let Ok(a) = alog().lock() else { return };
    if let Some(path) = path {
        use std::io::Write;
        if let Ok(mut f) = std::fs::File::create(path) {
            for l in a.iter() {
                let _ = writeln!(f, "{l}");
            }
            for l in w.iter() {
                let _ = writeln!(f, "{l}");
            }
        }
    }
    eprintln!("[wtrace] writes={} allocs={}", w.len(), a.len());
}

// ---- 区间读监视（`DYSIGN_RWATCH=0xADDR:len`，绝对地址）与参数打印（`DYSIGN_ARGLOG=0xPC`）----

static RSPEC: OnceLock<Option<(u64, u64)>> = OnceLock::new();
pub fn rwatch_spec() -> Option<(u64, u64)> {
    *RSPEC.get_or_init(|| {
        let s = std::env::var("DYSIGN_RWATCH").ok()?;
        let (lo, len) = s.split_once(':')?;
        Some((
            u64::from_str_radix(lo.trim().trim_start_matches("0x"), 16).ok()?,
            u64::from_str_radix(len.trim().trim_start_matches("0x"), 16).unwrap_or(0x100),
        ))
    })
}
pub fn read_hit(pc: u64, tid: u64, lr: u64, addr: u64, bytes: &[u8]) {
    if wlog().lock().map(|m| m.len() >= 1_000_000).unwrap_or(false) {
        return;
    }
    let mut s = format!("R {tid} {} {} 0x{addr:x} {} ", loc(pc), loc(lr), bytes.len());
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    if let Ok(mut m) = wlog().lock() {
        m.push(s);
    }
}
static MEMLOG: OnceLock<Option<(u64, u64)>> = OnceLock::new();
pub fn memlog_spec() -> Option<(u64, u64)> {
    *MEMLOG.get_or_init(|| {
        let s = std::env::var("DYSIGN_MEMLOG").ok()?;
        let (pc, addr) = s.split_once(':')?;
        Some((
            u64::from_str_radix(pc.trim().trim_start_matches("0x"), 16).ok()?,
            u64::from_str_radix(addr.trim().trim_start_matches("0x"), 16).ok()?,
        ))
    })
}
static CHASE: OnceLock<Option<(u64, u64)>> = OnceLock::new();
pub fn chase_spec() -> Option<(u64, u64)> {
    *CHASE.get_or_init(|| {
        let s = std::env::var("DYSIGN_CHASE").ok()?;
        let (pc, addr) = s.split_once(':')?;
        Some((
            u64::from_str_radix(pc.trim().trim_start_matches("0x"), 16).ok()?,
            u64::from_str_radix(addr.trim().trim_start_matches("0x"), 16).ok()?,
        ))
    })
}
pub fn chase_hit(pc: u64, tid: u64, addr: u64, bytes: &[u8]) {
    let mut s = format!("K {tid} 0x{pc:x} 0x{addr:x} {} ", bytes.len());
    for b in bytes { s.push_str(&format!("{b:02x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}
pub fn memlog_hit(dst: u64, src: u64, len: u64, bytes: &[u8]) {
    let mut s = format!("C 0x{dst:x} 0x{src:x} {len} ");
    for b in bytes { s.push_str(&format!("{b:02x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}
pub fn arg_specs() -> &'static Vec<u64> {
    static V: OnceLock<Vec<u64>> = OnceLock::new();
    V.get_or_init(|| {
        let Some(s) = std::env::var("DYSIGN_ARGLOG").ok() else { return Vec::new() };
        s.split(',').filter_map(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok()).collect()
    })
}
pub fn arg_spec() -> Option<u64> {
    arg_specs().first().copied()
}
pub fn arg_hit_sp(pc: u64, tid: u64, regs: &[u64], sp: u64, lr: u64) {
    let mut s = format!("P {tid} 0x{pc:x} sp=0x{sp:x} lr=0x{lr:x}");
    for r in regs { s.push_str(&format!(" 0x{r:x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}
pub fn arg_hit(pc: u64, tid: u64, regs: &[u64]) {
    let mut s = format!("P {tid} 0x{pc:x}");
    for r in regs {
        s.push_str(&format!(" 0x{r:x}"));
    }
    if let Ok(mut m) = wlog().lock() {
        m.push(s);
    }
}

// ---- 栈指针条件指令日志（`DYSIGN_SPLOG=0xSP[:max]`）：sp 相等时打印 PC 与 x0..x3 ----

static SPLOG: OnceLock<Option<(u64, usize)>> = OnceLock::new();
pub fn splog_spec() -> Option<(u64, usize)> {
    *SPLOG.get_or_init(|| {
        let s = std::env::var("DYSIGN_SPLOG").ok()?;
        let (sp, max) = s.split_once(':').unwrap_or((s.as_str(), "200000"));
        Some((
            u64::from_str_radix(sp.trim().trim_start_matches("0x"), 16).ok()?,
            max.trim().parse().ok()?,
        ))
    })
}
pub fn splog_hit(pc: u64, tid: u64, regs: &[u64]) -> bool {
    if wlog().lock().map(|m| m.len() >= 2_000_000).unwrap_or(false) {
        return false;
    }
    let mut s = format!("S {tid} {pc}");
    for r in regs.iter().take(4) {
        s.push_str(&format!(" 0x{r:x}"));
    }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
    true
}

// ---- 寄存器基址读监视（`DYSIGN_RWATCHREG=0xPC:xREG:len`）与可控改写（`DYSIGN_PATCH=pc:xREG:off:hex[:occ]`）----

pub struct RegRead { pub pc: u64, pub reg: usize, pub len: u64 }
static RREG: OnceLock<Vec<RegRead>> = OnceLock::new();
pub fn rwatchreg_specs() -> &'static Vec<RegRead> {
    RREG.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_RWATCHREG").ok() else { return v };
        for item in s.split(',') {
            let p: Vec<&str> = item.split(':').collect();
            if p.len() != 3 { continue; }
            let Ok(pc) = u64::from_str_radix(p[0].trim().trim_start_matches("0x"), 16) else { continue };
            let Ok(reg) = p[1].trim().trim_start_matches('x').parse::<usize>() else { continue };
            let Ok(len) = u64::from_str_radix(p[2].trim().trim_start_matches("0x"), 16) else { continue };
            v.push(RegRead { pc, reg, len });
        }
        v
    })
}

pub struct Patch { pub pc: u64, pub base: Option<usize>, pub eq: Option<u64>, pub abs: u64, pub off: u64, pub bytes: Vec<u8>, pub occ: u64 }
static PATCHES: OnceLock<Result<Vec<Patch>, String>> = OnceLock::new();
static PCOUNT: OnceLock<Mutex<HashMap<u64, u64>>> = OnceLock::new();

fn hexnum(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim().trim_start_matches("0x"), 16)
        .map_err(|_| format!("不是十六进制数: {s}"))
}

fn parse_patch_item(item: &str, origin: &str, no: usize) -> Result<Patch, String> {
    let err = |why: &str| format!("{origin} 第 {no} 项无效（{why}）: {item}");
    let p: Vec<&str> = item.split(':').collect();
    if p.len() < 4 || p.len() > 5 {
        return Err(err("应为 pc:BASE:off:hex[:occ]"));
    }
    let pc = hexnum(p[0]).map_err(|e| err(&e))?;
    let b = p[1].trim();
    // 寄存器基址可写 `x5` 或裸 `5`；裸数字只在小范围内当寄存器，其余按绝对地址。
    let reg_of = |t: &str| t.trim().trim_start_matches('x').parse::<usize>();
    let (base, eq, abs) = match b.find(|c| c == '=' || c == '+') {
        Some(i) => {
            let r = reg_of(&b[..i]).map_err(|e| err(&e.to_string()))?;
            let tail = &b[i..];
            match tail.strip_prefix('=') {
                Some(want) => (Some(r), Some(hexnum(want).map_err(|e| err(&e))?), 0),
                None => (Some(r), None, hexnum(&tail[1..]).map_err(|e| err(&e))?),
            }
        }
        None => match reg_of(b) {
            Ok(r) if r <= 30 => (Some(r), None, 0),
            _ => (None, None, hexnum(b).map_err(|e| err(&e))?),
        },
    };
    let off = hexnum(p[2]).map_err(|e| err(&e))?;
    let h = p[3].trim().trim_start_matches("0x");
    if h.is_empty() || h.len() % 2 != 0 {
        return Err(err("hex 须为非空偶数长度"));
    }
    let mut bytes = Vec::new();
    for i in (0..h.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&h[i..i + 2], 16).map_err(|e| err(&e.to_string()))?);
    }
    let occ = match p.get(4) {
        Some(x) => x.parse::<u64>().map_err(|e| err(&e.to_string()))?,
        None => 0,
    };
    Ok(Patch { pc, base, eq, abs, off, bytes, occ })
}

fn parse_patches() -> Result<Vec<Patch>, String> {
    let mut v = Vec::new();
    if let Ok(s) = std::env::var("DYSIGN_PATCH") {
        if !s.trim().is_empty() {
            for (i, item) in s.split(',').enumerate() {
                if item.trim().is_empty() { continue; }
                v.push(parse_patch_item(item, "DYSIGN_PATCH", i + 1)?);
            }
        }
    }
    if let Ok(path) = std::env::var("DYSIGN_PATCH_FILE") {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("读不了 DYSIGN_PATCH_FILE={path}: {e}"))?;
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            v.push(parse_patch_item(line, &path, i + 1)?);
        }
    }
    Ok(v)
}

pub fn patch_specs() -> &'static [Patch] {
    match PATCHES.get_or_init(parse_patches) {
        Ok(v) => v,
        Err(_) => &[],
    }
}
pub struct SetReg { pub pc: u64, pub reg: usize, pub val: u64, pub occ: u64 }
static SETREG: OnceLock<Vec<SetReg>> = OnceLock::new();
pub fn setreg_specs() -> &'static Vec<SetReg> {
    SETREG.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_SETREG").ok() else { return v };
        for item in s.split(',') {
            let p: Vec<&str> = item.split(':').collect();
            if p.len() < 3 || p.len() > 4 { continue; }
            let Ok(pc) = u64::from_str_radix(p[0].trim().trim_start_matches("0x"), 16) else { continue };
            let Ok(reg) = p[1].trim().trim_start_matches('x').parse::<usize>() else { continue };
            let Ok(val) = u64::from_str_radix(p[2].trim().trim_start_matches("0x"), 16) else { continue };
            let occ = p.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
            v.push(SetReg { pc, reg, val, occ });
        }
        v
    })
}
pub fn patch_occ(pc: u64) -> u64 {
    let m = PCOUNT.get_or_init(|| Mutex::new(HashMap::new()));
    let mut m = m.lock().unwrap_or_else(|e| e.into_inner());
    let v = m.entry(pc).or_insert(0);
    let cur = *v;
    *v += 1;
    cur
}

// ---- 返回日志（`DYSIGN_RETLOG=0xPC`）与锚点自证（`DYSIGN_ANCHOR=0xPC:0xADDR:len`）----

static SNAP_AT: OnceLock<Option<u64>> = OnceLock::new();
/// `DYSIGN_SNAP_AT=0xPC`：命中该 PC 时导出全量快照（配 `DYSIGN_SNAP_OUT`）。
pub fn snap_at() -> Option<u64> {
    *SNAP_AT.get_or_init(|| {
        std::env::var("DYSIGN_SNAP_AT")
            .ok()
            .and_then(|s| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok())
    })
}

static RETLOG: OnceLock<Result<Option<u64>, String>> = OnceLock::new();
pub fn retlog_pc() -> Option<u64> {
    match RETLOG.get_or_init(|| {
        match std::env::var("DYSIGN_RETLOG") {
            Ok(s) if !s.trim().is_empty() => hexnum(&s).map(Some).map_err(|e| format!("DYSIGN_RETLOG: {e}")),
            _ => Ok(None),
        }
    }) {
        Ok(v) => *v,
        Err(_) => None,
    }
}

#[derive(Clone)]
pub struct Anchor { pub pc: u64, pub addr: u64, pub len: u64 }
static ANCHORS: OnceLock<Result<Vec<Anchor>, String>> = OnceLock::new();
static ANCHOR_HITS: AtomicU64 = AtomicU64::new(0);

fn parse_anchors() -> Result<Vec<Anchor>, String> {
    match std::env::var("DYSIGN_ANCHOR") {
        Ok(s) => parse_anchor_specs(&s),
        Err(_) => Ok(Vec::new()),
    }
}

/// 解析 `DYSIGN_ANCHOR` 的逗号列表；任何一项写错都返回错误。
pub fn parse_anchor_specs(s: &str) -> Result<Vec<Anchor>, String> {
    let mut v = Vec::new();
    for (i, item) in s.split(',').enumerate() {
        if item.trim().is_empty() { continue; }
        let p: Vec<&str> = item.split(':').collect();
        let err = |why: &str| format!("DYSIGN_ANCHOR 第 {} 项无效（{why}）: {item}", i + 1);
        if p.len() != 3 { return Err(err("应为 pc:addr:len")); }
        let pc = hexnum(p[0]).map_err(|e| err(&e))?;
        let addr = hexnum(p[1]).map_err(|e| err(&e))?;
        let len = hexnum(p[2]).map_err(|e| err(&e))?;
        if len == 0 || len > 4096 { return Err(err("len 须在 1..=4096")); }
        v.push(Anchor { pc, addr, len });
    }
    Ok(v)
}

pub fn anchor_specs() -> &'static [Anchor] {
    match ANCHORS.get_or_init(parse_anchors) {
        Ok(v) => v,
        Err(_) => &[],
    }
}
pub fn anchor_reset() {
    ANCHOR_HITS.store(0, Ordering::Relaxed);
}
pub fn anchor_hit() {
    ANCHOR_HITS.fetch_add(1, Ordering::Relaxed);
}
pub fn anchor_hits() -> u64 {
    ANCHOR_HITS.load(Ordering::Relaxed)
}

/// 启动时把全部调试规格解析一遍并回显；任何一项写错都让进程直接失败。
pub fn validate_env() -> Result<(), String> {
    match PATCHES.get_or_init(parse_patches) {
        Ok(v) => {
            for p in v {
                eprintln!(
                    "[spec] patch pc=0x{:x} base={} off=0x{:x} bytes={} occ={}",
                    p.pc,
                    match p.base { Some(r) => format!("x{r}"), None => format!("0x{:x}", p.abs) },
                    p.off,
                    p.bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(),
                    p.occ
                );
            }
        }
        Err(e) => return Err(e.clone()),
    }
    match ANCHORS.get_or_init(parse_anchors) {
        Ok(v) => {
            for a in v {
                eprintln!("[spec] anchor pc=0x{:x} addr=0x{:x} len={}", a.pc, a.addr, a.len);
            }
        }
        Err(e) => return Err(e.clone()),
    }
    match std::env::var("DYSIGN_RETLOG") {
        Ok(s) if !s.trim().is_empty() => {
            let pc = hexnum(&s).map_err(|e| format!("DYSIGN_RETLOG: {e}"))?;
            eprintln!("[spec] retlog pc=0x{pc:x}");
        }
        _ => {}
    }
    Ok(())
}

// ---- 指令级区间跟踪（`DYSIGN_CTRACE=0xLO:0xHI`）与定点内存转储（`DYSIGN_DUMP=0xPC:0xADDR:len`）----

static CTRACE: OnceLock<Option<(u64, u64)>> = OnceLock::new();
static CTRACE_SP: OnceLock<Option<u64>> = OnceLock::new();
pub fn ctrace_sp() -> Option<u64> {
    *CTRACE_SP.get_or_init(|| {
        std::env::var("DYSIGN_CTRACE_SP")
            .ok()
            .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
    })
}
pub fn ctrace_spec() -> Option<(u64, u64)> {
    *CTRACE.get_or_init(|| {
        let s = std::env::var("DYSIGN_CTRACE").ok()?;
        let (lo, hi) = s.split_once(':')?;
        Some((
            u64::from_str_radix(lo.trim().trim_start_matches("0x"), 16).ok()?,
            u64::from_str_radix(hi.trim().trim_start_matches("0x"), 16).ok()?,
        ))
    })
}
pub fn ctrace_hit(pc: u64, tid: u64, insn: u32, regs: &[u64], sp: u64) {
    if wlog().lock().map(|m| m.len() >= 60_000_000).unwrap_or(false) {
        return;
    }
    let mut s = format!("I {tid} 0x{pc:x} {insn:08x} sp=0x{sp:x}");
    for r in regs {
        s.push_str(&format!(" 0x{r:x}"));
    }
    if let Ok(mut m) = wlog().lock() {
        m.push(s);
    }
}

static REGS_SP: OnceLock<Option<u64>> = OnceLock::new();
pub fn regs_sp() -> Option<u64> {
    *REGS_SP.get_or_init(|| {
        std::env::var("DYSIGN_REGS_SP")
            .ok()
            .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
    })
}
pub fn reg_hit(pc: u64, sp: u64, regs: &[u64]) {
    let mut s = format!("R 0x{pc:x} 0x{sp:x}");
    for r in regs { s.push_str(&format!(" 0x{r:x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}

static DUMPALL: OnceLock<Option<(u64, String, u64)>> = OnceLock::new();
/// `DYSIGN_DUMPALL=0xPC:dir[:N]`：命中 pc 第 N 次（默认 1）时把全部已映射内存落盘到 dir。
pub fn dumpall_spec() -> Option<(u64, String, u64)> {
    DUMPALL
        .get_or_init(|| {
            let s = std::env::var("DYSIGN_DUMPALL").ok()?;
            let mut it = s.split(':');
            let pc = it.next()?;
            let dir = it.next()?;
            let n = it.next().and_then(|x| x.trim().parse::<u64>().ok()).unwrap_or(1);
            Some((
                u64::from_str_radix(pc.trim().trim_start_matches("0x"), 16).ok()?,
                dir.to_string(),
                n,
            ))
        })
        .clone()
}

static REGS_PC: OnceLock<Option<u64>> = OnceLock::new();
/// `DYSIGN_REGS_PC=0xPC`：命中该 pc 时抓全部寄存器（不看 sp）。
pub fn regs_pc() -> Option<u64> {
    *REGS_PC.get_or_init(|| {
        std::env::var("DYSIGN_REGS_PC")
            .ok()
            .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
    })
}

static DUMP_SP: OnceLock<Option<u64>> = OnceLock::new();
pub fn dump_sp() -> Option<u64> {
    *DUMP_SP.get_or_init(|| {
        std::env::var("DYSIGN_DUMP_SP")
            .ok()
            .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
    })
}

static DUMP: OnceLock<Vec<(u64, u64, u64)>> = OnceLock::new();
pub fn dump_specs() -> &'static Vec<(u64, u64, u64)> {
    DUMP.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_DUMP").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            if let (Some(p), Some(a)) = (it.next(), it.next()) {
                if let (Ok(p), Ok(a)) = (
                    u64::from_str_radix(p.trim().trim_start_matches("0x"), 16),
                    u64::from_str_radix(a.trim().trim_start_matches("0x"), 16),
                ) {
                    let n = it
                        .next()
                        .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
                        .unwrap_or(0x100);
                    v.push((p, a, n));
                }
            }
        }
        v
    })
}
pub fn dump_hit(pc: u64, sp: u64, addr: u64, bytes: &[u8]) {
    let mut s = format!("D 0x{pc:x} 0x{sp:x} 0x{addr:x} {} ", bytes.len());
    for b in bytes { s.push_str(&format!("{b:02x}")); }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}

static DUMPPTR: OnceLock<Vec<(u64, usize, u64)>> = OnceLock::new();
pub fn dumpptr_specs() -> &'static Vec<(u64, usize, u64)> {
    DUMPPTR.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_DUMPPTR").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            if let (Some(p), Some(r)) = (it.next(), it.next()) {
                if let (Ok(p), Ok(r)) = (
                    u64::from_str_radix(p.trim().trim_start_matches("0x"), 16),
                    r.trim().parse::<usize>(),
                ) {
                    let n = it
                        .next()
                        .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
                        .unwrap_or(0x40);
                    v.push((p, r, n));
                }
            }
        }
        v
    })
}

static DUMPREG: OnceLock<Vec<(u64, usize, u64)>> = OnceLock::new();
pub fn dumpreg_specs() -> &'static Vec<(u64, usize, u64)> {
    DUMPREG.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_DUMPREG").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            if let (Some(p), Some(r)) = (it.next(), it.next()) {
                if let (Ok(p), Ok(r)) = (
                    u64::from_str_radix(p.trim().trim_start_matches("0x"), 16),
                    r.trim().parse::<usize>(),
                ) {
                    let n = it
                        .next()
                        .and_then(|x| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok())
                        .unwrap_or(0x40);
                    v.push((p, r, n));
                }
            }
        }
        v
    })
}

// ---- 调用链打印（`DYSIGN_FPCHAIN=0xPC:xREG:0xVAL[:depth]`）：命中且寄存器相等时打印 FP 链 ----

static FPCHAIN: OnceLock<Vec<(u64, usize, u64, u64)>> = OnceLock::new();
pub fn fpchain_specs() -> &'static Vec<(u64, usize, u64, u64)> {
    FPCHAIN.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_FPCHAIN").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            let (Some(p), Some(r)) = (it.next(), it.next()) else { continue };
            let Some(val) = it.next() else { continue };
            let (Ok(p), Ok(r)) = (
                u64::from_str_radix(p.trim().trim_start_matches("0x"), 16),
                r.trim().parse::<usize>(),
            ) else { continue };
            let Ok(val) = u64::from_str_radix(val.trim().trim_start_matches("0x"), 16) else { continue };
            let n = it
                .next()
                .and_then(|x| x.parse::<u64>().ok())
                .unwrap_or(24);
            v.push((p, r, val, n));
        }
        v
    })
}
pub fn fpchain_hit(pc: u64, tid: u64, sp: u64, chain: &[(u64, u64)]) {
    let mut s = format!("F {tid} 0x{pc:x} sp=0x{sp:x}");
    for (fp, ret) in chain {
        s.push_str(&format!(" {} {}", loc(*fp), loc(*ret)));
    }
    if let Ok(mut m) = wlog().lock() { m.push(s); }
}

// ---- 栈指针条件内存转储（`DYSIGN_DUMPSP=0xPC:0xSP:0xADDR:len`）：sp 相等时每次命中都转储 ----

static DUMPSP: OnceLock<Vec<(u64, u64, u64, u64)>> = OnceLock::new();
pub fn dumpsp_specs() -> &'static Vec<(u64, u64, u64, u64)> {
    DUMPSP.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_DUMPSP").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            let (Some(p), Some(sp), Some(a)) = (it.next(), it.next(), it.next()) else { continue };
            let h = |x: &str| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok();
            let (Some(p), Some(sp), Some(a)) = (h(p), h(sp), h(a)) else { continue };
            let n = it.next().and_then(h).unwrap_or(0x40);
            v.push((p, sp, a, n));
        }
        v
    })
}
// ---- 条件内存转储（`DYSIGN_DUMPX=0xPC:xREG:0xVAL:0xADDR:len`）：寄存器相等时转储指定内存 ----

static DUMPX: OnceLock<Vec<(u64, usize, u64, u64, u64)>> = OnceLock::new();
pub fn dumpx_specs() -> &'static Vec<(u64, usize, u64, u64, u64)> {
    DUMPX.get_or_init(|| {
        let mut v = Vec::new();
        let Some(s) = std::env::var("DYSIGN_DUMPX").ok() else { return v };
        for item in s.split(',') {
            let mut it = item.split(':');
            let (Some(p), Some(r), Some(val)) = (it.next(), it.next(), it.next()) else { continue };
            let (Some(a), Some(n)) = (it.next(), it.next()) else { continue };
            let h = |x: &str| u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok();
            let (Some(p), Ok(r), Some(val), Some(a), Some(n)) =
                (h(p), r.trim().parse::<usize>(), h(val), h(a), h(n))
            else { continue };
            v.push((p, r, val, a, n));
        }
        v
    })
}
