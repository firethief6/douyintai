//! VM 程序里的原生回调：C++ 运行时代码（字符串、格式化、分配）的 AArch64 执行器。
//!
//! 只在 VM 跳出到口令 0x1017a1b4（单参回调）时进入；指令表由
//! `scripts/medusa-vm/mknative.py` 从真机轨迹解码生成，运行时不依赖 capstone。
//! 语义与 `scripts/medusa-vm/arm.py` 一致，未覆盖的指令显式报错。

use super::{native_ops, Error, Mem, PROG_HI, PROG_LO};
use std::collections::HashMap;
use std::hash::Hasher;

/// 指令缓存键只有 pc，用一次乘法代替 SipHash。
#[derive(Default)]
pub struct Fx(u64);
impl Hasher for Fx {
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.write_u64(*b as u64);
        }
    }
    fn write_u32(&mut self, v: u32) {
        self.write_u64(v as u64);
    }
    fn write_u64(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}
pub static PCTR: std::sync::OnceLock<std::sync::Mutex<HashMap<u64, u64>>> =
    std::sync::OnceLock::new();

pub fn dump_pctr() {
    let Some(m) = PCTR.get() else { return };
    let m = m.lock().unwrap();
    let mut v: Vec<_> = m.iter().map(|(k, v)| (*k, *v)).collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.1));
    let mut pg: std::collections::BTreeMap<u64, u64> = Default::default();
    let mut all = 0u64;
    for (pc, n) in &v {
        *pg.entry(pc >> 12).or_default() += n;
        all += n;
    }
    let mut pv: Vec<_> = pg.into_iter().collect();
    pv.sort_by_key(|x| std::cmp::Reverse(x.1));
    eprintln!("PGTOT {all} distinct={}", v.len());
    eprintln!("FETCH {}", FETCH.load(std::sync::atomic::Ordering::Relaxed));
    for (p, n) in pv.iter().take(25) {
        eprintln!("PGT {:#x} {n}", p << 12);
    }
    for (pc, n) in v.iter().take(40) {
        eprintln!("PCTR {pc:#x} {n}");
    }
}

/// 库内 SM3 压缩函数（0x1017814c）的等价实现：只更新 8 个字的状态。
fn sm3_p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}
fn sm3_p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

fn sm3_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 68];
    for (i, c) in block.as_chunks::<4>().0.iter().enumerate() {
        w[i] = u32::from_be_bytes(*c);
    }
    for j in 16..68 {
        w[j] = sm3_p1(w[j - 16] ^ w[j - 9] ^ w[j - 3].rotate_left(15))
            ^ w[j - 13].rotate_left(7)
            ^ w[j - 6];
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for j in 0..64 {
        let t = if j < 16 { 0x79cc4519u32 } else { 0x7a879d8au32 };
        let ss1 = a
            .rotate_left(12)
            .wrapping_add(e)
            .wrapping_add(t.rotate_left(j as u32 & 31))
            .rotate_left(7);
        let ss2 = ss1 ^ a.rotate_left(12);
        let ff = if j < 16 {
            a ^ b ^ c
        } else {
            (a & b) | (a & c) | (b & c)
        };
        let gg = if j < 16 {
            e ^ f ^ g
        } else {
            (e & f) | (!e & g)
        };
        let tt1 = ff
            .wrapping_add(d)
            .wrapping_add(ss2)
            .wrapping_add(w[j] ^ w[j + 4]);
        let tt2 = gg.wrapping_add(h).wrapping_add(ss1).wrapping_add(w[j]);
        d = c;
        c = b.rotate_left(9);
        b = a;
        a = tt1;
        h = g;
        g = f.rotate_left(19);
        f = e;
        e = sm3_p0(tt2);
    }
    state[0] ^= a;
    state[1] ^= b;
    state[2] ^= c;
    state[3] ^= d;
    state[4] ^= e;
    state[5] ^= f;
    state[6] ^= g;
    state[7] ^= h;
}

/// 原生入口每次调用都要看的进程级开关。
struct Ncfg {
    fx: bool,
    x6d: bool,
    x0d: bool,
    hx: bool,
}

fn ncfg() -> &'static Ncfg {
    static V: std::sync::OnceLock<Ncfg> = std::sync::OnceLock::new();
    V.get_or_init(|| Ncfg {
        fx: std::env::var_os("MEDUSA_FX").is_some(),
        x6d: std::env::var_os("MEDUSA_X6").is_some(),
        x0d: std::env::var_os("MEDUSA_X0").is_some(),
        hx: std::env::var_os("MEDUSA_HX").is_some(),
    })
}

fn nofuse() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_NOFUSE").is_some())
}

fn nomem() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_NOMEM").is_some())
}

/// 关掉全部热类，让每条指令都走通用分派；用来核对快路径与通用路径等价。
fn nohot() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_NOHOT").is_some())
}

/// `TRAP`：越界/无指令现场；错误路径调用，仍缓存掉 getenv。
#[inline(always)]
fn trap() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("TRAP").is_some())
}

/// `MEDUSA_SLOW`：慢速探针强制打开。
#[inline(always)]
fn slow_probe() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_SLOW").is_some())
}

/// `MEDUSA_HIST`：pc 直方图。
#[inline(always)]
fn hist_on() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_HIST").is_some())
}

/// `MEDUSA_PICK`：打印装/存形状挑选。
#[inline(always)]
fn pick_dbg() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_PICK").is_some())
}

/// `SEMDBG2`：f64 转换桩现场。
#[inline(always)]
fn semdbg2() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("SEMDBG2").is_some())
}

/// `MEDUSA_NSOFF`：时钟纳秒平移量。
#[inline(always)]
fn nsoff() -> Option<i64> {
    static V: std::sync::OnceLock<Option<i64>> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        std::env::var("MEDUSA_NSOFF")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
    })
}

fn nosha() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_NOSHA").is_some())
}

/// `TRAPQ`：打印条件分支现场；在热 handler 里调用，必须缓存。
#[inline(always)]
fn trapq() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("TRAPQ").is_some())
}

pub const T_D: u32 = 1;
pub const T_4: u32 = 2;
pub const T_T: u32 = 4;
pub const T_2: u32 = 8;
pub const T_6: u32 = 16;
pub const T_H: u32 = 32;
pub const T_5: u32 = 64;

/// 逐指令探针开关位图：进程内只解析一次，执行循环里只做位测试。
#[inline(always)]
pub fn tbits() -> u32 {
    static V: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        [
            (T_D, "TRAPD"),
            (T_4, "TRAP4"),
            (T_T, "TRAPT"),
            (T_2, "TRAP2"),
            (T_6, "TRAP6"),
            (T_H, "TRAPH"),
            (T_5, "TRAP5"),
        ]
        .iter()
        .filter(|(_, n)| std::env::var_os(n).is_some())
        .fold(0, |m, (b, _)| m | b)
    })
}

/// 把数据吸收进 (state, 缓冲, 已用长度)；整块直接压缩，尾部留在缓冲。
fn sm3_absorb(state: &mut [u32; 8], buf: &mut [u8; 64], used: &mut usize, mut data: &[u8]) {
    if *used != 0 {
        let fill = 64 - *used;
        if data.len() < fill {
            buf[*used..*used + data.len()].copy_from_slice(data);
            *used += data.len();
            return;
        }
        buf[*used..].copy_from_slice(&data[..fill]);
        sm3_compress(state, buf);
        *used = 0;
        data = &data[fill..];
    }
    while data.len() >= 64 {
        let mut block = [0u8; 64];
        block.copy_from_slice(&data[..64]);
        sm3_compress(state, &block);
        data = &data[64..];
    }
    if !data.is_empty() {
        buf[..data.len()].copy_from_slice(data);
        *used = data.len();
    }
}

pub struct Ins {
    /// 装载/存储宽度（字节）；不适用时为 0。
    sz: u8,
    /// 装载是否符号扩展。
    sgn: bool,
    /// 条件码（`b.xx`），非条件分支为 14。
    cond: u8,
    /// 助记符以 `h` 结尾（半字存取）。
    hz: bool,
    /// 热类（0 = 走通用分派）：1 FLD、2 FST、3 ALU、4 寄存器搬、5 立即数搬、6 条件分支。
    h: u8,
    /// 稀有 pc 标志位（dispatch/reenter/PLT/输出长度/SM3 快路径），解码期算好。
    sp: u8,
    /// 第一操作数宽度（字节）；解码期算好，执行期直接用。
    w0: u8,
    /// ALU 形态：1 减法、2 置标志、4 有目的操作数；`ai`/`bi` 是源操作数下标。
    alu: u8,
    ai: u8,
    bi: u8,
    /// 超指令折叠出的寄存器/立即数/目标（只看对应 kind）。
    fa: u8,
    fb: u8,
    fc: u8,
    k: u16,
    fimm: i32,
    fpost: i32,
    /// 助记符前 8 字节，零填充；`==` 比较用整数替代字符串比较。
    n8: u64,
    /// 所属 pc，缓存表的标签就存在指令项里。
    pc: u64,
    /// `w0` 对应的位掩码，解码期算好。
    wm: u64,
    /// 分支目标（`b`/`b.xx`/`cbz`/`tbz` 与折叠超指令）。
    ftgt: u64,
    /// 冷侧：助记符与操作数。单独分配，好让上面这些热字段正好占满一个缓存行。
    slow: &'static Slow,
}

/// 解码项的冷侧数据：只有通用分派会用到。
pub struct Slow {
    m: &'static str,
    o: Box<[Opnd]>,
}

/// 冷侧占位：构造时先填它，解码后处理结束再换成真的。
fn empty_slow() -> &'static Slow {
    static D: std::sync::OnceLock<&'static Slow> = std::sync::OnceLock::new();
    D.get_or_init(|| {
        Box::leak(Box::new(Slow {
            m: "",
            o: Box::new([]),
        }))
    })
}
/// 指令解码缓存：按 pc 低位直接映射，标签存完整 pc，命中即用。
/// 线程本地、跨签名常驻；冲突时覆盖重解。
const OP_SLOTS: usize = 1 << 19;
pub static OP_MISS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 解码项堆场：按解码顺序连续分配，顺序流因此几乎线性访问。
/// 同一个槽被换主时旧项进回收表，下次解码原地覆写，占用只随在用项数增长。
const ARENA_CHUNK: usize = 1024;
static ARENA: std::sync::Mutex<Arena> = std::sync::Mutex::new(Arena {
    chunks: Vec::new(),
    free: Vec::new(),
});

struct Arena {
    chunks: Vec<Vec<Ins>>,
    free: Vec<usize>,
}

fn alloc_ins(ins: Ins) -> *const Ins {
    let mut g = match ARENA.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if let Some(p) = g.free.pop() {
        unsafe { (p as *mut Ins).write(ins) };
        return p as *const Ins;
    }
    let full = match g.chunks.last() {
        Some(c) => c.len() == c.capacity(),
        None => true,
    };
    if full {
        g.chunks.push(Vec::with_capacity(ARENA_CHUNK));
    }
    let c = g.chunks.last_mut().expect("just pushed");
    c.push(ins);
    c.last().expect("just pushed") as *const Ins
}

/// 当前在用的解码项数与回收表长度，供稳定性测试断言占用不随调用次数增长。
pub fn arena_stats() -> (usize, usize) {
    let g = match ARENA.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    let live = g.chunks.iter().map(|c| c.len()).sum::<usize>();
    (live, g.free.len())
}

/// 把被换下的解码项放回回收表。调用方必须先让槽指向别处，否则会悬垂。
fn recycle_ins(old: *const Ins) {
    if old.is_null() || old == dummy_ins() {
        return;
    }
    let mut g = match ARENA.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    g.free.push(old as usize);
}

/// 批量回收：一次加锁放一整批，供整表清空用。
fn recycle_many(old: &[usize]) {
    if old.is_empty() {
        return;
    }
    let mut g = match ARENA.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    g.free.extend_from_slice(old);
}

/// 空槽占位项：标签取不可能的 pc，命中判定只剩一次比较。
fn dummy_ins() -> *const Ins {
    static D: std::sync::OnceLock<&'static Ins> = std::sync::OnceLock::new();
    *D.get_or_init(|| {
        Box::leak(Box::new(Ins {
            n8: 0,
            pc: u64::MAX,
            ftgt: 0,
            k: 0,
            sz: 0,
            sgn: false,
            cond: 14,
            hz: false,
            h: 0,
            sp: 0,
            w0: 8,
            alu: 0,
            ai: 0,
            bi: 0,
            fa: 0,
            fb: 0,
            fc: 0,
            fimm: 0,
            fpost: 0,
            wm: 0,
            slow: empty_slow(),
        }))
    }) as *const Ins
}

/// 步数直方图计数：只在调试开关打开时进，单列出来免得占热循环的代码位。
#[cold]
fn bump_hist(pc: u64, k: u16, h: u8) {
    const R: std::sync::atomic::Ordering = std::sync::atomic::Ordering::Relaxed;
    unsafe {
        let slot = HIST.get_unchecked((pc >> 6) as usize & (HIST_N - 1));
        slot.fetch_add(1, R);
    }
    KHIST[(k & 255) as usize].fetch_add(1, R);
    HHIST[(h & 7) as usize].fetch_add(1, R);
}

pub fn clear_ops() {
    let d = dummy_ins();
    let mut old = Vec::new();
    OPS.with(|v| {
        for c in v {
            let p = c.replace(d);
            if p != d {
                old.push(p as usize);
            }
        }
    });
    recycle_many(&old);
}

/// 逐步跟踪缓冲：线程本地，并发签名时互不干扰。
#[derive(Default)]
struct Trace {
    pc: Vec<u64>,
    regs: Vec<[u64; 8]>,
    xs: Vec<[u64; 32]>,
}
thread_local! {
    static TRACE: std::cell::RefCell<Trace> = std::cell::RefCell::new(Trace::default());
}
pub fn trace_on() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        std::env::var_os("MEDUSA_TRACE").is_some() || std::env::var_os("SEMTRACE").is_some()
    })
}
pub fn trace_take() -> Vec<u64> {
    TRACE.with(|t| {
        let mut t = t.borrow_mut();
        t.regs.clear();
        std::mem::take(&mut t.pc)
    })
}
pub fn trace_r() -> Vec<[u64; 8]> {
    TRACE.with(|t| std::mem::take(&mut t.borrow_mut().regs))
}
pub fn trace_x() -> Vec<[u64; 32]> {
    TRACE.with(|t| std::mem::take(&mut t.borrow_mut().xs))
}
/// 把地址拉进一级缓存；只在有对应指令的架构上发出，其余架构为空实现。
/// 写成函数而非宏，非本机架构也会被逐目标类型检查到。
#[allow(dead_code)]
#[inline(always)]
fn prefetch(p: *const u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        std::arch::x86_64::_mm_prefetch(p.cast::<i8>(), std::arch::x86_64::_MM_HINT_T0)
    };
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "prfm pldl1keep, [{0}]",
            in(reg) p,
            options(nostack, readonly, preserves_flags)
        )
    };
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = p;
}

thread_local! {
    static OPS: Vec<std::cell::Cell<*const Ins>> =
        (0..OP_SLOTS)
            .map(|_| std::cell::Cell::new(dummy_ins()))
            .collect();
}

type JniReplay = HashMap<u64, std::collections::VecDeque<(u64, u64, Option<Vec<u8>>)>>;

static HIST: [std::sync::atomic::AtomicU64; HIST_N] =
    [const { std::sync::atomic::AtomicU64::new(0) }; HIST_N];
const HIST_N: usize = 1 << 23;

/// 按 kind / 热类统计的步数直方图，与 pc 直方图同开同关。
pub static KHIST: [std::sync::atomic::AtomicU64; 256] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 256];
pub static HHIST: [std::sync::atomic::AtomicU64; 8] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 8];

pub fn dump_hist() {
    if std::env::var_os("MEDUSA_HIST").is_none() {
        return;
    }
    const R: std::sync::atomic::Ordering = std::sync::atomic::Ordering::Relaxed;
    let kt: u64 = KHIST.iter().map(|c| c.load(R)).sum();
    let mut kv: Vec<(String, u64)> = KHIST
        .iter()
        .enumerate()
        .filter_map(|(i, c)| match c.load(R) {
            0 => None,
            n => Some((format!("k={i}"), n)),
        })
        .collect();
    for (i, c) in HHIST.iter().enumerate() {
        let n = c.load(R);
        if n != 0 {
            kv.push((format!("HOT{i}"), n));
        }
    }
    kv.sort_by_key(|x| std::cmp::Reverse(x.1));
    for (name, n) in kv.iter().take(12) {
        println!("KH {name} {n} {:.2}%", *n as f64 * 100.0 / kt.max(1) as f64);
    }
    let tot: u64 = HIST.iter().map(|c| c.load(R)).sum();
    let mut v: Vec<(u64, u64)> = HIST
        .iter()
        .enumerate()
        .filter_map(|(i, c)| match c.load(R) {
            0 => None,
            n => Some(((i << 6) as u64, n)),
        })
        .collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.1));
    let mut reg = [
        ("dispatch", 0x1017_47c8u64, 0x1017_5000u64, 0u64),
        ("decoder", 0x1005_5000, 0x1005_6000, 0),
        ("thunk", 0x1017_a000, 0x1017_a300, 0),
        ("h101b6", 0x101b_6000, 0x101b_7000, 0),
    ];
    for (a, c) in v.iter() {
        for r in reg.iter_mut() {
            if *a >= r.1 && *a < r.2 {
                r.3 += *c;
            }
        }
    }
    for r in reg.iter() {
        println!(
            "HREG {} {:.2}% {}",
            r.0,
            r.3 as f64 * 100.0 / tot.max(1) as f64,
            r.3
        );
    }
    for (a, c) in v.iter().take(30) {
        println!(
            "HPC {:#x} {:.2}% {}",
            a,
            *c as f64 * 100.0 / tot.max(1) as f64,
            c
        );
    }
}

/// 分派节点被执行的步数（A/B 用）。
static NDISP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// 分派树节点被执行的步数（A/B 用）。
pub static NODES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static FETCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// `MEDUSA_NODISP`：关掉低层 dispatch 快路径，用于 A/B 对拍。
fn nodisp() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_NODISP").is_some())
}

/// 原生代码返回 VM 的哨兵（放在 x30）。
pub const SENT: u64 = 0x1000_0000;
/// 正式程序跑完后 ret 的目标（加载器桩，不在库里）。
pub const END: u64 = 0x2_0000;

const PLT_LO: u64 = 0x1003_2c80;
const PLT_HI: u64 = 0x1003_3a90;
const GOT_BASE: u64 = 0x1034_53e8;
const HEAP_LO: u64 = 0x50c0_0000;
const HEAP_HI: u64 = 0x5800_0000;

#[derive(Clone, Debug)]
pub enum Opnd {
    X(u8, u8),
    Sp,
    Zr(u8),
    V(u8, u8, u8),
    Imm(i64),
    FImm(f64),
    Shift(Box<Opnd>, u8, u8),
    Mem {
        base: Box<Opnd>,
        off: Option<Box<Opnd>>,
        ext: u8,
        sh: u8,
        pre: bool,
        post: Option<i64>,
    },
    Cond(u8),
    Other(String),
}

pub struct Ctx<'a> {
    pub alloc: &'a [(String, u64, Option<u64>, Option<u64>)],
    pub aidx: &'a mut usize,
    pub bump: &'a mut u64,
    pub mapbump: &'a mut u64,
    pub jseq: &'a mut Vec<(u64, u64, u64)>,
    pub keys: &'a mut HashMap<u64, u64>,
    pub once: &'a mut std::collections::HashSet<u64>,
    pub urand: &'a mut Vec<u64>,
    pub rng: &'a mut Rand,
    pub fd: &'a mut u64,
    pub clocki: &'a mut usize,
    pub clock_sec: &'a u64,
    pub clock_ns: &'a [u64; 8],
    pub out_len: &'a mut usize,
    pub jni_next: &'a mut u64,
    pub jni_id: &'a mut u64,
    pub jni_str: &'a mut HashMap<u64, Vec<u8>>,
    pub jni_ptr: &'a mut HashMap<u64, u64>,
    pub jni_replay: &'a mut JniReplay,
}

pub struct Cpu<'a> {
    pub mem: &'a mut Mem,
    pub x: [u64; 32],
    pub v: [u128; 32],
    pub sp: u64,
    pub nzcv: [bool; 4],
    pub pc: u64,
    pub tpidr: u64,
    pub steps: u64,
    /// 指令解码缓存表基址（线程本地、定长，进程内不变）。
    ops: *const std::cell::Cell<*const Ins>,
    /// 逐步调试开关（进程级常量，构造时取一次）。
    semtr: bool,
    /// `TRAPQ`：条件分支/热点现场；每次 call4 取一次。
    tr: bool,
    nosha: bool,
    nomem: bool,
    nofuse: bool,
    pub ctx: Ctx<'a>,
}

