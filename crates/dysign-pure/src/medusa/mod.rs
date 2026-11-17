//! X-Medusa 的 VM 复刻：槽式字节码解释器。
//!
//! 只覆盖 32.2.6 固定程序；未在表中出现的字节码地址显式报错（不静默跳过）。
//! 表由 `scripts/medusa-vm/genwords.py` + `mkrust.py` 生成。

pub mod native;
mod native_ops;
pub mod native_program;
pub mod words;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub const PAGE: u64 = 0x1000;
/// VM 程序区间（含首尾页），走出区间即视为返回。入口帧 0x102754c0 只是区间中段，
/// 实测执行地址横跨 0x1026d390..0x102e03c0。
pub const PROG_LO: u64 = 0x1026d000;
pub const PROG_HI: u64 = 0x102e6000;

/// `MEDUSA_MC=1` 下的签名阶段计时（纳秒）：0 池取用/回滚、1 输入注入、
/// 2 AOT 执行、3 输出读取与编码。只在阶段边界取一次时间。
pub mod phases {
    pub const N: usize = 12;
    thread_local! {
        static CUR: std::cell::RefCell<Option<[u128; N]>> = const { std::cell::RefCell::new(None) };
        static SAMPLES: std::cell::RefCell<Vec<[u128; N]>> = const { std::cell::RefCell::new(Vec::new()) };
        static NATADDR: std::cell::RefCell<Vec<(u64, u128, u64)>> = const { std::cell::RefCell::new(Vec::new()) };

    }
    pub fn begin() -> bool {
        if std::env::var_os("MEDUSA_MC").is_none() {
            return false;
        }
        CUR.with(|c| *c.borrow_mut() = Some([0; N]));
        true
    }
    pub fn add(i: usize, ns: u128) {
        CUR.with(|c| {
            if let Some(v) = c.borrow_mut().as_mut() {
                v[i] += ns;
            }
        });
    }
    pub fn end() {
        CUR.with(|c| {
            if let Some(v) = c.borrow_mut().take() {
                SAMPLES.with(|s| s.borrow_mut().push(v));
            }
        });
    }
    pub fn busy() -> bool {
        CUR.with(|c| c.borrow().is_some())
    }
    pub fn take() -> Vec<[u128; N]> {
        SAMPLES.with(|s| std::mem::take(&mut *s.borrow_mut()))
    }
    pub fn nat(addr: u64, ns: u128, steps: u64) {
        NATADDR.with(|s| s.borrow_mut().push((addr, ns, steps)));
    }
    pub fn take_nat() -> Vec<(u64, u128, u64)> {
        NATADDR.with(|s| std::mem::take(&mut *s.borrow_mut()))
    }
    pub fn inc(i: usize) {
        CUR.with(|c| {
            if let Some(v) = c.borrow_mut().as_mut() {
                v[i] += 1;
            }
        });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum Kind {
    unknown,
    sltu,
    slt,
    or9,
    shlmask,
    ld8,
    ld8s,
    ld16,
    ld32,
    ld64,
    ld64ua,
    ld64ub,
    ld32u0,
    ld32m3,
    ld32ua,
    ld32ub,
    ld32s,
    lea32,
    sub32,
    sub32_rev,
    sub32_sext,
    nor32,
    shl32r,
    shl32r_rev,
    shr32r,
    shr32r_rev,
    cmpgt,
    cmplt,
    cmpeq,
    cmpgt_s,
    st8,
    st16,
    st32,
    st64,
    st64ua,
    st64ub,
    xchg,
    lea,
    xori,
    addi32,
    ori,
    mov,
    cbz_mov,
    bfx32,
    bfi,
    rev32,
    nop,
    andi,
    add,
    a_add,
    a_sub,
    a_shl32_rev,
    ext64,
    mul32_64,
    a_nor,
    a_shl64r,
    a_shr64r,
    a_shr32_rev,
    a_add32,
    a_and,
    a_or,
    a_xor,
    a_sub32,
    a_slt,
    a_shr32r,
    a_shl64i,
    a_shr64i,
    a_slt_rev,
    a_sltu,
    a_sltu_rev,
    a_sub_rev,
    a_sub32_rev,
    a_shl32r_rev,
    a_shr32r_rev,
    a_shl64r_rev,
    a_shr64r_rev,
    mov64,
    mvn64,
    add32,
    and,
    or,
    xor,
    nor,
    and32,
    or32,
    xor32,
    u32f64,
    i32f64,
    shl64i,
    shl32i,
    lsr64i,
    shr64i,
    shr32i,
    lsr32i,
    ror32i,
    sar32i,
    shrhi32,
    shrhi24,
    shrhi16,
    shrhi8,
    shr64r,
    shl64i_r,
    jge,
    jmp,
    call4,
    call5,
    call1a,
    call1d,
    call22,
    call27,
    call,
    callalu,
    tramp,
    alu,
    jne,
    callz,
    callgt0,
    calllt1,
    callind,
    jmpi,
    cmpge,
    jtab,
}

#[derive(Debug)]
pub enum Error {
    Unmapped(u64),
    BadSnapshot,
    BadQuery,
    BadOutputLen(usize),
    UnknownPc(u64),
    Unsupported(Kind, u64),
    Steps,
    Native(String),
}

/// 进程内固定地址区间。全部按页匿名映射，落在区间内但从未写过的页就是零。
/// 后续 Windows/macOS 只需替换 `map_region`/`unmap_region` 两个函数。
const REGIONS: &[(u64, u64)] = &[
    (0x1_0000, 0x4_0000),
    (0x1000_0000, 0x1100_0000),
    (0x4000_0000, 0x4010_0000),
    (0x5000_0000, 0x5800_0000),
    (0x7000_0000, 0x7040_0000),
    (0x7e17_0000, 0x7e18_0000),
    (0x8000_0000, 0x8010_0000),
];

/// 快照里烘焙死的绝对时间字：7 个「秒」字（u32 秒 + 4 字节零）。
const CLOCK_SEC_WORDS: &[u64] = &[
    0x701e_d660,
    0x701e_d698,
    0x701e_d808,
    0x701e_db30,
    0x701e_ddc0,
    0x701e_de50,
    0x701e_dfe0,
];
/// `{u32 秒, u32 微秒}` 结构：第二个字段也一并重写，免留旧微秒。
const CLOCK_MSEC_WORD: u64 = 0x5008_d920;

/// 环境块尾部：段 `0x1027ab20` 写 `0x503c_fb80..0x503c_fba2`，探针按这个区间判定。
const ENV_LO: u64 = 0x503c_fb80;
const ENV_HI: u64 = 0x503c_fba3;

/// 唯一按需提交的区间（128MB 堆区）：Windows 只保留地址空间，首次访问才提交单页。
const LAZY_REGION: usize = 3;

/// 提交单页；只有 Windows 的懒提交区间会走到这里。
#[cfg(windows)]
fn commit_at(addr: usize) {
    use windows_sys::Win32::System::Memory::{VirtualAlloc, MEM_COMMIT, PAGE_READWRITE};
    let p = unsafe {
        VirtualAlloc(
            addr as *const core::ffi::c_void,
            PAGE as usize,
            MEM_COMMIT,
            PAGE_READWRITE,
        )
    };
    assert!(!p.is_null(), "VirtualAlloc 提交 {addr:#x} 失败");
}

#[cfg(windows)]
fn map_region(lo: u64, hi: u64, lazy: bool) -> usize {
    use windows_sys::Win32::System::Memory::{
        VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE,
    };
    let len = (hi - lo) as usize;
    let flags = if lazy {
        MEM_RESERVE
    } else {
        MEM_RESERVE | MEM_COMMIT
    };
    unsafe {
        let p = VirtualAlloc(lo as *const core::ffi::c_void, len, flags, PAGE_READWRITE);
        if !p.is_null() {
            return p as usize;
        }
        let p = VirtualAlloc(std::ptr::null(), len, flags, PAGE_READWRITE);
        assert!(!p.is_null(), "VirtualAlloc {lo:#x}..{hi:#x} 失败");
        p as usize
    }
}

#[cfg(not(windows))]
fn map_region(lo: u64, hi: u64, _lazy: bool) -> usize {
    let len = (hi - lo) as usize;
    #[cfg(target_os = "linux")]
    unsafe {
        let p = libc::mmap(
            lo as *mut libc::c_void,
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE
                | libc::MAP_ANONYMOUS
                | libc::MAP_FIXED_NOREPLACE
                | libc::MAP_NORESERVE,
            -1,
            0,
        );
        if p != libc::MAP_FAILED {
            return p as usize;
        }
    }
    unsafe {
        let p = libc::mmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            -1,
            0,
        );
        assert!(p != libc::MAP_FAILED, "mmap {lo:#x}..{hi:#x} 失败");
        p as usize
    }
}

#[cfg(windows)]
fn unmap_region(base: usize, _lo: u64, _hi: u64) {
    use windows_sys::Win32::System::Memory::{VirtualFree, MEM_RELEASE};
    unsafe {
        VirtualFree(base as *mut core::ffi::c_void, 0, MEM_RELEASE);
    }
}

#[cfg(not(windows))]
fn unmap_region(base: usize, lo: u64, hi: u64) {
    unsafe {
        libc::munmap(base as *mut libc::c_void, (hi - lo) as usize);
    }
}

/// 固定虚拟地址空间的薄封装：地址全按低 32 位解释，区间外报 `Unmapped`。
/// 写过的页在首次写入前留一份原像，`reset_dirty` 可整体还原（供常驻进程复用地址空间）。
pub struct Mem {
    bases: [(usize, u64); REGIONS.len()],
    dirty_bits: Vec<u64>,
    /// 上次直写命中的页，用来省掉同页重复的脏位测试。
    dpage: std::cell::Cell<u64>,
    saved: HashMap<u64, Box<[u8; PAGE as usize]>>,
    /// 有任一探针开启；关时跳过所有按地址的打印判断。
    hot: bool,
    /// `DBGCLK`：时钟窗口访存打印；构造时取一次，热路径只读字段。
    dbgclk: bool,
    /// 最近一次命中的区间（lo/hi/基址），栈与堆访问几乎全是同一区间。
    rlo: std::cell::Cell<u64>,
    rhi: std::cell::Cell<u64>,
    rp: std::cell::Cell<usize>,
    /// 最近一次命中的区间下标与已确认提交的页，供 Windows 懒提交判定。
    ri: std::cell::Cell<usize>,
    committed: Vec<std::cell::Cell<u64>>,
    cpage: std::cell::Cell<u64>,
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        let bases = REGIONS
            .iter()
            .enumerate()
            .map(|(i, (lo, hi))| (map_region(*lo, *hi, i == LAZY_REGION), *hi))
            .collect::<Vec<_>>();
        let mut bases_arr = [(0usize, 0u64); REGIONS.len()];
        bases_arr.copy_from_slice(&bases);
        Mem {
            bases: bases_arr,
            dirty_bits: vec![0u64; (REGIONS.last().unwrap().1 >> 12) as usize / 64 + 1],
            committed: (0..(REGIONS.last().unwrap().1 >> 12) as usize / 64 + 1)
                .map(|_| std::cell::Cell::new(0u64))
                .collect(),
            cpage: std::cell::Cell::new(u64::MAX),
            dpage: std::cell::Cell::new(u64::MAX),
            saved: HashMap::new(),
            hot: probe::mask(u64::MAX) != 0,
            dbgclk: dbgclk(),
            rlo: std::cell::Cell::new(0),
            rhi: std::cell::Cell::new(0),
            rp: std::cell::Cell::new(0),
            ri: std::cell::Cell::new(0),
        }
    }
    pub fn maplazy(&mut self, _lo: u64, _hi: u64) {}
    #[inline]
    pub fn is_hot(&self) -> bool {
        self.hot
    }
    /// 只读代码区（0x1000_0000 段）的 32 位字；release 直读，debug 校验地址。
    #[inline]
    pub fn code_u32(&self, a: u64) -> u32 {
        debug_assert!(
            a >= REGIONS[1].0 && a + 4 <= REGIONS[1].1,
            "code_u32 越界 {a:#x}"
        );
        unsafe { *((self.bases[1].0 + (a - 0x1000_0000) as usize) as *const u32) }
    }
    /// 直接写槽区（0x7000_0000 段）的 64 位字；槽页在 VM 启动时已脏。
    #[inline]
    pub fn raw_sw64(&mut self, a: u64, v: u64) {
        debug_assert!(
            a >= REGIONS[4].0 && a + 8 <= REGIONS[4].1,
            "raw_sw64 越界 {a:#x}"
        );
        unsafe {
            *((self.bases[4].0 + (a - 0x7000_0000) as usize) as *mut u64) = v.to_le();
        }
    }
    /// 懒提交区间首次访问时补提交该页；非 Windows 是空函数。
    #[inline(always)]
    fn ensure_commit(&self, i: usize, lo: u64, base: usize, a: u64) {
        if i != LAZY_REGION {
            return;
        }
        let page = a & !(PAGE - 1);
        if self.cpage.get() == page {
            return;
        }
        let idx = (page >> 12) as usize;
        let (w, bit) = (idx / 64, 1u64 << (idx % 64));
        let cell = &self.committed[w];
        let v = cell.get();
        if v & bit == 0 {
            #[cfg(windows)]
            commit_at(base + (page - lo) as usize);
            #[cfg(not(windows))]
            let _ = (lo, base);
            cell.set(v | bit);
        }
        self.cpage.set(page);
    }
    /// 地址所在区间与区间上界。
    #[inline(always)]
    fn region(&self, a: u64) -> Option<(usize, u64)> {
        let (clo, chi) = (self.rlo.get(), self.rhi.get());
        if a >= clo && a < chi {
            let base = self.rp.get();
            self.ensure_commit(self.ri.get(), clo, base, a);
            return Some((base + (a - clo) as usize, chi));
        }
        let i = match a >> 24 {
            0x0 => 0,
            0x10 => 1,
            0x40 => 2,
            0x50..=0x57 => 3,
            0x70 => 4,
            0x7e => 5,
            0x80 => 6,
            _ => return None,
        };
        let (lo, hi) = REGIONS[i];
        if a < lo || a >= hi {
            return None;
        }
        self.rlo.set(lo);
        self.rhi.set(hi);
        self.rp.set(self.bases[i].0);
        self.ri.set(i);
        self.ensure_commit(i, lo, self.bases[i].0, a);
        Some((self.bases[i].0 + (a - lo) as usize, hi))
    }
    #[inline(always)]
    pub fn span(&self, addr: u64, n: usize) -> Option<usize> {
        let (p, hi) = self.region(addr)?;
        if addr + n as u64 > hi {
            return None;
        }
        Some(p)
    }
    /// 写前把页标脏（已脏的页只做一次位测试）。
    #[inline]
    pub fn touch_page(&mut self, page: u64) {
        let idx = (page >> 12) as usize;
        if self.dirty_bits[idx / 64] & (1u64 << (idx % 64)) == 0 {
            self.mark_dirty(page);
        }
    }
    fn mark_dirty(&mut self, page: u64) {
        let idx = (page >> 12) as usize;
        let (w, bit) = (idx / 64, 1u64 << (idx % 64));
        if self.dirty_bits[w] & bit != 0 {
            return;
        }
        self.dirty_bits[w] |= bit;
        if semtr() && page == 0x102d_6000 {
            eprintln!(
                "MARK102d6 region={:?} id={}",
                self.region(page).is_some(),
                CASEID.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        let mut b = Box::new([0u8; PAGE as usize]);
        if let Some(p) = self.region(page) {
            unsafe {
                std::ptr::copy_nonoverlapping(p.0 as *const u8, b.as_mut_ptr(), PAGE as usize);
            }
        }
        self.saved.insert(page, b);
    }
    /// 把写过的页恢复成首次写入前的内容。
    pub fn reset_dirty(&mut self) {
        let n = self.saved.len();
        if semtr() {
            eprintln!(
                "RESET dirty={n} has102d6={}",
                self.saved.contains_key(&0x102d_6000)
            );
        }
        for (page, data) in std::mem::take(&mut self.saved) {
            if let Some(p) = self.region(page) {
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), p.0 as *mut u8, PAGE as usize);
                }
            }
            let idx = (page >> 12) as usize;
            self.dirty_bits[idx / 64] &= !(1u64 << (idx % 64));
        }
        if semtr() {
            eprintln!("RESETDONE v={:#x}", self.ld(0x102d_6234, 4).unwrap_or(0));
        }
    }
    pub fn dirty_pages(&self) -> impl Iterator<Item = u64> + '_ {
        self.saved.keys().copied()
    }
    pub fn map(&mut self, addr: u64, data: &[u8]) {
        let addr = addr & 0xffff_ffff;
        let mut done = 0usize;
        while done < data.len() {
            let a = addr + done as u64;
            let page = a & !(PAGE - 1);
            let off = (a & (PAGE - 1)) as usize;
            let n = (PAGE as usize - off).min(data.len() - done);
            if let Some(p) = self.region(page) {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        data.as_ptr().add(done),
                        (p.0 + off) as *mut u8,
                        n,
                    );
                }
            }
            done += n;
        }
    }
    /// 地址是否落在已映射区间内。
    pub fn covers(&self, a: u64) -> bool {
        self.region(a).is_some()
    }

    pub fn page_bytes(&self, page: u64) -> Option<Vec<u8>> {
        let p = self.region(page)?;
        Some(unsafe { std::slice::from_raw_parts(p.0 as *const u8, PAGE as usize) }.to_vec())
    }
    pub fn read(&self, addr: u64, n: usize) -> Result<Vec<u8>, Error> {
        if self.dbgclk && (0x701e_c858..0x701e_c870).contains(&addr) {
            eprintln!(
                "CLKRD pc={:#x} a={addr:#x}",
                CURPC.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        // VM 的槽里存的是 32 位地址的无符号符号扩展形式，一律按低 32 位取。
        let addr = addr & 0xffff_ffff;
        if self.hot {
            if probe::on(probe::RQ) && addr < 0x5040_0000 && addr + n as u64 > 0x503c_f000 {
                eprintln!(
                    "RQ {:#x} n={} pc={:#x} step={}",
                    addr,
                    n,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
            if probe::on(probe::WAT) && addr <= 0x5080_d100 && addr + n as u64 > 0x5080_d000 {
                eprintln!(
                    "RAT {:#x} n={} pc={:#x} step={}",
                    addr,
                    n,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
        }
        let mut out = Vec::with_capacity(n);
        if flags::rdx() {
            let st = GSTEP.load(std::sync::atomic::Ordering::Relaxed);
            if (36700..36900).contains(&st) && (0x503f_f700..0x503f_f800).contains(&addr) {
                eprintln!(
                    "RDX {:#x} n={} pc={:#x} nat={:#x} step={}",
                    addr,
                    n,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    st
                );
            }
        }
        for i in 0..n as u64 {
            let a = addr + i;
            let p = match self.region(a) {
                Some(p) => p,
                None => {
                    if std::env::var_os("TRAP").is_some() {
                        eprintln!(
                            "TRAP mem.ld a={a:#x} n={n} curpc={:#x}",
                            CURPC.load(std::sync::atomic::Ordering::Relaxed)
                        );
                        eprintln!("{}", std::backtrace::Backtrace::force_capture());
                    }
                    return Err(Error::Unmapped(a));
                }
            };
            out.push(unsafe { *((p.0) as *const u8) });
        }
        Ok(out)
    }
    pub fn write(&mut self, addr: u64, data: &[u8]) -> Result<(), Error> {
        let addr = addr & 0xffff_ffff;
        let addr = addr & 0xffff_ffff;
        if self.hot {
            if STR_ON.load(std::sync::atomic::Ordering::Relaxed) {
                eprintln!(
                    "STR {:#x} n={} pc={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if semtr() && (0x102d_6000..0x102d_7000).contains(&addr) {
                eprintln!(
                    "WRCODE a={addr:#x} n={} id={} {:?}",
                    data.len(),
                    CASEID.load(std::sync::atomic::Ordering::Relaxed),
                    data
                );
            }
            if flags::wsx() && addr < 0x701e_c900 && addr + data.len() as u64 > 0x701e_c8c0 {
                eprintln!(
                    "WSXw {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            {
                let st = GSTEP.load(std::sync::atomic::Ordering::Relaxed);
                if (36500..36900).contains(&st) && flags::wrx() {
                    eprintln!(
                        "WRX {:#x} n={} pc={:#x} step={} {}",
                        addr,
                        data.len(),
                        CURPC.load(std::sync::atomic::Ordering::Relaxed),
                        st,
                        data.iter()
                            .map(|b| format!("{:02x}", b))
                            .collect::<String>()
                    );
                }
            }
            if flags::wr7() && addr < 0x503f_f800 && addr + data.len() as u64 > 0x503f_f000 {
                eprintln!(
                    "WR7 {:#x} n={} pc={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WENV) && addr < ENV_HI && addr + data.len() as u64 > ENV_LO {
                eprintln!(
                    "WENV {:#x} n={} pc={:#x} step={}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
            if probe::on(probe::WQ) && addr < 0x701e_d2b0 && addr + data.len() as u64 > 0x701e_d270
            {
                eprintln!(
                    "WQ {:#x} n={} pc={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WF) && addr < 0x701ed0e0 && addr + data.len() as u64 > 0x701ed060 {
                eprintln!(
                    "WF {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WFS)
                && addr <= 0x701e_dcf8
                && 0x701e_dcf8 < addr + data.len() as u64
            {
                eprintln!(
                    "WFS {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WFD)
                && addr <= 0x701e_d808
                && 0x701e_d808 < addr + data.len() as u64
            {
                eprintln!(
                    "WFD {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WFB)
                && addr <= 0x5009_ebb8
                && 0x5009_ebb8 < addr + data.len() as u64
            {
                eprintln!(
                    "WFB {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WFA)
                && addr <= 0x5009_df70
                && 0x5009_df70 < addr + data.len() as u64
            {
                eprintln!(
                    "WFA {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::WFC)
                && addr <= 0x701e_c570
                && 0x701e_c570 < addr + data.len() as u64
            {
                eprintln!(
                    "WFC {:#x} n={} pc={:#x} nat={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::W17) && addr < 0x701ed2c0 && addr + data.len() as u64 > 0x701ed2a8 {
                eprintln!(
                    "WOBJ {:#x} {:#x} {}",
                    addr,
                    addr,
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::W17) && addr < 0x701edb10 && addr + data.len() as u64 > 0x701eda40 {
                eprintln!(
                    "WCTX {:#x} pc={:#x} {}",
                    addr,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
            if probe::on(probe::W17)
                && ((addr < 0x701ec820 && addr + data.len() as u64 > 0x701ec740)
                    || (addr < 0x701ec810 && addr + data.len() as u64 > 0x701ec808))
            {
                eprintln!(
                    "WS2w {:#x} pc={:#x} {} step={}",
                    addr,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>(),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
            if probe::on(probe::WAT)
                && addr <= 0x5080_d100
                && addr + data.len() as u64 > 0x5080_d000
            {
                eprintln!(
                    "WAT {:#x} n={} pc={:#x} step={} {}",
                    addr,
                    data.len(),
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                    data.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                );
            }
        }
        let base = self.span(addr, data.len()).ok_or(Error::Unmapped(addr))?;
        for i in 0..data.len() as u64 {
            let page = (addr + i) & !(PAGE - 1);
            let idx = (page >> 12) as usize;
            if self.dirty_bits[idx / 64] & (1u64 << (idx % 64)) == 0 {
                self.mark_dirty(page);
            }
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), base as *mut u8, data.len());
        }
        Ok(())
    }
    #[inline(always)]
    pub fn ld(&self, addr: u64, n: usize) -> Result<u64, Error> {
        let a = addr & 0xffff_ffff;
        if self.dbgclk && (0x701e_c800..0x701e_d000).contains(&a) {
            eprintln!(
                "CLKLD pc={:#x} a={a:#x}",
                CURPC.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        if self.hot {
            let pb = probe::mask(probe::WQIN | probe::WCLK);
            if pb != 0 {
                let clk = CLKADDR.load(std::sync::atomic::Ordering::Relaxed);
                if pb & probe::WQIN != 0 && a < 0x5080_a310 && a + n as u64 > 0x5080_a000 {
                    eprintln!(
                        "WQIN {:#x} n={} pc={:#x} step={}",
                        a,
                        n,
                        CURPC.load(std::sync::atomic::Ordering::Relaxed),
                        GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                    );
                }
                if pb & probe::WCLK != 0 && clk != 0 && a < clk + 16 && a + n as u64 > clk {
                    eprintln!(
                        "WCLK {:#x} n={} pc={:#x} step={} clk={:#x}",
                        a,
                        n,
                        CURPC.load(std::sync::atomic::Ordering::Relaxed),
                        GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                        clk
                    );
                }
            }
            if probe::on(probe::W7E) && (0x7e170000..0x7e172000).contains(&a) {
                eprintln!(
                    "R7E {:#x} n={} pc={:#x} nat={:#x} step={}",
                    a,
                    n,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CURNAT.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
            if probe::on(probe::WAT2) {
                let pc = CURPC.load(std::sync::atomic::Ordering::Relaxed);
                if CURNAT.load(std::sync::atomic::Ordering::Relaxed) == 0x104bc1bc {
                    let v = self.read(0x5080_d000, 0xa0).unwrap_or_default();
                    eprintln!(
                        "DUMP pc={:#x} step={} {:02x?}",
                        pc,
                        GSTEP.load(std::sync::atomic::Ordering::Relaxed),
                        v
                    );
                }
            }
        }
        let base = match self.span(a, n) {
            Some(b) => b as *const u8,
            None => {
                if std::env::var_os("TRAP").is_some() {
                    eprintln!(
                        "TRAP mem.ld2 a={a:#x} n={n} curpc={:#x}",
                        CURPC.load(std::sync::atomic::Ordering::Relaxed)
                    );
                    eprintln!("{}", std::backtrace::Backtrace::force_capture());
                }
                return Err(Error::Unmapped(a));
            }
        };
        unsafe {
            Ok(match n {
                1 => *base as u64,
                2 => u16::from_le(std::ptr::read_unaligned(base as *const u16)) as u64,
                4 => u32::from_le(std::ptr::read_unaligned(base as *const u32)) as u64,
                8 => u64::from_le(std::ptr::read_unaligned(base as *const u64)),
                _ => {
                    let mut v = 0u64;
                    for i in 0..n as u64 {
                        v |= (*base.add(i as usize) as u64).wrapping_shl((i * 8) as u32);
                    }
                    v
                }
            })
        }
    }
    #[inline(always)]
    pub fn st(&mut self, addr: u64, n: usize, v: u64) -> Result<(), Error> {
        let a = addr & 0xffff_ffff;
        let wi = WATCH.load(std::sync::atomic::Ordering::Relaxed);
        if wi != 0 {
            let w = wi - 1;
            if a <= w && a + n as u64 > w {
                eprintln!(
                    "WATCH {:#x} <- {:#x} n={} pc={:#x} st={}",
                    a,
                    v,
                    n,
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
        }
        if self.hot {
            let pb = probe::mask(probe::WQIN | probe::WENV);
            if pb != 0 {
                if pb & probe::WENV != 0 && a < ENV_HI && a + n as u64 > ENV_LO {
                    eprintln!(
                        "WENVs {:#x} n={} pc={:#x} step={}",
                        a,
                        n,
                        CURPC.load(std::sync::atomic::Ordering::Relaxed),
                        GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                    );
                }
                if pb & probe::WQIN != 0 && a < 0x5080_a310 && a + n as u64 > 0x5080_a000 {
                    eprintln!(
                        "WQINs {:#x} n={} pc={:#x} step={}",
                        a,
                        n,
                        CURPC.load(std::sync::atomic::Ordering::Relaxed),
                        GSTEP.load(std::sync::atomic::Ordering::Relaxed)
                    );
                }
            }
            if semtr() && (0x102d_6000..0x102d_7000).contains(&a) {
                eprintln!(
                    "STCODE a={a:#x} n={n} v={v:#x} pc={:#x} id={}",
                    CURPC.load(std::sync::atomic::Ordering::Relaxed),
                    CASEID.load(std::sync::atomic::Ordering::Relaxed)
                );
            }
        }
        let base = self.span(a, n).ok_or(Error::Unmapped(a))? as *mut u8;
        let first = a & !(PAGE - 1);
        let idx = (first >> 12) as usize;
        if self.dirty_bits[idx / 64] & (1u64 << (idx % 64)) == 0 {
            self.mark_dirty(first);
        }
        // 跨页写很少见；单页时下面这段整段跳过。
        let last = (a + n as u64 - 1) & !(PAGE - 1);
        if last != first {
            let mut page = first + PAGE;
            while page <= last {
                let i = (page >> 12) as usize;
                if self.dirty_bits[i / 64] & (1u64 << (i % 64)) == 0 {
                    self.mark_dirty(page);
                }
                page += PAGE;
            }
        }
        unsafe {
            match n {
                1 => *base = v as u8,
                2 => std::ptr::write_unaligned(base as *mut u16, (v as u16).to_le()),
                4 => std::ptr::write_unaligned(base as *mut u32, (v as u32).to_le()),
                8 => std::ptr::write_unaligned(base as *mut u64, v.to_le()),
                _ => {
                    for i in 0..n as u64 {
                        *base.add(i as usize) = (v >> (i * 8)) as u8;
                    }
                }
            }
        }
        Ok(())
    }
    /// 快路径 8 字节写：区间缓存命中且所在页已脏时直接落笔，否则回退完整 `st`。
    #[inline(always)]
    pub fn st_q64(&mut self, addr: u64, v: u64) -> Result<(), Error> {
        let a = addr & 0xffff_ffff;
        let (clo, chi) = (self.rlo.get(), self.rhi.get());
        if a >= clo && a + 8 <= chi {
            let page = a & !(PAGE - 1);
            let idx = (page >> 12) as usize;
            if self.dpage.get() == page || self.dirty_bits[idx / 64] & (1u64 << (idx % 64)) != 0 {
                let p = self.rp.get() + (a - clo) as usize;
                unsafe { std::ptr::write_unaligned(p as *mut u64, v.to_le()) };
                self.dpage.set(page);
                return Ok(());
            }
        }
        self.st(addr, 8, v)
    }
    /// 快路径 8 字节读：区间缓存命中时直接读，否则回退完整 `ld`。
    #[inline(always)]
    pub fn ld_q64(&self, addr: u64) -> Result<u64, Error> {
        let a = addr & 0xffff_ffff;
        if self.dbgclk && (0x701e_c800..0x701e_d000).contains(&a) {
            eprintln!(
                "CLKLD pc={:#x} a={a:#x}",
                CURPC.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        let (clo, chi) = (self.rlo.get(), self.rhi.get());
        if a >= clo && a + 8 <= chi {
            let p = self.rp.get() + (a - clo) as usize;
            return Ok(unsafe { std::ptr::read_unaligned(p as *const u64) }.to_le());
        }
        self.ld(addr, 8)
    }
    /// 调试用：全地址空间按窗口搜字节串，返回命中地址。
    pub fn scan(&self, needle: &[u8]) -> Vec<u64> {
        let mut hits = Vec::new();
        for (lo, hi) in REGIONS {
            let mut a = *lo;
            while a + needle.len() as u64 <= *hi {
                if self
                    .read(a, needle.len())
                    .map(|v| v == needle)
                    .unwrap_or(false)
                {
                    hits.push(a);
                }
                a += 1;
            }
        }
        hits
    }
    pub fn r64(&self, a: u64) -> Result<u64, Error> {
        let b = self.read(a, 8)?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn w64(&mut self, a: u64, v: u64) -> Result<(), Error> {
        self.st(a, 8, v)
    }
}

impl Drop for Mem {
    fn drop(&mut self) {
        for (i, (lo, hi)) in REGIONS.iter().enumerate() {
            unmap_region(self.bases[i].0, *lo, *hi);
        }
    }
}

pub static STR_ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// `MEDUSA_STR`：打印串对象写入。
#[inline(always)]
fn str_on() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("MEDUSA_STR").is_ok())
}

thread_local! {
    /// 单线程复用整套内存映射与原生上下文：回滚比重解快照便宜一个量级。
    static POOL: std::cell::RefCell<Option<(Vm, std::time::Instant)>> =
        const { std::cell::RefCell::new(None) };
}

/// 空闲多久后丢弃线程本地实例（秒，0 = 每次重建）。默认 300。
static IDLE_SECS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

fn idle_secs() -> u64 {
    let v = IDLE_SECS.load(std::sync::atomic::Ordering::Relaxed);
    if v != u64::MAX {
        return v;
    }
    let v = std::env::var("MEDUSA_IDLE_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);
    IDLE_SECS.store(v, std::sync::atomic::Ordering::Relaxed);
    v
}

/// 覆盖空闲回收时长（秒）；0 表示每次重建。
pub fn set_idle_secs(v: u64) {
    IDLE_SECS.store(v, std::sync::atomic::Ordering::Relaxed);
}

/// 立即丢弃本线程的常驻实例，压测/空闲回收用。
pub fn release_pool() {
    POOL.with(|c| {
        c.borrow_mut().take();
    });
}
pub static GSTEP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CURPC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LAST2: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LAST3: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LAST4: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CURNAT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CLKADDR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CURSLOTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 热路径探针开关：每个开关一个原子，首次读到 0 时才查环境。
#[macro_export]
macro_rules! env_flag {
    ($f:ident, $name:literal) => {
        pub fn $f() -> bool {
            static V: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
            match V.load(std::sync::atomic::Ordering::Relaxed) {
                0 => {
                    let on = $crate::medusa::env_on($name);
                    V.store(if on { 2 } else { 1 }, std::sync::atomic::Ordering::Relaxed);
                    on
                }
                1 => false,
                _ => true,
            }
        }
    };
}

pub mod flags {
    env_flag!(x6, "MEDUSA_X6");
    env_flag!(cbz, "MEDUSA_CBZ");
    env_flag!(clx, "MEDUSA_CLX");
    env_flag!(ld6, "MEDUSA_LD6");
    env_flag!(wsx, "MEDUSA_WSX");
    env_flag!(wrx, "MEDUSA_WRX");
    env_flag!(wr7, "MEDUSA_WR7");
    env_flag!(w28, "MEDUSA_W28");
    env_flag!(ws18, "MEDUSA_WS18");
    env_flag!(at090, "MEDUSA_AT090");
    env_flag!(rdx, "MEDUSA_RDX");
    env_flag!(sdall, "MEDUSA_SDALL");
    env_flag!(sh, "MEDUSA_SH");
    env_flag!(always_engine, "MEDUSA_ALWAYS_ENGINE");
    env_flag!(oldeng, "MEDUSA_OLDENG");
    env_flag!(stx, "MEDUSA_STX");
    env_flag!(dump, "MEDUSA_DUMP");
    env_flag!(hist, "MEDUSA_HIST");
    env_flag!(nosent, "MEDUSA_NOSENT");
    env_flag!(v6, "MEDUSA_V6DBG");
    /// 任一逐字调试开关开启；关闭时整段跳过探针判断。
    pub fn hot() -> bool {
        static V: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(2);
        match V.load(std::sync::atomic::Ordering::Relaxed) {
            0 => false,
            1 => true,
            _ => {
                let b = x6()
                    || cbz()
                    || clx()
                    || ld6()
                    || wsx()
                    || wrx()
                    || wr7()
                    || w28()
                    || ws18()
                    || at090()
                    || rdx()
                    || sdall()
                    || sh()
                    || oldeng()
                    || stx()
                    || dump()
                    || npx().is_some();
                V.store(b as u8, std::sync::atomic::Ordering::Relaxed);
                b
            }
        }
    }
    pub fn npx() -> Option<u64> {
        static V: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        let v = V.load(std::sync::atomic::Ordering::Relaxed);
        if v != u64::MAX {
            return (v != u64::MAX - 1).then_some(v);
        }
        let p = super::env_num_opt("MEDUSA_NPX");
        V.store(
            p.unwrap_or(u64::MAX - 1),
            std::sync::atomic::Ordering::Relaxed,
        );
        p
    }
    /// 记录执行过的 pc 集合（MEDUSA_PCSET=1），退出时写 MEDUSA_PCSET_OUT。
    pub fn pcset() -> bool {
        static V: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(2);
        match V.load(std::sync::atomic::Ordering::Relaxed) {
            0 => false,
            1 => true,
            _ => {
                let b = std::env::var("MEDUSA_PCSET").is_ok();
                V.store(b as u8, std::sync::atomic::Ordering::Relaxed);
                b
            }
        }
    }
    pub fn win() -> u64 {
        static V: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        let v = V.load(std::sync::atomic::Ordering::Relaxed);
        if v != u64::MAX {
            return v;
        }
        let n = super::env_num("MEDUSA_WIN");
        V.store(n, std::sync::atomic::Ordering::Relaxed);
        n
    }
}

/// 一次性快照全部 MEDUSA_* 环境变量（仅探针首次使用，热路径走 `flags`）。
pub fn env_on(name: &'static str) -> bool {
    static M: std::sync::OnceLock<HashMap<String, bool>> = std::sync::OnceLock::new();
    M.get_or_init(|| std::env::vars().map(|(k, v)| (k, v != "0")).collect())
        .get(name)
        .copied()
        .unwrap_or(false)
}

pub fn env_num_opt(name: &'static str) -> Option<u64> {
    static M: std::sync::OnceLock<HashMap<String, Option<u64>>> = std::sync::OnceLock::new();
    *M.get_or_init(|| std::env::vars().map(|(k, v)| (k, v.parse().ok())).collect())
        .get(name)?
}

fn env_num(name: &'static str) -> u64 {
    static M: std::sync::OnceLock<HashMap<String, u64>> = std::sync::OnceLock::new();
    M.get_or_init(|| {
        std::env::vars()
            .filter_map(|(k, v)| Some((k, v.parse().ok()?)))
            .collect()
    })
    .get(name)
    .copied()
    .unwrap_or(0)
}

/// 探针开关：env 只在首次判断时读一次，热路径走位掩码。
pub mod probe {
    pub const ALL: u64 = 1 << 0;
    pub const DBG: u64 = 1 << 1;
    pub const FDBG: u64 = 1 << 2;
    pub const JNI_DBG: u64 = 1 << 3;
    pub const LD570: u64 = 1 << 4;
    pub const LEA: u64 = 1 << 5;
    pub const NATDBG: u64 = 1 << 6;
    pub const OLDENG: u64 = 1 << 7;
    pub const OPDBG: u64 = 1 << 8;
    pub const PC: u64 = 1 << 9;
    pub const RE: u64 = 1 << 10;
    pub const RG: u64 = 1 << 11;
    pub const RQ: u64 = 1 << 12;
    pub const SB: u64 = 1 << 13;
    pub const SD: u64 = 1 << 14;
    pub const SD2: u64 = 1 << 15;
    pub const SL2: u64 = 1 << 16;
    pub const SLH: u64 = 1 << 17;
    pub const SLOTW: u64 = 1 << 18;
    pub const SM: u64 = 1 << 19;
    pub const ST: u64 = 1 << 20;
    pub const STA: u64 = 1 << 21;
    pub const T48: u64 = 1 << 22;
    pub const TRP: u64 = 1 << 23;
    pub const V11: u64 = 1 << 24;
    pub const VP: u64 = 1 << 25;
    pub const W17: u64 = 1 << 26;
    pub const W7E: u64 = 1 << 27;
    pub const WAT: u64 = 1 << 28;
    pub const WAT2: u64 = 1 << 29;
    pub const WF: u64 = 1 << 30;
    pub const WFA: u64 = 1 << 31;
    pub const WFB: u64 = 1 << 32;
    pub const WFC: u64 = 1 << 33;
    pub const WFD: u64 = 1 << 34;
    pub const WFS: u64 = 1 << 35;
    pub const WO: u64 = 1 << 36;
    pub const WQ: u64 = 1 << 37;
    pub const MISC: u64 = 1 << 38;
    pub const WENV: u64 = 1 << 39;
    pub const WQIN: u64 = 1 << 40;
    pub const WCLK: u64 = 1 << 41;
    pub const CALLSTAT: u64 = 1 << 42;
    pub const AOTPC: u64 = 1 << 43;
    pub const CZTR: u64 = 1 << 44;
    pub const DIRTY: u64 = 1 << 45;
    static BITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
    const NAMES: &[&str] = &[
        "MEDUSA_ALL",
        "MEDUSA_DBG",
        "MEDUSA_FDBG",
        "MEDUSA_JNI_DBG",
        "MEDUSA_LD570",
        "MEDUSA_LEA",
        "MEDUSA_NATDBG",
        "MEDUSA_OLDENG",
        "MEDUSA_OPDBG",
        "MEDUSA_PC",
        "MEDUSA_RE",
        "MEDUSA_RG",
        "MEDUSA_RQ",
        "MEDUSA_SB",
        "MEDUSA_SD",
        "MEDUSA_SD2",
        "MEDUSA_SL2",
        "MEDUSA_SLH",
        "MEDUSA_SLOTW",
        "MEDUSA_SM",
        "MEDUSA_ST",
        "MEDUSA_STA",
        "MEDUSA_T48",
        "MEDUSA_TRP",
        "MEDUSA_V11",
        "MEDUSA_VP",
        "MEDUSA_W17",
        "MEDUSA_W7E",
        "MEDUSA_WAT",
        "MEDUSA_WAT2",
        "MEDUSA_WF",
        "MEDUSA_WFA",
        "MEDUSA_WFB",
        "MEDUSA_WFC",
        "MEDUSA_WFD",
        "MEDUSA_WFS",
        "MEDUSA_WO",
        "MEDUSA_WQ",
        "MEDUSA_MISC",
        "MEDUSA_WENV",
        "MEDUSA_WQIN",
        "MEDUSA_WCLK",
        "MEDUSA_CALLSTAT",
        "MEDUSA_AOTPC",
        "MEDUSA_CZTR",
        "MEDUSA_DIRTY",
    ];

    /// 单开关读取：一次原子载入，未初始化时走冷路径解析环境变量。
    #[inline(always)]
    pub fn on(bit: u64) -> bool {
        all() & bit != 0
    }
    /// 一次原子读取多个开关，热路径合并判断。
    #[inline(always)]
    pub fn mask(bits: u64) -> u64 {
        all() & bits
    }
    #[inline(always)]
    fn all() -> u64 {
        let b = BITS.load(std::sync::atomic::Ordering::Relaxed);
        if b != u64::MAX {
            return b;
        }
        init()
    }
    #[cold]
    fn init() -> u64 {
        let mut v = 0u64;
        for (i, n) in NAMES.iter().enumerate() {
            if std::env::var(n).is_ok() {
                v |= 1 << i;
            }
        }
        BITS.store(v, std::sync::atomic::Ordering::Relaxed);
        v
    }
}

pub static CASEID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `run_aot` 每轮都要看的进程级开关快照。
struct HotCfg {
    rdx: bool,
    at090: bool,
    sh: bool,
    always_engine: bool,
    nosent: bool,
    dbg: bool,
    aotpc: bool,
    callstat: bool,
    eng: bool,
}

pub fn tra() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("TRA").is_ok())
}

/// `MEDUSA_ALLENG`：每字都交回引擎，供 AOT 对拍。
#[inline(always)]
pub fn alleng() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("MEDUSA_ALLENG").is_some())
}

/// `DBGCLK`：打印时钟路径，热路径不能每步 getenv。
#[inline(always)]
pub fn dbgclk() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("DBGCLK").is_some())
}

/// `RPC`/`RPC2`/`RPC3`：重放期的定点打印。
#[inline(always)]
pub fn rpc(which: u8) -> bool {
    static V: [std::sync::OnceLock<bool>; 3] = [const { std::sync::OnceLock::new() }; 3];
    *V[which as usize].get_or_init(|| {
        std::env::var_os(match which {
            0 => "RPC",
            1 => "RPC2",
            _ => "RPC3",
        })
        .is_some()
    })
}

/// `TRACE2`：逐字打印 AOT 轨迹；在 AOT 主循环里调用，必须缓存。
#[inline(always)]
pub fn trace2() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("TRACE2").is_some())
}

static SEMTR: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(2);

#[inline(always)]
pub fn semtr() -> bool {
    match SEMTR.load(std::sync::atomic::Ordering::Relaxed) {
        0 => false,
        1 => true,
        _ => {
            let v = std::env::var("SEMTR").is_ok();
            SEMTR.store(v as u8, std::sync::atomic::Ordering::Relaxed);
            v
        }
    }
}

pub const NATIVE_CALL: u64 = 0x1017_a1b4;
pub const NATIVE_EXIT: u64 = 0x1015_c92c;
/// 引擎的默认原生回调蹦床：`mov x2,x0; mov x0,x1; br x2`，以槽 4 为函数、槽 5 为参数。
pub const DEFAULT_CALL: u64 = 0x1017_f264;
/// 真机首取指时的 sp。原生回调与 VM 引擎共用同一个栈，且回调会经 [sp+0x38]
/// 取当前的槽基址，所以必须沿用真机栈而不是另开一段。
pub const VM_STACK: u64 = 0x701e_c8c0;

fn stop_limit(limit: u64) -> u64 {
    match std::env::var("MEDUSA_STOP")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
    {
        Some(n) if n < limit => n,
        _ => limit,
    }
}
/// 真机首取指时的槽数组基址（槽 n 在 base+8+8n）。槽必须落在真机内存地址上：
/// 原生回调直接按绝对地址读写槽，不走 Rust 侧的数组。
pub const VM_SLOTS: u64 = 0x701e_dc08;
pub static WATCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 内置夹具：32.2.6 库与 case1 入口快照（压缩存，进程内只解压一次）。
static LIB_GZ: &[u8] = include_bytes!("../../resources/libmetasec_ml.so.gz");
static SNAP_GZ: &[u8] = include_bytes!("../../resources/medusa-snap.bin.gz");
/// 静态初态：快照裁掉库页与池内零页后的运行时页，格式与快照相同。
static INIT_GZ: &[u8] = include_bytes!("../../resources/medusa-initial.bin.gz");
const LIB_HI: u64 = 0x1060_0000;

fn gunzip(gz: &'static [u8]) -> Vec<u8> {
    use std::io::Read;
    let mut d = flate2::read::GzDecoder::new(gz);
    let mut out = Vec::new();
    d.read_to_end(&mut out).expect("内置资源解压失败");
    out
}

pub fn snapshot_bytes() -> &'static [u8] {
    static SNAP: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    SNAP.get_or_init(|| gunzip(SNAP_GZ))
}

fn initial_bytes() -> &'static [u8] {
    static INIT: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    INIT.get_or_init(|| gunzip(INIT_GZ))
}

/// 快照里 query 输入的 `u16` 长度与指针槽。
pub const QUERY_LEN: u64 = 0x701e_df74;
pub const QUERY_PTR: u64 = 0x701e_df78;
/// VM 输出缓冲。长度随输入/时钟浮动，运行时从串对象抓取。
pub const OUT_ADDR: u64 = 0x503c_fc00;
/// 注入 query 的缓冲：快照里 case1 的原位置，后续 0x508xx 页连续可供增长。
const SCRATCH: u64 = 0x5080_a000;
/// `x-ss-stub` 串对象：+0x0c 是 u32 长度，+0x10 是数据指针，缓冲容量 0x20。
const STUB_OBJ: u64 = 0x503f_f4e0;
const STUB_LEN: u64 = STUB_OBJ + 0x0c;
const STUB_PTR: u64 = STUB_OBJ + 0x10;
const STUB_BUF: u64 = 0x503f_f520;
/// 长度槽是 u16；更长的 query 由调用方回退常量。
pub const MAX_QUERY: usize = 0xffff;
/// 输出串对象的长度字段（对象 +12）；对象在 END 前会被回收，故运行中抓取。
pub use native::Rand as Urng;

/// 纯签名输入里的 stub；缺失与空值都按 16 字节全零注入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stub {
    Missing,
    Empty,
    Hex(Vec<u8>),
}

static PCSET: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<u64>>> =
    std::sync::OnceLock::new();

impl Drop for Vm {
    fn drop(&mut self) {
        let Some(p) = std::env::var_os("MEDUSA_PCSET_OUT") else {
            return;
        };
        let Some(set) = PCSET.get() else { return };
        let s = set.lock().unwrap();
        let mut v: Vec<u64> = s.iter().copied().collect();
        v.sort_unstable();
        let mut out = String::new();
        for pc in v {
            out.push_str(&format!("{pc:x}\n"));
        }
        let _ = std::fs::write(p, out);
    }
}

/// 构造时的初始标量态，供复用回滚。
struct Pr {
    slots: [u64; 64],
    regs: [u64; 32],
    pc: u64,
    sp: u64,
    slots_addr: u64,
}

pub struct Vm {
    pub mem: Mem,
    pub nat: native::Native,
    pub slots: [u64; 64],
    pub regs: [u64; 32],
    pub pc: u64,
    pub sp: u64,
    pub slots_addr: u64,
    pub steps: u64,
    last_ret: bool,
    ret_pc: u64,
    pub slot_dirty: u64,
    slot_touched: u64,
    pending: Option<u32>,
    /// 当前 mode 2 的延迟槽是一次调用（目标出程序区时要按回调绑定参数）。
    pub jump_call: bool,
    /// 由生成代码写入：1=调用字，2=jmpi，0=其它。
    pub jump_flag: u8,
    /// `MEDUSA_AT090`：`st` 里那条逐步打印开关，构造时取一次。
    at090: bool,
    pr: Pr,
}

impl Vm {
    pub fn new(mut mem: Mem, slots: [u64; 64], pc: u64) -> Self {
        // 引擎在 0x10054330.. 调用回调前使用的帧字段（sp=VM_STACK 上方）：
        // 0x20 默认处理地址、0x28 退出哨兵、0x38 调用者槽基址。
        let _ = mem.w64(VM_STACK + 0x20, NATIVE_CALL);
        let _ = mem.w64(VM_STACK + 0x28, NATIVE_EXIT);
        let _ = mem.w64(VM_STACK + 0x38, VM_SLOTS);
        // 入口寄存器文件取自真机首取指那一行（full4.log），解码块会读其中多数。
        let mut regs = [0u64; 32];
        regs[1] = 0x701e_dd50;
        regs[2] = 0x1032_e4b0;
        regs[3] = 0x1032_e670;
        regs[4] = 0x701e_c9e0;
        regs[5] = 0x7;
        regs[6] = VM_SLOTS;
        regs[7] = 0x701f_1000;
        regs[8] = 0x800;
        regs[9] = NATIVE_EXIT;
        regs[10] = NATIVE_CALL;
        regs[11] = 0x701e_dc30;
        regs[12] = 0xe1e;
        regs[13] = 0x700;
        regs[14] = 0x1017_679c;
        regs[15] = 0x1017_672c;
        regs[16] = 0xba67_af91;
        regs[17] = 0x1;
        regs[18] = 0x5008_2020;
        regs[19] = 0x701f_1000;
        regs[20] = 0x701e_e2b9;
        regs[21] = 0x1034_86b0;
        regs[22] = 0;
        regs[23] = 0x503f_f600;
        regs[24] = 0x701e_e340;
        regs[25] = 0x1026_f8b8;
        regs[26] = 0x19a_5143f700;
        regs[27] = VM_SLOTS + 0x140;
        regs[28] = 0x1015_9140;
        regs[29] = VM_STACK + 0x110;
        regs[30] = 0x1017_6818;
        let pr = Pr {
            slots,
            regs,
            pc,
            sp: VM_STACK,
            slots_addr: VM_SLOTS,
        };
        Vm {
            mem,
            nat: native::Native::new(),
            slots,
            regs,
            pc,
            sp: VM_STACK,
            slots_addr: VM_SLOTS,
            steps: 0,
            last_ret: false,
            ret_pc: 0,
            slot_dirty: 0,
            slot_touched: 0,
            pending: None,
            jump_call: false,
            jump_flag: 0,
            at090: flags::at090(),
            pr,
        }
    }

    /// 复用前把内存与寄存器恢复到构造态。
    pub fn rewind(&mut self) {
        if std::env::var_os("MEDUSA_OPCLEAR").is_some() {
            native::clear_ops();
        }
        self.mem.reset_dirty();
        let _ = self.mem.w64(VM_STACK + 0x20, NATIVE_CALL);
        let _ = self.mem.w64(VM_STACK + 0x28, NATIVE_EXIT);
        let _ = self.mem.w64(VM_STACK + 0x38, VM_SLOTS);
        self.slots = self.pr.slots;
        self.regs = self.pr.regs;
        self.pc = self.pr.pc;
        self.sp = self.pr.sp;
        self.slots_addr = self.pr.slots_addr;
        self.steps = 0;
        self.last_ret = false;
        self.ret_pc = 0;
        self.slot_dirty = 0;
        self.slot_touched = 0;
        self.pending = None;
        self.jump_call = false;
        self.jump_flag = 0;
        self.nat = native::Native::new();
    }

    /// 记下当前字，等真正要进引擎或原生回调时再做寄存器预置。
    /// AOT 字本身不读这些寄存器，逐字做预置只是白写。
    #[inline(always)]
    pub fn decode(&mut self, w: u32) {
        self.pending = Some(w);
    }

    #[inline]
    pub fn dbg_materialize(&mut self) {
        self.materialize()
    }

    fn materialize(&mut self) {
        if let Some(w) = self.pending.take() {
            self.decode_now(w);
        }
    }

    /// 真机 0x1004e710 起、分发前的寄存器预置：把指令字的各字段切进 x0..x28，
    /// 并写进栈上 handler 要读的偏移。原生回调入口依赖这些值。
    fn decode_now(&mut self, w: u32) {
        let w = w as u64;
        let r = &mut self.regs;
        r[0] = w << 7;
        r[1] = w << 3;
        r[3] = w >> 30;
        r[4] = (w << 9) & 0xffff_ffff;
        r[7] = w >> 29;
        r[8] = w;
        r[9] = w << 2;
        r[10] = w << 1;
        r[11] = w << 6;
        r[12] = w >> 27;
        r[13] = w >> 12;
        r[14] = w << 4;
        r[15] = w & 0x3f;
        r[16] = w << 5;
        r[17] = w << 8;
        r[19] = w >> 2;
        r[20] = w >> 3;
        r[21] = w >> 4;
        r[22] = w >> 23;
        r[23] = w >> 1;
        r[24] = w >> 6;
        r[25] = w >> 5;
        r[26] = w >> 10;
        r[28] = w >> 7;
        r[30] = w >> 19;
        r[6] = self.slots_addr;
        r[27] = self.slots_addr + 0x140;
        // 0x1004e9c4：op>=0x1f 时把 w26（w8>>10）写进 [sp+0x1c]，call handler 从这里读参数。
        if (w as u32 & 0x3f) >= 0x1f {
            let _ = self.mem.st(self.sp + 0x1c, 4, w >> 10);
        }
        for (off, sh) in [
            (0x34usize, 27u32),
            (0x40, 28),
            (0x44, 25),
            (0x48, 15),
            (0x4c, 22),
            (0x50, 8),
            (0x54, 26),
            (0x58, 11),
            (0x5c, 17),
            (0x60, 21),
            (0x64, 24),
            (0x68, 20),
            (0x6c, 14),
            (0x70, 16),
            (0x74, 18),
            (0x78, 9),
            (0x7c, 13),
        ] {
            let _ = self.mem.st(self.sp + off as u64, 4, w >> sh);
        }
    }

    fn flush_slots(&mut self) -> Result<(), Error> {
        self.write_back_slots(u64::MAX)?;
        self.slot_dirty = 0;
        self.slot_touched = 0;
        Ok(())
    }

    /// 进 native 前同步槽块：先读回被内存写覆盖的槽，再只写回脏槽。
    #[inline(always)]
    fn sync_slots(&mut self) -> Result<(), Error> {
        self.merge_slots()?;
        let d = self.slot_dirty;
        self.slot_dirty = 0;
        if d != 0 {
            self.write_back_slots(d)?;
        }
        Ok(())
    }

    /// 把 `dirty` 选中的槽写回槽块：整块取一次址，页脏位按首末页各标一次。
    /// 槽块固定 512B，逐槽走 `mem.st` 会把同一页的脏位判断重复 64 遍。
    #[inline]
    fn write_back_slots(&mut self, dirty: u64) -> Result<(), Error> {
        let base = self.slots_addr + 8;
        let p = self.mem.span(base, 64 * 8).ok_or(Error::Unmapped(base))? as *mut u8;
        let last = (base + 64 * 8 - 1) & !(PAGE - 1);
        let mut page = base & !(PAGE - 1);
        loop {
            self.mem.mark_dirty(page);
            if page >= last {
                break;
            }
            page += PAGE;
        }
        let mut m = dirty;
        while m != 0 {
            let n = m.trailing_zeros() as usize;
            m &= m - 1;
            unsafe {
                std::ptr::write_unaligned(p.add(n * 8) as *mut u64, self.slots[n].to_le());
            }
        }
        Ok(())
    }

    fn reload_slots(&mut self) -> Result<(), Error> {
        let base = self.slots_addr + 8;
        let p = self.mem.span(base, 64 * 8).ok_or(Error::Unmapped(base))? as *const u8;
        for (n, slot) in self.slots.iter_mut().enumerate().take(64) {
            *slot = u64::from_le(unsafe { std::ptr::read_unaligned(p.add(n * 8) as *const u64) });
        }
        self.slot_dirty = 0;
        self.slot_touched = 0;
        Ok(())
    }

    /// 槽块是内存视图：AOT 字可能经指针写回槽区，也可能直接 `st()` 写缓存。
    /// 前者要读回缓存，后者要写回内存。
    ///
    /// 写回不逐字做：`slot_dirty` 在连续 AOT 字之间累计，只在真正会读槽块的
    /// native 边界由 `flush_slots` 整块写出；这里只处理必须立刻可见的反向读回。
    #[inline(always)]
    fn merge_slots(&mut self) -> Result<(), Error> {
        let touched = self.slot_touched;
        if touched == 0 {
            return Ok(());
        }
        // 只回读被 AOT 内存写命中的槽；其余缓存值保持不动。
        let base = self.slots_addr + 8;
        let p = self.mem.span(base, 64 * 8).ok_or(Error::Unmapped(base))? as *const u64;
        let mut t = touched;
        while t != 0 {
            let n = t.trailing_zeros() as usize;
            t &= t - 1;
            self.slots[n] = u64::from_le(unsafe { std::ptr::read_unaligned(p.add(n)) });
        }
        // 被内存写覆盖过的槽不再需要用缓存值回写（缓存已是内存值）。
        self.slot_touched = 0;
        self.slot_dirty &= !touched;
        Ok(())
    }

    /// 直接写槽并标脏：生成的代码对 31/35/36/37 等记账槽也用这个，保证 merge_slots 不把它们读回旧值。
    #[inline]
    pub fn wr(&mut self, d: i16, v: u64) {
        if d >= 0 {
            if d as usize >= 64 {
                eprintln!("ST OOB d={} pc={:#x}", d, self.pc - 4);
                return;
            }
            self.slots[d as usize] = v;
            self.slot_dirty |= 1 << (d as u32);
        }
    }

    /// AOT 直接写内存后，若落在槽块内就标记该槽，下一步前只回读这些槽。
    #[inline]
    fn note_slot_write(&mut self, a: u64, n: usize) {
        let base = self.slots_addr + 8;
        let lo = (a & 0xffff_ffff).max(base);
        let hi = ((a & 0xffff_ffff) + n as u64).min(base + 64 * 8);
        if lo >= hi {
            return;
        }
        let first = ((lo - base) / 8) as u32;
        let last = ((hi - 1 - base) / 8) as u32;
        self.slot_touched |= (!0u64 << first) & (!0u64 >> (63 - last));
    }

    #[inline]
    pub fn sw64(&mut self, a: u64, v: u64) -> Result<(), Error> {
        self.mem.w64(a, v)?;
        self.note_slot_write(a, 8);
        Ok(())
    }

    #[inline]
    pub fn mwrite(&mut self, a: u64, data: &[u8]) -> Result<(), Error> {
        self.mem.write(a, data)?;
        self.note_slot_write(a, data.len());
        Ok(())
    }

    #[inline(always)]
    fn st(&mut self, d: i16, v: u64) {
        if self.at090 && self.steps == 25388 {
            eprintln!(
                "ST090 d={} v={:#x} s3={:#x} pc={:#x}",
                d, v, self.slots[3], self.pc
            );
        }
        self.wr(d, v)
    }
    /// 真机 0x100544a8..0x100544d4 的分发：pc 推进后按 mode 槽 35 选路。
    /// mode 3/1 跳到槽 36；mode 2 表示下一字是延迟槽（执行期间 mode 置 3）。
    fn transition(&mut self) {
        match self.slots[35] {
            1 | 3 => {
                self.slots[35] = 0;
                // 真机在转向前把延迟槽之后的地址写进 [x27-0x10]（0x10055184 起），
                // 原生回调返回时据此续跑（0x10054340）；x27 = 槽基址 + 0x140。
                self.ret_pc = self.pc;
                let frame = self.slots_addr + 0x140;
                let _ = self.mem.w64(frame - 0x20, 0);
                let _ = self.mem.w64(frame - 0x18, self.slots[36]);
                let _ = self.mem.w64(frame - 0x10, self.ret_pc);
                let _ = self.mem.w64(self.slots_addr, self.slots[36]);
                self.pc = self.slots[36];
                self.last_ret = !(PROG_LO..PROG_HI).contains(&self.pc)
                    && self.pc != NATIVE_CALL
                    && self.pc != DEFAULT_CALL;
            }
            2 => self.slots[35] = 3,
            _ => {}
        }
    }
    /// AOT 版的 mode 收尾：2 不在这里推进，由下一轮取指前转 3（与 0x1004e75c 一致）。
    #[inline(always)]
    fn transition_aot(&mut self) {
        match self.slots[35] {
            1 | 3 => {
                self.slots[35] = 0;
                self.ret_pc = self.pc;
                let frame = self.slots_addr + 0x140;
                let _ = self.mem.w64(frame - 0x20, 0);
                let _ = self.mem.w64(frame - 0x18, self.slots[36]);
                let _ = self.mem.w64(frame - 0x10, self.ret_pc);
                let _ = self.mem.w64(self.slots_addr, self.slots[36]);
                self.pc = self.slots[36];
                self.last_ret = !(PROG_LO..PROG_HI).contains(&self.pc)
                    && self.pc != NATIVE_CALL
                    && self.pc != DEFAULT_CALL;
            }
            _ => {}
        }
    }

    #[inline(always)]
    fn ld(&self, s: i16) -> u64 {
        if s < 0 {
            return 0;
        }
        if s as usize >= 64 {
            eprintln!("LD OOB s={} pc={:#x}", s, self.pc - 4);
            return 0;
        }
        self.slots[s as usize]
    }

    /// 执行一条指令；返回是否继续（false 表示程序结束）。
    pub fn step(&mut self) -> Result<bool, Error> {
        if flags::pcset() {
            PCSET
                .get_or_init(Default::default)
                .lock()
                .unwrap()
                .insert(self.pc);
        }
        if self.pc == 0x102cd11c && flags::rdx() {
            eprintln!(
                "RDXDUMP rep {:02x?}",
                self.mem.read(0x503f_f760, 0x30).unwrap_or_default()
            );
        }
        if flags::v6() && self.pc == 0x10278a54 {
            eprintln!("AT54 kind={:?}", words::find(self.pc).map(|i| i.kind));
            eprintln!("AT54b");
        }
        if flags::v6() && matches!(self.pc, 0x102cd1cc | 0x102cd1d4 | 0x102cd1dc | 0x102cd1a0) {
            eprintln!(
                "RSL pc={:#x} {:x?} base={:#x} m={} s36={:#x} mem={:x?} regs={:x?} sp={:#x}",
                self.pc,
                &self.slots[..],
                self.slots_addr,
                self.slots[35],
                self.slots[36],
                self.mem.read(0x701edb00, 0x80).unwrap_or_default(),
                &self.regs[..],
                self.sp
            );
        }
        if probe::on(probe::PC) {
            eprintln!("PC {:#x} {:#x}", self.pc, self.sp);
            if probe::on(probe::DBG) {
                eprintln!(
                    "RMEM {:#x} {:x?}",
                    0x701e_c740,
                    self.mem.read(0x701e_c740, 0x1c0).unwrap_or_default()
                );
            }
        }
        CURPC.store(self.pc, std::sync::atomic::Ordering::Relaxed);
        GSTEP.store(self.steps, std::sync::atomic::Ordering::Relaxed);
        CURSLOTS.store(self.slots_addr, std::sync::atomic::Ordering::Relaxed);
        if probe::on(probe::RQ) {
            let mv = self.mem.ld(0x701e_d278, 8).unwrap_or(0);
            let ov = LAST4.swap(mv, std::sync::atomic::Ordering::Relaxed);
            if mv != ov {
                eprintln!(
                    "MV {} {:#x} <- {:#x} pc={:#x} s29={:#x}",
                    self.steps, ov, mv, self.pc, self.slots[29]
                );
            }
        }
        if self.pc == 0x1027b3bc && rpc(2) {
            eprintln!("R3 step={} sa={:#x} s35={} s36={:#x} s37={:#x} s0={:#x} s4={:#x} s17={:#x} x0={:#x}", self.steps, self.slots_addr, self.slots[35], self.slots[36], self.slots[37], self.slots[0], self.slots[4], self.slots[17], self.regs[0]);
        }
        if self.pc == 0x1027b3e0 && rpc(1) {
            eprintln!("R2 step={} sp={:#x} x0={:#x} x1={:#x} x2={:#x} x8={:#x} x9={:#x} x17={:#x} s35={} s36={:#x} s37={:#x} s0={:#x} s4={:#x} s17={:#x} sa={:#x}", self.steps, self.sp, self.regs[0], self.regs[1], self.regs[2], self.regs[8], self.regs[9], self.regs[17], self.slots[35], self.slots[36], self.slots[37], self.slots[0], self.slots[4], self.slots[17], self.slots_addr);
        }
        if self.pc == 0x1027b00c && rpc(0) {
            eprintln!(
                "RP step={} x8={:#x} x9={:#x} x17={:#x} x0={:#x} slots16={:#x} s37={:#x}",
                self.steps,
                self.regs[8],
                self.regs[9],
                self.regs[17],
                self.regs[0],
                self.slots[16],
                self.slots[37]
            );
        }
        if probe::on(probe::RQ) && self.steps.is_multiple_of(500) && self.steps < 48000 {
            eprintln!(
                "CK {} {:x}",
                self.steps,
                self.mem.ld(0x701e_d278, 8).unwrap_or(0)
            );
        }
        if probe::on(probe::RQ) && self.pc == 0x1027ad58 {
            let a = self.slots[29] & 0xffff_ffff;
            eprintln!(
                "A58 s29={:#x} m278={:#x} m274={:#x}",
                a,
                self.mem.ld(a + 1800, 8).unwrap_or(0),
                self.mem.ld(a + 1796, 8).unwrap_or(0)
            );
        }
        if probe::on(probe::RQ) && self.pc == 0x1027ad78 && self.steps == 61079 {
            eprintln!("SLOTS sp={:#x} {:x?}", self.sp, &self.slots[..44]);
        }
        if flags::at090() && self.pc == 0x1028d090 {
            eprintln!(
                "AT090 kind={:?} s3={:#x} steps={}",
                words::find(self.pc).map(|i| i.kind),
                self.slots[3],
                self.steps
            );
        }
        if flags::at090() && self.steps == 25492 {
            eprintln!(
                "MEMD {:02x?}",
                self.mem.read(0x701ec340, 0x40).unwrap_or_default()
            );
        }
        if flags::v6() && self.pc == 0x10278a54 {
            eprintln!("AT54c");
        }
        let _p2 = LAST2.swap(self.slots[2], std::sync::atomic::Ordering::Relaxed);
        let _p3 = LAST3.swap(self.slots[3], std::sync::atomic::Ordering::Relaxed);
        if probe::on(probe::SL2)
            && self.steps > 40000
            && self.slots[2] != _p2
            && self.slots[2] > 0x10000
        {
            eprintln!(
                "SL2 step={} pc={:#x} s2={:#x} s3={:#x}",
                self.steps, self.pc, self.slots[2], self.slots[3]
            );
        }
        if probe::on(probe::SL2)
            && self.steps > 40000
            && self.slots[3] != _p3
            && self.slots[3] > 0x10000
        {
            eprintln!(
                "SL3 pc={:#x} s2={:#x} s3={:#x}",
                self.pc, self.slots[2], self.slots[3]
            );
        }
        if probe::on(probe::SLH) {
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            let mut mix = |v: u64| {
                for b in v.to_le_bytes() {
                    h ^= b as u64;
                    h = h.wrapping_mul(0x100_0000_01b3);
                }
            };
            for v in self.slots.iter().take(64) {
                mix(*v);
            }
            mix(self.pc);
            mix(self.sp);
            mix(self.slots_addr);
            eprintln!("SLH st={} {:#x} {:#x}", self.steps, self.pc, h);
            if self.pc >= PROG_LO && self.pc < PROG_HI && (self.steps < 4000 || flags::sdall()) {
                eprintln!(
                    "SD {} {} {:#x}",
                    self.steps,
                    self.slots
                        .iter()
                        .map(|v| format!("{:#x}", v))
                        .collect::<Vec<_>>()
                        .join(" "),
                    self.slots_addr
                );
            }
        }
        if flags::v6() && self.pc == 0x10278a54 {
            eprintln!("AT54d");
        }
        if flags::at090() && self.steps == 25387 {
            eprintln!("P1 pc={:#x}", self.pc);
        }
        if !flags::oldeng() {
            if self.pc == native::END {
                if flags::v6() && self.pc == 0x10278a54 {
                    eprintln!("ENDCHK54");
                }
                return Ok(false);
            }
            if flags::v6()
                && matches!(
                    self.pc,
                    0x102cd190
                        | 0x102cd194
                        | 0x102cd198
                        | 0x102cd19c
                        | 0x102cd1a0
                        | 0x102cd1a4
                        | 0x102cd1a8
                        | 0x102cd1ac
                )
            {
                eprintln!(
                    "V6 step={} pc={:#x} base={:#x} s5={:#x} s6={:#x} mem6={:#x}",
                    self.steps,
                    self.pc,
                    self.slots_addr,
                    self.slots[5],
                    self.slots[6],
                    self.mem.ld(self.slots_addr + 8 + 48, 8).unwrap_or(0)
                );
            }
            if probe::on(probe::SB) && self.steps == 484 {
                for a in (0x701ec600..0x701ec700).step_by(16) {
                    eprintln!(
                        "SB {:#x} {:02x?}",
                        a,
                        self.mem.read(a, 16).unwrap_or_default()
                    );
                }
                for a in (0x701ed090..0x701ed0e0).step_by(16) {
                    eprintln!(
                        "SB {:#x} {:02x?}",
                        a,
                        self.mem.read(a, 16).unwrap_or_default()
                    );
                }
            }
            if probe::on(probe::MISC) && self.pc == 0x102863d8 {
                let a = self.slots[4].wrapping_add(16);
                eprintln!(
                    "D8N step={} s4={:#x} a={:#x} v={:#x}",
                    self.steps,
                    self.slots[4],
                    a,
                    self.mem.ld(a, 2).unwrap_or(0)
                );
            }
            if probe::on(probe::MISC) && self.pc == 0x10286390 {
                eprintln!(
                    "D90 step={} s4={:#x} s5={:#x} s17={:#x}",
                    self.steps, self.slots[4], self.slots[5], self.slots[17]
                );
            }
            if probe::on(probe::MISC)
                && matches!(self.pc, 0x102869f0 | 0x102869f4 | 0x102869fc | 0x10286a00)
            {
                eprintln!(
                    "CZP pc={:#x} step={} s0={:#x} s1={:#x} s17={:#x} base={:#x}",
                    self.pc,
                    self.steps,
                    self.slots[0],
                    self.slots[1],
                    self.slots[17],
                    self.slots_addr
                );
            }
            if probe::on(probe::ST) {
                eprintln!(
                    "ST {} {:#x} {:#x} {:#x} {:#x}",
                    self.steps,
                    self.pc.wrapping_sub(4),
                    self.slots[1],
                    self.slots[17],
                    self.slots_addr
                );
            }
            if probe::on(probe::SM) {
                eprintln!(
                    "SM {} {}",
                    self.steps,
                    (0..48)
                        .map(|i| format!(
                            "{:#x}",
                            self.mem.ld(self.slots_addr + 8 + 8 * i, 8).unwrap_or(0)
                        ))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            if probe::on(probe::STA) {
                eprintln!(
                    "SA {} {:#x} s29={:#x} m29={:#x} {}",
                    self.steps,
                    self.slots_addr,
                    self.slots[29],
                    self.mem.ld(self.slots_addr + 8 + 8 * 29, 8).unwrap_or(0),
                    self.slots[..48]
                        .iter()
                        .map(|v| format!("{:#x}", v))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            let _ = self.mem.w64(self.slots_addr, self.pc);
            self.flush_slots()?;
            self.regs[6] = self.slots_addr;
            self.regs[27] = self.slots_addr + 0x140;
            self.regs[0] = self.pc;
            if probe::on(probe::RG) {
                eprintln!(
                    "RG {} {:#x} sp={:#x} sa={:#x} m420={:02x?} {}",
                    self.steps,
                    self.pc,
                    self.sp,
                    self.slots_addr,
                    self.mem.read(0x701ed3e0, 0xc0).unwrap_or_default(),
                    self.regs
                        .iter()
                        .map(|v| format!("{:#x}", v))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            self.steps += 1;
            self.materialize();
            let r = self.nat.call(
                &mut self.mem,
                &self.slots,
                if self.steps == 1 {
                    0x1004e6e0
                } else {
                    0x1004e710
                },
                &mut self.sp,
                &mut self.regs,
                false,
                true,
            )?;
            if let Some(re) = &r {
                if probe::on(probe::RE) {
                    eprintln!(
                        "REBACK step={} pc={:#x} slots={:#x}",
                        self.steps, re.pc, re.slots_addr
                    );
                }
            }
            match r {
                Some(re) => {
                    if re.pc == native::END {
                        self.pc = native::END;
                        return Ok(true);
                    }
                    self.slots_addr = re.slots_addr;
                    self.reload_slots()?;
                    self.pc = re.pc;
                }
                None => {
                    self.reload_slots()?;
                    self.pc = self.slots[37];
                }
            }
            if probe::on(probe::RE) {
                eprintln!("REEND step={} pc={:#x}", self.steps, self.pc);
            }
            return Ok(true);
        }
        if self.pc == 0x10275c8c {
            eprintln!(
                "SLOTSC8C {}",
                self.slots
                    .iter()
                    .map(|v| format!("{:#x}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        if self.pc == 0x10275e74 {
            eprintln!(
                "SLOTS74 {} OBJ {}",
                self.slots
                    .iter()
                    .map(|v| format!("{:#x}", v))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.mem
                    .read(0x701ed2a8, 0x18)
                    .map(|v| v.iter().map(|b| format!("{:02x}", b)).collect::<String>())
                    .unwrap_or_default()
            );
        }
        if self.pc == 0x10275e3c {
            let a = self.slots[29].wrapping_add(3848);
            eprintln!(
                "E3C s29={:#x} a={:#x} v={:#x}",
                self.slots[29],
                a,
                self.mem.ld(a, 4).unwrap_or(0)
            );
        }
        if self.pc == 0x10275e50 {
            eprintln!(
                "E50 s1={:#x} s29={:#x} tgt={:#x} cur={:#x}",
                self.slots[1],
                self.slots[29],
                self.slots[29].wrapping_add(3816),
                self.mem
                    .ld(self.slots[29].wrapping_add(3816), 4)
                    .unwrap_or(0)
            );
        }
        if self.pc == 0x102863d8 {
            let a = self.slots[4].wrapping_add(16);
            eprintln!(
                "D8 s4={:#x} a={:#x} v={:#x} r={:#x}",
                self.slots[4],
                a,
                self.mem.ld(a, 4).unwrap_or(0),
                self.mem.ld(0x701eda40, 8).unwrap_or(0)
            );
        }
        if matches!(self.pc, 0x102869f0 | 0x102869f4 | 0x102869fc | 0x10286a00) {
            eprintln!(
                "CZP pc={:#x} step={} s0={:#x} s1={:#x} s17={:#x} base={:#x}",
                self.pc, self.steps, self.slots[0], self.slots[1], self.slots[17], self.slots_addr
            );
        }
        if self.pc == 0x102866ec {
            eprintln!("SLT17 s17={:#x}", self.slots[17]);
            eprintln!(
                "SLOTSEC {}",
                self.slots
                    .iter()
                    .map(|v| format!("{:#x}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        if self.pc == 0x10286728 {
            eprintln!(
                "CZ s0={:#x} s1={:#x} s35={} s36={:#x} base={:#x}",
                self.slots[0], self.slots[1], self.slots[35], self.slots[36], self.slots_addr
            );
        }
        if self.pc == 0x1028672c {
            eprintln!(
                "DS s2={:#x} s29={:#x} s35={} s36={:#x} mem={:x?}",
                self.slots[2],
                self.slots[29],
                self.slots[35],
                self.slots[36],
                self.mem.read(self.slots[29] + 0x260, 8).unwrap_or_default()
            );
        }
        if (0x10286650..=0x1028665c).contains(&self.pc) {
            eprintln!(
                "DBG pc={:#x} s35={} s36={:#x} s31={:#x} s37={:#x}",
                self.pc, self.slots[35], self.slots[36], self.slots[31], self.slots[37]
            );
        }
        if matches!(self.pc, 0x102cd1cc | 0x102cd1d4 | 0x102cd1dc | 0x102cd1a0) {
            eprintln!("CALLZ s0..8 = {:x?}", &self.slots[..9]);
        }
        if flags::at090() && self.steps == 25387 {
            eprintln!("P2 pc={:#x}", self.pc);
        }
        if self.pc == NATIVE_CALL || self.pc == DEFAULT_CALL {
            if flags::at090() && self.steps == 25387 {
                eprintln!("NATCALL pc={:#x}", self.pc);
            }
            if probe::on(probe::NATDBG) {
                eprintln!(
                    "PRE s38={:#x} s20={:#x} s28={:#x}",
                    self.mem.ld(self.sp + 0x38, 8).unwrap_or(0),
                    self.mem.ld(self.sp + 0x20, 8).unwrap_or(0),
                    self.mem.ld(self.sp + 0x28, 8).unwrap_or(0)
                );
            }
            if probe::on(probe::DBG) {
                eprintln!(
                    "NATIVE step={} s4={:#x} s5={:#x} s17={:#x} s37={:#x} s20={:#x} [s20]={:#x}",
                    self.steps,
                    self.slots[4],
                    self.slots[5],
                    self.slots[17],
                    self.slots[37],
                    self.slots[20],
                    self.mem.ld(self.slots[20], 8).unwrap_or(0)
                );
                eprintln!(
                    "  s29={:#x} s2={:#x} [0x701edb90]={:#x}",
                    self.slots[29],
                    self.slots[2],
                    self.mem.ld(0x701edb90, 8).unwrap_or(0)
                );
                if self.slots[4] == 0x1017a788 {
                    eprintln!(
                        "  MEM68 {:x?}",
                        self.mem.read(0x701eda68, 0x38).unwrap_or_default()
                    );
                    eprintln!(
                        "  ARG {:x?}",
                        self.mem.read(0x701ed070, 0x60).unwrap_or_default()
                    );
                }
            }
            self.steps += 1;
            self.materialize();
            self.slots[35] = 0;
            let target = self.pc;
            let r = self.nat.call(
                &mut self.mem,
                &self.slots,
                target,
                &mut self.sp,
                &mut self.regs,
                true,
                false,
            )?;
            match r {
                // 回到解释器：槽基址取自真机当前的帧（x6），下一个 pc 取自 x0。
                Some(re) => {
                    eprintln!(
                        "REENTER step={} pc={:#x} slots={:#x} sp={:#x}",
                        self.steps, re.pc, re.slots_addr, self.sp
                    );
                    if re.slots_addr != self.slots_addr {
                        eprintln!(
                            "FRAME chg step={} {:#x} -> {:#x} pc={:#x} sp={:#x}",
                            self.steps, self.slots_addr, re.slots_addr, re.pc, self.sp
                        );
                    }
                    self.slots_addr = re.slots_addr;
                    self.reload_slots()?;
                    self.pc = re.pc;
                }
                // 真机 10054348：返回后从槽 37 继续（写的是 [x6] 里的 pc，槽 0 不动）。
                None => {
                    self.reload_slots()?;
                    self.pc = self.ret_pc;
                }
            }
            return Ok(true);
        }
        if self.pc == native::END {
            if self.pc == 0x10278a54 {
                eprintln!("ENDCHK54b");
            }
            return Ok(false);
        }
        if self.pc == 0x10278a54 {
            eprintln!(
                "AFTEREND lo={:#x} hi={:#x} prog={}",
                PROG_LO,
                PROG_HI,
                (PROG_LO..PROG_HI).contains(&self.pc)
            );
        }

        if self.pc < PROG_LO || self.pc >= PROG_HI {
            if flags::at090() && self.steps == 25387 {
                eprintln!("OOP pc={:#x}", self.pc);
            }
            // VM 之外：交回原生执行器接着跑（例如函数返回蹦床的后半段）。
            // VM 函数返回（jmpi 到调用者继续地址）要先走解释器收尾 0x1005538c，
            // 恢复 x19..x28 与调用者的 sp 后再继续。
            if self.pc == 0x10278a54 {
                eprintln!("NATPATH54");
            }
            self.steps += 1;
            self.materialize();
            let addr = if self.last_ret { 0x1005_538c } else { self.pc };
            let r = self.nat.call(
                &mut self.mem,
                &self.slots,
                addr,
                &mut self.sp,
                &mut self.regs,
                false,
                false,
            )?;
            match r {
                Some(re) => {
                    if re.pc == native::END {
                        self.pc = native::END;
                        return Ok(true);
                    }
                    self.slots_addr = re.slots_addr;
                    self.reload_slots()?;
                    self.pc = re.pc;
                }
                None => {
                    self.reload_slots()?;
                    self.pc = self.slots[37];
                }
            }
            return Ok(true);
        }
        if self.pc == 0x10278a54 {
            eprintln!("AT54parse");
        }
        self.last_ret = false;
        let Some(ins) = words::find(self.pc) else {
            return Err(Error::UnknownPc(self.pc));
        };
        let (d, s, imm) = (ins.d, ins.s, ins.imm);
        if (ins.d >= 0 && ins.d as usize >= 64) || (ins.s >= 0 && ins.s as usize >= 64) {
            eprintln!(
                "BADOP {:#x} d={} s={} imm={} kind={:?}",
                self.pc, d, s, imm, ins.kind
            );
        }
        if self.pc == 0x10278a54 {
            eprintln!(
                "ST64DBG d={} s={} imm={} kind={:?} w={:#x}",
                d, s, imm, ins.kind, ins.w
            );
        }
        if flags::at090() && (25380..=25395).contains(&self.steps) {
            eprintln!("POSTFIND pc={:#x} steps={}", self.pc, self.steps);
        }
        let next = self.pc.wrapping_add(4);
        // 引擎把当前 pc 放在槽块头部 [x6]（0x100544a8 起据此推进）。
        let _ = self.mem.w64(self.slots_addr, self.pc);
        self.decode(ins.w);
        self.materialize();
        if probe::on(probe::DBG) && self.steps > 240 {
            eprintln!("step {} pc={:#x} d={} s={} imm={} kind={:?} s0={:#x} s31={:#x} s35={} s36={:#x} s37={:#x}", self.steps, self.pc, d, s, imm, ins.kind, self.slots[0], self.slots[31], self.slots[35], self.slots[36], self.slots[37]);
        }
        if flags::at090() && self.steps == 25387 {
            eprintln!("POSTDEC pc={:#x}", self.pc);
        }
        self.pc = next;
        self.steps += 1;
        if flags::at090() && self.steps == 25388 {
            eprintln!(
                "PRE25388 pc={:#x} kind={:?} d={} s={} imm={} s3={:#x}",
                self.pc, ins.kind, d, s, imm, self.slots[3]
            );
        }
        let _old38 = self.mem.ld(self.sp + 0x38, 8).unwrap_or(0);
        let (_o0, _o1, _o17) = (self.slots[0], self.slots[1], self.slots[17]);
        match ins.kind {
            Kind::unknown | Kind::alu => return Err(Error::Unsupported(ins.kind, self.pc - 4)),
            Kind::sltu => {
                let v = (self.ld(s) < imm as u64) as u64;
                self.st(d, v)
            }
            Kind::slt => {
                let v = ((self.ld(s) as i64) < imm) as u64;
                self.st(d, v)
            }
            Kind::mov => {
                let v = self.ld(s) as u32 as u64;
                self.st(d, v)
            }
            // 条件复制：slot[s2] != 0 时 slot[d] = slot[s]（64 位）。
            Kind::cbz_mov => {
                if self.ld(ins.s2) != 0 {
                    let v = self.ld(s);
                    self.st(d, v);
                }
            }
            Kind::nop => {}
            // 立即与：slot[d] = slot[s] & imm（16 位掩码也按 64 位零扩展）。
            Kind::andi => {
                let v = self.ld(s) & imm as u64;
                self.st(d, v)
            }
            // 位域提取（32 位，结果符号扩展）：imm 低字节=移位，高位字节=位宽。
            Kind::bfx32 => {
                let sh = (imm & 0xff) as u32;
                let wd = (imm >> 8) as u32;
                let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
                let v = ((self.ld(s) as u32) >> sh) & mask;
                self.st(d, v as i32 as i64 as u64)
            }
            // 位域插入（32 位，实测 w=0x..e849）：dst = (src << 3) | (dst & 7)，按 32 位截断。
            Kind::bfi => {
                let src = self.ld(s) as u32;
                let dst = self.ld(d) as u32;
                let v = (src << 3) | (dst & 7);
                self.st(d, v as i32 as i64 as u64)
            }
            // 32 位字节序翻转，结果符号扩展。
            Kind::rev32 => {
                let x = self.ld(s) as u32;
                let v = ((x >> 8) & 0x00ff_00ff) | ((x << 8) & 0xff00_ff00);
                self.st(d, v as i32 as i64 as u64)
            }
            Kind::a_add => {
                let v = self.ld(ins.s2).wrapping_add(self.ld(s));
                self.st(d, v)
            }
            Kind::a_nor => {
                let v = !(self.ld(s) | self.ld(ins.s2));
                self.st(d, v)
            }
            Kind::a_shl64r => {
                let v = self.ld(s).wrapping_shl(self.ld(ins.s2) as u32);
                self.st(d, v)
            }
            Kind::a_shr64r => {
                let v = self.ld(s).wrapping_shr(self.ld(ins.s2) as u32);
                self.st(d, v)
            }
            Kind::a_shr32_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shr(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::mov64 => {
                let v = self.ld(s);
                self.st(d, v)
            }
            Kind::a_add32 => {
                let v = (self.ld(ins.s2) as u32).wrapping_add(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_and => {
                let v = self.ld(s) & self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::a_or => {
                let v = self.ld(s) | self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::a_xor => {
                let v = self.ld(s) ^ self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::a_sub32 => {
                let v = (self.ld(s) as u32).wrapping_sub(self.ld(ins.s2) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_slt => {
                let v = ((self.ld(s) as i64) < (self.ld(ins.s2) as i64)) as u64;
                self.st(d, v)
            }
            Kind::a_shr32r => {
                let v = (self.ld(s) as u32).wrapping_shr(self.ld(ins.s2) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_shl64i => {
                let v = self.ld(s).wrapping_shl(imm as u32);
                self.st(d, v)
            }
            Kind::a_shr64i => {
                let v = self.ld(s).wrapping_shr(imm as u32);
                self.st(d, v)
            }
            Kind::a_slt_rev => {
                let v = ((self.ld(ins.s2) as i64) < (self.ld(s) as i64)) as u64;
                self.st(d, v)
            }
            Kind::a_sltu => {
                let v = (self.ld(s) < self.ld(ins.s2)) as u64;
                self.st(d, v)
            }
            Kind::a_sltu_rev => {
                let v = (self.ld(ins.s2) < self.ld(s)) as u64;
                self.st(d, v)
            }
            Kind::a_sub_rev => {
                let v = self.ld(ins.s2).wrapping_sub(self.ld(s));
                self.st(d, v)
            }
            Kind::a_sub32_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_sub(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_shl32r_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shl(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_shr32r_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shr(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::a_shl64r_rev => {
                let v = self.ld(ins.s2).wrapping_shl(self.ld(s) as u32);
                self.st(d, v)
            }
            Kind::a_shr64r_rev => {
                let v = self.ld(ins.s2).wrapping_shr(self.ld(s) as u32);
                self.st(d, v)
            }
            // 64 位按位取反（op 0x35 sub 0x26 且移位探针不成立的那组字）。
            Kind::mvn64 => {
                let v = !self.ld(s);
                self.st(d, v)
            }
            Kind::a_sub => {
                let v = self.ld(s).wrapping_sub(self.ld(ins.s2));
                self.st(d, v)
            }
            Kind::a_shl32_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shl(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            // 64 位“右移 + 位宽掩码”，imm 低字节=移位、次字节=位宽（0xff 表示 64）。
            Kind::ext64 => {
                let sh = (imm & 0xff) as u32;
                let wd = (imm >> 8) as u32;
                let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
                let v = (self.ld(s) >> sh) & mask;
                self.st(d, v)
            }
            // 32x32→64 无符号乘：低 32 位写 d（符号扩展）、高 32 位写 d+1（符号扩展）。
            Kind::mul32_64 => {
                let p = (self.ld(s) as u32 as u64).wrapping_mul(self.ld(ins.s2) as u32 as u64);
                self.st(d, (p as u32 as i32 as i64) as u64);
                self.st(d.wrapping_add(1), ((p >> 32) as u32 as i32 as i64) as u64)
            }
            Kind::add => {
                let v = self.ld(imm as i16).wrapping_add(self.ld(s));
                self.st(d, v)
            }
            Kind::add32 => {
                let v = (self.ld(imm as i16) as u32).wrapping_add(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::and => {
                let v = self.ld(imm as i16) & self.ld(s);
                self.st(d, v)
            }
            Kind::or => {
                let v = self.ld(imm as i16) | self.ld(s);
                self.st(d, v)
            }
            Kind::ori => {
                let v = self.ld(s) | imm as u64;
                self.st(d, v)
            }
            Kind::xor => {
                let v = self.ld(imm as i16) ^ self.ld(s);
                self.st(d, v)
            }
            Kind::nor => {
                let v = !(self.ld(imm as i16) | self.ld(s));
                self.st(d, v)
            }
            Kind::shl64i => {
                let v = self.ld(s).wrapping_shl(imm as u32);
                self.st(d, v)
            }
            Kind::shl32i => {
                let v = (self.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::lsr64i => {
                let v = self.ld(s).wrapping_shr(imm as u32);
                self.st(d, v)
            }
            Kind::shl64i_r => {
                let v = self.ld(s).wrapping_shl(self.ld(imm as i16) as u32);
                self.st(d, v)
            }
            Kind::shr64i => {
                let v = self.ld(s).wrapping_shr(imm as u32);
                self.st(d, v)
            }
            Kind::shr32i => {
                let v = (self.ld(s) as u32).wrapping_shr(imm as u32);
                self.st(d, v as u64)
            }
            Kind::lsr32i => {
                let v = (self.ld(s) as u32).wrapping_shr(imm as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::ror32i => {
                let x = self.ld(s) as u32;
                let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::sar32i => {
                let v = ((self.ld(s) as i32) >> (imm as u32)) as i64;
                self.st(d, v as u64)
            }
            Kind::shr64r => {
                let n = self.ld(d) as u32;
                let v = self.ld(s).wrapping_shr(n);
                self.st(d, v)
            }
            Kind::jmp => {
                self.pc = self.pc.wrapping_add((imm * 4) as u64);
            }
            Kind::jge => {
                if (self.ld(imm as i16) as i64) >= (self.ld(s) as i64) {
                    self.pc = self.pc.wrapping_add((imm * 4) as u64);
                }
            }
            Kind::lea => {
                let v = self.ld(s).wrapping_add(imm as u64);
                self.st(d, v)
            }
            // 32 位 lea（sub 0x40/0x48/0x50/0x58/0x66/0x77/0x7f）：截断后符号扩展。
            Kind::lea32 => {
                let v = self.ld(s).wrapping_add(imm as u64) as u32 as i32 as i64;
                self.st(d, v as u64)
            }
            // 全宽 64 位回绕减法（实测 op 0x35: 0x1000000000000005 - 3 保留高位）。
            Kind::sub32 => {
                let v = self.ld(s).wrapping_sub(self.ld(ins.s2));
                self.st(d, v)
            }
            // 32 位减法后符号扩展（实测 0xa0808b35）。
            Kind::sub32_sext => {
                let v = (self.ld(s) as u32).wrapping_sub(self.ld(ins.s2) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            // 反向减法，32 位截断后符号扩展（实测 0x28c40b35）。
            Kind::sub32_rev => {
                let v = self.ld(ins.s2).wrapping_sub(self.ld(s)) as u32 as i32 as i64;
                self.st(d, v as u64)
            }
            // op 0x35 的 and/or/xor 名字带 32，实测是 64 位全宽。
            Kind::and32 => {
                let v = self.ld(s) & self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::or32 => {
                let v = self.ld(s) | self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::xor32 => {
                let v = self.ld(s) ^ self.ld(ins.s2);
                self.st(d, v)
            }
            Kind::nor32 => {
                let v = !(self.ld(s) | self.ld(ins.s2));
                self.st(d, v)
            }
            Kind::shl32r => {
                let v = (self.ld(s) as u32).wrapping_shl(self.ld(ins.s2) as u32);
                self.st(d, v as u64)
            }
            Kind::shl32r_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shl(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::shr32r => {
                let v = (self.ld(s) as u32).wrapping_shr(self.ld(ins.s2) as u32);
                self.st(d, v as u64)
            }
            Kind::shr32r_rev => {
                let v = (self.ld(ins.s2) as u32).wrapping_shr(self.ld(s) as u32) as i32 as i64;
                self.st(d, v as u64)
            }
            Kind::cmpgt => {
                let v = ((self.ld(s) as i32) > (self.ld(ins.s2) as i32)) as u64;
                self.st(d, v)
            }
            Kind::cmplt => {
                let v = ((self.ld(ins.s2) as i32) > (self.ld(s) as i32)) as u64;
                self.st(d, v)
            }
            // 命名与实测不符：op 0x35 的 0x21204375 实测是无符号 64 位 (s2 > s)。
            Kind::cmpeq => {
                let v = (self.ld(ins.s2) > self.ld(s)) as u64;
                self.st(d, v)
            }
            // 有符号 64 位 (s2 > s)（0x21204375 / 0x21280375）。
            Kind::cmpgt_s => {
                let v = ((self.ld(ins.s2) as i64) > (self.ld(s) as i64)) as u64;
                self.st(d, v)
            }
            // 取 s2 的高 32 位并按位宽截断（0x002300f5 等四条，宽度由字决定）。
            Kind::shrhi32 => {
                let v = self.ld(ins.s2) >> 32;
                self.st(d, v)
            }
            Kind::shrhi24 => {
                let v = self.ld(ins.s2) >> 40;
                self.st(d, v)
            }
            Kind::shrhi16 => {
                let v = self.ld(ins.s2) >> 48;
                self.st(d, v)
            }
            Kind::shrhi8 => {
                let v = self.ld(ins.s2) >> 56;
                self.st(d, v)
            }
            Kind::xori => {
                let v = self.ld(s) ^ imm as u64;
                self.st(d, v)
            }
            // 32 位字面量加法：源按 i32 符号扩展后做 64 位加（真机 0x10051484 是 ldrsw），
            // 结果不回绕到 32 位。
            Kind::addi32 => {
                let v = (self.ld(s) as i32 as i64).wrapping_add(imm) as u64;
                self.st(d, v)
            }
            Kind::ld64 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.mem.r64(a)?;
                self.st(d, v)
            }
            // 64 位未对齐取数的低位半：w0 右移后与旧目的槽高位拼接。
            Kind::ld64ua => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let w0 = self.mem.r64(a & !7u64)?;
                let sh = ((a & 7) * 8) as u32;
                let v = if sh == 0 {
                    w0
                } else {
                    w0.wrapping_shr(sh) | (self.ld(d) & u64::MAX.wrapping_shl(64 - sh))
                };
                self.st(d, v)
            }
            // 64 位未对齐取数的高位半：w0 左移后由旧目的槽低位补齐。
            Kind::ld64ub => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let w0 = self.mem.r64(a & !7u64)?;
                let sh = ((a & 7) * 8) as u32;
                let v = w0.wrapping_shl(56 - sh)
                    | (self.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
                self.st(d, v)
            }
            Kind::ld32 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                if ins.pc == 0x102863d8 {
                    eprintln!(
                        "LD32 pc={:#x} d={} s={} imm={} base={:#x} a={:#x}",
                        ins.pc,
                        d,
                        s,
                        imm,
                        self.ld(s),
                        a
                    );
                }
                let b = self.mem.read(a, 4)?;
                let v = u32::from_le_bytes(b.try_into().unwrap());
                self.st(d, v as u64)
            }
            Kind::ld32s => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let b = self.mem.read(a, 4)?;
                let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
                self.st(d, v as u64)
            }
            Kind::ld32u0 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let b = self.mem.read(a, 4)?;
                let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
                self.st(d, v as u64)
            }
            // 非对齐 u32 读，左拼（op 0x2e，真机 0x1005156c..0x10051fb8）：对齐取字后
            // 左移，低位移入槽 d 的旧值；a 低两位为 3 时直接取字。
            Kind::ld32ua => {
                let a = self.ld(s).wrapping_add(imm as u64).wrapping_add(3) as u32;
                let sh = (a & 3) * 8;
                let b = self.mem.read((a & !3) as u64, 4)?;
                let w0 = u32::from_le_bytes(b.try_into().unwrap());
                let v = if sh == 24 {
                    w0 as i32 as i64
                } else {
                    let x = self.ld(d) as u32;
                    (w0.wrapping_shl(24 - sh) | (x & (0x00ff_ffffu32 >> sh))) as i32 as i64
                };
                self.st(d, v as u64)
            }
            // 非对齐 u32 读，右拼（op 0x1f，真机 0x1004ebcc..0x10051fb8）：对齐取字后
            // 右移，高位移入槽 d 的旧值；a 低两位为 0 时直接取字。
            Kind::ld32m3 | Kind::ld32ub => {
                let a = self.ld(s).wrapping_add(imm as u64) as u32;
                let sh = (a & 3) * 8;
                let b = self.mem.read((a & !3) as u64, 4)?;
                let w0 = u32::from_le_bytes(b.try_into().unwrap());
                let v = if sh == 0 {
                    w0 as i32 as i64
                } else {
                    let x = self.ld(d) as u32;
                    (w0.wrapping_shr(sh) | (x & 0xffff_ffffu32.wrapping_shl(32 - sh))) as i32 as i64
                };
                self.st(d, v as u64)
            }
            Kind::ld16 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let b = self.mem.read(a, 2)?;
                let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
                self.st(d, v)
            }
            Kind::ld8 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.mem.read(a, 1)?[0] as u64;
                self.st(d, v)
            }
            Kind::ld8s => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.mem.read(a, 1)?[0] as i8 as i64;
                self.st(d, v as u64)
            }
            Kind::st8 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.ld(d) as u8;
                self.mem.write(a, &[v])?
            }
            Kind::st16 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.ld(d) as u16;
                self.mem.write(a, &v.to_le_bytes())?
            }
            Kind::st32 => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.ld(d) as u32;
                self.mem.write(a, &v.to_le_bytes())?
            }
            Kind::st64 => {
                if (36500..36900).contains(&self.steps) && flags::stx() {
                    eprintln!(
                        "STX st={} pc={:#x} d={} s={} imm={:#x} a={:#x} v={:#x}",
                        self.steps,
                        self.pc - 4,
                        d,
                        s,
                        imm,
                        self.ld(s).wrapping_add(imm as u64),
                        self.ld(d)
                    );
                }
                eprintln!("ST64S2 pc={:#x} d={} s={}", self.pc - 4, d, s);
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.ld(d);
                if self.pc == 0x10278a54 || self.pc == 0x10278a58 {
                    eprintln!(
                        "ST64S pc={:#x} d={} s={} a={:#x} v={:#x} base={:#x}",
                        self.pc - 4,
                        d,
                        s,
                        a,
                        v,
                        self.slots_addr
                    );
                }
                self.mem.w64(a, v)?
            }
            // 无槽操作数的整数转双精度（引擎 0x10053074 / 0x10051d04）：d/s 已是实测槽。
            Kind::u32f64 => {
                let v = (self.ld(s) as u32) as f64;
                self.st(d, v.to_bits())
            }
            Kind::i32f64 => {
                let v = ((self.ld(s) as u32) as i32) as f64;
                self.st(d, v.to_bits())
            }
            // 8 字节非对齐写：st64ua 写到 (s+imm) 向上对齐的字、沿用字内高字节；
            // st64ub 写到向下对齐的字、把值左移进字内并保留低字节。
            Kind::st64ua => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let w8 = a & !7u64;
                let sh = ((a & 7) * 8) as u32;
                let v = self.ld(d);
                let old = self.mem.r64(w8)?;
                let mk = if sh + 8 >= 64 {
                    u64::MAX
                } else {
                    (1u64 << (sh + 8)) - 1
                };
                self.mem.w64(w8, (old & !mk) | (v >> (56 - sh)))?
            }
            Kind::st64ub => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let w8 = a & !7u64;
                let sh = ((a & 7) * 8) as u32;
                let v = self.ld(d);
                let old = self.mem.r64(w8)?;
                let mk = if sh == 0 { 0 } else { (1u64 << sh) - 1 };
                self.mem.w64(w8, (old & mk) | (v << sh))?
            }
            Kind::xchg => {
                let a = self.ld(s).wrapping_add(imm as u64);
                let v = self.ld(d);
                let old = self.mem.r64(a)?;
                self.mem.w64(a, v)?;
                self.st(d, old)
            }
            // 调用对：本字只把返回地址 pc+8 放进槽 36，紧接的 0xbb5 跳板字再跳过去。
            Kind::call4
            | Kind::call5
            | Kind::call1a
            | Kind::call1d
            | Kind::call22
            | Kind::call27
            | Kind::call => {}
            // 跳板（恒为 0x00000bb5）：槽 0 清零后跳到槽 36。
            Kind::tramp => {
                if probe::on(probe::PC) {
                    eprintln!("  TRAMP s36={:#x} s35={}", self.slots[36], self.slots[35]);
                }
                self.slots[0] = 0;
                if self.slots[35] == 2 {
                    self.slots[35] = 3;
                }
                if self.slots[35] != 0 {
                    self.pc = self.slots[36];
                }
            }
            // call 经 ALU 家族 sub 0x14：只在 mode 0 时布置返回地址与目标，随后 mode 2
            // “再执行一条就跳”。mode 已非 0 时整套布置跳过，只沿用既有的槽 36。
            // 真机 0x100552b4 起：不论当前 mode，都写槽 36 = 槽 s、槽 31 与槽 37（= pc+8，
            // 即延迟槽之后），再把 mode 置 2。mode 非 0 时相当于再插一个延迟槽。
            Kind::callalu => {
                self.slots[36] = self.ld(s);
                self.slots[35] = 2;
                self.slots[31] = next + 4;
                self.slots[37] = next + 4;
            }
            // 字面量装载：把 21 位有符号立即数写进槽 d。
            Kind::shlmask => self.st(ins.d, ins.imm as u64),
            // 分支族：槽 36 = 条件真 ? pc+4+4t : pc+8，随后 mode 2（下一字是延迟槽）。
            // 延迟槽永远执行，所以“条件假”只是落到 pc+8，而不是提前返回。
            Kind::jne => {
                let t = if self.ld(d) != self.ld(s) {
                    next.wrapping_add((imm * 4) as u64)
                } else {
                    next + 4
                };
                self.slots[36] = t;
                self.slots[35] = 2;
            }
            Kind::callz => {
                if probe::on(probe::PC) {
                    static N: AtomicU64 = AtomicU64::new(0);
                    eprintln!(
                        "  callz#{} pc={:#x} d={} s={} imm={} vd={:#x} vs={:#x} take={}",
                        N.fetch_add(1, Ordering::Relaxed) + 1,
                        self.pc - 4,
                        d,
                        s,
                        imm,
                        self.ld(d),
                        self.ld(s),
                        self.ld(d) == self.ld(s)
                    );
                }
                let t = if self.ld(d) == self.ld(s) {
                    next.wrapping_add((imm * 4) as u64)
                } else {
                    next + 4
                };
                self.slots[36] = t;
                self.slots[35] = 2;
            }
            Kind::callgt0 => {
                let t = if (self.ld(s) as i64) > 0 {
                    next.wrapping_add((imm * 4) as u64)
                } else {
                    next + 4
                };
                self.slots[36] = t;
                self.slots[35] = 2;
            }
            Kind::calllt1 => {
                let t = if (self.ld(s) as i64) < 1 {
                    next.wrapping_add((imm * 4) as u64)
                } else {
                    next + 4
                };
                self.slots[36] = t;
                self.slots[35] = 2;
            }
            // 间接调用：槽 36 = 槽 s + imm。
            Kind::callind => {
                self.slots[36] = self.ld(s).wrapping_add(imm as u64);
                self.slots[35] = 2;
            }
            // 间接跳转：槽 36 = 槽 31。
            Kind::jmpi => {
                self.slots[36] = self.slots[31];
                self.slots[35] = 2;
            }
            // 有符号 >= 比较，写槽 d。
            Kind::cmpge => {
                let v = ((self.ld(imm as i16) as i64) > (self.ld(s) as i64)) as u64;
                self.st(d, v)
            }
            // 跳表下标：槽 d = imm + (槽 s >= 1 ? 槽 s : 0)。
            Kind::jtab => {
                let x = self.ld(s);
                let v = (imm as u64).wrapping_add(if (x as i64) >= 1 { x } else { 0 });
                self.st(d, v)
            }
            Kind::or9 => return Err(Error::Unsupported(ins.kind, self.pc - 4)),
        }
        // 调用字成对出现：本字执行完把返回地址写进槽 36，紧跟的 0xbb5 跳板再按 mode 跳。
        // 分支族自己写槽 36，不能在这里覆盖。
        if ins.call
            && !matches!(
                ins.kind,
                Kind::jne
                    | Kind::callz
                    | Kind::callgt0
                    | Kind::calllt1
                    | Kind::callind
                    | Kind::callalu
            )
        {
            self.slots[36] = next + 4;
        }
        if probe::on(probe::PC) && (ins.kind == Kind::callz || ins.kind == Kind::tramp) {
            eprintln!(
                "  AFTER kind={:?} pc={:#x} s36={:#x} s35={}",
                ins.kind, self.pc, self.slots[36], self.slots[35]
            );
        }
        self.transition();
        if self.slots[17] != _o17 {
            eprintln!(
                "S17 step={} pc={:#x} k={:?} {:x} -> {:x} s4={:x} s16={:x}",
                self.steps, self.pc, ins.kind, _o17, self.slots[17], self.slots[4], self.slots[16]
            );
        }
        if self.slots[0] != _o0 || self.slots[1] != _o1 {
            eprintln!(
                "S01 step={} pc={:#x} k={:?} {:x},{} -> {:x},{}",
                self.steps, self.pc, ins.kind, _o0, _o1, self.slots[0], self.slots[1]
            );
        }
        if probe::on(probe::PC) && (ins.kind == Kind::callz || ins.kind == Kind::tramp) {
            eprintln!(
                "  TRANS pc={:#x} s36={:#x} s35={}",
                self.pc, self.slots[36], self.slots[35]
            );
        }
        if self.mem.ld(self.sp + 0x38, 8).unwrap_or(0) != _old38 {
            eprintln!(
                "S38 changed at step {} pc={:#x} kind={:?} {:x} -> {:x}",
                self.steps,
                self.pc,
                ins.kind,
                _old38,
                self.mem.ld(self.sp + 0x38, 8).unwrap_or(0)
            );
        }
        Ok(true)
    }
    /// 从快照字节流恢复：pc、48 个槽、若干个 4KiB 页。
    /// 内置库（`MEDUSA_LIB` 可覆盖），两条初态路径共用。
    fn lib() -> std::borrow::Cow<'static, [u8]> {
        static LIB: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        match std::env::var("MEDUSA_LIB") {
            Ok(p) => match std::fs::read(p) {
                Ok(v) => std::borrow::Cow::Owned(v),
                Err(_) => std::borrow::Cow::Borrowed(LIB.get_or_init(|| gunzip(LIB_GZ)).as_slice()),
            },
            Err(_) => std::borrow::Cow::Borrowed(LIB.get_or_init(|| gunzip(LIB_GZ)).as_slice()),
        }
    }

    fn map_lib(mem: &mut Mem, lib: &[u8]) {
        for (i, chunk) in lib.chunks(PAGE as usize).enumerate() {
            mem.map(0x10000000 + i as u64 * PAGE, chunk);
        }
    }

    /// 静态初态：内置库 + 裁剪后的运行时页，不依赖整幅快照。
    pub fn from_static() -> Option<Vm> {
        let buf = initial_bytes();
        let mut p = 0usize;
        let rd = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(buf.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let pc = rd(&mut p)?;
        let mut slots = [0u64; 64];
        for slot in slots.iter_mut().take(48) {
            *slot = rd(&mut p)?;
        }
        let n = u32::from_le_bytes(buf.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        let mut mem = Mem::new();
        let lib = Self::lib();
        Self::map_lib(&mut mem, &lib);
        // 库文件尾部到映像上界、以及两段暂存池在快照里是零页，按 lazy 零页补。
        let bss = (0x10000000 + lib.len() as u64 + PAGE - 1) & !(PAGE - 1);
        if bss < LIB_HI {
            mem.maplazy(bss, LIB_HI);
        }
        mem.maplazy(0x50000000, 0x58000000);
        mem.maplazy(0x70000000, 0x70300000);
        for _ in 0..n {
            let a = rd(&mut p)?;
            let d = buf.get(p..p + PAGE as usize)?;
            p += PAGE as usize;
            // 页地址不对就说明快照坏了；`map` 对区间外的页是静默丢弃。
            if a & (PAGE - 1) != 0 || !mem.covers(a) {
                return None;
            }
            mem.map(a, d);
        }
        for (i, slot) in slots.iter_mut().enumerate().skip(48) {
            *slot = mem.ld(VM_SLOTS + 8 + 8 * i as u64, 8).unwrap_or(0);
        }
        Some(Vm::new(mem, slots, pc))
    }

    /// 默认初态（静态）；`MEDUSA_BASE=snap` 回退到整幅快照。
    pub fn from_base() -> Option<Vm> {
        if std::env::var("MEDUSA_BASE").as_deref() == Ok("snap") {
            Vm::from_snapshot(snapshot_bytes())
        } else {
            Vm::from_static()
        }
    }

    pub fn from_snapshot(buf: &[u8]) -> Option<Vm> {
        let mut p = 0usize;
        let rd = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(buf.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let pc = rd(&mut p)?;
        // 快照头是 pc + 48 个槽；handler 的槽 n 在 [x6+8+8n]，与表里的索引一致。
        let _ = pc;
        let mut slots = [0u64; 64];
        for slot in slots.iter_mut().take(48) {
            *slot = rd(&mut p)?;
        }
        let n = u32::from_le_bytes(buf.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        let mut mem = Mem::new();
        mem.maplazy(0x50c00000, 0x58000000);
        // 引擎自己的暂存/分配池：快照后新建，起步为零页。
        mem.maplazy(0x70200000, 0x70300000);
        // 程序与只读常量在库文件里（快照不重复存库页），按基址 0x10000000 整幅映射。
        let lib = Self::lib();
        Self::map_lib(&mut mem, &lib);
        for _ in 0..n {
            let a = rd(&mut p)?;
            let d = buf.get(p..p + PAGE as usize)?;
            p += PAGE as usize;
            // 页地址不对就说明快照坏了；`map` 对区间外的页是静默丢弃。
            if a & (PAGE - 1) != 0 || !mem.covers(a) {
                return None;
            }
            mem.map(a, d);
        }
        // 头部只存 48 槽；48..63 仍活在栈上（如槽 52/53 就是保存的 x29/x30），从页里补齐。
        for (i, slot) in slots.iter_mut().enumerate().skip(48) {
            *slot = mem.ld(VM_SLOTS + 8 + 8 * i as u64, 8).unwrap_or(0);
        }
        Some(Vm::new(mem, slots, pc))
    }

    pub fn set_clock(&mut self, sec: u64) {
        // 快照把夹具时间烘焙进了引擎状态。这些字是绝对时间，不跟着宿主时钟走就会
        // 每次都用夹具那一秒签；按 `sec` 改写。两个夹具的布局一致，地址见 docs/research/medusa-vm。
        if std::env::var_os("MEDUSA_NOCLOCK").is_none() {
            let sec32 = (sec as u32).to_le_bytes();
            for a in CLOCK_SEC_WORDS {
                let _ = self.mem.write(*a, &sec32);
            }
            let _ = self.mem.write(CLOCK_MSEC_WORD, &sec32);
            let us = (self.nat.clock_ns[0] / 1_000) as u32;
            let _ = self.mem.write(CLOCK_MSEC_WORD + 4, &us.to_le_bytes());
        }
        if std::env::var_os("MEDUSA_PATCHTIME").is_some() {
            for (lo, hi) in REGIONS {
                if let Ok(k) = std::env::var("MEDUSA_PATCHREG") {
                    let want: usize = k.parse().unwrap_or(usize::MAX);
                    if REGIONS.iter().position(|r| r.0 == *lo) != Some(want) {
                        continue;
                    }
                }
                let mut p = *lo;
                let mut end = *hi;
                if let Ok(a) = std::env::var("MEDUSA_PATCHADDR") {
                    let want = a
                        .split(',')
                        .filter_map(|x| {
                            u64::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok()
                        })
                        .collect::<Vec<_>>();
                    if !want.iter().any(|w| *w >= *lo && *w < *hi) {
                        continue;
                    }
                    p = *lo;
                    end = *hi;
                }
                while p < end {
                    if let Some(b) = self.mem.page_bytes(p) {
                        let mut i = 0;
                        while i + 8 <= b.len() {
                            let v = u64::from_le_bytes(b[i..i + 8].try_into().unwrap());
                            let only = std::env::var("MEDUSA_PATCHADDR").ok();
                            if let Some(list) = &only {
                                let want = list.split(',').any(|a| {
                                    u64::from_str_radix(a.trim().trim_start_matches("0x"), 16)
                                        == Ok(p + i as u64)
                                });
                                if !want {
                                    i += 4;
                                    continue;
                                }
                            }
                            if v == native::CLOCKSEC_CLK {
                                eprintln!("PATCH {:#x}", p + i as u64);
                                let _ = self.mem.write(p + i as u64, &sec.to_le_bytes());
                            } else if v / 1000 == native::CLOCKSEC_CLK
                                || v / 1000 == native::CLOCKSEC_CLK + 1
                            {
                                let _ = self
                                    .mem
                                    .write(p + i as u64, &(sec * 1000 + v % 1000).to_le_bytes());
                            }
                            i += 4;
                        }
                    }
                    p += PAGE;
                }
            }
            if std::env::var_os("MEDUSA_PATCHREPORT").is_some() {
                let pat = native::CLOCKSEC_CLK.to_le_bytes();
                for (lo, hi) in REGIONS {
                    let mut p = *lo;
                    while p < *hi {
                        if let Some(b) = self.mem.page_bytes(p) {
                            let mut i = 0;
                            while i + 8 <= b.len() {
                                if b[i..i + 4] == pat {
                                    eprintln!("LEFT {:#x}", p + i as u64);
                                }
                                i += 4;
                            }
                        }
                        p += PAGE;
                    }
                }
            }
        }
        self.nat.set_clock(sec);
    }

    pub fn set_clock_ns(&mut self, ns: [u64; 8]) {
        self.nat.clock_ns = ns;
    }

    /// 调试：整体平移注入的纳秒序列（`MEDUSA_NSOFF`，单位纳秒）。
    pub fn shift_ns(&mut self, off: i64) {
        for v in &mut self.nat.clock_ns {
            *v = (*v as i64).wrapping_add(off) as u64;
        }
    }

    /// 在既有 Vm 上注入输入并跑完整段。
    pub fn sign_injected(
        &mut self,
        query: &str,
        stub: &Stub,
        clock: u64,
        rng: &mut Urng,
    ) -> Result<String, Error> {
        let t_inj = std::time::Instant::now();
        self.set_clock(clock);
        self.nat.rng = std::mem::take(rng);
        self.mem.map(SCRATCH, query.as_bytes());
        self.mem.map(QUERY_PTR, &SCRATCH.to_le_bytes());
        self.mem.map(QUERY_LEN, &(query.len() as u16).to_le_bytes());
        const ZERO_STUB: [u8; 16] = [0; 16];
        let stub = match stub {
            Stub::Missing | Stub::Empty => &ZERO_STUB[..],
            Stub::Hex(v) => &v[..16.min(v.len())],
        };
        self.mem.map(STUB_BUF, stub);
        self.mem.map(STUB_LEN, &(stub.len() as u32).to_le_bytes());
        self.mem.map(STUB_PTR, &STUB_BUF.to_le_bytes());
        if phases::busy() {
            phases::add(1, t_inj.elapsed().as_nanos());
        }
        // 默认走 AOT；`MEDUSA_IMPL=replay` 保留 .so 重放作差分 oracle 与回退。
        let _t2 = std::time::Instant::now();
        let r = if std::env::var("MEDUSA_IMPL").as_deref() == Ok("replay") {
            self.run(20_000_000)
        } else {
            self.run_aot(20_000_000)
        };
        let _t3 = std::time::Instant::now();
        if phases::busy() {
            phases::add(2, (_t3 - _t2).as_nanos());
        }
        if std::env::var_os("MEDUSA_MC").is_some() {
            eprintln!("PHASE run={}us", (_t3 - _t2).as_micros());
        }
        *rng = std::mem::take(&mut self.nat.rng);
        if let Err(e) = &r {
            eprintln!(
                "RUNERR {:?} step={} pc={:#x} sa={:#x} sp={:#x} s35={} s37={:#x} w={:#x} x0={:#x} x1={:#x} x8={:#x} x19={:#x} x20={:#x} x21={:#x}",
                e,
                self.steps,
                self.pc,
                self.slots_addr,
                self.sp,
                self.slots[35],
                self.slots[37],
                self.mem.code_u32(self.pc),
                self.regs[0],
                self.regs[1],
                self.regs[8],
                self.regs[19],
                self.regs[20],
                self.regs[21]
            );
        }
        r?;
        if std::env::var_os("MEDUSA_STEPS").is_some() {
            eprintln!(
                "STEPS {} NAT {} MISS {}",
                self.steps,
                self.nat.steps,
                native::OP_MISS.load(std::sync::atomic::Ordering::Relaxed)
            );
        }
        let len = self.nat.out_len;
        if len == 0 || len > 4096 {
            return Err(Error::BadOutputLen(len));
        }
        if probe::on(probe::DIRTY) {
            eprintln!(
                "DIRTY {} steps={} {:x?}",
                self.mem.dirty_pages().count(),
                self.steps,
                self.mem.dirty_pages().collect::<Vec<_>>()
            );
        }
        if std::env::var("MEDUSA_DUMP").is_ok() {
            let mut a = 0x701e_0000u64;
            while a < 0x701f_0000 {
                if let Ok(b) = self.mem.read(a, 16) {
                    if b.iter().any(|x| *x != 0) {
                        eprintln!("DMP {:#x} {:02x?}", a, b);
                    }
                }
                a += 16;
            }
        }
        if probe::on(probe::CALLSTAT) {
            native::dump_pctr();
        }
        let t_enc = std::time::Instant::now();
        let out: Vec<u8> = {
            let p = self
                .mem
                .span(OUT_ADDR, len)
                .ok_or(Error::Unmapped(OUT_ADDR))? as *const u8;
            unsafe { std::slice::from_raw_parts(p, len) }.to_vec()
        };
        use base64::Engine;
        let enc = base64::engine::general_purpose::STANDARD.encode(out);
        if phases::busy() {
            phases::add(3, t_enc.elapsed().as_nanos());
        }
        Ok(enc)
    }

    /// 注入 query 与 stub 并跑完整段，返回 Base64 明文。
    pub fn sign_query_with_stub(
        query: &str,
        stub: &Stub,
        clock: u64,
        rng: &mut Urng,
    ) -> Result<String, Error> {
        if query.len() > MAX_QUERY {
            return Err(Error::BadQuery);
        }
        STR_ON.store(str_on(), std::sync::atomic::Ordering::Relaxed);
        phases::begin();
        let _t0 = std::time::Instant::now();
        let mut vm = match POOL.with(|c| c.borrow_mut().take()) {
            Some((mut v, last)) if last.elapsed().as_secs() < idle_secs() => {
                v.rewind();
                v
            }
            _ => Vm::from_base().ok_or(Error::BadSnapshot)?,
        };
        if phases::busy() {
            phases::add(0, _t0.elapsed().as_nanos());
        }
        let r = vm.sign_injected(query, stub, clock, rng);
        POOL.with(|c| *c.borrow_mut() = Some((vm, std::time::Instant::now())));
        phases::end();
        r
    }

    /// 快照默认 stub（空包 MD5），供 KAT 与不解析头块的调用方使用。
    pub fn sign_query(query: &str, clock: u64, rng: &mut Urng) -> Result<String, Error> {
        const EMPTY_MD5: [u8; 16] = [
            0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
            0x42, 0x7e,
        ];
        Self::sign_query_with_stub(query, &Stub::Hex(EMPTY_MD5.to_vec()), clock, rng)
    }

    pub fn run(&mut self, limit: u64) -> Result<(), Error> {
        if let Ok(a) = std::env::var("WATCHADDR") {
            WATCH.store(
                u64::from_str_radix(a.trim().trim_start_matches("0x"), 16).unwrap_or(0),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        let limit = stop_limit(limit);
        while self.steps < limit {
            if !self.step()? {
                return Ok(());
            }
        }
        Err(Error::Steps)
    }

    /// 跳板：槽 0 清零后按 mode 跳槽 36；mode 3 时按真机 0x10055184 起写返回帧。
    pub fn tramp(&mut self) -> u64 {
        let next = self.pc.wrapping_add(4);
        let target = self.slots[36];
        self.pc = target;
        match self.slots[35] {
            1 | 3 => {
                self.slots[35] = 0;
                self.ret_pc = next;
                let frame = self.slots_addr + 0x140;
                let _ = self.mem.w64(frame - 0x20, 0);
                let _ = self.mem.w64(frame - 0x18, target);
                let _ = self.mem.w64(frame - 0x10, next);
                let _ = self.mem.w64(self.slots_addr, target);
                self.last_ret = !(PROG_LO..PROG_HI).contains(&target)
                    && target != NATIVE_CALL
                    && target != DEFAULT_CALL;
            }
            2 => self.slots[35] = 3,
            _ => {}
        }
        target
    }

    /// AOT 路径：字节码由 `native_program` 直接执行，原生回调仍走 `Native`。
    pub fn run_aot(&mut self, limit: u64) -> Result<(), Error> {
        if let Ok(a) = std::env::var("WATCHADDR") {
            WATCH.store(
                u64::from_str_radix(a.trim().trim_start_matches("0x"), 16).unwrap_or(0),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        let limit = stop_limit(limit);
        if self.steps == 0 {
            // 真机首步在 0x100544a8 已把 pc 推进到下一字并写回 [x6]，随后经 0x1004e6e0
            // 建立帧再取指；这里复刻同一次引擎前导，避免 frame/slot 状态整体漂移。
            let _ = self.mem.w64(self.slots_addr, self.pc + 4);
            self.flush_slots()?;
            self.regs[6] = self.slots_addr;
            self.regs[27] = self.slots_addr + 0x140;
            self.regs[0] = self.pc;
            self.steps += 1;
            self.materialize();
            let r = self.nat.call(
                &mut self.mem,
                &self.slots,
                0x1004_e6e0,
                &mut self.sp,
                &mut self.regs,
                false,
                true,
            )?;
            match r {
                Some(re) => {
                    if re.pc == native::END {
                        self.pc = native::END;
                        return Ok(());
                    }
                    self.slots_addr = re.slots_addr;
                    self.reload_slots()?;
                    self.pc = re.pc;
                }
                None => {
                    self.reload_slots()?;
                    self.pc = self.slots[37];
                }
            }
        }
        let mc = phases::busy();
        // 下列开关都是进程级常量：逐字重复读原子量与环境变量只白耗时间。
        // 每次进入 run_aot 取一次，循环内只看局部 bool。
        let cfg = HotCfg {
            rdx: flags::rdx(),
            at090: flags::at090(),
            sh: flags::sh(),
            always_engine: flags::always_engine(),
            nosent: flags::nosent(),
            dbg: probe::on(probe::DBG),
            aotpc: probe::on(probe::AOTPC),
            callstat: probe::on(probe::CALLSTAT),
            eng: std::env::var_os("MEDUSA_ENG").is_some(),
        };
        let hist = flags::hist();
        // CURPC/GSTEP 只服务探针输出，没有探针时不逐字写。
        let mem_hot = self.mem.is_hot();
        while self.steps < limit {
            let pc = self.pc;
            if hist {
                static H: std::sync::OnceLock<std::sync::Mutex<std::collections::VecDeque<u64>>> =
                    std::sync::OnceLock::new();
                let h = H.get_or_init(Default::default);
                let mut g = h.lock().unwrap();
                if g.len() >= 24 {
                    g.pop_front();
                }
                g.push_back(pc);
                if self.steps >= 60_000 && self.steps.is_multiple_of(10_000) {
                    eprintln!(
                        "HIST {:x?} retpc={:#x} m={}",
                        *g, self.ret_pc, self.slots[35]
                    );
                }
            }
            if mem_hot {
                CURPC.store(pc, std::sync::atomic::Ordering::Relaxed);
                GSTEP.store(self.steps, std::sync::atomic::Ordering::Relaxed);
            }
            if trace2() {
                eprintln!("APC {pc:#x} st={} s35={} jc={} s36={:#x} s37={:#x} x0={:#x} x1={:#x} s0={:#x} s4={:#x} s17={:#x} sa={:#x}", self.steps, self.slots[35], self.jump_call, self.slots[36], self.slots[37], self.regs[0], self.regs[1], self.slots[0], self.slots[4], self.slots[17], self.slots_addr);
            }
            if pc == native::END {
                return Ok(());
            }
            if !(PROG_LO..PROG_HI).contains(&pc) {
                if self.slots[35] == 2 {
                    self.slots[35] = 3;
                }
                let _ = self.mem.w64(self.slots_addr, pc);
                self.steps += 1;
                self.materialize();
                self.sync_slots()?;
                if cfg.callstat {
                    eprintln!(
                        "BIND {:#x} jc={} ret={:#x} s4={:#x} s5={:#x} x0={:#x} x1={:#x} m={}",
                        pc,
                        self.jump_call,
                        self.ret_pc,
                        self.slots[4],
                        self.slots[5],
                        self.regs[0],
                        self.regs[1],
                        self.slots[35]
                    );
                }
                if pc == NATIVE_CALL || pc == DEFAULT_CALL || self.jump_call {
                    if cfg.aotpc {
                        eprintln!(
                            "NATBIND pc={:#x} s4={:#x} s5={:#x} ret={:#x} s37={:#x} m={}",
                            pc,
                            self.slots[4],
                            self.slots[5],
                            self.ret_pc,
                            self.slots[37],
                            self.slots[35]
                        );
                    }
                    self.slots[35] = 0;
                    let t_nat = if mc {
                        Some(std::time::Instant::now())
                    } else {
                        None
                    };
                    let st0 = self.nat.steps;
                    let r = self.nat.call(
                        &mut self.mem,
                        &self.slots,
                        pc,
                        &mut self.sp,
                        &mut self.regs,
                        true,
                        false,
                    )?;
                    if let Some(t) = t_nat {
                        let ns = t.elapsed().as_nanos();
                        phases::add(4, ns);
                        phases::nat(pc, ns, self.nat.steps - st0);
                        phases::inc(8);
                    }
                    match r {
                        Some(re) => {
                            if re.pc == native::END {
                                self.pc = native::END;
                                return Ok(());
                            }
                            self.slots_addr = re.slots_addr;
                            self.reload_slots()?;
                            self.pc = re.pc;
                        }
                        None => {
                            self.reload_slots()?;
                            self.pc = self.ret_pc;
                            // 复刻 0x10054340..0x10054348：把续跑 pc 写回 [x6]。
                            let _ = self.mem.w64(self.slots_addr, self.pc);
                            self.regs[6] = self.slots_addr;
                        }
                    }
                } else {
                    let addr = if self.last_ret { 0x1005_538c } else { pc };
                    self.regs[30] = if cfg.nosent {
                        native::SENT
                    } else {
                        0x1005_4340
                    };
                    let t_nat = if mc {
                        Some(std::time::Instant::now())
                    } else {
                        None
                    };
                    let st0 = self.nat.steps;
                    let r = self.nat.call(
                        &mut self.mem,
                        &self.slots,
                        addr,
                        &mut self.sp,
                        &mut self.regs,
                        false,
                        false,
                    )?;
                    if let Some(t) = t_nat {
                        let ns = t.elapsed().as_nanos();
                        phases::add(4, ns);
                        phases::nat(addr, ns, self.nat.steps - st0);
                        phases::inc(if addr == 0x1005_538c { 9 } else { 10 });
                    }
                    match r {
                        Some(re) => {
                            if re.pc == native::END {
                                self.pc = native::END;
                                return Ok(());
                            }
                            self.slots_addr = re.slots_addr;
                            self.reload_slots()?;
                            // 返回路径会把调用者的续跑字继续分发掉；直接回 AOT 从
                            // ret_pc 重跑，省下这段原生 dispatch。
                            self.pc = re.pc;
                        }
                        None => {
                            self.reload_slots()?;
                            self.pc = self.slots[37];
                            let _ = self.mem.w64(self.slots_addr, self.pc);
                            self.regs[6] = self.slots_addr;
                        }
                    }
                }
                continue;
            }
            self.last_ret = false;
            // mode 2 表示上一字已经布置了延迟槽；取指时先转 3，延迟槽本身可 AOT。
            if self.slots[35] == 2 {
                self.slots[35] = 3;
            }
            if cfg.aotpc {
                eprintln!(
                    "APC {:#x} m={} s36={:#x}",
                    pc, self.slots[35], self.slots[36]
                );
                eprintln!(
                    "ASL step={} pc={:#x} {:x?} base={:#x} mem={:x?} regs={:x?} sp={:#x}",
                    self.steps,
                    pc,
                    &self.slots[..],
                    self.slots_addr,
                    self.mem.read(0x701edb00, 0x80).unwrap_or_default(),
                    &self.regs[..],
                    self.sp
                );
                if cfg.dbg {
                    eprintln!(
                        "AMEM {:#x} {:x?}",
                        0x701e_c740,
                        self.mem.read(0x701e_c740, 0x1c0).unwrap_or_default()
                    );
                }
            }
            if pc == 0x102cd11c && cfg.rdx {
                eprintln!(
                    "RDXDUMP aot {:02x?}",
                    self.mem.read(0x503f_f760, 0x30).unwrap_or_default()
                );
            }
            if cfg.at090 && self.steps == 25492 {
                eprintln!(
                    "MEMD {:02x?}",
                    self.mem.read(0x701ec340, 0x40).unwrap_or_default()
                );
            }
            if cfg.sh {
                let mut h: u64 = 0xcbf2_9ce4_8422_2325;
                let mut mix = |v: u64| {
                    for b in v.to_le_bytes() {
                        h ^= b as u64;
                        h = h.wrapping_mul(0x100_0000_01b3);
                    }
                };
                for v in self.slots.iter() {
                    mix(*v);
                }
                mix(pc);
                mix(self.sp);
                mix(self.slots_addr);
                eprintln!("ASH {} {:#x}", self.steps, h);
            }
            // 本字结束后要跳出 VM 程序区（原生调用或返回原生调用者）：交给引擎执行这一条。
            // handler 结束时的寄存器（x19..x28 等）会被原生侧读取，AOT 只靠 decode
            // 复现不了；原生跳转占比不到 1%，代价可接受。
            let kind = if cfg.callstat || cfg.aotpc {
                words::find(pc).map(|i| i.kind)
            } else {
                None
            };
            // 本字已在程序区内，`pc+4` 出区等价于 `pc` 是最后一个字。
            let next_out = pc + 4 >= PROG_HI;
            let native_tr = next_out || cfg.always_engine;
            if native_tr {
                if cfg.callstat {
                    eprintln!(
                        "CSTK {:?} pc={:#x} m={} out={} nx={} tgt={:#x}",
                        kind,
                        pc,
                        self.slots[35],
                        !(PROG_LO..PROG_HI).contains(&self.slots[36]),
                        next_out,
                        self.slots[36]
                    );
                }
                if cfg.aotpc {
                    eprintln!(
                        "ENGTR pc={:#x} m={} k={:?} sp={:#x} sa={:#x} regs={:x?} m420={:02x?}",
                        pc,
                        self.slots[35],
                        kind,
                        self.sp,
                        self.slots_addr,
                        &self.regs[..],
                        self.mem.read(0x701ed3e0, 0xc0).unwrap_or_default()
                    );
                }
                if cfg.eng {
                    eprintln!(
                        "ENG tr pc={:#x} w={:#x} m={}",
                        pc,
                        self.mem.code_u32(pc),
                        self.slots[35]
                    );
                }
                if !self.engine_word(pc)? {
                    return Ok(());
                }
                continue;
            }
            // MEDUSA_ALLENG 把所有字交回引擎，供 AOT 与引擎逐字对拍。
            if alleng() {
                if !self.engine_word(pc)? {
                    return Ok(());
                }
                continue;
            }
            // 引擎在 0x100544a8 取指后就把 pc+4 写回 [x6]；native callz 以 [x6] 计算返回地址。
            self.mem.raw_sw64(self.slots_addr, pc + 4);
            self.steps += 1;
            let t_aot = if mc {
                Some(std::time::Instant::now())
            } else {
                None
            };
            match native_program::exec(self, pc) {
                Ok(next) => {
                    self.pc = next;
                    if let Some(t) = t_aot {
                        phases::add(5, t.elapsed().as_nanos());
                        phases::inc(11);
                    }
                }
                Err(Error::UnknownPc(_)) => {
                    if std::env::var_os("MEDUSA_UNK").is_some() {
                        eprintln!("UNK pc={:#x} w={:#x}", pc, self.mem.code_u32(pc));
                    }
                    // 表里没有的字（例如 mode 2 延迟槽）退回引擎逐字执行。
                    // engine_word 会先整块写回，这里不用自己清脏位。
                    self.steps -= 1;
                    // 表里没有的字要先由引擎真正执行完，这里不能停在 dispatch 前。
                    if cfg.eng {
                        eprintln!("ENG unk pc={:#x} w={:#x}", pc, self.mem.code_u32(pc));
                    }
                    if !self.engine_word(pc)? {
                        return Ok(());
                    }
                    continue;
                }
                Err(e) => {
                    // 出错路径不会走到 merge_slots；把脏位清掉，避免下一次把本次半途
                    // 写过的槽当成"待写回"。
                    self.slot_dirty = 0;
                    self.slot_touched = 0;
                    return Err(e);
                }
            }
            let t_mg = if mc {
                Some(std::time::Instant::now())
            } else {
                None
            };
            self.merge_slots()?;
            if let Some(t) = t_mg {
                phases::add(6, t.elapsed().as_nanos());
            }
            match self.jump_flag {
                1 => self.jump_call = true,
                2 => self.jump_call = false,
                _ => {}
            }
            self.jump_flag = 0;
            self.transition_aot();
        }
        Err(Error::Steps)
    }

    /// 调试用：跑 run_aot 的出区 bind 路径（原生调用/返回跳板）。
    pub fn dbg_native_bound(&mut self, pc: u64) -> Result<(), Error> {
        if self.slots[35] == 2 {
            self.slots[35] = 3;
        }
        let _ = self.mem.w64(self.slots_addr, pc);
        self.steps += 1;
        let w = self.mem.code_u32(pc);
        self.decode(w);
        self.dbg_materialize();
        self.sync_slots()?;
        let bind = pc == NATIVE_CALL || pc == DEFAULT_CALL || self.jump_call;
        if bind {
            self.slots[35] = 0;
        }
        let r = self.nat.call(
            &mut self.mem,
            &self.slots,
            pc,
            &mut self.sp,
            &mut self.regs,
            bind,
            false,
        )?;
        match r {
            Some(re) => {
                self.slots_addr = re.slots_addr;
                self.reload_slots()?;
                self.pc = re.pc;
            }
            None => {
                self.reload_slots()?;
                self.pc = self.slots[37];
                let _ = self.mem.w64(self.slots_addr, self.pc);
                self.regs[6] = self.slots_addr;
            }
        }
        Ok(())
    }

    pub fn dbg_transition_aot(&mut self) {
        self.transition_aot()
    }

    pub fn dbg_engine_word(&mut self, pc: u64) -> Result<bool, Error> {
        self.engine_word(pc)
    }

    /// 把单个字交给原生引擎执行：AOT 出程序区与 AOT 未覆盖的字共用。
    /// 返回 false 表示程序结束。
    fn engine_word(&mut self, pc: u64) -> Result<bool, Error> {
        self.steps += 1;
        if std::env::var_os("TRAP5").is_some() {
            eprintln!(
                "EW pc={pc:#x} s35={} m35={} s0={:#x} m0={:#x}",
                self.slots[35],
                self.mem.r64(self.slots_addr + 0x120).unwrap_or(0),
                self.slots[0],
                self.mem.r64(self.slots_addr).unwrap_or(0)
            );
            eprintln!(
                "EW25 s25={:#x} m25={:#x} dirty={:#x} touched={:#x}",
                self.slots[25],
                self.mem.r64(self.slots_addr + 8 + 8 * 25).unwrap_or(0),
                self.slot_dirty,
                self.slot_touched
            );
        }
        // 引擎会从内存读槽块：先把累计的 AOT 写整块写回。
        self.sync_slots()?;
        // 与 replay 的 step() 一致：只写 [x6] 的取指单元，不动槽 0。
        let _ = self.mem.w64(self.slots_addr, pc);
        // 从分派入口 0x1004e710 进入：引擎自己 `ldr w8,[x0]` 取指并跑完整条
        // 分字段/选路前缀，`[x6]` 在 0x100544a8 被推进。不能跳进中段的区段分叉点
        // （0x1004e8b8/0x1004e9c0）——那里 x27 还是上一字的残值。
        if self.slots[35] == 2 {
            self.slots[35] = 3;
        }
        if std::env::var_os("TRAP5").is_some() {
            eprintln!(
                "EW2 pc={pc:#x} r27={:#x} sa={:#x} mem[r27-32]={:#x}",
                self.regs[27],
                self.slots_addr,
                self.mem.r64(self.regs[27].wrapping_sub(32)).unwrap_or(0)
            );
        }
        let entry = if std::env::var_os("MEDUSA_FASTENTRY").is_some() {
            let w = self.mem.code_u32(pc);
            self.decode_now(w);
            if w & 0x3f >= 0x1f {
                0x1004_e9c0
            } else {
                0x1004_e8b8
            }
        } else {
            self.regs[0] = pc;
            self.regs[6] = self.slots_addr;
            self.regs[27] = self.slots_addr + 0x140;
            0x1004_e710u64
        };
        let mc = phases::busy();
        let t_nat = if mc {
            Some(std::time::Instant::now())
        } else {
            None
        };
        let r = self.nat.call(
            &mut self.mem,
            &self.slots,
            entry,
            &mut self.sp,
            &mut self.regs,
            false,
            true,
        )?;
        if let Some(t) = t_nat {
            phases::add(4, t.elapsed().as_nanos());
            phases::inc(7);
        }
        match r {
            Some(re) => {
                if re.pc == native::END {
                    self.pc = native::END;
                    return Ok(false);
                }
                self.slots_addr = re.slots_addr;
                self.reload_slots()?;
                self.pc = re.pc;
            }
            None => {
                self.reload_slots()?;
                self.pc = self.slots[37];
            }
        }
        Ok(true)
    }
}
