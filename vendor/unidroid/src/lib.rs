mod apk;
pub use apk::Apk;
mod arch;
mod builder;
pub use builder::EmulatorBuilder;
mod checkpoint;
pub use checkpoint::Checkpoint;
mod debug;
mod dl;
mod elf;
mod hook;
mod host;
pub use hook::ImportCall;
mod jni;
mod loader;
pub use loader::LibraryInfo;
mod memory;
mod policy;
pub use policy::{Policy, PolicyHandler};
mod probe;
pub mod profile;
pub use probe::{Probe, ProbedCall};
mod run;
pub mod serve;
mod source;
mod syscalls;
mod threads;
mod vfs;
pub use debug::{
    Crash, Frame,
    control::{Resume, Stop, StopReason},
    coverage::BlockCount,
    crypto::CryptoConstant,
    events::{Event, MemoryAccess, Trace},
    heap::{Allocation, Field, HeapObject, Referrer, Target},
    maps::Mapping,
    modules::ModuleInfo,
    search::SEARCH_LIMIT,
    snapshot::{Change, Diff, Snapshot},
    tracepoint::Tracepoint,
    watch::{Watch, WatchAction, WatchHit},
};
pub use jni::{
    Handle, JniCall, JniHandler, JniOp, JniValue, PrimitiveType, ReflectedMember,
    ReflectedMemberKind,
};