/// 助记符前 8 字节装进 u64，零填充。
#[inline(always)]
const fn nm(s: &str) -> u64 {
    let b = s.as_bytes();
    let mut v = 0u64;
    let mut i = 0;
    while i < 8 {
        if i < b.len() {
            v |= (b[i] as u64) << (i * 8);
        }
        i += 1;
    }
    v
}

fn mask(w: u8) -> u64 {
    const MASKS: [u64; 9] = [
        0,
        0xff,
        0xffff,
        0xff_ffff,
        0xffff_ffff,
        0xff_ffff_ffff,
        0xffff_ffff_ffff,
        0xff_ffff_ffff_ffff,
        u64::MAX,
    ];
    MASKS[(w as usize).min(8)]
}

fn mask128(w: u8) -> u128 {
    if w >= 16 {
        u128::MAX
    } else {
        (1u128 << (w as u32 * 8)) - 1
    }
}

fn felem(o: &Opnd) -> u8 {
    match o {
        Opnd::V(_, e, _) => *e,
        _ => 8,
    }
}

fn cond_ok(c: u8, nzcv: [bool; 4]) -> bool {
    let [n, z, cv, v] = nzcv;
    match c {
        0 => z,
        1 => !z,
        2 => cv,
        3 => !cv,
        4 => n,
        5 => !n,
        6 => v,
        7 => !v,
        8 => cv && !z,
        9 => !cv || z,
        10 => n == v,
        11 => n != v,
        12 => !z && n == v,
        13 => z || n != v,
        _ => true,
    }
}

fn cond_of(s: &str) -> Option<u8> {
    Some(match s {
        "eq" => 0,
        "ne" => 1,
        "hs" => 2,
        "lo" => 3,
        "mi" => 4,
        "pl" => 5,
        "vs" => 6,
        "vc" => 7,
        "hi" => 8,
        "ls" => 9,
        "ge" => 10,
        "lt" => 11,
        "gt" => 12,
        "le" => 13,
        "al" => 14,
        "nv" => 15,
        _ => return None,
    })
}

fn parse_imm(s: &str) -> i64 {
    let s = s.trim();
    let (neg, t) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s),
    };
    // 逻辑立即数（如 0xffffffffffffffe0）超出 i64 正数范围，按位模式以 u64 解析。
    let v = if let Some(h) = t.strip_prefix("0x") {
        u64::from_str_radix(h, 16).unwrap_or(0)
    } else {
        t.parse::<u64>().unwrap_or(0)
    };
    if neg {
        (v as i64).wrapping_neg()
    } else {
        v as i64
    }
}

fn parse_reg(s: &str) -> Option<Opnd> {
    if s == "sp" || s == "wsp" {
        return Some(Opnd::Sp);
    }
    if s == "xzr" {
        return Some(Opnd::Zr(8));
    }
    if s == "wzr" {
        return Some(Opnd::Zr(4));
    }
    if let Some(r) = s.strip_prefix('x') {
        return Some(Opnd::X(r.parse().ok()?, 8));
    }
    if let Some(r) = s.strip_prefix('w') {
        return Some(Opnd::X(r.parse().ok()?, 4));
    }
    for (p, sz) in [('s', 4u8), ('d', 8), ('q', 16)] {
        if let Some(r) = s.strip_prefix(p) {
            return Some(Opnd::V(r.parse().ok()?, sz, 1));
        }
    }
    if let Some((r, arr)) = s.strip_prefix('v')?.split_once('.') {
        let (e, l) = match arr {
            "16b" => (1u8, 16u8),
            "8b" => (1, 8),
            "8h" => (2, 8),
            "4h" => (2, 4),
            "4s" => (4, 4),
            "2s" => (4, 2),
            "2d" => (8, 2),
            "1d" => (8, 1),
            "1q" => (16, 1),
            _ => return None,
        };
        return Some(Opnd::V(r.parse().ok()?, e, l));
    }
    None
}

fn parse_mem(raw: &str) -> Opnd {
    let pre = raw.ends_with('!');
    let inner = raw
        .trim_start_matches('[')
        .trim_end_matches('!')
        .trim_end_matches(']');
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    let base = Box::new(parse_reg(parts[0]).unwrap_or(Opnd::Sp));
    let mut off = None;
    let mut ext = 0u8;
    let mut sh = 0u8;
    if parts.len() >= 2 {
        if let Some(i) = parts[1].strip_prefix('#') {
            off = Some(Box::new(Opnd::Imm(parse_imm(i))));
        } else {
            off = Some(Box::new(parse_reg(parts[1]).unwrap_or(Opnd::Zr(8))));
            if parts.len() >= 3 {
                let e = parts[2];
                ext = if e.starts_with("uxtw") {
                    1
                } else if e.starts_with("lsl") {
                    2
                } else if e.starts_with("sxtw") {
                    3
                } else if e.starts_with("sxth") {
                    4
                } else if e.starts_with("sxtb") {
                    5
                } else {
                    0
                };
                if let Some(h) = e.split('#').nth(1) {
                    sh = h.parse().unwrap_or(0);
                }
            }
        }
    }
    Opnd::Mem {
        base,
        off,
        ext,
        sh,
        pre,
        post: None,
    }
}

const K_BCOND: u16 = 0xffff;
const K_NOP: u16 = 1;
const K_CLREX: u16 = 2;
const K_HINT: u16 = 3;
const K_B: u16 = 4;
const K_CBZ: u16 = 5;
const K_CBNZ: u16 = 6;
const K_TBZ: u16 = 7;
const K_TBNZ: u16 = 8;
const K_BR: u16 = 9;
const K_BL: u16 = 10;
const K_BLR: u16 = 11;
const K_RET: u16 = 12;
const K_MOV: u16 = 13;
const K_MOVZ: u16 = 14;
const K_MOVK: u16 = 15;
const K_MOVN: u16 = 16;
const K_MVN: u16 = 17;
const K_SXTB: u16 = 18;
const K_SXTH: u16 = 19;
const K_SXTW: u16 = 20;
const K_UXTB: u16 = 21;
const K_UXTH: u16 = 22;
const K_UXTW: u16 = 23;
const K_ADD: u16 = 24;
const K_ADDS: u16 = 25;
const K_SUB: u16 = 26;
const K_SUBS: u16 = 27;
const K_CMP: u16 = 28;
const K_CMN: u16 = 29;
const K_CCMP: u16 = 30;
const K_CCMN: u16 = 31;
const K_AND: u16 = 32;
const K_ANDS: u16 = 33;
const K_ORR: u16 = 34;
const K_EOR: u16 = 35;
const K_BIC: u16 = 36;
const K_BICS: u16 = 37;
const K_ORN: u16 = 38;
const K_EON: u16 = 39;
const K_TST: u16 = 40;
const K_LSL: u16 = 41;
const K_LSR: u16 = 42;
const K_ASR: u16 = 43;
const K_ROR: u16 = 44;
const K_NEG: u16 = 45;
const K_NEGS: u16 = 46;
const K_EXTR: u16 = 47;
const K_UBFX: u16 = 48;
const K_SBFX: u16 = 49;
const K_BFI: u16 = 50;
const K_BFXIL: u16 = 51;
const K_BFC: u16 = 52;
const K_SBFIZ: u16 = 53;
const K_UBFIZ: u16 = 54;
const K_REV: u16 = 55;
const K_REV16: u16 = 56;
const K_REV32: u16 = 57;
const K_RBIT: u16 = 58;
const K_CLZ: u16 = 59;
const K_SHL: u16 = 60;
const K_USHR: u16 = 61;
const K_SSHR: u16 = 62;
const K_SLI: u16 = 63;
const K_SRI: u16 = 64;
const K_SMULH: u16 = 65;
const K_UMULH: u16 = 66;
const K_MUL: u16 = 67;
const K_MADD: u16 = 68;
const K_MSUB: u16 = 69;
const K_UMULL: u16 = 70;
const K_SMULL: u16 = 71;
const K_UMADDL: u16 = 72;
const K_SMADDL: u16 = 73;
const K_UMSUBL: u16 = 74;
const K_SMSUBL: u16 = 75;
const K_UDIV: u16 = 76;
const K_SDIV: u16 = 77;
const K_CSEL: u16 = 78;
const K_CSINC: u16 = 79;
const K_CSINV: u16 = 80;
const K_CSNEG: u16 = 81;
const K_CSET: u16 = 82;
const K_CSETM: u16 = 83;
const K_CINC: u16 = 84;
const K_CNEG: u16 = 85;
const K_ADRP: u16 = 86;
const K_ADR: u16 = 87;
const K_MRS: u16 = 88;
const K_MSR: u16 = 89;
const K_DC: u16 = 90;
const K_STR: u16 = 91;
const K_STUR: u16 = 92;
const K_STRB: u16 = 93;
const K_STRH: u16 = 94;
const K_STURB: u16 = 95;
const K_STURH: u16 = 96;
const K_STLR: u16 = 97;
const K_STLRB: u16 = 98;
const K_STLLR: u16 = 99;
const K_STXR: u16 = 100;
const K_STLXR: u16 = 101;
const K_STXRH: u16 = 102;
const K_STLXRH: u16 = 103;
const K_LDP: u16 = 104;
const K_STP: u16 = 105;
const K_LDPSW: u16 = 106;
const K_FMADD: u16 = 107;
const K_FMSUB: u16 = 108;
const K_FNMADD: u16 = 109;
const K_FNMSUB: u16 = 110;
const K_FABS: u16 = 111;
const K_FNEG: u16 = 112;
const K_FABD: u16 = 113;
const K_FCSEL: u16 = 114;
const K_FDIV: u16 = 115;
const K_FMUL: u16 = 116;
const K_FADD: u16 = 117;
const K_FSUB: u16 = 118;
const K_FCMP: u16 = 119;
const K_FCMPE: u16 = 120;
const K_FMOV: u16 = 121;
const K_MOVI: u16 = 122;
const K_SSHLL: u16 = 123;
const K_USHLL: u16 = 124;
const K_SHLL: u16 = 125;
const K_SXTL: u16 = 126;
const K_UXTL: u16 = 127;
const K_SCVTF: u16 = 128;
const K_UCVTF: u16 = 129;
const K_FCVTZS: u16 = 130;
const K_FCVTZU: u16 = 131;
const K_SVC: u16 = 132;

/// 助记符 → 数值 opcode。分派只比整数，字符串只在解析时出现一次。
fn kind(m: &str) -> u16 {
    if m.starts_with("b.") {
        return K_BCOND;
    }
    match m {
        "nop" => K_NOP,
        "clrex" => K_CLREX,
        "hint" => K_HINT,
        "b" => K_B,
        "cbz" => K_CBZ,
        "cbnz" => K_CBNZ,
        "tbz" => K_TBZ,
        "tbnz" => K_TBNZ,
        "br" => K_BR,
        "bl" => K_BL,
        "blr" => K_BLR,
        "ret" => K_RET,
        "mov" => K_MOV,
        "movz" => K_MOVZ,
        "movk" => K_MOVK,
        "movn" => K_MOVN,
        "mvn" => K_MVN,
        "sxtb" => K_SXTB,
        "sxth" => K_SXTH,
        "sxtw" => K_SXTW,
        "uxtb" => K_UXTB,
        "uxth" => K_UXTH,
        "uxtw" => K_UXTW,
        "add" => K_ADD,
        "adds" => K_ADDS,
        "sub" => K_SUB,
        "subs" => K_SUBS,
        "cmp" => K_CMP,
        "cmn" => K_CMN,
        "ccmp" => K_CCMP,
        "ccmn" => K_CCMN,
        "and" => K_AND,
        "ands" => K_ANDS,
        "orr" => K_ORR,
        "eor" => K_EOR,
        "bic" => K_BIC,
        "bics" => K_BICS,
        "orn" => K_ORN,
        "eon" => K_EON,
        "tst" => K_TST,
        "lsl" => K_LSL,
        "lsr" => K_LSR,
        "asr" => K_ASR,
        "ror" => K_ROR,
        "neg" => K_NEG,
        "negs" => K_NEGS,
        "extr" => K_EXTR,
        "ubfx" => K_UBFX,
        "sbfx" => K_SBFX,
        "bfi" => K_BFI,
        "bfxil" => K_BFXIL,
        "bfc" => K_BFC,
        "sbfiz" => K_SBFIZ,
        "ubfiz" => K_UBFIZ,
        "rev" => K_REV,
        "rev16" => K_REV16,
        "rev32" => K_REV32,
        "rbit" => K_RBIT,
        "clz" => K_CLZ,
        "shl" => K_SHL,
        "ushr" => K_USHR,
        "sshr" => K_SSHR,
        "sli" => K_SLI,
        "sri" => K_SRI,
        "smulh" => K_SMULH,
        "umulh" => K_UMULH,
        "mul" => K_MUL,
        "madd" => K_MADD,
        "msub" => K_MSUB,
        "umull" => K_UMULL,
        "smull" => K_SMULL,
        "umaddl" => K_UMADDL,
        "smaddl" => K_SMADDL,
        "umsubl" => K_UMSUBL,
        "smsubl" => K_SMSUBL,
        "udiv" => K_UDIV,
        "sdiv" => K_SDIV,
        "csel" => K_CSEL,
        "csinc" => K_CSINC,
        "csinv" => K_CSINV,
        "csneg" => K_CSNEG,
        "cset" => K_CSET,
        "csetm" => K_CSETM,
        "cinc" => K_CINC,
        "cneg" => K_CNEG,
        "adrp" => K_ADRP,
        "adr" => K_ADR,
        "mrs" => K_MRS,
        "msr" => K_MSR,
        "dc" => K_DC,
        "str" => K_STR,
        "stur" => K_STUR,
        "strb" => K_STRB,
        "strh" => K_STRH,
        "sturb" => K_STURB,
        "sturh" => K_STURH,
        "stlr" => K_STLR,
        "stlrb" => K_STLRB,
        "stllr" => K_STLLR,
        "stxr" => K_STXR,
        "stlxr" => K_STLXR,
        "stxrh" => K_STXRH,
        "stlxrh" => K_STLXRH,
        "ldp" => K_LDP,
        "stp" => K_STP,
        "ldpsw" => K_LDPSW,
        "fmadd" => K_FMADD,
        "fmsub" => K_FMSUB,
        "fnmadd" => K_FNMADD,
        "fnmsub" => K_FNMSUB,
        "fabs" => K_FABS,
        "fneg" => K_FNEG,
        "fabd" => K_FABD,
        "fcsel" => K_FCSEL,
        "fdiv" => K_FDIV,
        "fmul" => K_FMUL,
        "fadd" => K_FADD,
        "fsub" => K_FSUB,
        "fcmp" => K_FCMP,
        "fcmpe" => K_FCMPE,
        "fmov" => K_FMOV,
        "movi" => K_MOVI,
        "sshll" => K_SSHLL,
        "ushll" => K_USHLL,
        "shll" => K_SHLL,
        "sxtl" => K_SXTL,
        "uxtl" => K_UXTL,
        "scvtf" => K_SCVTF,
        "ucvtf" => K_UCVTF,
        "fcvtzs" => K_FCVTZS,
        "fcvtzu" => K_FCVTZU,
        "svc" => K_SVC,
        "ldar" => K_LDAR,
        "ldarb" => K_LDARB,
        "ldarh" => K_LDARH,
        "ldaxr" => K_LDAXR,
        "ldaxrh" => K_LDAXRH,
        "ldr" => K_LDR,
        "ldrb" => K_LDRB,
        "ldrh" => K_LDRH,
        "ldrsb" => K_LDRSB,
        "ldrsh" => K_LDRSH,
        "ldrsw" => K_LDRSW,
        "ldur" => K_LDUR,
        "ldurb" => K_LDURB,
        "ldurh" => K_LDURH,
        "ldursb" => K_LDURSB,
        "ldursh" => K_LDURSH,
        "ldursw" => K_LDURSW,
        "ldxr" => K_LDXR,
        "ldxrb" => K_LDXRB,
        "ldxrh" => K_LDXRH,
        _ => 0,
    }
}

const K_LDAR: u16 = 133;
const K_LDARB: u16 = 134;
const K_LDARH: u16 = 135;
const K_LDAXR: u16 = 136;
const K_LDAXRH: u16 = 137;
const K_LDR: u16 = 138;
const K_LDRB: u16 = 139;
const K_LDRH: u16 = 140;
const K_LDRSB: u16 = 141;
const K_LDRSH: u16 = 142;
const K_LDRSW: u16 = 143;
const K_LDUR: u16 = 144;
const K_LDURB: u16 = 145;
const K_LDURH: u16 = 146;
const K_LDURSB: u16 = 147;
const K_LDURSH: u16 = 148;
const K_LDURSW: u16 = 149;
const K_LDXR: u16 = 150;
const K_LDXRB: u16 = 151;
const K_LDXRH: u16 = 152;

/// 装载/存储助记符的宽度与符号扩展，解析期算一次。
fn mem_class(m: &str) -> (u8, bool) {
    let sz = match m {
        "ldrb" | "ldrsb" | "ldurb" | "ldursb" | "ldarb" | "ldxrb" | "strb" | "sturb" | "stlrb" => 1,
        "ldrh" | "ldrsh" | "ldurh" | "ldursh" | "ldarh" | "ldxrh" | "ldaxrh" | "strh" | "sturh" => {
            2
        }
        "ldrsw" | "ldursw" => 4,
        _ => 0,
    };
    let sgn = m.starts_with("ldrs") || m.starts_with("ldurs") || m.contains("sw");
    (sz, sgn)
}

fn parse(pc: u64) -> (&'static str, Vec<Opnd>) {
    let Some(t) = native_ops::find(pc) else {
        return ("", Vec::new());
    };
    let (mn, rest) = t.split_once('\t').unwrap_or((t, ""));
    let mut raw_parts = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in rest.chars() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                raw_parts.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(ch);
    }
    if !cur.trim().is_empty() {
        raw_parts.push(cur);
    }

    let mut ops: Vec<Opnd> = Vec::new();
    for raw in raw_parts {
        let raw = raw.trim().to_string();
        if let Some(prev) = ops.last_mut() {
            let is_mem_plain = matches!(prev, Opnd::Mem { post: None, .. });
            if is_mem_plain {
                if let Some(i) = raw.strip_prefix('#') {
                    if let Opnd::Mem { post, .. } = prev {
                        *post = Some(parse_imm(i));
                        continue;
                    }
                }
            }
        }
        if raw.starts_with('{') {
            ops.push(Opnd::Other(raw));
            continue;
        }
        if let Some(r) = parse_reg(&raw) {
            ops.push(r);
            continue;
        }
        if let Some(c) = cond_of(&raw) {
            ops.push(Opnd::Cond(c));
            continue;
        }
        if raw.starts_with('[') {
            ops.push(parse_mem(&raw));
            continue;
        }
        if let Some(i) = raw.strip_prefix('#') {
            if i.contains('.') {
                ops.push(Opnd::FImm(i.parse().unwrap_or(0.0)));
            } else {
                ops.push(Opnd::Imm(parse_imm(i)));
            }
            continue;
        }
        if raw.contains('.') {
            ops.push(Opnd::Other(raw));
            continue;
        }
        // 移位后缀并到前一个操作数
        if let Some(sh) = parse_shift(&raw) {
            if let Some(prev) = ops.pop() {
                let (k, n) = sh;
                let prev = match prev {
                    Opnd::Shift(a, _, _) => a,
                    other => Box::new(other),
                };
                ops.push(Opnd::Shift(prev, k, n));
                continue;
            }
        }
        if let Some(r) = parse_reg(&raw) {
            ops.push(r);
            continue;
        }
        ops.push(Opnd::Other(raw));
    }
    (mn, ops)
}

fn parse_shift(s: &str) -> Option<(u8, u8)> {
    let (name, n) = match s.split_once('#') {
        Some((a, b)) => (
            a.trim(),
            b.trim_start_matches("0x")
                .trim()
                .parse::<u8>()
                .ok()
                .or_else(|| u8::from_str_radix(b.trim().trim_start_matches("0x"), 16).ok())?,
        ),
        None => (s.trim(), 0),
    };
    let k = match name {
        "lsl" => 0,
        "lsr" => 1,
        "asr" => 2,
        "ror" => 3,
        "uxtw" => 4,
        "sxtw" => 5,
        "sxth" => 6,
        "sxtb" => 7,
        _ => return None,
    };
    Some((k, n))
}

/// 操作数取用：条数在解码期已定，通用路径上省掉每处下标检查。
#[inline(always)]
fn oat(o: &[Opnd], i: usize) -> &Opnd {
    unsafe { o.get_unchecked(i) }
}

impl<'a> Cpu<'a> {
    #[inline(always)]
    fn op(&mut self) -> Result<*const Ins, Error> {
        let pc = self.pc;
        let tg = (pc >> 2) as u32;
        let i = tg as usize & (OP_SLOTS - 1);
        // 表指针在构造时取定：定长 Vec，生命周期内不重分配，省掉每步的 TLS 取值。
        let p = unsafe { (*self.ops.add(i)).get() };
        if unsafe { (*p).pc } == pc {
            return Ok(p);
        }
        // unidbg 的 svc 桩区：0x10000 起每 8 字节一对 `svc #0; ret`，桩号 = (pc-0x10000)/8。
        // 这些页不在库文件里，也不进 native_ops 表，按固定模式执行。
        OP_MISS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut b = if (0x10000..0x20000).contains(&pc) {
            let m = if pc & 4 == 0 { "svc" } else { "ret" };
            Box::new(Ins {
                slow: empty_slow(),
                k: kind(m),
                sz: 0,
                sgn: false,
                cond: 14,
                hz: false,
                h: 0,
                sp: 0,
                n8: nm(m),
                pc,
                w0: 8,
                alu: 0,
                ai: 0,
                bi: 0,
                fa: 0,
                fb: 0,
                fc: 0,
                fimm: 0,
                fpost: 0,
                ftgt: 0,
                wm: 0,
            })
        } else {
            let (m, o) = parse(pc);
            if super::probe::on(super::probe::OPDBG)
                && ((0x1004e780..0x1004e7b0).contains(&pc)
                    || (0x101dc720..0x101dc735).contains(&pc))
            {
                let w = self.mem.ld(pc, 4).unwrap_or(0);
                eprintln!("OPDBG {pc:#x} word={w:#x} m={m:?} o={o:?}");
            }
            if m.is_empty() {
                if trap() {
                    eprintln!(
                        "NOINSN pc={pc:#x} x0={:#x} x1={:#x} x2={:#x} x8={:#x} sp={:#x} ipc={:#x}",
                        self.x[0], self.x[1], self.x[2], self.x[8], self.sp, pc
                    );
                    eprintln!("{}", std::backtrace::Backtrace::force_capture());
                }
                return Err(Error::Native(format!("no insn at {pc:#x}")));
            }
            let k = kind(m);
            let n8 = nm(m);
            let alu = match k {
                K_CMP => 3,
                K_CMN => 2,
                K_SUB => 1 + 4,
                K_SUBS => 3 + 4,
                K_ADD => 4,
                K_ADDS => 6,
                _ => 0,
            };
            let (ai, bi) = if alu & 4 == 0 { (0, 1) } else { (1, 2) };
            let (sz, sgn) = mem_class(m);
            let cond = if m.len() > 2 {
                cond_of(&m[2..]).unwrap_or(14)
            } else {
                14
            };
            let hz = m.ends_with('h');
            let btgt = match k {
                K_B | K_BCOND => o.first(),
                K_CBZ | K_CBNZ => o.get(1).or(o.first()),
                K_TBZ | K_TBNZ => o.get(2).or(o.first()),
                _ => None,
            };
            let btgt = match btgt {
                Some(Opnd::Imm(v)) => *v as u64,
                Some(Opnd::Other(s)) => s.parse().unwrap_or(0),
                _ => 0,
            };
            let sp_val: u8 = {
                let mut v = 0u8;
                if pc == 0x1017_47c8 {
                    v |= 1;
                }
                if pc == 0x1004_e710 {
                    v |= 2;
                }
                if (PLT_LO..PLT_HI).contains(&pc) {
                    v |= 4;
                }
                if pc == 0x101a_63c4 {
                    v |= 8;
                }
                if pc == 0x1017_7bf8 {
                    v |= 16;
                }
                if pc == 0x1017_7ef4 {
                    v |= 32;
                }
                v
            };
            let mut b = Box::new(Ins {
                k,
                sz,
                sgn,
                cond,
                hz,
                h: 0,
                sp: sp_val,
                n8,
                pc,
                w0: 8,
                alu,
                ai,
                bi,
                fa: 0,
                fb: 0,
                fc: 0,
                fimm: 0,
                fpost: 0,
                ftgt: btgt,
                wm: 0,
                slow: empty_slow(),
            });
            b.w0 = match o.first() {
                Some(Opnd::X(_, s)) => *s,
                Some(Opnd::V(_, e, l)) => *e * *l,
                Some(Opnd::Zr(w)) => *w,
                _ => 8,
            };
            let nh = nohot();
            if !self.nomem && !nh {
                if let Some(f) = self.pick_mem(b.k, &o) {
                    b.k = f.0;
                    b.fa = f.1;
                    b.fb = f.2;
                    b.fc = f.3;
                    b.fimm = f.4;
                    b.fpost = f.5;
                }
            }
            match b.k {
                _ if nh => {}
                // 无条件跳转等价于恒真条件分支，走同一个快类。
                K_B => b.cond = 14,
                // `fc` 在这几类里当“形状已定型”标记：1 直接目标/单寄存器，2 寄存器调用。
                K_ADRP => {
                    if let (Some(Opnd::X(d, _)), Some(Opnd::Imm(v))) = (o.first(), o.get(1)) {
                        if *d < 31 {
                            b.fa = *d;
                            b.ftgt = *v as u64;
                            b.fc = 1;
                        }
                    }
                }
                K_CBZ | K_CBNZ | K_BR => {
                    if let Some(Opnd::X(r, _)) = o.first() {
                        if *r < 31 {
                            b.fa = *r;
                            b.fc = 1;
                        }
                    }
                }
                K_BL => {
                    if let Some(Opnd::Imm(v)) = o.first() {
                        b.ftgt = *v as u64;
                        b.fc = 1;
                    }
                }
                K_BLR => {
                    if let Some(Opnd::X(r, _)) = o.first() {
                        if *r < 31 {
                            b.fa = *r;
                            b.fc = 2;
                        }
                    }
                }
                K_RET => b.fa = 30,
                K_MOV => {
                    if let (Opnd::X(d, w), Opnd::X(s2, _)) = (&o[0], &o[1]) {
                        b.k = K_MOVR;
                        b.fa = *d;
                        b.fb = *s2;
                        b.w0 = *w;
                    } else if let (Opnd::X(d, w), Opnd::Imm(v)) = (&o[0], &o[1]) {
                        if i32::try_from(*v).is_ok() {
                            b.k = K_MOVL;
                            b.fa = *d;
                            b.fimm = *v as i32;
                            b.w0 = *w;
                        }
                    }
                }
                _ => {}
            }
            if !self.nofuse {
                if let Some(f) = self.fuse(pc) {
                    b.k = f.0;
                    b.fa = f.1;
                    b.fb = f.2;
                    b.fc = f.3;
                    b.fimm = f.4;
                    b.ftgt = f.5;
                    b.cond = f.6;
                }
            }
            let hval = match b.k {
                K_FLD => 1,
                K_FST => 2,
                K_ADD | K_ADDS | K_SUB | K_SUBS | K_CMP | K_CMN => {
                    let (ai, bi) = (b.ai as usize, b.bi as usize);
                    let alu = b.alu;
                    let d = match &o[0] {
                        Opnd::X(r, _) if alu & 4 != 0 => *r,
                        _ if alu & 4 == 0 => 0,
                        _ => 255,
                    };
                    match (&o[ai], &o[bi]) {
                        (Opnd::X(rn, _), Opnd::X(rm, _)) if *rn < 31 && *rm < 31 && d != 255 => {
                            b.fa = *rn;
                            b.fb = *rm;
                            b.fc = d;
                            3
                        }
                        (Opnd::X(rn, _), Opnd::Imm(v))
                            if *rn < 31 && d != 255 && i32::try_from(*v).is_ok() =>
                        {
                            b.fa = *rn;
                            b.fb = 0xfd;
                            b.fc = d;
                            b.fimm = *v as i32;
                            3
                        }
                        _ => 0,
                    }
                }
                K_B => 6,
                K_ADRP if b.fc == 1 => 7,
                K_CBZ | K_CBNZ if b.fc == 1 => 8,
                K_BL | K_BLR if b.fc != 0 => 9,
                K_RET => 10,
                K_BR if b.fc == 1 => 10,
                K_MOVR if b.fa < 31 => 4,
                K_MOVL if b.fa < 31 => 5,
                K_MOVW => 5,
                K_BCOND => 6,
                _ => 0,
            };
            b.h = if nohot() { 0 } else { hval };
            b.slow = Box::leak(Box::new(Slow {
                m,
                o: o.into_boxed_slice(),
            }));
            b
        };
        b.wm = mask(b.w0);
        let p = alloc_ins(*b);
        let slot = unsafe { self.ops.add(i) };
        let old = unsafe { (*slot).replace(p) };
        recycle_ins(old);
        Ok(p)
    }

    /// 简单装/存形状 `x[t], [x[b], #imm]{!}`：返回 (kind, 目标, 基址, 标志, 偏移, 后写)。
    /// 基址 0xfe 表示 sp；标志位 1 = 前写回、2 = 后写回。
    fn pick_mem(&self, k: u16, o: &[Opnd]) -> Option<(u16, u8, u8, u8, i32, i32)> {
        let (kind, is_ld) = match k {
            K_LDR | K_LDRB | K_LDRH | K_LDRSB | K_LDRSH | K_LDRSW | K_LDUR | K_LDURB | K_LDURH
            | K_LDURSB | K_LDURSH | K_LDURSW => (K_FLD, true),
            K_STR | K_STRB | K_STRH | K_STUR | K_STURB | K_STURH => (K_FST, false),
            _ => return None,
        };
        let d = match o.first() {
            Some(Opnd::X(r, _)) => *r,
            Some(Opnd::Zr(_)) if !is_ld => 31,
            _ => return None,
        };
        let Opnd::Mem {
            base,
            off,
            ext,
            sh,
            pre,
            post,
        } = o.get(1)?
        else {
            return None;
        };
        if *ext != 0 || *sh != 0 {
            return None;
        }
        let b = match &**base {
            Opnd::X(r, _) => *r,
            Opnd::Sp => 0xfe,
            _ => return None,
        };
        let foff = match off {
            None => 0,
            Some(v) => match &**v {
                Opnd::Imm(x) => *x as i32,
                _ => return None,
            },
        };
        if pick_dbg() {
            eprintln!("PICK {:#x} k={} ext={} sh={}", self.pc, k, ext, sh);
        }
        let flags = u8::from(*pre) | (u8::from(post.is_some()) << 1);
        Some((kind, d, b, flags, foff, post.unwrap_or(0) as i32))
    }

    /// 识别取值分派阶梯，把整段折成一条超指令；形状不符返回 None。
    /// 只接受库内四字可读、且与逐条执行完全等价的形状。
    fn fuse(&self, pc: u64) -> Option<(u16, u8, u8, u8, i32, u64, u8)> {
        if !(0x1000_0000..0x105f_0000).contains(&pc) {
            return None;
        }
        let word = |i: u64| self.mem.code_u32(pc + i * 4);
        let w0 = word(0);
        let w1 = word(1);
        // mov wA,#lo ; movk wA,#hi
        if w0 & 0xffe0_0000 == 0x5280_0000 && w1 & 0xffe0_0000 == 0x72a0_0000 {
            let a = (w0 & 31) as u8;
            if a != 31 && a == (w1 & 31) as u8 {
                let imm = ((w0 >> 5) & 0xffff) | (((w1 >> 5) & 0xffff) << 16);
                return Some((K_MOVW, a, 0, 0, imm as i32, 0, 0));
            }
        }
        // cmp wA,wB（SUBS wzr,wA,wB）
        if w0 & 0xffe0_fc1f != 0x6b00_001f {
            return None;
        }
        let a = ((w0 >> 5) & 31) as u8;
        let b = ((w0 >> 16) & 31) as u8;
        if a == 31 || b == 31 {
            return None;
        }
        if w1 & 0xff00_0010 == 0x5400_0000 {
            let cond = (w1 & 15) as u8;
            return Some((K_CMPBB, a, b, 0, 0, branch_target(pc + 4, w1), cond));
        }
        // cmp wA,wB ; mov wC,#lo ; movk wC,#hi ; b.cc
        if w1 & 0xffe0_0000 != 0x5280_0000 {
            return None;
        }
        let c = (w1 & 31) as u8;
        let w2 = word(2);
        let w3 = word(3);
        if c == 31 || c != (w2 & 31) as u8 || w2 & 0xffe0_0000 != 0x72a0_0000 {
            return None;
        }
        if w3 & 0xff00_0010 != 0x5400_0000 {
            return None;
        }
        let imm = ((w1 >> 5) & 0xffff) | (((w2 >> 5) & 0xffff) << 16);
        let cond = (w3 & 15) as u8;
        Some((
            K_CMPBW,
            a,
            b,
            c,
            imm as i32,
            branch_target(pc + 12, w3),
            cond,
        ))
    }

    #[inline(always)]
    fn base(&self, b: u8) -> u64 {
        if b == 0xfe {
            self.sp
        } else {
            unsafe { *self.x.get_unchecked(b as usize) }
        }
    }

    #[inline(always)]
    fn set_base(&mut self, b: u8, v: u64) {
        if b == 0xfe {
            self.sp = v;
        } else {
            unsafe { *self.x.get_unchecked_mut(b as usize) = v };
        }
    }

    #[inline(always)]
    fn rx(&self, o: &Opnd, w: u8) -> u64 {
        match o {
            Opnd::X(r, _) => unsafe { (*self.x.get_unchecked(*r as usize)) & mask(w) },
            Opnd::Sp => self.sp,
            Opnd::Zr(_) => 0,
            Opnd::Imm(v) => *v as u64 & mask(w),
            Opnd::V(r, _, _) => unsafe { (*self.v.get_unchecked(*r as usize) as u64) & mask(w) },
            _ => panic!("rx bad operand {o:?}"),
        }
    }

    fn rv(&self, o: &Opnd) -> u128 {
        match o {
            Opnd::V(r, e, l) => self.v[*r as usize] & mask128(*e * *l),
            _ => panic!("rv bad operand"),
        }
    }

    #[inline(always)]
    fn wx(&mut self, o: &Opnd, v: u64, w: u8) {
        match o {
            Opnd::X(r, _) => unsafe { *self.x.get_unchecked_mut(*r as usize) = v & mask(w) },
            Opnd::Sp => self.sp = v & mask(w),
            Opnd::Zr(_) => {}
            Opnd::V(r, _, _) => unsafe {
                *self.v.get_unchecked_mut(*r as usize) = (v & mask(w)) as u128
            },
            _ => panic!("wx bad operand {o:?}"),
        }
    }

    fn fval(&self, o: &Opnd, e: u8) -> f64 {
        if e == 4 {
            f32::from_bits(self.rv(o) as u32) as f64
        } else {
            f64::from_bits(self.rv(o) as u64)
        }
    }

    fn fput(&mut self, o: &Opnd, v: f64, e: u8) {
        if e == 4 {
            let b = (v as f32).to_bits() as u128;
            self.wv(o, b);
        } else {
            let b = v.to_bits() as u128;
            self.wv(o, b);
        }
    }

    fn wv(&mut self, o: &Opnd, v: u128) {
        if let Opnd::V(r, e, l) = o {
            self.v[*r as usize] = v & mask128(*e * *l);
        }
    }

    fn ldv(&self, a: u64, n: usize) -> Result<u128, Error> {
        let a = a & 0xffff_ffff;
        let p = match self.mem.span(a, n) {
            Some(p) => p as *const u8,
            None => {
                if trap() {
                    eprintln!("TRAP ld a={a:#x} n={n} curpc={:#x} lr={:#x} ipc={:#x} ins={:?} sp={:#x} x29={:#x}", super::CURPC.load(std::sync::atomic::Ordering::Relaxed), self.x[30], self.pc.wrapping_sub(4), native_ops::find(self.pc.wrapping_sub(4)), self.sp, self.x[29]);
                    eprintln!("{}", std::backtrace::Backtrace::force_capture());
                }
                return Err(Error::Unmapped(a));
            }
        };
        let mut v = 0u128;
        for i in 0..n {
            v |= (unsafe { *p.add(i) } as u128) << (i * 8);
        }
        Ok(v)
    }

    fn stv(&mut self, a: u64, n: usize, v: u128) -> Result<(), Error> {
        let a32 = a & 0xffff_ffff;
        let p = self.mem.span(a32, n).ok_or(Error::Unmapped(a32))? as *mut u8;
        self.mem.touch_page(a32 & !(super::PAGE - 1));
        for i in 0..n {
            unsafe { *p.add(i) = (v >> (i * 8)) as u8 };
        }
        Ok(())
    }

    fn flags_add(&mut self, a: u64, b: u64, cin: u64, w: u8) -> u64 {
        let m = mask(w);
        let (a, b) = (a & m, b & m);
        let wide = a as u128 + b as u128 + cin as u128;
        let r = wide as u64 & m;
        let bits = w as u32 * 8;
        self.nzcv = [
            (r >> (bits - 1)) & 1 != 0,
            r & m == 0,
            wide > m as u128,
            (((a ^ r) & (b ^ r)) >> (bits - 1)) & 1 != 0,
        ];
        r
    }

    fn flags_sub(&mut self, a: u64, b: u64, cin: u64, w: u8) -> u64 {
        let m = mask(w);
        let (a, b) = (a & m, b & m);
        let r = ((a as u128)
            .wrapping_sub(b as u128)
            .wrapping_sub((1 - cin) as u128) as u64)
            & m;
        let bits = w as u32 * 8;
        self.nzcv = [
            (r >> (bits - 1)) & 1 != 0,
            r & m == 0,
            (a as u128) >= b as u128 + (1 - cin) as u128,
            (((a ^ b) & (a ^ r)) >> (bits - 1)) & 1 != 0,
        ];
        r
    }

    fn flags_logic(&mut self, r: u64, w: u8, cy: bool) {
        let bits = w as u32 * 8;
        self.nzcv = [(r >> (bits - 1)) & 1 != 0, r & mask(w) == 0, cy, false];
    }

    #[inline(always)]
    fn imm_or_reg(&self, o: &Opnd, w: u8) -> u64 {
        match o {
            Opnd::Imm(v) => *v as u64 & mask(w),
            Opnd::Shift(inner, k, sh) => {
                let src_w = match **inner {
                    Opnd::X(_, s) => s,
                    _ => 8,
                };
                let mut v = self.rx(inner, src_w);
                let bits = src_w as u32 * 8;
                match k {
                    0 => v <<= sh,
                    1 => v = (v & mask(src_w)) >> sh,
                    2 => {
                        let sv = ((v << (64 - bits)) as i64) >> (64 - bits);
                        v = (sv >> sh) as u64;
                    }
                    3 => {
                        let b = bits as u64;
                        v = if b == 64 {
                            v.rotate_right(*sh as u32)
                        } else {
                            ((v >> sh) | (v << (b - *sh as u64))) & mask(src_w)
                        };
                    }
                    4 => v = (v & 0xffff_ffff) << sh,
                    5 => v = ((v as u32 as i32) as i64 as u64) << sh,
                    6 => v = ((v as u16 as i16) as i64 as u64) << sh,
                    7 => v = ((v as u8 as i8) as i64 as u64) << sh,
                    _ => {}
                }
                v
            }
            _ => self.rx(o, w),
        }
    }

    #[inline(always)]
    fn addr(&self, base: &Opnd, off: &Option<Box<Opnd>>, ext: u8, sh: u8) -> u64 {
        let mut a = self.rx(base, 8);
        if let Some(off) = off {
            match &**off {
                Opnd::Imm(v) => a = a.wrapping_add(*v as u64),
                other => {
                    let v0 = self.rx(other, 8);
                    let v = match ext {
                        1 => (v0 & 0xffff_ffff) << sh,
                        2 => v0 << sh,
                        3 => ((v0 as u32 as i32) as i64 as u64) << sh,
                        4 => ((v0 as u16 as i16) as i64 as u64) << sh,
                        5 => ((v0 as u8 as i8) as i64 as u64) << sh,
                        _ => v0,
                    };
                    a = a.wrapping_add(v);
                }
            }
        }
        a
    }

    #[inline(always)]
    fn post(&mut self, base: &Opnd, pre: bool, post: Option<i64>, addr: u64) {
        if pre {
            self.wx(base, addr, 8);
        }
        if let Some(p) = post {
            self.wx(base, addr.wrapping_add(p as u64), 8);
        }
    }

    fn shift_carry(&self, o: &Opnd) -> bool {
        match o {
            Opnd::Shift(_, _, sh) => *sh != 0,
            _ => true,
        }
    }