use arch::Architecture;
use elf::Image;
use std::{collections::{HashMap, HashSet}, path::PathBuf, sync::Arc};
use unicorn_engine::{
    Unicorn,
    unicorn_const::{ControlType, HookType, MemType, Prot, uc_ctl, uc_error},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const NO_LIBRARY: &str = "no library loaded yet: the first load_library picks arm64 or arm32";
const PAGE: u64 = 4096;
const TRAMPOLINE: u64 = 0x10000;
const TRAMPOLINE_SIZE: u64 = 0x10000;
const TRAMPOLINE_STUBS: usize = 8192;
const RETURN: u64 = TRAMPOLINE + TRAMPOLINE_SIZE;
const HOST_MALLOC_START: u64 = 0x40000000;
pub(crate) const MMAP_START: u64 = 0x50000000;
pub(crate) const MMAP_END: u64 = 0x60000000;
const STACK: u64 = 0x70000000;
const STACK_SIZE: u64 = 0x200000;
const INITIAL_SP: u64 = STACK + STACK_SIZE - 0x10000;
const BRK_START: u64 = 0x60000000;
pub(crate) fn page_up(n: u64) -> u64 {
    (n + PAGE - 1) & !(PAGE - 1)
}
fn page_down(n: u64) -> u64 {
    n & !(PAGE - 1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Module(pub usize);
#[derive(Clone)]
struct Loaded {
    app: bool,
    path: PathBuf,
    guest_path: Option<String>,
    name: String,
    image: Arc<Image>,
    base: u64,
    dependencies: Vec<Module>,
    relocated: bool,
    initialized: bool,
    path_ptr: u64,
    info_ptr: u64,
    dl_references: usize,
}
#[derive(Clone)]
struct Region {
    start: u64,
    end: u64,
    shared: Option<SharedMapping>,
}
#[derive(Clone)]
struct SharedMapping {
    image: Arc<Image>,
    start: u64,
    file_offset: usize,
}
impl SharedMapping {
    fn offset(&self, address: u64) -> usize {
        self.file_offset + (address - self.start) as usize
    }
}
#[derive(Clone)]
enum HostFunction {
    Jni(usize),
    Vm(usize),
    Property,
    PropertySet,
    Dl(&'static str),
    Asset(&'static str),
    Import { name: String, original: Option<u64> },
    Unresolved { symbol: String, module: String },
}

#[derive(Default)]
struct CpuData {
    interrupt: Option<u32>,
    source_error: Option<(u64, Box<dyn std::error::Error>)>,
    memory_access: Option<MemoryFault>,
    slice_remaining: u64,
    thread: u32,
    depth: usize,
    import: Option<u64>,
    stop: Option<StopReason>,
    watch: Option<(WatchHit, WatchAction)>,
    step_started: bool,
    skip_breakpoint: Option<(u32, u64)>,
    finish: Option<debug::control::FinishTarget>,
    arch: Option<Architecture>,
}
struct MemoryFault {
    kind: MemType,
    address: u64,
    size: usize,
}

pub struct Emulator {
    // Unicorn must be destroyed before the Arc-backed host pointers it uses.
    uc: Option<Unicorn<'static, CpuData>>,
    cache_size: u32,
    arch: Option<Architecture>,
    modules: Vec<Loaded>,
    module_table: debug::modules::SharedModules,
    regions: Vec<Region>,
    host: Vec<HostFunction>,
    overrides: HashMap<String, u64>,
    symbol_aliases: HashMap<String, String>,
    rootfs: PathBuf,
    apk: Option<std::rc::Rc<Apk>>,
    assets: apk::assets::Assets,
    library_dirs: Vec<PathBuf>,
    properties: HashMap<String, String>,
    trace: Trace,
    events: std::rc::Rc<std::cell::RefCell<debug::events::Events>>,
    next_module: u64,
    heap: u64,
    heap_end: u64,
    brk: u64,
    free_blocks: Vec<(u64, u64)>,
    allocations: HashMap<u64, u64>,
    heap_high_water: u64,
    process_name: String,
    dlerror_buf: u64,
    dlerror_pending: bool,
    dlerror_capacity: usize,
    vfs: vfs::Vfs,
    source: source::Source,
    jni: jni::JniState,
    threads: threads::Threads,
    imports: debug::imports::Imports,
    code: debug::code::Code,
    calls: Option<debug::calls::Calls>,
    coverage: debug::coverage::Coverage,
    heap_trace: debug::heap::Heap,
    breakpoints: HashMap<u64, unicorn_engine::UcHookId>,
    watchpoints: debug::watch::Watchpoints,
    tracepoints: debug::tracepoint::Tracepoints,
}
impl Emulator {
    fn create_cpu(&mut self, arch: Architecture) -> Result<()> {
        let (cpu_arch, mode) = arch.cpu();
        let mut uc = Unicorn::new_with_data(cpu_arch, mode, CpuData::default())?;
        // Set before the first CPU access: this allocates Unicorn's TCG cache.
        let status = unsafe {
            uc_ctl(
                uc.get_handle(),
                ControlType(ControlType::TCG_BUFFER_SIZE.0 | ControlType::IO_WRITE.0 | (1 << 26)),
                self.cache_size,
            )
        };
        if status != uc_error::OK {
            return Err(status.into());
        }
        arch.enable_fp(&mut uc)?;
        uc.add_intr_hook(|uc, interrupt| {
            uc.get_data_mut().interrupt = Some(interrupt);
            let _ = uc.emu_stop();
        })?;
        uc.add_mem_hook(HookType::MEM_INVALID, 1, 0, |uc, kind, address, size, _| {
            uc.get_data_mut().memory_access = Some(MemoryFault {
                kind,
                address,
                size,
            });
            false
        })?;
        self.source.configure_counter(&mut uc, arch)?;
        self.uc = Some(uc);
        self.arch = Some(arch);
        Ok(())
    }
    fn setup_cpu(&mut self, arch: Architecture) -> Result<()> {
        let pending_imports = std::mem::take(&mut self.host);
        self.create_cpu(arch)?;
        let uc = self.cpu_mut()?;
        uc.mem_map(TRAMPOLINE, TRAMPOLINE_SIZE, Prot::READ | Prot::EXEC)?;
        uc.mem_map(RETURN, PAGE, Prot::READ | Prot::EXEC)?;
        uc.mem_map(STACK, STACK_SIZE, Prot::READ | Prot::WRITE)?;
        if arch == Architecture::Arm32 {
            uc.mem_map(0xffff0000, PAGE, Prot::READ | Prot::EXEC)?;
            // Linux's cmpxchg64 occupies two slots (0xf60..0xf9f). Preserve r1
            // as well as the scratch callee-saved registers used for the values.
            for (address, words) in [
                (
                    0xffff0f60,
                    &[
                        0xe92d00f2u32, // push {r1,r4-r7}
                        0xe1c040d0,    // ldrd r4,r5,[r0]
                        0xe1c160d0,    // ldrd r6,r7,[r1]
                        0xe1b20f9f,    // retry: ldrexd r0,r1,[r2]
                        0xe0303004,    // eors r3,r0,r4
                        0x00313005,    // eorseq r3,r1,r5
                        0x01a23f96,    // strexdeq r3,r6,r7,[r2]
                        0x03330001,    // teqeq r3,#1
                        0x0afffff9,    // beq retry
                        0xe2730000,    // rsbs r0,r3,#0 (success => C set)
                        0xe8bd00f2,    // pop {r1,r4-r7}
                        0xe12fff1e,    // bx lr
                    ][..],
                ),
                (
                    0xffff0fc0,
                    &[
                        0xe1923f9f, // retry: ldrex r3,[r2]
                        0xe0533000, // subs r3,r3,r0
                        0x01823f91, // strexeq r3,r1,[r2]
                        0x03330001, // teqeq r3,#1
                        0x0afffffa, // beq retry
                        0xe2730000, // rsbs r0,r3,#0 (success => C set)
                        0xe12fff1e, // bx lr
                    ][..],
                ),
            ] {
                for (i, word) in words.iter().enumerate() {
                    uc.mem_write(address + i as u64 * 4, &word.to_le_bytes())?;
                }
            }
            // All guest accesses run on one CPU, so the memory barrier just returns.
            uc.mem_write(0xffff0fa0, &0xe12fff1eu32.to_le_bytes())?; // bx lr
            uc.mem_write(0xffff0fe0, &0xee1d0f70u32.to_le_bytes())?; // mrc p15,0,r0,c13,c0,3
            uc.mem_write(0xffff0fe4, &0xe12fff1eu32.to_le_bytes())?; // bx lr
            uc.mem_write(0xffff0ffc, &5u32.to_le_bytes())?;
        }
        self.setup_tls(&self.process_name.clone())?;
        let property = self.stub(HostFunction::Property)?;
        self.overrides
            .insert("__system_property_get".into(), property);
        let property_set = self.stub(HostFunction::PropertySet)?;
        self.overrides
            .insert("__system_property_set".into(), property_set);
        for name in [
            "dlopen",
            "dlsym",
            "dlclose",
            "dlerror",
            "dladdr",
            "dl_iterate_phdr",
        ] {
            let address = self.stub(HostFunction::Dl(name))?;
            self.overrides.insert(name.into(), address);
        }
        self.setup_assets()?;
        self.install_pending_imports(pending_imports)?;
        self.setup_jni()?;
        self.dlerror_buf = self.malloc(self.dlerror_capacity)?;
        self.trace_imports(self.trace.imports)?;
        Ok(())
    }
    /// 改写已映射的代码字节：整页换成匿名映射（共享文件映射上 mprotect 不可靠）。
    pub fn patch_code(&mut self, address: u64, bytes: &[u8]) -> Result<()> {
        let page = address & !0xfff;
        let mut data = self.read_bytes(page, 0x1000)?;
        let off = (address - page) as usize;
        data[off..off + bytes.len()].copy_from_slice(bytes);
        {
            let uc = self.cpu_mut()?;
            uc.mem_unmap(page, 0x1000)?;
            uc.mem_map(page, 0x1000, Prot(Prot::READ.0 | Prot::WRITE.0 | Prot::EXEC.0))?;
            uc.mem_write(page, &data)?;
            uc.mem_protect(page, 0x1000, Prot(Prot::READ.0 | Prot::EXEC.0))?;
            uc.ctl_flush_tb()?;
        }
        Ok(())
    }
    /// 铺一段零页窗口（录制下来的裸指针落点用）。
    pub fn map_window(&mut self, begin: u64, end: u64) -> Result<()> {
        self.cpu_mut()?.mem_map(begin, end - begin, Prot::READ | Prot::WRITE)?;
        Ok(())
    }
    /// 映射一页 RX 并写入指令（调试补丁用）。
    pub fn write_code(&mut self, address: u64, bytes: &[u8]) -> Result<()> {
        let size = ((bytes.len() as u64) + 0xfff) & !0xfff;
        self.cpu_mut()?.mem_map(address, size, Prot(Prot::READ.0 | Prot::EXEC.0))?;
        self.write_bytes(address, bytes)?;
        self.cpu_mut()?.ctl_flush_tb()?;
        Ok(())
    }
    /// 让 noreturn 的 `abort()` 变可返回：按帧指针把 SP/LR 还原到调用者。
    /// 仅用于调试（例如 libc++abi 的递归静态初始化错误）。
    pub fn abort_return(&mut self) -> Result<()> {
        let arch = self.architecture();
        let frame = if matches!(arch, Architecture::Arm64) { 29 } else { 11 };
        let fp = self.reg(frame)?;
        let (saved_fp, saved_lr, sp) = if matches!(arch, Architecture::Arm64) {
            let b = self.read_bytes(fp, 16)?;
            (
                u64::from_le_bytes(b[0..8].try_into().unwrap()),
                u64::from_le_bytes(b[8..16].try_into().unwrap()),
                fp + 0x10,
            )
        } else {
            let b = self.read_bytes(fp, 8)?;
            (
                u32::from_le_bytes(b[0..4].try_into().unwrap()) as u64,
                u32::from_le_bytes(b[4..8].try_into().unwrap()) as u64,
                fp + 8,
            )
        };
        self.write_reg(frame, saved_fp)?;
        self.cpu_mut()?.reg_write(arch.sp(), sp)?;
        self.cpu_mut()?.reg_write(arch.lr(), saved_lr)?;
        Ok(())
    }
    /// 调试：在地址区间打印每次命中
    pub fn hook_code_range(&mut self, begin: u64, end: u64) -> Result<()> {
        let uc = self.cpu_mut()?;
        uc.add_code_hook(begin, end, move |uc, address, _| {
            let a = uc.get_data().arch.unwrap();
            eprintln!("  [ch] tid={} 0x{address:x} x0=0x{:x} x9=0x{:x} x1=0x{:x} x2=0x{:x}", uc.get_data().thread, uc.reg_read(a.gp(0)).unwrap_or(0), uc.reg_read(a.gp(9)).unwrap_or(0), uc.reg_read(a.gp(1)).unwrap_or(0), uc.reg_read(a.gp(2)).unwrap_or(0));
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }
    /// 调试：监视对指定区间的读（`DYSIGN_WATCH=0xLO:len`，相对 libmetasec 基址）
    pub(crate) fn install_watch_hook(&mut self) -> Result<()> {
        let Some((lo, len)) = profile::watch_spec() else { return Ok(()) };
        let Some(base) = profile::module_base("libmetasec") else { return Ok(()) };
        let uc = self.cpu_mut()?;
        uc.add_mem_hook(HookType::MEM_READ, base + lo, base + lo + len, move |uc, _kind, address, _size, _| {
            let a = uc.get_data().arch.unwrap();
            let pc = uc.reg_read(a.pc()).unwrap_or(0);
            let tid = uc.get_data().thread;
            profile::watch_hit(pc, tid.into(), address - (base + lo));
            true
        })?;
        Ok(())
    }
    /// 调试：监视 guest 堆写（`DYSIGN_WWRITE=1`），配 `DYSIGN_TRACE_FILE` 落盘
    pub fn hook_heap_writes(&mut self) -> Result<()> {
        if !profile::wwrite_on() {
            return Ok(());
        }
        let uc = self.cpu_mut()?;
        let dsp = profile::dump_sp();
        for (lo, hi) in [(MMAP_START, BRK_START + 0x200_0000), (0x7000_0000u64, 0x7020_0000u64)] {
            uc.add_mem_hook(
                HookType::MEM_WRITE,
                lo,
                hi,
                move |uc, _kind, address, size, value| {
                    let a = uc.get_data().arch.unwrap();
                    let pc = uc.reg_read(a.pc()).unwrap_or(0);
                    let lr = uc.reg_read(a.lr()).unwrap_or(0);
                    let tid = uc.get_data().thread;
                    if let Some(s) = dsp {
                        let spv = uc.reg_read(a.sp()).unwrap_or(0);
                        if spv > s || spv + 0x8000 < s { return true; }
                    }
                    let bytes = (value as u64).to_le_bytes();
                    let n = size.min(bytes.len());
                    profile::write_hit(pc, tid.into(), lr, address, &bytes[..n]);
                    true
                },
            )?;
        }
        Ok(())
    }
    /// 调试：记录 guest malloc 调用与返回（`DYSIGN_ALLOC=1`）
    pub fn hook_alloc(&mut self, base: u64) -> Result<()> {
        if !profile::alloc_on() {
            return Ok(());
        }
        let site = base + 0x1bb08;
        if self.read_bytes(site, 4).is_err() {
            return Ok(());
        }
        let uc = self.cpu_mut()?;
        uc.add_code_hook(site, site + 4, move |uc, _addr, _size| {
            let a = uc.get_data().arch.unwrap();
            let size = uc.reg_read(a.gp(0)).unwrap_or(0);
            let lr = uc.reg_read(a.lr()).unwrap_or(0);
            let tid = uc.get_data().thread as u64;
            profile::alloc_hit(tid, size, lr, None);
            let one = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let _ = uc.add_code_hook(lr, lr + 4, move |uc, addr, _size| {
                if one.swap(true, std::sync::atomic::Ordering::Relaxed) {
                    return;
                }
                let a = uc.get_data().arch.unwrap();
                let ret = uc.reg_read(a.gp(0)).unwrap_or(0);
                let tid = uc.get_data().thread as u64;
                profile::alloc_hit(tid, size, addr, Some(ret));
                if profile::allocw_size() == size && ret != 0 {
                    profile::note_watched(ret, ret + size);
                    let _ = uc.add_mem_hook(
                        HookType::MEM_WRITE, ret, ret + size,
                        move |uc, _kind, address, size, value| {
                            let a = uc.get_data().arch.unwrap();
                            let pc = uc.reg_read(a.pc()).unwrap_or(0);
                            let tid = uc.get_data().thread;
                            let bytes = (value as u64).to_le_bytes();
                            let n = size.min(bytes.len());
                            profile::chase_hit(pc, tid.into(), address, &bytes[..n]);
                            true
                        },
                    );
                }
            });
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }
    /// 调试：监视对绝对区间的读（`DYSIGN_RWATCH=0xADDR:len`）
    pub fn hook_reads(&mut self) -> Result<()> {
        let regs = profile::rwatchreg_specs();
        if !regs.is_empty() {
            let uc = self.cpu_mut()?;
            let seen = std::sync::Arc::new(std::sync::Mutex::new(HashSet::<u64>::new()));
            uc.add_code_hook(0x10000000, 0x10600000, move |uc, address, _| {
                let a = uc.get_data().arch.unwrap();
                for r in regs.iter() {
                    if r.pc != address { continue; }
                    let p = uc.reg_read(a.gp(r.reg)).unwrap_or(0);
                    if p == 0 { continue; }
                    let key = p ^ (r.pc.rotate_left(17)) ^ (r.len << 48);
                    if !seen.lock().map(|mut m| m.insert(key)).unwrap_or(false) { continue; }
                    let tid = uc.get_data().thread;
                    eprintln!("[rreg] pc={} addr={} len={} tid={tid}", profile::loc(address), profile::loc(p), r.len);
                    let _ = uc.add_mem_hook(HookType::MEM_READ, p, p + r.len,
                        move |uc, _kind, addr, size, _| {
                            let a = uc.get_data().arch.unwrap();
                            let pc = uc.reg_read(a.pc()).unwrap_or(0);
                            let lr = uc.reg_read(a.lr()).unwrap_or(0);
                            let tid = uc.get_data().thread;
                            let mut buf = [0u8; 64];
                            let n = size.min(buf.len());
                            if n > 0 && uc.mem_read(addr, &mut buf[..n]).is_ok() {
                                profile::read_hit(pc, tid.into(), lr, addr, &buf[..n]);
                            }
                            true
                        });
                }
            })?;
            uc.ctl_flush_tb()?;
        }
        let Some((lo, len)) = profile::rwatch_spec() else { return Ok(()) };
        let uc = self.cpu_mut()?;
        uc.add_mem_hook(HookType::MEM_READ, lo, lo + len, move |uc, _kind, address, size, _| {
            let a = uc.get_data().arch.unwrap();
            let pc = uc.reg_read(a.pc()).unwrap_or(0);
            let lr = uc.reg_read(a.lr()).unwrap_or(0);
            let tid = uc.get_data().thread;
            let mut buf = [0u8; 64];
            let n = size.min(buf.len());
            if n > 0 && uc.mem_read(address, &mut buf[..n]).is_ok() {
                profile::read_hit(pc, tid.into(), lr, address, &buf[..n]);
            }
            true
        })?;
        Ok(())
    }
    /// 调试：在指定 PC 打印 x0..x3（`DYSIGN_ARGLOG=0xPC`）
    pub fn hook_args(&mut self) -> Result<()> {
        let Some(pc) = profile::arg_spec() else { return Ok(()) };
        let _ = pc;
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, addr, _| {
            if !profile::arg_specs().contains(&addr) { return; }
            let a = uc.get_data().arch.unwrap();
            let mut regs = [0u64; 31];
            for (i, r) in regs.iter_mut().take(29).enumerate() {
                *r = uc.reg_read(a.gp(i)).unwrap_or(0);
            }
            regs[29] = uc.reg_read(a.fp()).unwrap_or(0);
            regs[30] = uc.reg_read(a.lr()).unwrap_or(0);
            let tid = uc.get_data().thread;
            let lr = uc.reg_read(a.lr()).unwrap_or(0);
            if let Some(mx) = profile::memlogx_pc() {
                if mx == addr && regs[1] > 0 && regs[1] <= 256 {
                    let mut buf = vec![0u8; regs[1] as usize];
                    if uc.mem_read(regs[0], &mut buf).is_ok() {
                        profile::memlogx_hit(lr, regs[0], regs[1], &buf);
                        if regs[4] != 0 {
                            let mut kb = [0u8; 32];
                            if uc.mem_read(regs[4], &mut kb).is_ok() {
                                profile::memlogx_hit(lr, regs[4], 32, &kb);
                            }
                        }
                    }
                }
            }
            if let Some(m2) = profile::memlog2_pc() {
                if m2 == addr && profile::is_watched(regs[0]) && regs[2] > 0 && regs[2] <= 128 {
                    let mut buf = vec![0u8; regs[2] as usize];
                    if uc.mem_read(regs[1], &mut buf).is_ok() {
                        profile::memlog2_hit(lr, regs[0], regs[1], regs[2], &buf);
                    }
                }
            }
            if let Some((cpc, caddr)) = profile::chase_spec() {
                if cpc == addr && regs[0] == caddr && regs[2] > 0 && regs[2] <= 128 {
                    let src = regs[1];
                    let len = regs[2];
                    let _ = uc.add_mem_hook(
                        HookType::MEM_WRITE, src, src + len,
                        move |uc, _kind, address, size, value| {
                            let a = uc.get_data().arch.unwrap();
                            let pc = uc.reg_read(a.pc()).unwrap_or(0);
                            let tid = uc.get_data().thread;
                            let bytes = (value as u64).to_le_bytes();
                            let n = size.min(bytes.len());
                            profile::chase_hit(pc, tid.into(), address, &bytes[..n]);
                            true
                        },
                    );
                }
            }
            if let Some((mpc, maddr)) = profile::memlog_spec() {
                if mpc == addr {
                    let dst = regs[0];
                    let src = regs[1];
                    let len = regs[2];
                    if dst == maddr && len <= 128 && len > 0 {
                        let mut buf = vec![0u8; len as usize];
                        if uc.mem_read(src, &mut buf).is_ok() {
                            profile::memlog_hit(dst, src, len, &buf);
                        }
                    }
                }
            }
            let mut insn = [0u8; 4];
            let _ = uc.mem_read(addr, &mut insn);
            let sp = uc.reg_read(a.sp()).unwrap_or(0);
            profile::arg_hit_sp(addr, tid.into(), &regs, sp, lr);
            profile::arg_hit(addr, tid.into(), &[]);
            profile::arg_hit(0, u32::from_le_bytes(insn) as u64, &[]);
            profile::arg_hit(1, lr, &[]);
            if std::env::var("DYSIGN_STACK").is_ok() {
                let mut fp = uc.reg_read(unicorn_engine::RegisterARM64::FP as i32).unwrap_or(0);
                for _ in 0..40 {
                    if fp < 0x1000 || fp % 8 != 0 { break; }
                    let mut pair = [0u8; 16];
                    if uc.mem_read(fp, &mut pair).is_err() { break; }
                    let prev = u64::from_le_bytes(pair[0..8].try_into().unwrap());
                    let ret = u64::from_le_bytes(pair[8..16].try_into().unwrap());
                    profile::arg_hit(2, ret, &[]);
                    if prev <= fp { break; }
                    fp = prev;
                }
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：在指定 PC 执行前按寄存器/绝对地址改写内存（`DYSIGN_PATCH=pc:xREG:off:hex[:occ]`）
    pub fn hook_patch(&mut self) -> Result<()> {
        if profile::patch_specs().is_empty() && profile::setreg_specs().is_empty() { return Ok(()); }
        // 镜像代码段是只读文件映射；打补丁前先整体私有化并放开写权限。
        if !profile::patch_specs().is_empty() {
            let runs: Vec<(u64, u64)> = self
                .regions
                .iter()
                .filter(|r| r.shared.is_some())
                .map(|r| (r.start, r.end))
                .collect();
            for (a, b) in runs {
                self.protect(a, b - a, Prot(Prot::READ.0 | Prot::WRITE.0 | Prot::EXEC.0))?;
            }
        }
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, address, _| {
            let a = uc.get_data().arch.unwrap();
            for r in profile::setreg_specs() {
                if r.pc != address { continue; }
                let cur = profile::patch_occ(address ^ 0x5e7);
                if cur != r.occ { continue; }
                if uc.reg_write(a.gp(r.reg), r.val).is_ok() {
                    eprintln!("[setreg] pc={} occ={} x{}={:#x}", profile::loc(address), cur, r.reg, r.val);
                }
            }
            for p in profile::patch_specs() {
                if p.pc != address { continue; }
                let cur = profile::patch_occ(address);
                // 带寄存器等值条件时按条件命中，不再看序号。
                if p.eq.is_none() && cur != p.occ { continue; }
                eprintln!("[patchdbg] pc={} occ={} x0=0x{:x} x19=0x{:x} x20=0x{:x} sp=0x{:x}", profile::loc(address), cur, uc.reg_read(a.gp(0)).unwrap_or(0), uc.reg_read(a.gp(19)).unwrap_or(0), uc.reg_read(a.gp(20)).unwrap_or(0), uc.reg_read(a.sp()).unwrap_or(0));
                let base = match p.base {
                    Some(r) => uc.reg_read(a.gp(r)).unwrap_or(0),
                    None => 0,
                };
                if p.base.is_some() && base == 0 { continue; }
                if let Some(want) = p.eq {
                    if base != want { continue; }
                }
                let dst = base.wrapping_add(p.abs).wrapping_add(p.off);
                if uc.mem_write(dst, &p.bytes).is_ok() {
                    eprintln!("[patch] pc={} dst={} occ={} bytes={}", profile::loc(address), profile::loc(dst), cur, p.bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
                }
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：在指定 PC 打入口参数与 x30，并在返回地址打 x0（`DYSIGN_RETLOG=0xPC`）。
    /// 尾调用（`b`）不算调用者，按指令字区分。
    pub fn hook_retlog(&mut self) -> Result<()> {
        let Some(pc) = profile::retlog_pc() else { return Ok(()) };
        let frames = std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::HashMap::<(u64, u64), Vec<u64>>::new(),
        ));
        let last = std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::HashMap::<u64, u64>::new(),
        ));
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, addr, _| {
            let a = uc.get_data().arch.unwrap();
            let tid = uc.get_data().thread as u64;
            let lr = uc.reg_read(a.lr()).unwrap_or(0);
            let prev = last.lock().unwrap_or_else(|e| e.into_inner()).insert(tid, addr);
            if addr != pc {
                let mut f = frames.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(v) = f.get_mut(&(tid, addr)) {
                    if let Some(entry) = v.pop() {
                        let x0 = uc.reg_read(a.gp(0)).unwrap_or(0);
                        eprintln!("[retlog] RETURN entry={} tid={tid} x0=0x{x0:x}", profile::loc(entry));
                    }
                }
                return;
            }
            let mut regs = [0u64; 8];
            for (i, r) in regs.iter_mut().enumerate() {
                *r = uc.reg_read(a.gp(i)).unwrap_or(0);
            }
            let sp = uc.reg_read(a.sp()).unwrap_or(0);
            let mut insn = [0u8; 4];
            let kind = match prev {
                Some(p) if uc.mem_read(p, &mut insn).is_ok() => {
                    let w = u32::from_le_bytes(insn);
                    let target = p.wrapping_add(((w as i32) << 6 >> 4) as u64);
                    match w & 0xfc00_0000 {
                        0x1400_0000 if target == addr => "tail",
                        0x9400_0000 => "bl",
                        _ => "seq",
                    }
                }
                _ => "?",
            };
            eprintln!(
                "[retlog] ENTRY pc={} tid={tid} caller={} kind={kind} x0..7={:x?} x30=0x{lr:x} sp=0x{sp:x}",
                profile::loc(addr),
                profile::loc(lr),
                regs
            );
            frames.lock().unwrap_or_else(|e| e.into_inner())
                .entry((tid, lr)).or_default().push(addr);
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：命中 PC 时向固定地址写哨兵字节（`DYSIGN_ANCHOR=0xPC:0xADDR:len`）。
    pub fn hook_anchor(&mut self) -> Result<()> {
        let specs = profile::anchor_specs().to_vec();
        if specs.is_empty() { return Ok(()); }
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, addr, _| {
            for a in &specs {
                if a.pc != addr { continue; }
                let bytes = vec![0xa5u8; a.len as usize];
                if uc.mem_write(a.addr, &bytes).is_ok() {
                    profile::anchor_hit();
                    eprintln!("[anchor] pc={} addr=0x{:x} len={}", profile::loc(addr), a.addr, a.len);
                }
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：记录执行过的代码页（`DYSIGN_EXECPAGES=path`，每页一行十六进制，去重）。
    pub fn hook_execpages(&mut self) -> Result<()> {
        let Ok(out) = std::env::var("DYSIGN_EXECPAGES") else { return Ok(()) };
        let bits = std::sync::Arc::new((0..64).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>());
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |_uc, addr, _| {
            let pg = (addr >> 12) as u32;
            let (i, b) = (((pg & 0xfff) >> 6) as usize, 1u64 << (pg & 63));
            if bits[i].fetch_or(b, std::sync::atomic::Ordering::Relaxed) & b == 0 {
                use std::io::Write;
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&out) {
                    let _ = writeln!(f, "{pg:x}");
                }
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：命中指定 PC 时导出与 `medusa-snap.bin` 同格式的全量快照
    /// （`DYSIGN_SNAP_AT=0xPC` + `DYSIGN_SNAP_OUT=path`，只导 < 4G 的低地址页）。
    pub fn hook_snap_pc(&mut self) -> Result<()> {
        let Some(pc) = profile::snap_at() else { return Ok(()) };
        let Ok(out) = std::env::var("DYSIGN_SNAP_OUT") else { return Ok(()) };
        let ranges: Vec<(u64, u64, String)> = self
            .maps()?
            .into_iter()
            .filter(|m| m.end <= 0x1_0000_0000)
            .map(|m| (m.start, m.end, m.name))
            .collect();
        eprintln!("[snap] hook pc={} out={out} ranges={:?}", profile::loc(pc), ranges);
        let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let uc = self.cpu_mut()?;
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, addr, _| {
            if addr != 0x1004e710 || done.load(std::sync::atomic::Ordering::Relaxed) { return; }
            let a = uc.get_data().arch.unwrap();
            let x0 = uc.reg_read(a.gp(0)).unwrap_or(0);
            if x0 != pc { return; }
            done.store(true, std::sync::atomic::Ordering::Relaxed);
            let sp = uc.reg_read(a.sp()).unwrap_or(0);
            let mut b = [0u8; 8];
            let base = if uc.mem_read(sp + 0x38, &mut b).is_ok() { u64::from_le_bytes(b) } else { 0 };
            let mut buf = Vec::new();
            buf.extend_from_slice(&x0.to_le_bytes());
            for i in 0..48u64 {
                let mut s = [0u8; 8];
                let _ = uc.mem_read(base + 8 + 8 * i, &mut s);
                buf.extend_from_slice(&s);
            }
            let mut pages: Vec<u64> = Vec::new();
            for (lo, hi, _n) in &ranges {
                let mut p = *lo & !0xfff;
                while p < *hi {
                    pages.push(p);
                    p += 0x1000;
                }
            }
            buf.extend_from_slice(&(pages.len() as u32).to_le_bytes());
            let mut ok = true;
            for p in pages {
                let mut d = [0u8; 0x1000];
                if uc.mem_read(p, &mut d).is_err() { ok = false; break; }
                buf.extend_from_slice(&p.to_le_bytes());
                buf.extend_from_slice(&d);
            }
            if ok {
                let _ = std::fs::write(&out, &buf);
                eprintln!("[snap] pc={} pages={} bytes={} -> {out}", profile::loc(addr), (buf.len() - 8 - 48 * 8 - 4) / 0x1008, buf.len());
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：按区间逐条记录指令与寄存器（`DYSIGN_CTRACE=0xLO:0xHI`，绝对地址）
    pub fn hook_ctrace(&mut self) -> Result<()> {
        let Some((lo, hi)) = profile::ctrace_spec() else { return Ok(()) };
        let uc = self.cpu_mut()?;
        let csp = profile::ctrace_sp();
        uc.add_code_hook(lo, hi, move |uc, address, _| {
            let a = uc.get_data().arch.unwrap();
            if let Some(w) = csp {
                if uc.reg_read(a.sp()).unwrap_or(0) != w { return; }
            }
            let mut regs = [0u64; 31];
            for (i, r) in regs.iter_mut().take(29).enumerate() {
                *r = uc.reg_read(a.gp(i)).unwrap_or(0);
            }
            regs[29] = uc.reg_read(a.fp()).unwrap_or(0);
            regs[30] = uc.reg_read(a.lr()).unwrap_or(0);
            let sp = uc.reg_read(a.sp()).unwrap_or(0);
            let tid = uc.get_data().thread;
            let mut insn = [0u8; 4];
            let _ = uc.mem_read(address, &mut insn);
            profile::ctrace_hit(address, tid.into(), u32::from_le_bytes(insn), &regs, sp);
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：sp 等于指定值时逐条打印 PC 与 x0..x3（`DYSIGN_SPLOG=0xSP[:max]`）
    pub fn hook_splog(&mut self) -> Result<()> {
        let Some((want, max)) = profile::splog_spec() else { return Ok(()) };
        let uc = self.cpu_mut()?;
        let seen = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, addr, _| {
            let a = uc.get_data().arch.unwrap();
            if uc.reg_read(a.sp()).unwrap_or(0) != want { return; }
            if seen.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= max { return; }
            let tid = uc.get_data().thread;
            let mut regs = [0u64; 4];
            for (i, r) in regs.iter_mut().enumerate() {
                *r = uc.reg_read(a.gp(i)).unwrap_or(0);
            }
            profile::splog_hit(addr, tid.into(), &regs);
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 调试：命中指定 PC 时转储内存（`DYSIGN_DUMP=0xPC:0xADDR:len`）
    pub fn hook_dump(&mut self) -> Result<()> {
        let specs = profile::dump_specs().clone();
        if specs.is_empty()
            && profile::dumpptr_specs().is_empty()
            && profile::dumpreg_specs().is_empty()
            && profile::fpchain_specs().is_empty()
            && profile::dumpx_specs().is_empty()
            && profile::dumpsp_specs().is_empty()
            && profile::regs_sp().is_none()
            && profile::dumpall_spec().is_none()
            && profile::regs_pc().is_none()
        {
            return Ok(());
        }
        let uc = self.cpu_mut()?;
        let dsp = profile::dump_sp();
        let dall = profile::dumpall_spec();
        let dall_done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let dall_cnt = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let mut seen: std::collections::HashSet<(u64, u64, u64)> = std::collections::HashSet::new();
        uc.add_code_hook(0x10000000, 0x10600000, move |uc, address, _| {
            let a_arch = uc.get_data().arch.unwrap();
            let spv = uc.reg_read(a_arch.sp()).unwrap_or(0);
            if let Some(s) = dsp {
                if spv != s { return; }
            }
            if profile::regs_pc() == Some(address) {
                let mut regs = Vec::new();
                for i in 0..29u32 {
                    regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X0 as i32 + i as i32).unwrap_or(0));
                }
                regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X29).unwrap_or(0));
                regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X30).unwrap_or(0));
                profile::reg_hit(address, spv, &regs);
            }
            if let Some(rs) = profile::regs_sp() {
                if spv == rs {
                    let mut regs = Vec::new();
                    for i in 0..29u32 {
                        regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X0 as i32 + i as i32).unwrap_or(0));
                    }
                    regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X29).unwrap_or(0));
                    regs.push(uc.reg_read(unicorn_engine::RegisterARM64::X30).unwrap_or(0));
                    profile::reg_hit(address, spv, &regs);
                }
            }
            if let Some((apc, dir, nth)) = &dall {
                if address == *apc
                    && profile::regs_sp().is_none_or(|r| r == spv)
                    && dall_cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1 >= *nth
                    && !dall_done.swap(true, std::sync::atomic::Ordering::Relaxed)
                {
                    let _ = std::fs::create_dir_all(dir);
                    for r in uc.mem_regions().unwrap_or_default() {
                        let len = r.end - r.begin + 1;
                        if len == 0 || len > 0x400_0000 { continue; }
                        let mut buf = vec![0u8; len as usize];
                        if uc.mem_read(r.begin, &mut buf).is_ok() {
                            let _ = std::fs::write(format!("{dir}/{:012x}.bin", r.begin), &buf);
                        }
                    }
                    eprintln!("[dumpall] wrote to {dir} at pc=0x{address:x}");
                }
            }
            for (pc, addr, len) in &specs {
                if *pc != address { continue; }
                if !seen.insert((*pc, *addr, *len)) { continue; }
                let mut buf = vec![0u8; *len as usize];
                if uc.mem_read(*addr, &mut buf).is_ok() {
                    profile::dump_hit(address, spv, *addr, &buf);
                }
            }
            for (pc, reg, len) in profile::dumpreg_specs() {
                if *pc != address { continue; }
                if !seen.insert((*pc, 0xdead0000 | *reg as u64, *len)) { continue; }
                let a = uc.get_data().arch.unwrap();
                let Some(p) = uc.reg_read(a.gp(*reg)).ok() else { continue };
                let mut data = vec![0u8; *len as usize];
                if p != 0 && uc.mem_read(p, &mut data).is_ok() {
                    profile::dump_hit(address, spv, p, &data);
                }
            }
            for (pc, sp, addr, len) in profile::dumpsp_specs() {
                if *pc != address || *sp != spv { continue; }
                let mut buf = vec![0u8; *len as usize];
                if uc.mem_read(*addr, &mut buf).is_ok() {
                    profile::dump_hit(*pc, spv, *addr, &buf);
                }
            }
            for (pc, reg, val, addr, len) in profile::dumpx_specs() {
                if *pc != address { continue; }
                let a = uc.get_data().arch.unwrap();
                let Some(r) = uc.reg_read(a.gp(*reg)).ok() else { continue };
                if r != *val { continue; }
                let mut buf = vec![0u8; *len as usize];
                if uc.mem_read(*addr, &mut buf).is_ok() {
                    profile::dump_hit(*pc, spv, *addr, &buf);
                }
            }
            for (pc, reg, val, depth) in profile::fpchain_specs() {
                if *pc != address { continue; }
                let a = uc.get_data().arch.unwrap();
                let Some(r) = uc.reg_read(a.gp(*reg)).ok() else { continue };
                if r != *val { continue; }
                let tid = uc.get_data().thread;
                let sp = uc.reg_read(a.sp()).unwrap_or(0);
                let mut chain = Vec::new();
                let mut fp = uc.reg_read(unicorn_engine::RegisterARM64::FP as i32).unwrap_or(0);
                for _ in 0..*depth {
                    if fp < 0x1000 || fp % 8 != 0 { break; }
                    let mut pair = [0u8; 16];
                    if uc.mem_read(fp, &mut pair).is_err() { break; }
                    let prev = u64::from_le_bytes(pair[0..8].try_into().unwrap());
                    let ret = u64::from_le_bytes(pair[8..16].try_into().unwrap());
                    chain.push((fp, ret));
                    if prev <= fp || prev % 8 != 0 || prev >= 0x70200000 { break; }
                    fp = prev;
                }
                profile::fpchain_hit(*pc, tid.into(), sp, &chain);
            }
            for (pc, reg, len) in profile::dumpptr_specs() {
                if *pc != address { continue; }
                eprintln!("[dumpptr] hit pc=0x{address:x} reg={reg}");
                let a = uc.get_data().arch.unwrap();
                let Some(p) = uc.reg_read(a.gp(*reg)).ok() else { continue };
                let mut buf = vec![0u8; *len as usize];
                match uc.mem_read(p, &mut buf[..8]) {
                    Ok(()) => {
                        let ptr = u64::from_le_bytes(buf[..8].try_into().unwrap());
                        eprintln!("[dumpptr] p=0x{p:x} ptr=0x{ptr:x}");
                        let mut data = vec![0u8; *len as usize];
                        if ptr != 0 {
                            match uc.mem_read(ptr, &mut data) { Ok(()) => profile::dump_hit(address, spv, ptr, &data), Err(e) => eprintln!("[dumpptr] read err {e:?}") }
                        }
                    }
                    Err(e) => eprintln!("[dumpptr] ptr read err {e:?}"),
                }
            }
        })?;
        uc.ctl_flush_tb()?;
        Ok(())
    }

    /// 当前 guest 线程号（观测锁/线程用）
    pub fn tid(&self) -> u32 {
        self.threads.tid()
    }
    fn cpu(&self) -> Result<&Unicorn<'static, CpuData>> {
        self.uc.as_ref().ok_or_else(|| NO_LIBRARY.into())
    }
    fn cpu_mut(&mut self) -> Result<&mut Unicorn<'static, CpuData>> {
        self.uc.as_mut().ok_or_else(|| NO_LIBRARY.into())
    }
    fn setup_tls(&mut self, process: &str) -> Result<()> {
        let p = self.pointer_size();
        let tls = INITIAL_SP + 0x1000;
        let thread = tls + 0x400;
        let kab = tls + 0x800;
        let argv = tls + 0x900;
        let envp = tls + 0xa00;
        let auxv = tls + 0xb00;
        let name = tls + 0xc00;
        self.write_bytes(name, process.as_bytes())?;
        self.write_bytes(name + process.len() as u64, &[0])?;
        self.write_pointer(argv, name)?;
        self.write_pointer(kab, 1)?;
        self.write_pointer(kab + p, argv)?;
        self.write_pointer(kab + 2 * p, envp)?;
        self.write_pointer(kab + 3 * p, auxv)?;
        let mut cursor = name + page_up(process.len() as u64 + 1);
        for (i, value) in [
            "ANDROID_DATA=/data",
            "ANDROID_ROOT=/system",
            "PATH=/sbin:/vendor/bin:/system/sbin:/system/bin:/system/xbin",
            "NO_ADDR_COMPAT_LAYOUT_FIXUP=1",
        ]
        .iter()
        .enumerate()
        {
            self.write_pointer(envp + i as u64 * p, cursor)?;
            self.write_bytes(cursor, value.as_bytes())?;
            cursor += value.len() as u64 + 1;
        }
        self.write_pointer(auxv, 25)?;
        self.write_pointer(auxv + p, tls + 0xe00)?;
        self.write_pointer(auxv + 2 * p, 6)?;
        self.write_pointer(auxv + 3 * p, PAGE)?;
        self.write_pointer(tls, tls)?;
        self.write_pointer(tls + p, thread)?;
        self.write_pointer(tls + 3 * p, kab)?;
        self.write_bytes(thread + 2 * p, &threads::MAIN_TID.to_le_bytes())?;
        let reg = self.architecture().tls();
        self.cpu_mut()?.reg_write(reg, tls)?;
        self.set_sp(INITIAL_SP)?;
        Ok(())
    }
    fn stub(&mut self, function: HostFunction) -> Result<u64> {
        if self.host.len() >= TRAMPOLINE_STUBS {
            return Err(
                "trampoline area full: more than 8192 host functions and unresolved symbols".into(),
            );
        }
        let address = TRAMPOLINE + self.host.len() as u64 * 8;
        if std::env::var("DYSIGN_STUBS").is_ok() {
            match &function {
                HostFunction::Import { name, original } => eprintln!(
                    "STUB {:#x} idx {} import {} original {:?}",
                    address,
                    self.host.len(),
                    name,
                    original
                ),
                HostFunction::Unresolved { symbol, module } => eprintln!(
                    "STUB {:#x} idx {} unresolved {}@{}",
                    address,
                    self.host.len(),
                    symbol,
                    module
                ),
                HostFunction::Jni(i) => {
                    eprintln!("STUB {:#x} idx {} jni {}", address, self.host.len(), i)
                }
                HostFunction::Vm(i) => {
                    eprintln!("STUB {:#x} idx {} vm {}", address, self.host.len(), i)
                }
                HostFunction::Dl(n) => {
                    eprintln!("STUB {:#x} idx {} dl {}", address, self.host.len(), n)
                }
                HostFunction::Asset(n) => {
                    eprintln!("STUB {:#x} idx {} asset {}", address, self.host.len(), n)
                }
                HostFunction::Property => {
                    eprintln!("STUB {:#x} idx {} property", address, self.host.len())
                }
                HostFunction::PropertySet => {
                    eprintln!("STUB {:#x} idx {} property_set", address, self.host.len())
                }
            }
        }
        self.write_bytes(address, &self.architecture().stub())?;
        self.host.push(function);
        Ok(address)
    }
}

#[cfg(test)]
#[path = "../tests/support/apk.rs"]
mod test_apk;

#[cfg(test)]
#[path = "../tests/support/elf.rs"]
mod test_elf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm32_kuser_barrier_tls_and_version_work_without_threads() -> Result<()> {
        let mut emu = Emulator::builder().build()?;
        emu.load_library("rootfs/lib/libc.so")?;
        assert_eq!(emu.call_function(0xffff0fa0, &[42])?, 42);
        let tls = emu.cpu()?.reg_read(emu.architecture().tls())?;
        assert_eq!(emu.call_function(0xffff0fe0, &[])?, tls);
        assert_eq!(emu.read_bytes(0xffff0ffc, 4)?, 5u32.to_le_bytes());
        let region = emu
            .cpu()?
            .mem_regions()?
            .into_iter()
            .find(|region| region.begin == 0xffff0000)
            .unwrap();
        assert_eq!(region.end, 0xffff0fff);
        assert_eq!(region.perms, (Prot::READ | Prot::EXEC).0);
        Ok(())
    }

    #[test]
    fn arm32_kuser_cmpxchg_results_carry_and_registers() -> Result<()> {
        use unicorn_engine::RegisterARM;

        let mut emu = Emulator::builder().build()?;
        emu.load_library("rootfs/lib/libc.so")?;
        let old = 0x1234_5678_89ab_cdefu64;
        let new = 0xfedc_ba98_7654_3210u64;
        let old_ptr = emu.malloc(8)?;
        let new_ptr = emu.malloc(8)?;
        let target = emu.malloc(16)?;
        emu.write_u64(old_ptr, old)?;
        emu.write_u64(new_ptr, new)?;
        for (helper, width) in [(0xffff0fc0, 4), (0xffff0f60, 8)] {
            let args = if width == 4 {
                [old as u32 as u64, new as u32 as u64, target]
            } else {
                [old_ptr, new_ptr, target]
            };
            let mismatches: &[u64] = if width == 4 {
                &[0, 1]
            } else {
                &[0, 1, 1 << 32]
            };
            for &mismatch in mismatches {
                let before = old ^ mismatch;
                emu.write_bytes(target, &[0xa5; 16])?;
                emu.write_bytes(target, &before.to_le_bytes()[..width])?;
                for reg in 4..12 {
                    emu.write_reg(reg, 0x12340000 + reg as u64)?;
                }
                let sp = emu.sp()?;
                let cpsr = emu.cpu()?.reg_read(RegisterARM::CPSR)?;
                // Start with the opposite C flag, so both outcomes must set it.
                emu.cpu_mut()?.reg_write(
                    RegisterARM::CPSR,
                    (cpsr & !(1 << 29)) | if mismatch == 0 { 0 } else { 1 << 29 },
                )?;
                let result = emu.call_function(helper, &args)?;
                assert_eq!(result == 0, mismatch == 0, "helper=0x{helper:x}");
                assert_eq!(
                    emu.cpu()?.reg_read(RegisterARM::CPSR)? & (1 << 29) != 0,
                    mismatch == 0
                );
                let expected = if mismatch == 0 { new } else { before };
                assert_eq!(
                    emu.read_bytes(target, width)?,
                    expected.to_le_bytes()[..width]
                );
                assert_eq!(
                    emu.read_bytes(target + width as u64, 16 - width)?,
                    vec![0xa5; 16 - width]
                );
                assert_eq!(emu.reg(1)?, args[1]);
                assert_eq!(emu.reg(2)?, target);
                for reg in 4..12 {
                    assert_eq!(emu.reg(reg)?, 0x12340000 + reg as u64);
                }
                assert_eq!(emu.sp()?, sp);
                assert_eq!(emu.lr()?, RETURN);
                assert_eq!(emu.read_u64(old_ptr)?, old);
                assert_eq!(emu.read_u64(new_ptr)?, new);
            }
        }
        Ok(())
    }
}