    #[inline(always)]
    fn ld(&self, a: u64, n: usize, sign: bool) -> Result<u64, Error> {
        let v = match self.mem.ld(a, n) {
            Ok(v) => v,
            Err(e) => {
                if trap() {
                    eprintln!("TRAP cpu.ld a={a:#x} n={n} ipc={:#x} ins={:?} sp={:#x} x29={:#x} x0={:#x} x1={:#x} x8={:#x} x9={:#x} x17={:#x}", self.pc.wrapping_sub(4), native_ops::find(self.pc.wrapping_sub(4)), self.sp, self.x[29], self.x[0], self.x[1], self.x[8], self.x[9], self.x[17]);
                }
                return Err(e);
            }
        };
        if self.semtr && a == 0x102d6234 {
            eprintln!("LD a={a:#x} n={n} v={v:#x} x0={:#x}", self.x[0]);
        }
        Ok(if sign && n < 8 && v >> (n * 8 - 1) != 0 {
            v | (!0u64 << (n * 8))
        } else {
            v
        })
    }
}

impl<'a> Cpu<'a> {
    fn cstr(&self, a: u64) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        while out.len() < (1 << 20) {
            let b = self.mem.ld(a + out.len() as u64, 1)?;
            if b == 0 {
                break;
            }
            out.push(b as u8);
        }
        Ok(out)
    }

    /// 库内 SM3 上下文更新（0x10177bf8）的 Rust 快路径。
    fn sha_update(&mut self) -> Result<(), Error> {
        let ctx = self.x[0];
        let data = self.x[1];
        let len = self.x[2] as u32;
        if len == 0 {
            self.x[0] = 0xffff_ffff;
            self.pc = self.x[30];
            return Ok(());
        }
        let mut lo = self.mem.ld(ctx, 4)? as u32;
        let mut hi = self.mem.ld(ctx + 4, 4)? as u32;
        let mut used = (lo & 0x3f) as usize;
        let (n, carry) = lo.overflowing_add(len);
        lo = n;
        if carry {
            hi = hi.wrapping_add(1);
        }
        self.mem.st(ctx, 4, lo as u64)?;
        self.mem.st(ctx + 4, 4, hi as u64)?;
        let mut state = [0u32; 8];
        for (i, v) in state.iter_mut().enumerate() {
            *v = self.mem.ld(ctx + 8 + i as u64 * 4, 4)? as u32;
        }
        let mut buf = [0u8; 64];
        if used != 0 {
            let p = self
                .mem
                .span(ctx + 0x28, used)
                .ok_or(Error::Unmapped(ctx + 0x28))? as *const u8;
            unsafe { std::ptr::copy_nonoverlapping(p, buf.as_mut_ptr(), used) };
        }
        let p = self
            .mem
            .span(data, len as usize)
            .ok_or(Error::Unmapped(data))? as *const u8;
        let input = unsafe { std::slice::from_raw_parts(p, len as usize) };
        sm3_absorb(&mut state, &mut buf, &mut used, input);
        for (i, v) in state.iter().enumerate() {
            self.mem.st(ctx + 8 + i as u64 * 4, 4, *v as u64)?;
        }
        let q = self
            .mem
            .span(ctx + 0x28, 64)
            .ok_or(Error::Unmapped(ctx + 0x28))? as *mut u8;
        unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), q, 64) };
        self.x[0] = 0;
        self.pc = self.x[30];
        Ok(())
    }

    /// 库内 SM3 收尾（0x10177ef4）的 Rust 快路径：补位、写 64 位大端长度、输出摘要。
    fn sha_final(&mut self) -> Result<(), Error> {
        let ctx = self.x[0];
        let out = self.x[1];
        let lo = self.mem.ld(ctx, 4)? as u32;
        let hi = self.mem.ld(ctx + 4, 4)? as u32;
        let mut used = (lo & 0x3f) as usize;
        let mut state = [0u32; 8];
        for (i, v) in state.iter_mut().enumerate() {
            *v = self.mem.ld(ctx + 8 + i as u64 * 4, 4)? as u32;
        }
        let mut buf = [0u8; 64];
        if used != 0 {
            let p = self
                .mem
                .span(ctx + 0x28, used)
                .ok_or(Error::Unmapped(ctx + 0x28))? as *const u8;
            unsafe { std::ptr::copy_nonoverlapping(p, buf.as_mut_ptr(), used) };
        }
        let pad = (if used < 0x38 { 0x38 } else { 0x78 }) - used;
        let bits = (((hi as u64) << 32) | lo as u64).wrapping_shl(3);
        let mut tail = [0u8; 72];
        tail[0] = 0x80;
        tail[pad..pad + 8].copy_from_slice(&bits.to_be_bytes());
        sm3_absorb(&mut state, &mut buf, &mut used, &tail[..pad + 8]);
        let mut digest = [0u8; 32];
        for (i, v) in state.iter().enumerate() {
            digest[i * 4..i * 4 + 4].copy_from_slice(&v.to_be_bytes());
        }
        self.mem.write(out, &digest)?;
        self.x[0] = 0;
        self.pc = self.x[30];
        Ok(())
    }

    /// 高频形状：直接按解码期算好的字段执行，不碰操作数枚举与助记符。
    #[inline(always)]
    fn exec_hot(&mut self, ins: &Ins) -> Result<(), Error> {
        let next = self.pc;
        match ins.h {
            1 => {
                let a = self.base(ins.fb).wrapping_add(ins.fimm as i64 as u64);
                let n = if ins.sz != 0 { ins.sz } else { ins.w0 } as usize;
                let v = if n == 8 && !ins.sgn {
                    self.mem.ld_q64(a)?
                } else {
                    self.ld(a, n, ins.sgn)?
                };
                self.x[ins.fa as usize] = v & ins.wm;
                if ins.fc != 0 {
                    if ins.fc & 1 != 0 {
                        self.set_base(ins.fb, a);
                    }
                    if ins.fc & 2 != 0 {
                        self.set_base(ins.fb, a.wrapping_add(ins.fpost as i64 as u64));
                    }
                }
            }
            2 => {
                let a = self.base(ins.fb).wrapping_add(ins.fimm as i64 as u64);
                let v = if ins.fa == 31 {
                    0
                } else {
                    unsafe { *self.x.get_unchecked(ins.fa as usize) }
                };
                let n = if ins.sz != 0 { ins.sz } else { ins.w0 } as usize;
                if n == 8 {
                    self.mem.st_q64(a, v)?;
                } else {
                    self.mem.st(a, n, v & ins.wm)?;
                }
                if ins.fc != 0 {
                    if ins.fc & 1 != 0 {
                        self.set_base(ins.fb, a);
                    }
                    if ins.fc & 2 != 0 {
                        self.set_base(ins.fb, a.wrapping_add(ins.fpost as i64 as u64));
                    }
                }
            }
            3 => {
                let w = ins.w0;
                let a = unsafe { *self.x.get_unchecked(ins.fa as usize) & ins.wm };
                let b = if ins.fb == 0xfd {
                    ins.fimm as i64 as u64 & ins.wm
                } else {
                    unsafe { *self.x.get_unchecked(ins.fb as usize) & mask(w) }
                };
                let alu = ins.alu;
                let r = if alu & 1 != 0 {
                    if alu & 2 != 0 {
                        self.flags_sub(a, b, 1, w)
                    } else {
                        a.wrapping_sub(b) & ins.wm
                    }
                } else if alu & 2 != 0 {
                    self.flags_add(a, b, 0, w)
                } else {
                    a.wrapping_add(b) & ins.wm
                };
                if alu & 4 != 0 {
                    unsafe { *self.x.get_unchecked_mut(ins.fc as usize) = r };
                }
            }
            4 => unsafe {
                let v = *self.x.get_unchecked(ins.fb as usize) & ins.wm;
                *self.x.get_unchecked_mut(ins.fa as usize) = v;
            },
            5 => {
                let v = match ins.k {
                    K_MOVW => ins.fimm as u32 as u64,
                    _ => ins.fimm as i64 as u64 & ins.wm,
                };
                unsafe { *self.x.get_unchecked_mut(ins.fa as usize) = v };
                if ins.k == K_MOVW {
                    self.pc = next.wrapping_add(4);
                }
            }
            7 => unsafe { *self.x.get_unchecked_mut(ins.fa as usize) = ins.ftgt },
            8 => {
                let v = unsafe { *self.x.get_unchecked(ins.fa as usize) } & ins.wm;
                if self.pc == 0x1005_3f94 && self.tr {
                    eprintln!(
                        "HOT8 pc={:#x} next={next:#x} v={v:#x} k={} fa={} h={}",
                        ins.pc, ins.k, ins.fa, ins.h
                    );
                }
                self.pc = if (v == 0) == (ins.k == K_CBZ) {
                    ins.ftgt
                } else {
                    next
                };
            }
            9 => {
                let t = if ins.k == K_BL {
                    ins.ftgt
                } else {
                    unsafe { *self.x.get_unchecked(ins.fa as usize) }
                };
                self.x[30] = next;
                self.pc = t;
            }
            10 => self.pc = unsafe { *self.x.get_unchecked(ins.fa as usize) },
            _ => {
                self.pc = if cond_ok(ins.cond, self.nzcv) {
                    ins.ftgt
                } else {
                    next
                }
            }
        }
        if ins.pc == 0x1005_3f90 && self.tr {
            eprintln!("ENDHOT pc={:#x}", self.pc);
        }
        Ok(())
    }

    /// 慢路径用的整步执行：取指后只补 SM3 快路径判定（其余稀有 pc 调用方已处理）。
    #[inline(always)]
    fn exec(&mut self) -> Result<(), Error> {
        let ip = self.op()?;
        let sp = unsafe { (*ip).sp };
        if sp & 16 != 0 && !self.nosha {
            return self.sha_update();
        }
        if sp & 32 != 0 && !self.nosha {
            return self.sha_final();
        }
        self.exec_ins(ip)
    }

    /// 取指后执行：稀有 pc 的判定已在调用方按 `Ins::sp` 处理。
    #[inline(always)]
    fn exec_ins(&mut self, ip: *const Ins) -> Result<(), Error> {
        let ins = unsafe { &*ip };
        let next = self.pc.wrapping_add(4);
        self.pc = next;
        if ins.h != 0 {
            return self.exec_hot(ins);
        }
        // 下面三项只有通用路径要用，放在热类判断之后，免得热步白读白存。
        let slow = ins.slow;
        let m = slow.m;
        let o = &slow.o;
        let w0 = ins.w0;
        match ins.k {
            K_MOVW => {
                self.x[ins.fa as usize] = ins.fimm as u32 as u64;
                self.pc = next.wrapping_add(4);
            }
            K_CMPBB => {
                let a = self.x[ins.fa as usize] as u32;
                let b = self.x[ins.fb as usize] as u32;
                self.flags_sub(a as u64, b as u64, 1, 4);
                self.pc = if cond_ok(ins.cond, self.nzcv) {
                    ins.ftgt
                } else {
                    next.wrapping_add(4)
                };
            }
            K_CMPBW => {
                let a = self.x[ins.fa as usize] as u32;
                let b = self.x[ins.fb as usize] as u32;
                self.flags_sub(a as u64, b as u64, 1, 4);
                self.x[ins.fc as usize] = ins.fimm as u32 as u64;
                self.pc = if cond_ok(ins.cond, self.nzcv) {
                    ins.ftgt
                } else {
                    next.wrapping_add(12)
                };
            }
            K_FLD => {
                let a = self.base(ins.fb).wrapping_add(ins.fimm as i64 as u64);
                let n = if ins.sz != 0 { ins.sz } else { ins.w0 } as usize;
                let v = if n == 8 && !ins.sgn {
                    self.mem.ld_q64(a)?
                } else {
                    self.ld(a, n, ins.sgn)?
                };
                self.x[ins.fa as usize] = v & mask(ins.w0);
                if ins.fc & 1 != 0 {
                    self.set_base(ins.fb, a);
                }
                if ins.fc & 2 != 0 {
                    self.set_base(ins.fb, a.wrapping_add(ins.fpost as i64 as u64));
                }
            }
            K_FST => {
                let a = self.base(ins.fb).wrapping_add(ins.fimm as i64 as u64);
                let v = if ins.fa == 31 {
                    0
                } else {
                    self.x[ins.fa as usize]
                };
                let n = if ins.sz != 0 { ins.sz } else { ins.w0 } as usize;
                if n == 8 {
                    self.mem.st_q64(a, v)?;
                } else {
                    self.mem.st(a, n, v & mask(ins.w0))?;
                }
                if ins.fc & 1 != 0 {
                    self.set_base(ins.fb, a);
                }
                if ins.fc & 2 != 0 {
                    self.set_base(ins.fb, a.wrapping_add(ins.fpost as i64 as u64));
                }
            }
            K_NOP | K_CLREX | K_HINT => {}
            K_B => self.pc = ins.ftgt,
            K_BCOND => {
                let c = ins.cond;
                self.pc = if cond_ok(c, self.nzcv) {
                    ins.ftgt
                } else {
                    next
                };
            }
            K_CBZ | K_CBNZ => {
                let v = self.rx(oat(o, 0), w0);
                if self.pc == 0x1005_3f94 && self.tr {
                    eprintln!("SLOWCB v={v:#x} k={}", ins.k);
                }
                let z = v == 0;
                if tbits() & T_5 != 0 && self.pc == 0x1005_3f90 {
                    eprintln!(
                        "CB v={v:#x} z={z} n8={:#x} cbz={:#x} cbnz={:#x}",
                        ins.n8,
                        nm("cbz"),
                        nm("cbnz")
                    );
                }
                self.pc = if (z && ins.n8 == nm("cbz")) || (!z && ins.n8 == nm("cbnz")) {
                    ins.ftgt
                } else {
                    next
                };
            }
            K_TBZ | K_TBNZ => {
                let v = self.rx(oat(o, 0), w0);
                let bit = (v >> imm_of(oat(o, 1))) & 1;
                self.pc = if (bit == 0 && ins.n8 == nm("tbz")) || (bit == 1 && ins.n8 == nm("tbnz"))
                {
                    ins.ftgt
                } else {
                    next
                };
            }
            K_BR => self.pc = self.rx(oat(o, 0), 8),
            K_BL | K_BLR => {
                self.x[30] = next;
                self.pc = if ins.n8 == nm("blr") {
                    self.rx(oat(o, 0), 8)
                } else {
                    self.target(oat(o, 0))
                };
            }
            K_RET => self.pc = self.x[30],
            K_MOV | K_MOVZ => {
                if let Opnd::V(..) = oat(o, 0) {
                    if let Opnd::V(..) = oat(o, 1) {
                        let v = self.rv(oat(o, 1));
                        self.wv(oat(o, 0), v);
                    } else {
                        let v = self.rx(oat(o, 1), 8);
                        self.wx(oat(o, 0), v, w0);
                    }
                    return Ok(());
                }
                if let (Opnd::Other(_), Opnd::Other(_)) = (oat(o, 0), oat(o, 1)) {
                    // 向量寄存器按标签搬
                    if let (Opnd::Other(d), Opnd::Other(s)) = (oat(o, 0), oat(o, 1)) {
                        if let (Some(dr), Some(sr)) = (vec_reg(d), vec_reg(s)) {
                            self.v[dr] = self.v[sr];
                        }
                    }
                    return Ok(());
                }
                if let Opnd::Other(d) = oat(o, 0) {
                    if let Some(spec) = vec_lane(d) {
                        let (r, sz, lane) = spec;
                        let mask_bits = mask(sz) as u128;
                        let sh = lane as u32 * sz as u32 * 8;
                        let src = self.rx(oat(o, 1), 8) as u128;
                        self.v[r] = (self.v[r] & !(mask_bits << sh)) | ((src & mask_bits) << sh);
                        return Ok(());
                    }
                }
                let v = self.imm_or_reg(oat(o, 1), w0);
                self.wx(oat(o, 0), v, w0);
            }
            K_MOVK => {
                let (imm, sh) = match oat(o, 1) {
                    Opnd::Shift(inner, _, sh) => (imm_of(inner), *sh),
                    Opnd::Imm(v) => (*v, 0),
                    _ => (0, 0),
                };
                let v = self.rx(oat(o, 0), w0);
                let m = !(0xffffu64 << sh);
                self.wx(oat(o, 0), (v & m) | ((imm as u64) << sh), w0);
            }
            K_MOVN => self.wx(oat(o, 0), !(imm_of(oat(o, 1)) as u64), w0),
            K_MVN => {
                let v = !self.rx(oat(o, 1), w0);
                self.wx(oat(o, 0), v, w0);
            }
            K_SXTB | K_SXTH | K_SXTW | K_UXTB | K_UXTH | K_UXTW => {
                let src_w = if let Opnd::X(_, s) = *oat(o, 1) { s } else { 8 };
                let v = self.rx(oat(o, 1), src_w);
                let bits = match m {
                    "sxtb" | "uxtb" => 8,
                    "sxth" | "uxth" => 16,
                    _ => 32,
                };
                let mut v = v & ((1u64 << bits) - 1);
                if (ins.n8 as u8) == b's' && v >> (bits - 1) != 0 {
                    v |= !0u64 << bits;
                }
                self.wx(oat(o, 0), v, w0);
            }
            K_ADD | K_ADDS | K_SUB | K_SUBS | K_CMP | K_CMN => {
                let alu = ins.alu;
                let rn = unsafe { o.get_unchecked(ins.ai as usize) };
                let opnd = unsafe { o.get_unchecked(ins.bi as usize) };
                let w = if let Opnd::X(_, s) = rn { *s } else { 8 };
                let a = self.rx(rn, w);
                let b = self.imm_or_reg(opnd, w);
                let r = if alu & 1 != 0 {
                    if alu & 2 != 0 {
                        self.flags_sub(a, b, 1, w)
                    } else {
                        a.wrapping_sub(b) & mask(w)
                    }
                } else if alu & 2 != 0 {
                    self.flags_add(a, b, 0, w)
                } else {
                    a.wrapping_add(b) & mask(w)
                };
                if alu & 4 != 0 {
                    let d = unsafe { o.get_unchecked(0) };
                    self.wx(d, r, w);
                }
            }
            K_CCMP | K_CCMN => {
                let c = if let Opnd::Cond(c) = *oat(o, 3) {
                    c
                } else {
                    14
                };
                if cond_ok(c, self.nzcv) {
                    let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                    let a = self.rx(oat(o, 0), w);
                    let b = self.imm_or_reg(oat(o, 1), w);
                    if ins.n8 == nm("ccmp") {
                        self.flags_sub(a, b, 1, w);
                    } else {
                        self.flags_add(a, b, 0, w);
                    }
                } else {
                    let v = imm_of(oat(o, 2)) as u64;
                    self.nzcv = [
                        (v >> 3) & 1 != 0,
                        (v >> 2) & 1 != 0,
                        (v >> 1) & 1 != 0,
                        v & 1 != 0,
                    ];
                }
            }
            K_AND | K_ANDS | K_ORR | K_EOR | K_BIC | K_BICS | K_ORN | K_EON => {
                let a = self.rx(oat(o, 1), w0);
                let b = self.imm_or_reg(oat(o, 2), w0);
                let r = match m {
                    "and" | "ands" => a & b,
                    "orr" => a | b,
                    "eor" => a ^ b,
                    "bic" | "bics" => a & !b,
                    "orn" => a | !b,
                    _ => a ^ !b,
                };
                self.wx(oat(o, 0), r, w0);
                if ins.n8 == nm("ands") || ins.n8 == nm("bics") {
                    self.flags_logic(r, w0, self.shift_carry(oat(o, 2)));
                }
            }
            K_TST => {
                let r = self.rx(oat(o, 0), w0) & self.imm_or_reg(oat(o, 1), w0);
                self.flags_logic(r, w0, self.shift_carry(oat(o, 1)));
            }
            K_LSL | K_LSR | K_ASR | K_ROR => {
                let a = self.rx(oat(o, 1), w0);
                let mut b = match oat(o, 2) {
                    Opnd::Imm(v) => *v as u64,
                    other => self.rx(other, w0),
                };
                b &= w0 as u64 * 8 - 1;
                let r = match m {
                    "lsl" => a << b,
                    "lsr" => (a & mask(w0)) >> b,
                    "asr" => {
                        ((((a << (64 - w0 as u32 * 8)) as i64) >> (64 - w0 as u32 * 8)) >> b) as u64
                    }
                    _ => {
                        if w0 == 8 {
                            a.rotate_right(b as u32)
                        } else {
                            let bits = w0 as u64 * 8;
                            ((a >> b) | (a << (bits - b))) & mask(w0)
                        }
                    }
                };
                self.wx(oat(o, 0), r, w0);
            }
            K_NEG | K_NEGS => {
                let v = self.imm_or_reg(oat(o, 1), w0);
                let r = if ins.n8 == nm("negs") {
                    self.flags_sub(0, v, 1, w0)
                } else {
                    0u64.wrapping_sub(v) & mask(w0)
                };
                self.wx(oat(o, 0), r, w0);
            }
            K_EXTR => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let bits = w as u32 * 8;
                let a = self.rx(oat(o, 1), w) as u128;
                let b = self.rx(oat(o, 2), w) as u128;
                let sh = imm_of(oat(o, 3)) as u32;
                let v = ((a << bits) | b) >> sh;
                self.wx(oat(o, 0), v as u64, w);
            }
            K_UBFX | K_SBFX => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let bits = w as u32 * 8;
                let lsb = imm_of(oat(o, 2)) as u32;
                let width = imm_of(oat(o, 3)) as u32;
                let bm = if width >= 64 {
                    u64::MAX
                } else {
                    (1u64 << width) - 1
                };
                let mut v = (self.rx(oat(o, 1), w) >> lsb) & bm;
                if ins.n8 == nm("sbfx") && width < bits && (v >> (width - 1)) & 1 != 0 {
                    v |= !0u64 << width;
                }
                self.wx(oat(o, 0), v & mask(w), w);
            }
            K_BFI | K_BFXIL => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let lsb = imm_of(oat(o, 2)) as u32;
                let width = imm_of(oat(o, 3)) as u32;
                let bm = if width >= 64 {
                    u64::MAX
                } else {
                    (1u64 << width) - 1
                };
                let src = self.rx(oat(o, 1), w);
                let dst = self.rx(oat(o, 0), w);
                // BFI 插到 lsb 位置；BFXIL 取出的字段插到低位。
                let (msk, val) = if ins.n8 == nm("bfi") {
                    (bm << lsb, (src & bm) << lsb)
                } else {
                    (bm, (src >> lsb) & bm)
                };
                self.wx(oat(o, 0), (dst & !msk) | val, w);
            }
            K_BFC => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let lsb = imm_of(oat(o, 1)) as u32;
                let width = imm_of(oat(o, 2)) as u32;
                let m = (if width >= 64 {
                    u64::MAX
                } else {
                    (1u64 << width) - 1
                }) << lsb;
                let dst = self.rx(oat(o, 0), w);
                self.wx(oat(o, 0), dst & !m, w);
            }
            K_SBFIZ | K_UBFIZ => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let lsb = imm_of(oat(o, 2)) as u32;
                let width = imm_of(oat(o, 3)) as u32;
                let bm = if width >= 64 {
                    u64::MAX
                } else {
                    (1u64 << width) - 1
                };
                let bits = w as u32 * 8;
                let v = (self.rx(oat(o, 1), w) & bm) << lsb;
                let out = if ins.n8 == nm("sbfiz")
                    && lsb + width < bits
                    && (v >> (lsb + width - 1)) & 1 != 0
                {
                    v | (!0u64 << (lsb + width))
                } else {
                    v
                };
                self.wx(oat(o, 0), out, w);
            }
            K_REV | K_REV16 | K_REV32 | K_RBIT => {
                let v = self.rx(oat(o, 1), w0);
                let r = match m {
                    "rbit" => v.reverse_bits() >> (64 - w0 as u32 * 8),
                    "rev" => match w0 {
                        8 => v.swap_bytes(),
                        4 => (v as u32).swap_bytes() as u64,
                        _ => (v as u16).swap_bytes() as u64,
                    },
                    "rev16" => match w0 {
                        8 => v.swap_bytes().rotate_left(16),
                        _ => (v as u32).rotate_left(8) as u64 & 0xffff_ffff,
                    },
                    _ => match w0 {
                        8 => v.swap_bytes().rotate_right(32),
                        _ => (v as u32).swap_bytes() as u64,
                    },
                };
                self.wx(oat(o, 0), r, w0);
            }
            K_CLZ => {
                let v = self.rx(oat(o, 1), w0);
                let r = v.leading_zeros() as u64 - (64 - w0 as u32 * 8) as u64;
                self.wx(oat(o, 0), r, w0);
            }
            // 向量立即移位（d 寄存器宽度的简单形式）。
            K_SHL | K_USHR | K_SSHR | K_SLI | K_SRI => {
                let bits = w0 as u32 * 8;
                let raw = self.rx(oat(o, 1), w0);
                let sh = imm_of(oat(o, 2)) as u32 % bits;
                let dst = self.rx(oat(o, 0), w0);
                let r = match m {
                    "shl" => raw.wrapping_shl(sh) & mask(w0),
                    "ushr" => (raw & mask(w0)) >> sh,
                    "sshr" => ((((raw << (64 - bits)) as i64) >> (64 - bits)) >> sh) as u64,
                    "sli" => {
                        if sh == 0 {
                            dst
                        } else {
                            (dst & ((1u64 << (bits - sh)) - 1)) | ((raw << sh) & mask(w0))
                        }
                    }
                    _ => {
                        if sh == 0 {
                            dst
                        } else {
                            (dst & (u64::MAX << sh)) | ((raw & mask(w0)) >> sh)
                        }
                    }
                };
                self.wx(oat(o, 0), r & mask(w0), w0);
            }
            K_SMULH | K_UMULH => {
                let a = self.rx(oat(o, 1), 8);
                let b = self.rx(oat(o, 2), 8);
                let r = if ins.n8 == nm("smulh") {
                    (((a as i64 as i128) * (b as i64 as i128)) >> 64) as u64
                } else {
                    (((a as u128) * (b as u128)) >> 64) as u64
                };
                self.wx(oat(o, 0), r, 8);
            }
            K_MUL => {
                let v = self.rx(oat(o, 1), w0).wrapping_mul(self.rx(oat(o, 2), w0));
                self.wx(oat(o, 0), v, w0);
            }
            K_MADD | K_MSUB => {
                let a = self.rx(oat(o, 1), w0).wrapping_mul(self.rx(oat(o, 2), w0));
                let b = self.rx(oat(o, 3), w0);
                let r = if ins.n8 == nm("madd") {
                    a.wrapping_add(b)
                } else {
                    b.wrapping_sub(a)
                };
                self.wx(oat(o, 0), r, w0);
            }
            K_UMULL => {
                let v = self.rx(oat(o, 1), 4) * self.rx(oat(o, 2), 4);
                self.wx(oat(o, 0), v, 8);
            }
            K_SMULL => {
                let a = self.rx(oat(o, 1), 4) as u32 as i32 as i64;
                let b = self.rx(oat(o, 2), 4) as u32 as i32 as i64;
                self.wx(oat(o, 0), (a * b) as u64, 8);
            }
            K_UMADDL | K_SMADDL | K_UMSUBL | K_SMSUBL => {
                let (a, b) = if (ins.n8 as u8) == b's' {
                    (
                        self.rx(oat(o, 1), 4) as u32 as i32 as i64,
                        self.rx(oat(o, 2), 4) as u32 as i32 as i64,
                    )
                } else {
                    (self.rx(oat(o, 1), 4) as i64, self.rx(oat(o, 2), 4) as i64)
                };
                let c = self.rx(oat(o, 3), 8) as i64;
                let r = if m.contains("add") {
                    c.wrapping_add(a.wrapping_mul(b))
                } else {
                    c.wrapping_sub(a.wrapping_mul(b))
                };
                self.wx(oat(o, 0), r as u64, 8);
            }
            K_UDIV | K_SDIV => {
                let w = if let Opnd::X(_, s) = *oat(o, 0) { s } else { 8 };
                let a = self.rx(oat(o, 1), w);
                let b = self.rx(oat(o, 2), w);
                let r = if b == 0 {
                    0
                } else if ins.n8 == nm("udiv") {
                    a / b
                } else {
                    let bits = w as u32 * 8;
                    let sa = ((a << (64 - bits)) as i64) >> (64 - bits);
                    let sb = ((b << (64 - bits)) as i64) >> (64 - bits);
                    (sa / sb) as u64
                };
                self.wx(oat(o, 0), r, w);
            }
            K_CSEL | K_CSINC | K_CSINV | K_CSNEG => {
                let c = if let Opnd::Cond(c) = *oat(o, 3) {
                    c
                } else {
                    14
                };
                let t = cond_ok(c, self.nzcv);
                let a = self.rx(oat(o, 1), w0);
                let b = self.rx(oat(o, 2), w0);
                let r = match m {
                    "csel" => {
                        if t {
                            a
                        } else {
                            b
                        }
                    }
                    "csinc" => {
                        if t {
                            a
                        } else {
                            b.wrapping_add(1)
                        }
                    }
                    "csinv" => {
                        if t {
                            a
                        } else {
                            !b
                        }
                    }
                    _ => {
                        if t {
                            a
                        } else {
                            0u64.wrapping_sub(b)
                        }
                    }
                };
                self.wx(oat(o, 0), r, w0);
            }
            K_CSET => {
                let c = if let Opnd::Cond(c) = *oat(o, 1) {
                    c
                } else {
                    14
                };
                self.wx(oat(o, 0), cond_ok(c, self.nzcv) as u64, w0);
            }
            K_CSETM => {
                let c = if let Opnd::Cond(c) = *oat(o, 1) {
                    c
                } else {
                    14
                };
                self.wx(
                    oat(o, 0),
                    if cond_ok(c, self.nzcv) { mask(w0) } else { 0 },
                    w0,
                );
            }
            K_CINC => {
                let c = if let Opnd::Cond(c) = *oat(o, 2) {
                    c
                } else {
                    14
                };
                let v = self
                    .rx(oat(o, 1), w0)
                    .wrapping_add(cond_ok(c, self.nzcv) as u64);
                self.wx(oat(o, 0), v, w0);
            }
            K_CNEG => {
                let c = if let Opnd::Cond(c) = *oat(o, 2) {
                    c
                } else {
                    14
                };
                let v = self.rx(oat(o, 1), w0);
                let v = if cond_ok(c, self.nzcv) {
                    0u64.wrapping_sub(v)
                } else {
                    v
                };
                self.wx(oat(o, 0), v, w0);
            }
            K_ADRP => {
                if super::probe::on(super::probe::DBG) {
                    eprintln!("ADRP ops={:?}", o);
                }
                self.wx(oat(o, 0), imm_of(oat(o, 1)) as u64, 8)
            }
            K_ADR => {
                let w = self.mem.ld(self.pc.wrapping_sub(4), 4)? as u32;
                let mut imm = (((w >> 5) & 0x7ffff) << 2 | ((w >> 29) & 3)) as i64;
                if imm & 0x100000 != 0 {
                    imm -= 0x200000;
                }
                let v = self.pc.wrapping_sub(4).wrapping_add(imm as u64);
                self.wx(oat(o, 0), v, 8);
            }
            K_MRS => {
                let sysr = other_of(oat(o, 1));
                let v = match sysr.as_str() {
                    "tpidr_el0" => self.tpidr,
                    "nzcv" => {
                        (self.nzcv[0] as u64) << 31
                            | (self.nzcv[1] as u64) << 30
                            | (self.nzcv[2] as u64) << 29
                            | (self.nzcv[3] as u64) << 28
                    }
                    "dczid_el0" => 4,
                    "ctr_el0" => 0x8444_c004,
                    "fpcr" => 0,
                    "cntvct_el0" => 0x1000,
                    other => return Err(Error::Native(format!("mrs {other}"))),
                };
                self.wx(oat(o, 0), v, 8);
            }
            K_MSR => {
                let sysr = other_of(oat(o, 0));
                let v = self.rx(oat(o, 1), 8);
                match sysr.as_str() {
                    "tpidr_el0" => self.tpidr = v,
                    "nzcv" => {
                        self.nzcv = [
                            (v >> 31) & 1 != 0,
                            (v >> 30) & 1 != 0,
                            (v >> 29) & 1 != 0,
                            (v >> 28) & 1 != 0,
                        ]
                    }
                    other => return Err(Error::Native(format!("msr {other}"))),
                }
            }
            K_DC => {
                if let Opnd::Mem { base, .. } = oat(o, 1) {
                    let a = self.rx(base, 8) & !63;
                    self.mem.write(a, &[0u8; 64])?;
                }
            }
            K_LDR | K_LDUR | K_LDRB | K_LDRH | K_LDRSB | K_LDRSH | K_LDRSW | K_LDURB | K_LDURH
            | K_LDURSB | K_LDURSH | K_LDURSW | K_LDAR | K_LDXR | K_LDARB | K_LDARH | K_LDAXR
            | K_LDXRB | K_LDAXRH | K_LDXRH => {
                let size = if ins.sz != 0 && !matches!(*oat(o, 0), Opnd::V(..)) {
                    ins.sz
                } else {
                    w0
                };
                let sign = ins.sgn;
                let addr = match oat(o, 1) {
                    Opnd::Imm(v) => *v as u64,
                    Opnd::Mem {
                        base, off, ext, sh, ..
                    } => self.addr(base, off, *ext, *sh),
                    other => self.rx(other, 8),
                };
                let v = self.ld(addr, size as usize, sign)?;
                if let Opnd::V(_, e, l) = oat(o, 0) {
                    let x = self.ldv(addr, (*e * *l) as usize)?;
                    self.wv(oat(o, 0), x);
                } else {
                    self.wx(oat(o, 0), v, w0);
                }
                if let Opnd::Mem {
                    base, pre, post, ..
                } = oat(o, 1)
                {
                    self.post(base, *pre, *post, addr);
                }
            }
            K_STR | K_STUR | K_STRB | K_STRH | K_STURB | K_STURH | K_STLR | K_STLRB | K_STLLR => {
                let vec = matches!(*oat(o, 0), Opnd::V(..));
                let size = if ins.sz != 0 {
                    ins.sz
                } else if let Opnd::Zr(w) = *oat(o, 0) {
                    w
                } else {
                    w0
                };
                let v = self.rx(oat(o, 0), size);
                if let Opnd::Mem {
                    base,
                    off,
                    ext,
                    sh,
                    pre,
                    post,
                } = oat(o, 1)
                {
                    let addr = self.addr(base, off, *ext, *sh);
                    if vec {
                        let x = self.rv(oat(o, 0));
                        self.stv(addr, size as usize, x)?;
                    } else {
                        self.mem.st(addr, size as usize, v)?;
                    }
                    self.post(base, *pre, *post, addr);
                }
            }
            K_STXR | K_STLXR | K_STXRH | K_STLXRH => {
                let size = if ins.hz { 2 } else { w0.max(1) };
                let v = self.rx(oat(o, 1), size);
                if let Opnd::Mem {
                    base, off, ext, sh, ..
                } = oat(o, 2)
                {
                    let addr = self.addr(base, off, *ext, *sh);
                    self.mem.st(addr, size as usize, v)?;
                }
                self.wx(oat(o, 0), 0, 4);
            }
            K_LDP | K_STP => {
                let vec = matches!(*oat(o, 0), Opnd::V(..));
                let sz = if vec {
                    16
                } else if let Opnd::Zr(w) = *oat(o, 0) {
                    w
                } else {
                    w0
                };
                if let Opnd::Mem {
                    base,
                    off,
                    ext,
                    sh,
                    pre,
                    post,
                } = oat(o, 2)
                {
                    let addr = self.addr(base, off, *ext, *sh);
                    if ins.n8 == nm("ldp") {
                        if vec {
                            let bw = match oat(o, 0) {
                                Opnd::V(_, e, l) => (*e as usize * *l as usize).min(16),
                                _ => 16,
                            };
                            let a = self.ldv(addr, bw)?;
                            let b = self.ldv(addr + bw as u64, bw)?;
                            self.wv(oat(o, 0), a);
                            self.wv(oat(o, 1), b);
                        } else {
                            let a = self.ld(addr, sz as usize, false)?;
                            let b = self.ld(addr + sz as u64, sz as usize, false)?;
                            self.wx(oat(o, 0), a, sz);
                            self.wx(oat(o, 1), b, sz);
                        }
                    } else if vec {
                        let bw = match oat(o, 0) {
                            Opnd::V(_, e, l) => (*e as usize * *l as usize).min(16),
                            _ => 16,
                        };
                        let a = self.rv(oat(o, 0));
                        let b = self.rv(oat(o, 1));
                        self.stv(addr, bw, a)?;
                        self.stv(addr + bw as u64, bw, b)?;
                    } else {
                        let a = self.rx(oat(o, 0), sz);
                        let b = self.rx(oat(o, 1), sz);
                        self.mem.st(addr, sz as usize, a)?;
                        self.mem.st(addr + sz as u64, sz as usize, b)?;
                    }
                    self.post(base, *pre, *post, addr);
                }
            }
            K_LDPSW => {
                if let Opnd::Mem {
                    base,
                    off,
                    ext,
                    sh,
                    pre,
                    post,
                } = oat(o, 2)
                {
                    let addr = self.addr(base, off, *ext, *sh);
                    let a = self.ld(addr, 4, true)?;
                    let b = self.ld(addr + 4, 4, true)?;
                    self.wx(oat(o, 0), a, 8);
                    self.wx(oat(o, 1), b, 8);
                    self.post(base, *pre, *post, addr);
                }
            }
            K_FMADD | K_FMSUB | K_FNMADD | K_FNMSUB => {
                let e = felem(oat(o, 0));
                let a = self.fval(oat(o, 1), e);
                let b = self.fval(oat(o, 2), e);
                let c = self.fval(oat(o, 3), e);
                let r = match m {
                    "fmadd" => a * b + c,
                    "fmsub" => c - a * b,
                    "fnmadd" => -a * b - c,
                    _ => -a * b + c,
                };
                self.fput(oat(o, 0), r, e);
            }
            K_FABS | K_FNEG => {
                let e = felem(oat(o, 0));
                let sign: u128 = if e == 4 {
                    0x8000_0000
                } else {
                    0x8000_0000_0000_0000
                };
                let v = self.rv(oat(o, 1));
                let r = if m == "fneg" { v ^ sign } else { v & !sign };
                self.wv(oat(o, 0), r);
            }
            K_FABD => {
                let e = felem(oat(o, 0));
                let a = self.fval(oat(o, 1), e);
                let b = self.fval(oat(o, 2), e);
                self.fput(oat(o, 0), (a - b).abs(), e);
            }
            K_FCSEL => {
                let c = if let Opnd::Cond(c) = *oat(o, 3) {
                    c
                } else {
                    14
                };
                let v = if cond_ok(c, self.nzcv) {
                    self.rv(oat(o, 1))
                } else {
                    self.rv(oat(o, 2))
                };
                self.wv(oat(o, 0), v);
            }
            K_FDIV | K_FMUL | K_FADD | K_FSUB => {
                let e = felem(oat(o, 0));
                let a = self.fval(oat(o, 1), e);
                let b = self.fval(oat(o, 2), e);
                let r = match m {
                    "fdiv" => a / b,
                    "fmul" => a * b,
                    "fadd" => a + b,
                    _ => a - b,
                };
                self.fput(oat(o, 0), r, e);
            }
            K_FCMP | K_FCMPE => {
                let e = felem(oat(o, 0));
                let a = self.fval(oat(o, 0), e);
                let b = match oat(o, 1) {
                    Opnd::Imm(v) => *v as f64,
                    Opnd::FImm(v) => {
                        if e == 4 {
                            *v as f32 as f64
                        } else {
                            *v
                        }
                    }
                    other => self.fval(other, e),
                };
                if super::probe::on(super::probe::FDBG) {
                    eprintln!(
                        "FCMP a={:?}({:016x}) b={:?}({:016x}) lt={}",
                        a,
                        a.to_bits(),
                        b,
                        b.to_bits(),
                        a < b
                    );
                }
                self.nzcv = if a.is_nan() || b.is_nan() {
                    [false, false, true, true]
                } else if a == b {
                    [false, true, true, false]
                } else if a < b {
                    [true, false, false, false]
                } else {
                    [false, false, true, false]
                };
            }
            K_FMOV => {
                if let Opnd::V(_, e, _) = oat(o, 0) {
                    let e = *e;
                    let v = match oat(o, 1) {
                        Opnd::FImm(f) => {
                            if e == 4 {
                                (*f as f32).to_bits() as u128
                            } else {
                                f.to_bits() as u128
                            }
                        }
                        Opnd::Imm(i) => *i as u64 as u128,
                        other => self.rx(other, e) as u128,
                    };
                    self.wv(oat(o, 0), v);
                } else {
                    let v = self.rv(oat(o, 1)) as u64;
                    self.wx(oat(o, 0), v, w0);
                }
            }
            K_MOVI => {
                let (e, l) = match oat(o, 0) {
                    Opnd::V(_, e, l) => (*e, *l),
                    _ => (8u8, 1u8),
                };
                let imm = imm_of(oat(o, 1));
                let unit = match e {
                    1 => imm & 0xff,
                    2 => (imm & 0xff) * 0x0101,
                    4 => (imm & 0xff) * 0x0101_0101,
                    _ => imm,
                };
                let mut val: u128 = 0;
                for k in 0..l as usize {
                    val |= (unit as u128) << (8 * e as usize * k);
                }
                self.wv(oat(o, 0), val);
            }
            // 长移位：源窄元素符号/零扩展成双倍宽再左移。
            K_SSHLL | K_USHLL | K_SHLL | K_SXTL | K_UXTL => {
                let (de, dl) = match oat(o, 0) {
                    Opnd::V(_, e, l) => (*e, *l),
                    _ => (8u8, 2u8),
                };
                let (se, sl) = match oat(o, 1) {
                    Opnd::V(_, e, l) => (*e, *l),
                    _ => (4u8, 2u8),
                };
                let sh = if o.len() > 2 {
                    imm_of(oat(o, 2)) as u32
                } else {
                    0
                };
                let src = self.rv(oat(o, 1));
                let smask = mask128(se);
                let bits = se as u32 * 8;
                let mut out: u128 = 0;
                for k in 0..dl.min(sl) as usize {
                    let e = (src >> (8 * se as usize * k)) & smask;
                    let v = if m == "ushll" || m == "uxtl" {
                        e
                    } else if (e >> (bits - 1)) & 1 != 0 {
                        e | !smask
                    } else {
                        e
                    };
                    out |= ((v << sh) & mask128(de)) << (8 * de as usize * k);
                }
                self.wv(oat(o, 0), out);
            }
            K_SCVTF | K_UCVTF => {
                let w = match oat(o, 1) {
                    Opnd::X(_, s) => *s,
                    Opnd::V(_, e, _) => *e,
                    _ => 8,
                };
                let v = self.rx(oat(o, 1), w);
                let f = if m == "ucvtf" {
                    v as f64
                } else {
                    let bits = w as u32 * 8;
                    (((v << (64 - bits)) as i64) >> (64 - bits)) as f64
                };
                let de = match oat(o, 0) {
                    Opnd::V(_, e, _) => *e,
                    _ => 8,
                };
                let out = if de == 4 {
                    (f as f32).to_bits() as u128
                } else {
                    f.to_bits() as u128
                };
                self.wv(oat(o, 0), out);
            }
            K_FCVTZS | K_FCVTZU => {
                let e = felem(oat(o, 1));
                let f = self.fval(oat(o, 1), e);
                let v = if m == "fcvtzs" {
                    f as i64 as u64
                } else {
                    f as u64
                };
                self.wx(oat(o, 0), v, w0);
            }
            K_SVC => {
                let x8 = self.x[8];
                let stub = (0x10000..0x20000).contains(&x8) && (x8 - 0x10000).is_multiple_of(8);
                let nr = if stub {
                    (x8 - 0x10000) / 8
                } else {
                    x8 & 0xffff
                };
                if self
                    .ctx
                    .jseq
                    .first()
                    .map(|e| e.0 == self.x[30] && e.1 == nr)
                    .unwrap_or(false)
                {
                    self.x[0] = self.ctx.jseq.remove(0).2;
                } else if stub {
                    self.jni(nr)?;
                } else {
                    self.syscall(nr)?;
                }
            }
            other => return Err(Error::Native(format!("k={other} m={m:?} {o:?}"))),
        }
        Ok(())
    }

    fn target(&self, o: &Opnd) -> u64 {
        match o {
            Opnd::Imm(v) => *v as u64,
            Opnd::Other(s) => s.parse().unwrap_or(0),
            other => self.rx(other, 8),
        }
    }

    /// unidbg 的 JNI 桩：编号即 JNIEnv 函数下标 + 8。
    fn jni(&mut self, n: u64) -> Result<(), Error> {
        let idx = n.wrapping_sub(8);
        let mut hit = None;
        let mut pay = None;
        if let Some(q) = self.ctx.jni_replay.get_mut(&self.x[30]) {
            if let Some(p) = q.iter().position(|e| e.0 == idx) {
                let e = q.remove(p).unwrap();
                hit = Some(e.1);
                pay = e.2;
            }
        }
        if let (Some(h), Some(b)) = (hit, pay) {
            self.ctx.jni_str.insert(h, b);
        }
        if super::probe::on(super::probe::JNI_DBG) {
            eprintln!(
                "JNI lr={:#x} n={:#x} idx={} x1={:#x} x2={:#x} hit={}",
                self.x[30],
                n,
                idx,
                self.x[1],
                self.x[2],
                hit.is_some()
            );
        }
        // 复放只提供返回值，out 参数与字符串登记等副作用仍要执行。
        match idx {
            239 => {
                self.mem.write(self.x[1], &(0x4000_0750u64).to_le_bytes())?;
            }
            163 | 167 => {
                let bytes = if idx == 167 {
                    self.cstr(self.x[1])?
                } else {
                    let n16 = (self.x[2] as usize).min(4096);
                    let mut v = Vec::with_capacity(n16);
                    for i in 0..n16 {
                        let u = self.mem.ld(self.x[1] + 2 * i as u64, 2)? as u16;
                        if u < 0x80 {
                            v.push(u as u8);
                        } else if u < 0x800 {
                            v.push(0xc0 | (u >> 6) as u8);
                            v.push(0x80 | (u & 0x3f) as u8);
                        } else {
                            v.push(0xe0 | (u >> 12) as u8);
                            v.push(0x80 | ((u >> 6) & 0x3f) as u8);
                            v.push(0x80 | (u & 0x3f) as u8);
                        }
                    }
                    v
                };
                let h = match hit {
                    Some(h) => h,
                    None => {
                        let h = *self.ctx.jni_next;
                        *self.ctx.jni_next += 1;
                        h
                    }
                };
                self.ctx.jni_str.insert(h, bytes);
                self.x[0] = h;
                return Ok(());
            }
            // Java 侧 getBytes：返回值是字符串的 UTF-8 字节数组。
            35 => {
                if let (Some(h), Some(b)) = (hit, self.ctx.jni_str.get(&self.x[1]).cloned()) {
                    self.ctx.jni_str.insert(h, b);
                }
            }
            // 数组长度与区间拷贝按登记内容给出，复放值只作校验。
            171 => {
                if let Some(b) = self.ctx.jni_str.get(&self.x[1]) {
                    self.x[0] = b.len() as u64;
                }
                return Ok(());
            }
            200 => {
                let (a, st, n, buf) =
                    (self.x[1], self.x[2] as usize, self.x[3] as usize, self.x[4]);
                if let Some(b) = self.ctx.jni_str.get(&a) {
                    if st + n <= b.len() {
                        let d = b[st..st + n].to_vec();
                        self.mem.write(buf, &d)?;
                    }
                }
                self.x[0] = 0;
                return Ok(());
            }
            _ => {}
        }
        if let Some(h) = hit {
            self.x[0] = h;
            return Ok(());
        }
        self.x[0] = match n.checked_sub(8) {
            Some(6) | Some(15) | Some(21) | Some(25) | Some(27) | Some(28..=31) | Some(34..=36)
            | Some(64..=66) | Some(95) | Some(114..=116) | Some(145) | Some(172) | Some(173)
            | Some(226) | Some(229) | Some(237) => {
                let h = *self.ctx.jni_next;
                *self.ctx.jni_next += 1;
                h
            }
            Some(168) | Some(164) => self
                .ctx
                .jni_str
                .get(&self.x[1])
                .map_or(0, |s| s.len() as u64),
            Some(33) | Some(94) | Some(113) | Some(144) => {
                let h = *self.ctx.jni_id;
                *self.ctx.jni_id += 1;
                h
            }
            Some(169) => {
                if let Some(a) = self.ctx.jni_ptr.get(&self.x[1]) {
                    *a
                } else {
                    let mut s = self
                        .ctx
                        .jni_str
                        .get(&self.x[1])
                        .cloned()
                        .unwrap_or_default();
                    s.push(0);
                    let p = self.ctx.nx(s.len() as u64);
                    self.mem.write(p, &s)?;
                    self.ctx.jni_ptr.insert(self.x[1], p);
                    p
                }
            }
            Some(232) => 1,
            Some(239) => 0,
            _ => 0,
        };
        Ok(())
    }

    fn syscall(&mut self, n: u64) -> Result<(), Error> {
        let dk = super::dbgclk();
        if dk {
            eprintln!("SVC {n} x0={:#x} pc={:#x}", self.x[0], self.x[8]);
        }
        let lr = self.x[30];
        if n == 208 {
            const V: &[u8] = b"32.2.6";
            let k = (self.x[3] as usize).min(V.len());
            if k > 0 {
                self.mem.write(self.x[4], &V[..k])?;
            }
            if self.ctx.jseq.first() == Some(&(lr, n, 0))
                || self
                    .ctx
                    .jseq
                    .first()
                    .map(|e| e.0 == lr && e.1 == n)
                    .unwrap_or(false)
            {
                self.ctx.jseq.remove(0);
            }
            self.x[0] = 0;
            return Ok(());
        }
        if self
            .ctx
            .jseq
            .first()
            .map(|e| e.0 == lr && e.1 == n)
            .unwrap_or(false)
        {
            let e = self.ctx.jseq.remove(0);
            self.x[0] = e.2;
            return Ok(());
        }
        self.x[0] = match n {
            172 | 178 => 1234,
            173 | 29 => (-38i64) as u64,
            56 => {
                let s = self.cstr(self.x[1])?;
                let fd = *self.ctx.fd;
                if s.ends_with(b"urandom") {
                    self.ctx.urand.push(fd);
                    self.ctx.urand.push(0);
                }
                *self.ctx.fd += 1;
                fd
            }
            63 => {
                let fd = self.x[0];
                if self.ctx.urand.contains(&fd) {
                    let k = (self.x[2] as usize).min(4096);
                    for i in 0..k {
                        let b = self.ctx.rng.byte();
                        self.mem.write(self.x[1] + i as u64, &[b])?;
                    }
                    k as u64
                } else {
                    0
                }
            }
            62 | 64 => 0,
            80 => {
                let a = self.x[1];
                self.mem.write(a, &[0u8; 0x80])?;
                self.mem.write(a + 0x10, &(0x21b6u32).to_le_bytes())?;
                self.mem.write(a + 0x38, &(0x1000u64).to_le_bytes())?;
                0
            }
            247 => {
                self.mem.write(self.x[1], &(0x4000_0750u64).to_le_bytes())?;
                0
            }
            169 | 228 | 233 | 215 | 214 | 96 | 94 | 129 | 98 | 95 | 174 | 160 => {
                if dk && (n == 169 || n == 228 || n == 233) {
                    eprintln!("CLK num={n} a={:#x} x0={:#x}", self.x[1], self.x[0]);
                }
                if n == 169 || n == 228 {
                    self.mem.write(self.x[0], &[0u8; 16])?;
                }
                0
            }
            222 => {
                let size = (self.x[1] + 0xfff) & !0xfff;
                let p = *self.ctx.mapbump;
                *self.ctx.mapbump = p + size;
                let fd = self.x[4];
                if fd < 0xffff_ffff {
                    let d = self.mem.read(self.x[5], size as usize)?;
                    self.mem.write(p, &d)?;
                }
                p
            }
            113 => {
                let a = self.x[1];
                super::CLKADDR.store(a, std::sync::atomic::Ordering::Relaxed);
                let i = *self.ctx.clocki;
                *self.ctx.clocki = i + 1;
                let mut ns = if i < 8 {
                    self.ctx.clock_ns[i]
                } else {
                    self.ctx.clock_ns[7] + 1000 * (i as u64 - 7)
                };
                if let Some(o) = nsoff() {
                    ns = (ns as i64).wrapping_add(o) as u64;
                }
                if dk {
                    eprintln!(
                        "CLK pure id={} a={a:#x} {} {}",
                        self.x[0], *self.ctx.clock_sec, ns
                    );
                }
                self.mem.write(a, &(*self.ctx.clock_sec).to_le_bytes())?;
                self.mem.write(a + 8, &ns.to_le_bytes())?;
                0
            }
            _ => 0,
        };
        Ok(())
    }
}

pub const CLOCKNS_CLK: [u64; 8] = [12000, 13000, 14000, 15000, 17000, 18000, 19000, 20000];
pub const CLOCKSEC_CLK: u64 = 1_762_300_000;

/// urandom 的 splitmix64 字节流；状态随 `Native` 走，多次 VM 实例互不影响。
pub struct Rand {
    pub taken: u64,
    state: u64,
    buf: [u8; 8],
    pos: usize,
}

impl Default for Rand {
    fn default() -> Self {
        Rand::new(1)
    }
}

impl Rand {
    pub fn new(seed: u64) -> Self {
        Rand {
            taken: 0,
            state: seed,
            buf: [0; 8],
            pos: 8,
        }
    }

    pub fn byte(&mut self) -> u8 {
        if self.pos == 8 {
            self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut v = self.state;
            v = (v ^ (v >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            v = (v ^ (v >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            self.buf = (v ^ (v >> 31)).to_le_bytes();
            self.pos = 0;
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        self.taken += 1;
        b
    }
}

fn imm_of(o: &Opnd) -> i64 {
    match o {
        Opnd::Imm(v) => *v,
        Opnd::Shift(i, _, _) => imm_of(i),
        _ => 0,
    }
}

fn other_of(o: &Opnd) -> String {
    match o {
        Opnd::Other(s) => s.clone(),
        _ => String::new(),
    }
}

fn vec_reg(s: &str) -> Option<usize> {
    s.strip_prefix('v')?.parse().ok()
}

fn vec_lane(s: &str) -> Option<(usize, u8, u8)> {
    let (r, rest) = s.split_once('.')?;
    let r: usize = r.strip_prefix('v')?.parse().ok()?;
    let (szc, lane) = rest.split_once('[')?;
    let lane: u8 = lane.trim_end_matches(']').parse().ok()?;
    let sz = match szc {
        "b" => 1,
        "h" => 2,
        "s" => 4,
        "d" => 8,
        _ => return None,
    };
    Some((r, sz, lane))
}

/// 原生回调的跨调用状态（分配复放、mmap 游标、随机数、时钟序号）。
pub struct Native {
    pub alloc: Vec<(String, u64, Option<u64>, Option<u64>)>,
    pub aidx: usize,
    pub bump: u64,
    pub mapbump: u64,
    pub jseq: Vec<(u64, u64, u64)>,
    keys: HashMap<u64, u64>,
    once: std::collections::HashSet<u64>,
    pub urand: Vec<u64>,
    pub rng: Rand,
    pub fd: u64,
    pub clocki: usize,
    pub clock_sec: u64,
    pub clock_ns: [u64; 8],
    pub out_len: usize,
    pub jni_next: u64,
    pub jni_id: u64,
    pub jni_str: HashMap<u64, Vec<u8>>,
    pub jni_ptr: HashMap<u64, u64>,
    pub jni_replay: JniReplay,
    pub steps: u64,
    aidx0: usize,
}

fn fixture(name: &str) -> String {
    let rel = format!("scripts/fixtures/medusa-vm/{name}");
    if std::path::Path::new(&rel).is_file() {
        return rel;
    }
    format!(
        "{}/../../scripts/fixtures/medusa-vm/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

impl Native {
    pub fn set_clock(&mut self, sec: u64) {
        self.clock_sec = sec;
    }

    /// 复用前恢复到构造态；`alloc` 与 `jni_replay` 是只读语料，原样保留。
    pub fn rewind(&mut self) {
        self.aidx = self.aidx0;
        self.bump = HEAP_LO;
        self.mapbump = 0x5200_0000;
        self.jseq.clear();
        self.keys.clear();
        self.once.clear();
        self.urand.clear();
        self.fd = 3;
        self.clocki = 4;
        self.clock_sec = CLOCKSEC_CLK;
        self.clock_ns = CLOCKNS_CLK;
        self.out_len = 0;
        self.jni_next = 0x73a;
        self.jni_id = 0x8000_0000;
        self.jni_str.clear();
        self.jni_ptr.clear();
        self.rng = Rand::new(1);
        self.steps = 0;
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Rand::new(seed);
    }

    pub fn new() -> Self {
        let mut alloc = Vec::new();
        let p = std::env::var("MEDUSA_ALLOC").unwrap_or_else(|_| fixture("alloc.tsv"));
        if let Ok(s) = std::fs::read_to_string(p) {
            for l in s.lines() {
                let f: Vec<&str> = l.split('\t').collect();
                if f.len() < 3 {
                    continue;
                }
                let g = |i: usize| {
                    f.get(i)
                        .and_then(|v| u64::from_str_radix(v.trim_start_matches("0x"), 16).ok())
                };
                alloc.push((f[0].to_string(), g(1).unwrap_or(0), g(2), g(3)));
            }
        }
        Native {
            alloc,
            aidx: std::env::var("ALLOC_IDX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(876),
            bump: HEAP_LO,
            mapbump: 0x5200_0000,
            jseq: Vec::new(),
            keys: HashMap::new(),
            once: std::collections::HashSet::new(),
            urand: Vec::new(),
            rng: Rand::new(1),
            fd: 3,
            clocki: 4,
            clock_sec: CLOCKSEC_CLK,
            clock_ns: CLOCKNS_CLK,
            out_len: 0,
            jni_next: 0x73a,
            jni_id: 0x8000_0000,
            jni_str: HashMap::new(),
            jni_ptr: HashMap::new(),
            jni_replay: {
                let mut m: JniReplay = HashMap::new();
                let p = std::env::var("MEDUSA_JNI").unwrap_or_else(|_| fixture("jni.tsv"));
                if let Ok(s) = std::fs::read_to_string(p) {
                    for l in s.lines() {
                        let f: Vec<&str> = l.split('\t').collect();
                        if f.len() < 3 {
                            continue;
                        }
                        let lr = u64::from_str_radix(f[0], 16).unwrap_or(0);
                        let idx = f[1].parse().unwrap_or(0);
                        let v = u64::from_str_radix(f[2], 16).unwrap_or(0);
                        let payload = f.get(4).and_then(|h| {
                            let h = h.trim();
                            (0..h.len() / 2)
                                .map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).ok())
                                .collect::<Option<Vec<u8>>>()
                        });
                        m.entry(lr).or_default().push_back((idx, v, payload));
                    }
                }
                m
            },
            steps: 0,
            aidx0: std::env::var("ALLOC_IDX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(876),
        }
    }

    #[allow(dead_code)]
    fn nx(&mut self, sz: u64) -> u64 {
        let i = self.aidx;
        if i >= self.alloc.len() {
            let p = (self.bump + 15) & !15;
            self.bump = p + ((sz + 15) & !15);
            return p;
        }
        let e = &self.alloc[i];
        self.aidx = i + 1;
        match e.2 {
            Some(a) => a,
            None => {
                let p = (self.bump + 15) & !15;
                self.bump = p + ((sz + 15) & !15);
                p
            }
        }
    }
}

impl<'a> Ctx<'a> {
    fn nx(&mut self, sz: u64) -> u64 {
        let i = *self.aidx;
        if i >= self.alloc.len() {
            let p = (*self.bump + 15) & !15;
            *self.bump = p + ((sz + 15) & !15);
            return p;
        }
        let e = &self.alloc[i];
        *self.aidx = i + 1;
        match e.2 {
            Some(a) => a,
            None => {
                let p = (*self.bump + 15) & !15;
                *self.bump = p + ((sz + 15) & !15);
                p
            }
        }
    }
}

impl<'a> Cpu<'a> {
    fn import(&mut self, name: &str) -> Result<Option<u64>, Error> {
        let x = self.x;
        let mut jump = None;
        self.x[0] = match name {
            "malloc" => self.ctx.nx(x[0]),
            "calloc" => {
                let n = x[0].wrapping_mul(x[1]);
                let p = self.ctx.nx(n);
                self.mem.write(p, &vec![0u8; n as usize])?;
                p
            }
            "realloc" => {
                if x[0] == 0 {
                    self.ctx.nx(x[1])
                } else {
                    let p = self.ctx.nx(x[1]);
                    let d = self.mem.read(x[0], x[1] as usize)?;
                    self.mem.write(p, &d)?;
                    p
                }
            }
            "free" | "srand" => 0,
            "fopen" => {
                self.mem.write(x[1], b"r\0")?;
                let got = GOT_BASE + (self.pc.wrapping_sub(PLT_LO)) / 2;
                jump = Some(self.mem.ld(got, 8)?);
                x[0]
            }
            "gettid" | "getpid" => 1234,
            "getuid" | "geteuid" | "getgid" => 10000,
            "sysconf" => {
                if x[0] == 0x1e {
                    4096
                } else {
                    8
                }
            }
            "pthread_key_create" => {
                let k = (x[0] << 4) | 1;
                self.mem.st(x[0], 4, k)?;
                self.ctx.keys.entry(k).or_insert(0);
                0
            }
            "pthread_getspecific" => *self.ctx.keys.get(&x[0]).unwrap_or(&0),
            "pthread_setspecific" => {
                self.ctx.keys.insert(x[0], x[1]);
                0
            }
            "pthread_once" => {
                if self.ctx.once.contains(&x[0]) {
                    0
                } else {
                    self.ctx.once.insert(x[0]);
                    jump = Some(x[1]);
                    0
                }
            }
            "pthread_key_delete" => {
                self.ctx.keys.remove(&x[0]);
                0
            }
            "rand" => 0x15b8_cb42,
            "clock_gettime" => {
                super::CLKADDR.store(x[1], std::sync::atomic::Ordering::Relaxed);
                self.mem.st(x[1], 8, *self.ctx.clock_sec)?;
                self.mem.st(x[1] + 8, 8, 0)?;
                0
            }
            "memcpy" | "memmove" | "wmemcpy" | "wmemmove" => {
                let k = if name.starts_with('w') { 4 } else { 1 };
                let n = x[2].wrapping_mul(k) as usize;
                let d = self.mem.read(x[1], n)?;
                self.mem.write(x[0], &d)?;
                x[0]
            }
            "memset" | "wmemset" => {
                let k = if name.starts_with('w') { 4 } else { 1 };
                self.mem
                    .write(x[0], &vec![x[1] as u8; x[2].wrapping_mul(k) as usize])?;
                x[0]
            }
            "strlen" => self.cstr(x[0])?.len() as u64,
            "strcmp" | "strncmp" => {
                let mut a = self.cstr(x[0])?;
                let mut b = self.cstr(x[1])?;
                if name == "strncmp" {
                    a.truncate(x[2] as usize);
                    b.truncate(x[2] as usize);
                }
                match a.cmp(&b) {
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Less => -1i64 as u64,
                    _ => 1,
                }
            }
            "strchr" => {
                let s = self.cstr(x[0])?;
                let ch = x[1] as u8;
                if ch == 0 {
                    x[0] + s.len() as u64
                } else {
                    match s.iter().position(|b| *b == ch) {
                        Some(i) => x[0] + i as u64,
                        None => 0,
                    }
                }
            }
            "strcspn" => {
                let s = self.cstr(x[0])?;
                let rej = self.cstr(x[1])?;
                s.iter().position(|b| rej.contains(b)).unwrap_or(s.len()) as u64
            }
            "strstr" => {
                let s = self.cstr(x[0])?;
                let sub = self.cstr(x[1])?;
                match find_sub(&s, &sub) {
                    Some(i) => x[0] + i as u64,
                    None => 0,
                }
            }
            _ => {
                let got = GOT_BASE + (self.pc.wrapping_sub(PLT_LO)) / 2;
                let tgt = self.mem.ld(got, 8)?;
                if tgt > 0x1000_0000 && !(0x10000..0x20000).contains(&tgt) && tgt != self.pc {
                    jump = Some(tgt);
                }
                0
            }
        };
        Ok(jump)
    }
}

fn find_sub(h: &[u8], n: &[u8]) -> Option<usize> {
    if n.is_empty() {
        return Some(0);
    }
    h.windows(n.len()).position(|w| w == n)
}

/// 原生回调重新进入解释器时的现场：下一个 VM pc 与该帧的槽基址（真机 x6）。
pub struct Reenter {
    pub pc: u64,
    pub slots_addr: u64,
}

pub static DEPTH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
struct Dep;
impl Drop for Dep {
    fn drop(&mut self) {
        DEPTH.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 当前线程解码表基址；表定长，指针在进程内稳定。
fn ops_ptr() -> *const std::cell::Cell<*const Ins> {
    OPS.with(|v| v.as_ptr())
}

fn x0_of(regs: &[u64; 32]) -> u64 {
    regs[0]
}

impl Default for Native {
    fn default() -> Self {
        Self::new()
    }
}

impl Native {
    /// 进入原生回调：x0 = 槽 4（蹦床），x1 = 槽 5（闭包对象）。
    /// `bind` 为假表示从 VM 返回地址续跑，寄存器原样沿用。
    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &mut self,
        mem: &mut Mem,
        slots: &[u64; 64],
        addr: u64,
        sp: &mut u64,
        regs: &mut [u64; 32],
        bind: bool,
        _engine: bool,
    ) -> Result<Option<Reenter>, Error> {
        self.call4(mem, slots, addr, sp, regs, bind, _engine, false)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn call4(
        &mut self,
        mem: &mut Mem,
        slots: &[u64; 64],
        addr: u64,
        sp: &mut u64,
        regs: &mut [u64; 32],
        bind: bool,
        _engine: bool,
        preamble: bool,
    ) -> Result<Option<Reenter>, Error> {
        mem.maplazy(HEAP_LO, HEAP_HI);
        DEPTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let _dep = Dep;
        let bind_target = x0_of(regs);
        let trace_callz = addr == 0x1004e710 && regs[0] == 0x102cd1a0;
        let mut x = *regs;
        if bind && addr == 0x101746b4 {
            eprintln!(
                "HXARGS s4..12={:x?} x2..8={:x?}",
                &slots[4..12],
                &regs[2..8]
            );
        }
        if bind {
            x[0] = slots[4];
            x[1] = slots[5];
            x[14] = x[6].wrapping_add(8);
            // 直接返回哨兵：省掉 0x10054340 起的引擎返回路径；AOT 侧据 ret_pc 续跑。
            x[30] = if super::flags::nosent() {
                SENT
            } else {
                0x1005_4340
            };
        }
        if addr == END {
            return Ok(None);
        }
        let mut cpu = Cpu {
            mem,
            x,
            v: [0u128; 32],
            sp: *sp,
            nzcv: [false; 4],
            pc: addr,
            tpidr: 0x701f_1000,
            ops: ops_ptr(),
            semtr: super::semtr(),
            tr: trapq(),
            nosha: nosha(),
            nomem: nomem(),
            nofuse: nofuse(),
            steps: 0,
            ctx: Ctx {
                alloc: &self.alloc,
                aidx: &mut self.aidx,
                bump: &mut self.bump,
                mapbump: &mut self.mapbump,
                jseq: &mut self.jseq,
                keys: &mut self.keys,
                once: &mut self.once,
                urand: &mut self.urand,
                rng: &mut self.rng,
                fd: &mut self.fd,
                clocki: &mut self.clocki,
                clock_sec: &self.clock_sec,
                clock_ns: &self.clock_ns,
                out_len: &mut self.out_len,
                jni_next: &mut self.jni_next,
                jni_id: &mut self.jni_id,
                jni_str: &mut self.jni_str,
                jni_ptr: &mut self.jni_ptr,
                jni_replay: &mut self.jni_replay,
            },
        };
        let pbits = super::probe::mask(u64::MAX);
        let dbg = super::flags::hot();
        let hot = pbits != 0 || dbg;
        let tracing = trace_on();
        let mut hbuf = [0u64; 25];
        let mut hn = 0usize;
        let mut executed = false;
        let nd = nodisp();
        let hist = hist_on();
        let nc = ncfg();
        let (fx, x6d, x0d, hx) = (nc.fx, nc.x6d, nc.x0d, nc.hx);
        let tb = tbits();
        let tr = cpu.tr;
        let pfx = tr || tb != 0;
        let probe_on = hot || slow_probe();
        if tb & T_D != 0 {
            eprintln!("PROBE hot={hot} pbits={pbits:#x} probe_on={probe_on} nd={nd}");
        }
        let dbgclk = super::dbgclk();
        loop {
            let pc = cpu.pc;
            if pfx {
                if pc == 0x1004e710 && tb & T_4 != 0 {
                    eprintln!(
                        "E710 x0={:#x} w@x0={:#x} bind={}",
                        cpu.x[0],
                        cpu.mem.ld(cpu.x[0] & 0xffffffff, 4).unwrap_or(0),
                        bind
                    );
                }
                if tb & T_T != 0 {
                    eprintln!(
                        "T3 {pc:#x} st={} d={}",
                        cpu.steps,
                        DEPTH.load(std::sync::atomic::Ordering::Relaxed)
                    );
                }
                if pc == 0x1017a1b4 && tb & T_2 != 0 {
                    eprintln!(
                        "THK x0={:#x} x1={:#x} x2={:#x} x8={:#x} x17={:#x} sp={:#x} st={}",
                        cpu.x[0], cpu.x[1], cpu.x[2], cpu.x[8], cpu.x[17], cpu.sp, cpu.steps
                    );
                }
                if tr && pc == 0x1005_3f8c {
                    eprintln!(
                        "H53F8C x27={:#x} x6={:#x} sp={:#x} m={:#x}",
                        cpu.x[27],
                        cpu.x[6],
                        cpu.sp,
                        cpu.mem.ld(cpu.x[27].wrapping_sub(32), 8).unwrap_or(0)
                    );
                    if tb & T_6 != 0 {
                        let s35 = cpu.x[6].wrapping_add(8 + 8 * 35);
                        eprintln!(
                            "SLOT35 @{s35:#x} = {:#x} x27-32={:#x} x27-0x20-8={:#x}",
                            cpu.mem.r64(s35).unwrap_or(0),
                            cpu.x[27].wrapping_sub(32),
                            cpu.mem.r64(cpu.x[27].wrapping_sub(0x28)).unwrap_or(0)
                        );
                        let base = 0x701e_dc08u64 + 8;
                        eprintln!("W2 base-0x20={:#x} k25={:#x} k29={:#x} k35={:#x} k36={:#x} k37={:#x} s25addr={:#x}", cpu.mem.r64(base - 0x20).unwrap_or(u64::MAX), cpu.mem.r64(base + 8*25).unwrap_or(0), cpu.mem.r64(base + 8*29).unwrap_or(0), cpu.mem.r64(base + 8*35).unwrap_or(0), cpu.mem.r64(base + 8*36).unwrap_or(0), cpu.mem.r64(base + 8*37).unwrap_or(0), base + 8*25);
                    }
                }
                if tr && pc == 0x1005_3f90 {
                    eprintln!("H5390 x10={:#x}", cpu.x[10]);
                }
                if pc == 0x1004_e714 && tb & T_H != 0 {
                    eprintln!(
                        "H714 x27={:#x} x6={:#x} m={:#x} b20={:#x} x0={:#x}",
                        cpu.x[27],
                        cpu.x[6],
                        cpu.mem.ld(cpu.x[27].wrapping_sub(32), 8).unwrap_or(0),
                        cpu.mem.ld(cpu.x[6].wrapping_sub(32), 8).unwrap_or(0),
                        cpu.x[0]
                    );
                }
            }
            if dbgclk {
                super::CURPC.store(pc, std::sync::atomic::Ordering::Relaxed);
            }
            // 探针全关的主路径：取指后先看解码期算好的稀有 pc 标志，再执行。
            if !probe_on {
                if pc == SENT {
                    *sp = cpu.sp;
                    *regs = cpu.x;
                    self.steps += cpu.steps;
                    regs[30] = 0;
                    return Ok(None);
                }
                if pc == END {
                    *sp = cpu.sp;
                    *regs = cpu.x;
                    self.steps += cpu.steps;
                    return Ok(Some(Reenter {
                        pc: END,
                        slots_addr: cpu.x[6],
                    }));
                }
                let ip = cpu.op()?;
                if pc == 0x1005_3f8c && tr {
                    eprintln!("A x10={:#x}", cpu.x[10]);
                }
                if pc == 0x1005_3f90 && tr {
                    eprintln!("B x10={:#x}", cpu.x[10]);
                }
                if hist {
                    let g = unsafe { &*ip };
                    bump_hist(pc, g.k, g.h);
                }
                let spb = unsafe { (*ip).sp };
                if spb != 0 {
                    if spb & 1 != 0 && !nd {
                        let fp = cpu.x[29];
                        let a7 = fp.wrapping_sub(0x70);
                        cpu.mem.st_q64(a7, cpu.x[30])?;
                        let h = cpu.x[14] as i32;
                        cpu.pc = if h >= cpu.x[22] as i32 {
                            0x101748d8
                        } else {
                            cpu.mem.st_q64(fp.wrapping_sub(0x78), 0)?;
                            cpu.mem.st_q64(fp.wrapping_sub(0x80), cpu.x[27])?;
                            cpu.mem.st_q64(a7, 0)?;
                            if h < cpu.x[11] as i32 {
                                cpu.x[28]
                            } else {
                                cpu.x[25]
                            }
                        };
                        executed = true;
                        continue;
                    }
                    if spb & 2 != 0
                        && (executed || preamble)
                        && (PROG_LO..PROG_HI).contains(&cpu.x[0])
                    {
                        *sp = cpu.sp;
                        *regs = cpu.x;
                        self.steps += cpu.steps;
                        return Ok(Some(Reenter {
                            pc: cpu.x[0],
                            slots_addr: cpu.x[6],
                        }));
                    }
                    if spb & 8 != 0 {
                        if let Ok(p) = cpu.mem.ld(cpu.x[0] + 16, 8) {
                            if p == super::OUT_ADDR {
                                *cpu.ctx.out_len =
                                    cpu.mem.ld(cpu.x[0] + 12, 4).unwrap_or(0) as usize;
                            }
                        }
                    }
                    if spb & 16 != 0 && !cpu.nosha {
                        cpu.steps += 1;
                        if let Err(e) = cpu.sha_update() {
                            self.steps += cpu.steps;
                            return Err(e);
                        }
                        executed = true;
                        continue;
                    }
                    if spb & 32 != 0 && !cpu.nosha {
                        cpu.steps += 1;
                        if let Err(e) = cpu.sha_final() {
                            self.steps += cpu.steps;
                            return Err(e);
                        }
                        executed = true;
                        continue;
                    }
                    if spb & 4 != 0 {
                        let ret = cpu.x[30];
                        let got = GOT_BASE + (pc - PLT_LO) / 2;
                        let tgt = cpu.mem.ld(got, 8)?;
                        let real =
                            tgt > 0x1000_0000 && !(0x10000..0x20000).contains(&tgt) && tgt != pc;
                        if !real {
                            let name = plt_name(got)
                                .ok_or_else(|| Error::Native(format!("plt {:#x}", cpu.pc)))?;
                            let j = cpu.import(name)?;
                            cpu.pc = j.unwrap_or(ret);
                            continue;
                        }
                    }
                }
                cpu.steps += 1;
                let t5 = pfx && tb & T_5 != 0 && pc == 0x1005_3f90;
                if t5 {
                    eprintln!(
                        "PRE-EXEC k={} x10={:#x} ftgt={:#x}",
                        unsafe { (*ip).k },
                        cpu.x[10],
                        unsafe { (*ip).ftgt }
                    );
                }
                if let Err(e) = cpu.exec_ins(ip) {
                    if t5 {
                        eprintln!("POST-EXEC ERR pc={:#x}", cpu.pc);
                    }
                    self.steps += cpu.steps;
                    return Err(e);
                }
                if t5 {
                    eprintln!("POST-EXEC pc={:#x}", cpu.pc);
                }
                executed = true;
                continue;
            }
            if pc > 0x1000_0000 && pc < 0x1100_0000 {
                if hx && (pc == 0x1004_e710 || pc == 0x1004_e714) {
                    eprintln!("HF {pc:#x} x0={:#x} st={}", cpu.x[0], cpu.steps);
                }
                if pc == 0x1004_e710 {
                    if probe_on {
                        FETCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    if fx {
                        let x0 = cpu.x[0];
                        let tag = if (super::PROG_LO..super::PROG_HI).contains(&x0) {
                            "P"
                        } else {
                            "N"
                        };
                        eprintln!("FX {tag} {x0:#x} {:#x} {}", cpu.mem.code_u32(x0), cpu.steps);
                    }
                }
                if pc == 0x101747c8 && !nd {
                    if probe_on {
                        NDISP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    // 低层字节码 dispatch 的第一层：等价于 0x101747c8..0x10174828 的
                    // 纯计算前缀（写三个栈槽后按 w14 与两个界值比较选路）。
                    let fp = cpu.x[29];
                    cpu.mem.st(fp.wrapping_sub(0x70), 8, cpu.x[30])?;
                    let h = cpu.x[14] as i32;
                    cpu.pc = if h >= cpu.x[22] as i32 {
                        0x101748d8
                    } else {
                        cpu.mem.st(fp.wrapping_sub(0x78), 8, 0)?;
                        cpu.mem.st(fp.wrapping_sub(0x80), 8, cpu.x[27])?;
                        cpu.mem.st(fp.wrapping_sub(0x70), 8, 0)?;
                        if h < cpu.x[11] as i32 {
                            cpu.x[28]
                        } else {
                            cpu.x[25]
                        }
                    };
                    continue;
                }
                if probe_on {
                    if cpu.pc == 0x1004_e710 && pbits & super::probe::CALLSTAT != 0 {
                        if cpu.steps % 5000 < 3 {
                            eprintln!("RT x0={:#x} addr={:#x} steps={}", cpu.x[0], addr, cpu.steps);
                        }
                        if pbits & super::probe::RE != 0 {
                            eprintln!(
                                "RE x0={:#x} lo={:#x} hi={:#x}",
                                cpu.x[0],
                                cpu.mem.ld(cpu.sp + 0x20, 8).unwrap_or(0),
                                cpu.mem.ld(cpu.sp + 0x28, 8).unwrap_or(0)
                            );
                        }
                    }
                    if cpu.pc == 0x1017a1b4 && pbits & super::probe::CALLSTAT != 0 {
                        eprintln!(
                            "THK x0={:#x} x1={:#x} o={:02x?}",
                            cpu.x[0],
                            cpu.x[1],
                            cpu.mem.read(cpu.x[1], 0x20).unwrap_or_default()
                        );
                    }
                }
                if pc == 0x1004_e710
                    && (executed || preamble)
                    && (PROG_LO..PROG_HI).contains(&cpu.x[0])
                {
                    if pbits & super::probe::CALLSTAT != 0 {
                        eprintln!(
                            "CSTE {:#x} {} reenter addr={:#x} ndisp={}",
                            bind_target,
                            cpu.steps,
                            addr,
                            NDISP.load(std::sync::atomic::Ordering::Relaxed)
                        );
                    }
                    if pbits & super::probe::CALLSTAT != 0 && bind {
                        eprintln!("CSTB {:#x} {} reenter", bind_target, cpu.steps);
                    }
                    *sp = cpu.sp;
                    *regs = cpu.x;
                    self.steps += cpu.steps;
                    return Ok(Some(Reenter {
                        pc: cpu.x[0],
                        slots_addr: cpu.x[6],
                    }));
                }
                if (PLT_LO..PLT_HI).contains(&pc) {
                    let ret = cpu.x[30];
                    let got = GOT_BASE + (pc - PLT_LO) / 2;
                    let tgt = cpu.mem.ld(got, 8)?;
                    let real = tgt > 0x1000_0000 && !(0x10000..0x20000).contains(&tgt) && tgt != pc;
                    if !real {
                        let name = plt_name(got)
                            .ok_or_else(|| Error::Native(format!("plt {:#x}", cpu.pc)))?;
                        let j = cpu.import(name)?;
                        cpu.pc = j.unwrap_or(ret);
                        continue;
                    }
                }
            }
            if pc == SENT {
                if pbits & super::probe::CALLSTAT != 0 {
                    eprintln!(
                        "CSTE {:#x} {} sent sp={:#x} x6={:#x} frame={:#x} addr={:#x}",
                        bind_target,
                        cpu.steps,
                        addr,
                        cpu.sp,
                        cpu.x[6],
                        cpu.mem.ld(cpu.sp + 0x38, 8).unwrap_or(0)
                    );
                    if (addr == 0x1017a1b4 || addr == 0x1005538c) && cpu.steps > 20000 {
                        eprintln!("HIST {:x?}", &hbuf[..]);
                    }
                }
                if pbits & super::probe::CALLSTAT != 0 && bind {
                    eprintln!("CSTB {:#x} {} ret", bind_target, cpu.steps);
                }
                *sp = cpu.sp;
                *regs = cpu.x;
                self.steps += cpu.steps;
                regs[30] = 0;
                return Ok(None);
            }
            // 程序结束哨兵：交回解释器（pc 直接给 END），由其收尾。
            if pc == END {
                if pbits & super::probe::CALLSTAT != 0 {
                    eprintln!("CSTE {:#x} {} end addr={:#x}", bind_target, cpu.steps, addr);
                }
                if pbits & super::probe::CALLSTAT != 0 && bind {
                    eprintln!("CSTB {:#x} {} end", bind_target, cpu.steps);
                }
                *sp = cpu.sp;
                *regs = cpu.x;
                self.steps += cpu.steps;
                return Ok(Some(Reenter {
                    pc: END,
                    slots_addr: cpu.x[6],
                }));
            }
            cpu.steps += 1;
            let here = cpu.pc;
            let (v11b, w6a, v8a, v0a, x20a, m8a) = if probe_on {
                probe_pre(
                    &mut cpu,
                    pbits,
                    probe_on,
                    x6d,
                    x0d,
                    addr,
                    trace_callz,
                    tracing,
                    dbg,
                    &mut hbuf,
                    &mut hn,
                    here,
                )
            } else {
                (0, 0, 0, 0, 0, 0)
            };
            if here == 0x101a63c4 {
                if let Ok(p) = cpu.mem.ld(cpu.x[0] + 16, 8) {
                    if p == super::OUT_ADDR {
                        *cpu.ctx.out_len = cpu.mem.ld(cpu.x[0] + 12, 4).unwrap_or(0) as usize;
                    }
                }
            }
            if let Err(e) = cpu.exec() {
                if pbits & super::probe::DBG != 0 {
                    eprintln!(
                        "NATERR at {:#x} next={:#x} {:?} sp={:#x} regs {} \nH {}",
                        here,
                        cpu.pc,
                        e,
                        cpu.sp,
                        cpu.x[..31]
                            .iter()
                            .map(|v| format!("{:#x}", v))
                            .collect::<Vec<_>>()
                            .join(" "),
                        hbuf.iter()
                            .map(|v| format!("{:#x}", v))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
                self.steps += cpu.steps;
                return Err(e);
            }
            executed = true;
            if probe_on {
                probe_post(&mut cpu, pbits, here, v11b, w6a, v8a, v0a, x20a, m8a, &hbuf);
            }
        }
    }
}

type ProbeState = (u64, u64, u64, u64, u64, u64);

/// 逐指令探针的前置部分；`probe_on` 为假时整段不进入循环体。
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn probe_pre(
    cpu: &mut Cpu,
    pbits: u64,
    probe_on: bool,
    x6d: bool,
    x0d: bool,
    addr: u64,
    trace_callz: bool,
    tracing: bool,
    dbg: bool,
    hbuf: &mut [u64; 25],
    hn: &mut usize,
    here: u64,
) -> ProbeState {
    hbuf[*hn % 25] = cpu.pc;
    *hn += 1;
    if cpu.steps == 25000 && pbits & super::probe::CALLSTAT != 0 {
        let mut v: Vec<u64> = Vec::with_capacity(25);
        for i in 0..25 {
            v.push(hbuf[(*hn + i) % 25]);
        }
        eprintln!(
            "H25 addr={:#x} x0={:#x} here={:#x} {:x?} bytes={:02x?}",
            addr,
            cpu.x[0],
            cpu.pc,
            v,
            cpu.mem.read(cpu.x[0], 24).unwrap_or_default()
        );
    }
    if probe_on && x6d && here == 0x1017_47c8 {
        static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        {
            let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 40 {
                eprintln!(
                    "X6 x6={:#x} v=[x6]={:#x} w14={:#x} x8={:#x} w8={:#x}",
                    cpu.x[6],
                    cpu.mem.ld(cpu.x[6], 8).unwrap_or(0),
                    cpu.x[14] as u32,
                    cpu.x[8],
                    cpu.x[8] as u32
                );
            }
        }
    }
    if probe_on && x0d && here == 0x1017_469c {
        eprintln!(
            "X0 x0={:#x} x1={:#x} x2={:#x} x6={:#x} x8={:#x} x27={:#x} sp={:#x} [{x0:#x}]={:#x}",
            cpu.x[0],
            cpu.x[1],
            cpu.x[2],
            cpu.x[6],
            cpu.x[8],
            cpu.x[27],
            cpu.sp,
            cpu.mem.ld(cpu.x[0], 4).unwrap_or(0),
            x0 = cpu.x[0]
        );
    }
    if probe_on && here == 0x1005_3074 && semdbg2() {
        eprintln!(
                "CAST sp={:#x} f10={:#x} f18={:#x} f38={:#x} x6={:#x} x8={:#x} x9={:#x} x27={:#x} s4={:#x}",
                cpu.sp,
                cpu.mem.ld(cpu.sp + 0x10, 8).unwrap_or(0),
                cpu.mem.ld(cpu.sp + 0x18, 8).unwrap_or(0),
                cpu.mem.ld(cpu.sp + 0x38, 8).unwrap_or(0),
                cpu.x[6],
                cpu.x[8],
                cpu.x[9],
                cpu.x[27],
                cpu.x[4],
            );
    }
    if pbits & super::probe::PC != 0 {
        eprintln!("PC {:#x} {:#x}", cpu.pc, cpu.sp);
    }
    if super::flags::x6() {
        eprintln!(
            "X6 {:#x} x6={:#x} x27={:#x} sp={:#x} x8={:#x}",
            cpu.pc, cpu.x[6], cpu.x[27], cpu.sp, cpu.x[8]
        );
    }
    if here == 0x101d_0a54 {
        eprintln!(
            "S54 x9={:#x} x21={:#x} x8={:#x}",
            cpu.x[9], cpu.x[21], cpu.x[8]
        );
    }
    if tracing {
        let word = if (0x1000_0000..0x1100_0000).contains(&here) {
            cpu.mem.code_u32(here) as u64
        } else {
            0xdead
        };
        TRACE.with(|t| {
            let mut t = t.borrow_mut();
            t.pc.push(here);
            if t.pc.len() < 400 {
                t.xs.push(cpu.x);
            }
            if (16340..16352).contains(&t.pc.len()) {
                let m = unsafe { (*cpu.op().unwrap()).slow.m };
                eprintln!(
                    "T {here:#x} x28={:#x} nzcv={:?} m={m:?}",
                    cpu.x[28], cpu.nzcv
                );
            }
            t.regs.push([
                cpu.x[0],
                cpu.x[1],
                cpu.x[2],
                cpu.x[3],
                cpu.x[8],
                cpu.nzcv.iter().fold(0u64, |a, b| (a << 1) | *b as u64),
                cpu.x[28],
                word,
            ]);
        });
    }
    if dbg {
        if super::tra() {
            eprintln!(
                "TRA {here:#x} x0={:#x} x8={:#x} x6={:#x} x27={:#x} c={:#x}",
                cpu.x[0],
                cpu.x[8],
                cpu.x[6],
                cpu.x[27],
                super::CURPC.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        super::CURNAT.store(here, std::sync::atomic::Ordering::Relaxed);
        if super::flags::npx() == Some(super::GSTEP.load(std::sync::atomic::Ordering::Relaxed)) {
            eprintln!("NPX2 {:#x} sp={:#x} x0={:#x} x1={:#x} x2={:#x} x8={:#x} x9={:#x} x10={:#x} x14={:#x} x19={:#x} x20={:#x} x21={:#x}", here, cpu.sp, cpu.x[0], cpu.x[1], cpu.x[2], cpu.x[8], cpu.x[9], cpu.x[10], cpu.x[14], cpu.x[19], cpu.x[20], cpu.x[21]);
        }
        if pbits & super::probe::NATDBG != 0 {
            let mut s = format!("NP {} {:#x} sp={:#x}", cpu.steps, here, cpu.sp);
            for x in cpu.x.iter().take(31) {
                s.push_str(&format!(" {:#x}", x));
            }
            eprintln!("{}", s);
        }
        if trace_callz && pbits & super::probe::CZTR != 0 {
            eprintln!(
                "NPX {:#x} x8={:#x} x9={:#x} x10={:#x} x6={:#x}",
                here, cpu.x[8], cpu.x[9], cpu.x[10], cpu.x[6]
            );
        }
        if super::flags::cbz() && here == 0x10054c2c {
            eprintln!(
                "CBZ pc={:#x} x6={:#x} w9={:#x} g={}",
                super::CURPC.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[6],
                cpu.x[9],
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        if super::flags::clx() && matches!(here, 0x102087e0 | 0x102088e8) {
            eprintln!(
                "CLX {:#x} step={} x0={:#x} x1={:#x} x2={:#x} x19={:#x} x20={:#x} x21={:#x}",
                here,
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[0],
                cpu.x[1],
                cpu.x[2],
                cpu.x[19],
                cpu.x[20],
                cpu.x[21]
            );
        }
        if pbits & super::probe::CALLSTAT != 0 && here == 0x1005433c {
            eprintln!(
                "CB {:#x} x0={:#x} x1={:#x} sp={:#x}",
                cpu.x[8], cpu.x[0], cpu.x[1], cpu.sp
            );
        }
        if pbits & super::probe::TRP != 0 && here == 0x1017a1b4 {
            eprintln!(
                "TRP g={} sp={:#x} {}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.sp,
                (0..31)
                    .map(|i| format!("{:#x}", cpu.x[i]))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            eprintln!("TRPM18 {:#x}", cpu.mem.ld(0x701edc18, 8).unwrap_or(0));
            for (a, n) in [
                (0x701edb00u64, 0x40usize),
                (0x701edc08, 0x108),
                (0x701eda40, 0x60),
            ] {
                eprintln!(
                    "TRPM {:#x} {:02x?}",
                    a,
                    cpu.mem.read(a, n).unwrap_or_default()
                );
            }
        }
        if pbits & super::probe::LEA != 0 && (here == 0x1005104c || here == 0x10051080) {
            eprintln!(
                "LEA g={} pc={:#x} x14={:#x} w8={:#x} w12={:#x} x8={:#x} x20={:#x} sp={:#x}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                here,
                cpu.x[14],
                cpu.x[8],
                cpu.x[12],
                cpu.x[8],
                cpu.x[20],
                cpu.sp
            );
        }
        if super::flags::ld6() && matches!(here, 0x1004ee80 | 0x1004ee94 | 0x1004eeac) {
            eprintln!(
                "LD6 {:#x} x17={:#x} x19={:#x} w8={:#x} x8={:#x} x9={:#x} w9={:#x}",
                here, cpu.x[17], cpu.x[19], cpu.x[8], cpu.x[8], cpu.x[9], cpu.x[9]
            );
        }
        if pbits & super::probe::SD != 0 && here == 0x10208930 {
            eprintln!(
                "SD g={} src={:02x?}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.mem.read(cpu.x[21], 0x40).unwrap_or_default()
            );
            eprintln!(
                "SDX g={} x19={:#x} x20={:#x} x21={:#x}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[19],
                cpu.x[20],
                cpu.x[21]
            );
        }
        if pbits & super::probe::SD2 != 0 && here == 0x10208934 {
            eprintln!(
                "SD2 g={} dst={:02x?}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.mem.read(0x701ed090, 0x40).unwrap_or_default()
            );
        }
        if pbits & super::probe::T48 != 0 && here == 0x10178948 {
            let a = cpu.x[10].wrapping_add(cpu.x[2].wrapping_shl(2));
            eprintln!(
                "T48 g={} x2={:#x} x10={:#x} x8={:#x} a={:#x} v={:#x}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[2],
                cpu.x[10],
                cpu.x[8],
                a,
                cpu.mem.ld(a, 4).unwrap_or(0)
            );
        }
        if pbits & super::probe::FDBG != 0 && (here == 0x104fcf54 || here == 0x104fcf70) {
            eprintln!(
                "X20P {:#x} x20={:#x} x19={:#x} hist={:x?}",
                here, cpu.x[20], cpu.x[19], hbuf
            );
        }
        if pbits & super::probe::FDBG != 0 && (here == 0x104e0b00 || here == 0x104e0ae4) && {
            let _ = super::GSTEP.load(std::sync::atomic::Ordering::Relaxed);
            true
        } {
            eprintln!("DGIT {:#x} g={} x19={:#x} x22={:#x} d5={:016x} d13={:016x} d15={:016x} d21={:016x} d24={:016x} d25={:016x} d3={:016x}", here, super::GSTEP.load(std::sync::atomic::Ordering::Relaxed), cpu.x[19], cpu.x[22], cpu.v[5] as u64, cpu.v[13] as u64, cpu.v[15] as u64, cpu.v[21] as u64, cpu.v[24] as u64, cpu.v[25] as u64, cpu.v[3] as u64);
        }
        if pbits & super::probe::FDBG != 0 && here == 0x104fcf54 {
            let a = cpu.x[3].wrapping_add(cpu.x[5].wrapping_shl(4));
            eprintln!(
                "CF54 x3={:#x} x5={:#x} a={:#x} m={:02x?} d11={:016x}",
                cpu.x[3],
                cpu.x[5],
                a,
                cpu.mem.read(a, 16).unwrap_or_default(),
                cpu.v[11] as u64
            );
        }
        if pbits & super::probe::FDBG != 0 && here == 0x1017aa08 {
            eprintln!(
                "AA08 x19={:#x} m={:02x?}",
                cpu.x[19],
                cpu.mem.read(cpu.x[19] + 16, 32).unwrap_or_default()
            );
        }
        if pbits & super::probe::FDBG != 0 && here == 0x101dc6cc {
            eprintln!("CC v0={:016x} hist={:x?}", cpu.v[0] as u64, hbuf);
        }
        if pbits & super::probe::FDBG != 0 && (here == 0x1052fa58 || here == 0x1052fbb0) {
            eprintln!(
                "D8X {:#x} d8={:016x} sp+96={:02x?}",
                here,
                cpu.v[8] as u64,
                cpu.mem
                    .read(cpu.sp.wrapping_add(96), 16)
                    .unwrap_or_default()
            );
        }
        if pbits & super::probe::FDBG != 0
            && (here == 0x101dc724 || here == 0x101dc730 || here == 0x101dc700)
        {
            eprintln!(
                "FD {:#x} d8={:016x} d0={:016x} nzcv={:?} x19={:#x} x0={:#x} x1={:#x} hist={:x?}",
                here,
                cpu.v[8] as u64,
                cpu.v[0] as u64,
                cpu.nzcv,
                cpu.x[19],
                cpu.x[0],
                cpu.x[1],
                hbuf
            );
        }
        if pbits & super::probe::VP != 0 && here == 0x1004e714 {
            eprintln!(
                "VP {} {:#x}",
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[0]
            );
        }
        if pbits & super::probe::LD570 != 0 && (here == 0x104fe4d0 || here == 0x104e08ec) {
            eprintln!(
                "LD570 here={:#x} x4={:#x} mem={:#x} d1={:#x} d0={:#x} x24={:#x} m570={:02x?}",
                here,
                cpu.x[4],
                cpu.mem.ld(0x701ec570, 8).unwrap_or(0),
                cpu.v[1] as u64,
                cpu.v[0] as u64,
                cpu.x[24],
                cpu.mem.read(0x701ec560, 32).unwrap_or_default()
            );
        }
        if pbits & super::probe::MISC != 0 && here == 0x101a63c4 {
            eprintln!(
                "B64 x0={:#x} len={:?} ptr={:?}",
                cpu.x[0],
                cpu.mem.ld(cpu.x[0] + 12, 4),
                cpu.mem.ld(cpu.x[0] + 16, 8)
            );
        }
        if pbits & super::probe::MISC != 0 && here == 0x10051b64 {
            eprintln!(
                "S2 cur={:#x} g={} x6={:#x} m2={:#x} m1={:#x}",
                super::CURPC.load(std::sync::atomic::Ordering::Relaxed),
                super::GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                cpu.x[6],
                cpu.mem.ld(cpu.x[6] + 16, 8).unwrap_or(0),
                cpu.mem.ld(cpu.x[6] + 8, 8).unwrap_or(0)
            );
        }
        if (here == 0x1052f00c || here == 0x1052f174 || here == 0x104bbb0c || here == 0x104bbb20)
            && pbits & super::probe::DBG != 0
        {
            eprintln!("TR {:#x} sp={:#x} x30={:#x}", here, cpu.sp, cpu.x[30]);
        }
    }
    if pbits & super::probe::CALLSTAT != 0 && here == 0x10177bf8 {
        eprintln!(
            "UPD x0={:#x} x1={:#x} x2={:#x} n={:02x?}",
            cpu.x[0],
            cpu.x[1],
            cpu.x[2],
            cpu.mem
                .read(cpu.x[1], (cpu.x[2] as usize).min(32))
                .unwrap_or_default()
        );
    }
    if pbits & super::probe::CALLSTAT != 0 && here == 0x10177ef4 {
        eprintln!(
            "FIN x0={:#x} x1={:#x} st={:02x?}",
            cpu.x[0],
            cpu.x[1],
            cpu.mem.read(cpu.x[0], 0x30).unwrap_or_default()
        );
    }
    if pbits & super::probe::CALLSTAT != 0 {
        *PCTR
            .get_or_init(Default::default)
            .lock()
            .unwrap()
            .entry(here)
            .or_default() += 1;
    }
    if pbits & super::probe::V11 != 0 && here == 0x104fba18 {
        eprintln!(
            "V11IN v={:#x} x18={:#x} sp={:#x}",
            cpu.v[11] as u64, cpu.x[18], cpu.sp
        );
    }
    if pbits & super::probe::V11 != 0 && (here == 0x104fbad4 || here == 0x104fc74c) {
        eprintln!(
            "V11P {:#x} v={:#x} x18={:#x}",
            here, cpu.v[11] as u64, cpu.x[18]
        );
    }
    (
        cpu.v[11] as u64,
        cpu.mem.ld(0x701e_c778, 8).unwrap_or(0),
        cpu.v[8] as u64,
        cpu.v[0] as u64,
        cpu.x[20],
        cpu.mem.ld(0x701e_d808, 8).unwrap_or(0),
    )
}

/// 逐指令探针的后置部分，参数是前置时抓到的现场。
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn probe_post(
    cpu: &mut Cpu,
    pbits: u64,
    here: u64,
    v11b: u64,
    w6a: u64,
    v8a: u64,
    v0a: u64,
    x20a: u64,
    m8a: u64,
    hbuf: &[u64; 25],
) {
    if pbits & super::probe::V11 != 0 && cpu.v[11] as u64 != v11b {
        eprintln!(
            "V11 {:#x} {:#x} -> {:#x} sp={:#x} g={}",
            here,
            v11b,
            cpu.v[11] as u64,
            cpu.sp,
            super::GSTEP.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
    if pbits & super::probe::V11 != 0 && (here == 0x104e0894 || here == 0x104e0a1c) {
        eprintln!(
            "V11M {:#x} sp={:#x} mem={:02x?} d10={:#x} d11={:#x} x29={:#x}",
            here,
            cpu.sp,
            cpu.mem.read(cpu.sp + 112, 16).unwrap_or_default(),
            cpu.v[10] as u64,
            cpu.v[11] as u64,
            cpu.x[29]
        );
    }
    if pbits & super::probe::FDBG != 0 && cpu.mem.ld(0x701e_d808, 8).unwrap_or(0) != m8a {
        eprintln!(
            "M8W {:#x} {:016x} -> {:016x} x0={:#x} hist={:x?}",
            here,
            m8a,
            cpu.mem.ld(0x701e_d808, 8).unwrap_or(0),
            cpu.x[0],
            hbuf
        );
    }
    if pbits & super::probe::FDBG != 0 && cpu.x[20] != x20a {
        eprintln!(
            "X20W {:#x} {:016x} -> {:016x} hist={:x?}",
            here, x20a, cpu.x[20], hbuf
        );
    }
    if pbits & super::probe::FDBG != 0 && cpu.v[0] as u64 != v0a {
        eprintln!(
            "V0W {:#x} {:016x} -> {:016x} sp={:#x} hist={:x?}",
            here, v0a, cpu.v[0] as u64, cpu.sp, hbuf
        );
    }
    if pbits & super::probe::FDBG != 0 && cpu.v[8] as u64 != v8a {
        eprintln!(
            "V8W {:#x} {:016x} -> {:016x} sp={:#x}",
            here, v8a, cpu.v[8] as u64, cpu.sp
        );
    }
    if pbits & super::probe::MISC != 0 {
        let w6b = cpu.mem.ld(0x701e_c778, 8).unwrap_or(0);
        if w6b != w6a {
            eprintln!("W6 {:#x} {:#x} -> {:#x}", here, w6a, w6b);
        }
    }
}

fn plt_name(got: u64) -> Option<&'static str> {
    let go = 0x3453e8 + (got - GOT_BASE) * 2;
    for l in include_str!("../../../../scripts/fixtures/medusa-vm/plt.txt").lines() {
        if let Some((a, n)) = l.split_once(' ') {
            if u64::from_str_radix(a.trim_start_matches("0x"), 16).ok()? == go {
                return Some(n.trim());
            }
        }
    }
    None
}
const K_MOVW: u16 = 202;
const K_CMPBB: u16 = 203;
const K_CMPBW: u16 = 204;
const K_FLD: u16 = 205;
const K_FST: u16 = 206;
const K_MOVR: u16 = 207;
const K_MOVL: u16 = 208;

/// `b.cond` 的相对目标：pc 为分支自身的地址。
#[inline(always)]
fn branch_target(pc: u64, w: u32) -> u64 {
    let off = ((((w >> 5) & 0x7ffff) as i32) << 13) >> 13;
    pc.wrapping_add((off as i64 * 4) as u64)
}
