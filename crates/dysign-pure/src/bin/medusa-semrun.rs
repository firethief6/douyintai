//! 单字差分：同一个字分别用 Rust 语义与真实引擎 handler 执行，比较可观察结果。
//!
//! 用例 JSONL：{"id":n,"pc":"0x..","slots":["0x..";64],"mem":[["0x..","hex"],..]}
//! 只比较用例内存窗口和确实被改写的槽，排除 pc/mode/frame 一类记账槽。
use dysign_pure::medusa::{native_program, Vm};
use std::io::{BufRead, Write};

const SCRATCH_A: u64 = 0x7000_0000;
const SCRATCH_B: u64 = 0x7000_0800;
const WIN: u64 = 0x400;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

struct Case {
    id: u64,
    pc: u64,
    slots: [u64; 64],
    mem: Vec<(u64, Vec<u8>)>,
}

fn parse(l: &str) -> Option<Case> {
    let v: serde_json::Value = serde_json::from_str(l).ok()?;
    let mut slots = [0u64; 64];
    for (i, s) in v["slots"].as_array()?.iter().enumerate() {
        let s = s.as_str()?;
        slots[i] = u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()?;
    }
    let mut mem = Vec::new();
    for m in v["mem"].as_array()? {
        let a = u64::from_str_radix(m[0].as_str()?.trim_start_matches("0x"), 16).ok()?;
        let hs = m[1].as_str()?;
        let d: Vec<u8> = (0..hs.len() / 2)
            .map(|i| u8::from_str_radix(&hs[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        mem.push((a, d));
    }
    Some(Case {
        id: v["id"].as_u64()?,
        pc: u64::from_str_radix(v["pc"].as_str()?.trim_start_matches("0x"), 16).ok()?,
        slots,
        mem,
    })
}

fn reset(vm: &mut Vm, c: &Case) {
    vm.slots = c.slots;
    vm.pc = c.pc;
    vm.sp = 0x7000_3000;
    vm.slots_addr = 0x7000_4000;
    vm.slot_dirty = 0;
    vm.regs = [0u64; 32];
    vm.regs[6] = vm.slots_addr;
    vm.regs[27] = vm.slots_addr + 0x140;
    vm.regs[14] = vm.slots_addr + 8;
    vm.regs[29] = vm.sp + 0x110;
    for (a, d) in &c.mem {
        let _ = vm.mem.write(*a, d);
    }
    for i in 0..64u64 {
        let _ = vm.mem.w64(vm.slots_addr + 8 + 8 * i, c.slots[i as usize]);
    }
    let _ = vm.mem.w64(vm.sp + 0x38, vm.slots_addr);
    let _ = vm.mem.w64(vm.slots_addr, c.pc);
    let _ = vm.mem.w64(vm.sp + 0x38, vm.slots_addr);
    let _ = vm.mem.w64(vm.sp + 0x6c, vm.slots_addr & 0xffff_ffff);
}

fn win(vm: &Vm) -> (Vec<u8>, Vec<u8>) {
    (
        vm.mem.read(SCRATCH_A, WIN as usize).unwrap_or_default(),
        vm.mem.read(SCRATCH_B, WIN as usize).unwrap_or_default(),
    )
}

fn main() {
    let snap = std::fs::read(std::env::args().nth(1).expect("缺快照")).expect("读快照失败");
    let path = std::env::args().nth(2).expect("缺用例文件");
    let mut vm = Vm::from_snapshot(&snap).expect("快照损坏");
    vm.mem.map(SCRATCH_A, &vec![0u8; 0x6000]);
    let skipped = 0usize;
    let out = std::io::stdout();
    let mut out = out.lock();
    for line in std::io::BufReader::new(std::fs::File::open(&path).unwrap()).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let Some(c) = parse(&line) else {
            eprintln!("BAD {line}");
            continue;
        };
        dysign_pure::medusa::CASEID.store(c.id, std::sync::atomic::Ordering::Relaxed);
        let _pv = vm.mem.ld(0x102d_6234, 4).unwrap_or(0);
        if _pv != 0x8000_c118 {
            eprintln!("CLOB id={} v={:#x}", c.id, _pv);
        }
        reset(&mut vm, &c);
        let before = vm.slots;
        let (_ra, _rb) = win(&vm);
        let mut err_r = String::new();
        if std::env::var("SEMDBG").is_ok() {
            eprintln!(
                "DBG pc={:#x} s29={:#x} s1={:#x} s6={:#x}",
                c.pc, vm.slots[29], vm.slots[1], vm.slots[6]
            );
        }
        let mut r_next = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            native_program::exec(&mut vm, c.pc)
        })) {
            Ok(Ok(n)) => r_next = Some(n),
            Ok(Err(e)) => err_r = format!("{e:?}"),
            Err(_) => err_r = "panic".into(),
        }
        for n in 0..64u64 {
            if vm.slot_dirty & (1 << n) != 0 {
                let _ = vm
                    .mem
                    .st(vm.slots_addr + 8 + 8 * n, 8, vm.slots[n as usize]);
            }
        }
        vm.slot_dirty = 0;
        let mut rs = [0u64; 64];
        for n in 0..64u64 {
            rs[n as usize] = vm.mem.ld(vm.slots_addr + 8 + 8 * n, 8).unwrap_or(0);
        }
        let rw = win(&vm);
        vm.mem.reset_dirty();

        reset(&mut vm, &c);
        // 转换桩 0x10053074 从帧里取源/目的槽指针（[sp+8]/[sp+0x10]），
        // 单字入口不走写这两个字段的外层循环，这里按指令的 d/s 补齐。
        if let Some(i) = dysign_pure::medusa::words::find(c.pc) {
            if matches!(
                i.kind,
                dysign_pure::medusa::Kind::u32f64 | dysign_pure::medusa::Kind::i32f64
            ) {
                let sa = vm.slots_addr;
                let _ = vm.mem.w64(vm.sp + 8, sa + 8 + 8 * i.d as u64);
                let _ = vm.mem.w64(vm.sp + 0x10, sa + 8 + 8 * i.s as u64);
            }
        }
        let mut err_e = String::new();
        let mut sp = vm.sp;
        let mut regs = vm.regs;
        let sc = vm.slots;
        regs[0] = c.pc;
        if std::env::var("SEMDBG").is_ok() {
            eprintln!(
                "CALL pc={:#x} x0={:#x} regs6={:#x} word={:?}",
                c.pc,
                regs[0],
                regs[6],
                vm.mem.read(c.pc, 4)
            );
        }
        let tracing = std::env::var("SEMTRACE").is_ok();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vm.nat.call(
                &mut vm.mem,
                &sc,
                0x1004_e710,
                &mut sp,
                &mut regs,
                false,
                true,
            )
        }));
        if tracing {
            let t = dysign_pure::medusa::native::trace_take();
            eprintln!("TRACE {:x?}", &t[t.len().saturating_sub(60)..]);
        }
        match r {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => err_e = format!("{e:?}"),
            Err(_) => err_e = "panic".into(),
        }
        if std::env::var("SEMDBG").is_ok() {
            eprintln!(
                "DBGR pc={:#x} word={:?} rs10={:#x} rs11={:#x} rs53={:#x} rs54={:#x} rs1={:#x} m={:02x?}",
                c.pc,
                vm.mem.read(c.pc, 4),
                rs[10], rs[11],
                rs[53],
                rs[54],
                rs[1],
                vm.mem.read(0x700011b0, 16).unwrap_or_default()
            );
        }
        let mut es = [0u64; 64];
        for n in 0..64u64 {
            es[n as usize] = vm.mem.ld(vm.slots_addr + 8 + 8 * n, 8).unwrap_or(0);
        }
        if std::env::var("SEMDBG").is_ok() {
            eprintln!(
                "DBGE ee={err_e} x0={:#x} x27={:#x} es1={:#x} es2={:#x} es3={:#x} rs1={:#x} rs2={:#x} s1={:#x} s2={:#x}",
                regs[0], regs[27], es[1], es[2], es[3],
                rs[1], rs[2],
                vm.slots[1], vm.slots[2]
            );
        }
        let ew = win(&vm);
        // 跳板的下一 pc 由 mode 决定（0 落到 [x6]，其余跳槽 36），引擎在分发点停下后
        // 可直接读取；把它纳入比较，避免“跳到槽 36”和“落到下一字”互相冒充。
        let is_tramp = matches!(
            dysign_pure::medusa::words::find(c.pc).map(|i| i.kind),
            Some(dysign_pure::medusa::Kind::tramp)
        );
        let e_next = if is_tramp {
            let m = vm.mem.ld(vm.slots_addr + 8 + 8 * 35, 8).unwrap_or(0);
            Some(match m {
                0 => vm.mem.ld(vm.slots_addr, 8).unwrap_or(0),
                _ => vm.mem.ld(vm.slots_addr + 8 + 8 * 36, 8).unwrap_or(0),
            })
        } else {
            None
        };
        vm.mem.reset_dirty();

        let mut ds = Vec::new();
        for i in 0..64u64 {
            if i == 0 || i == 35 || i == 36 || i == 37 || i == 29 {
                continue;
            }
            if (before[i as usize] != rs[i as usize] || before[i as usize] != es[i as usize])
                && rs[i as usize] != es[i as usize]
            {
                ds.push(format!("{i}:{:x}/{:x}", rs[i as usize], es[i as usize]));
            }
        }
        let mut dm = Vec::new();
        for (r, e) in [(rw.0, ew.0), (rw.1, ew.1)] {
            if r != e {
                dm.push(format!("{}:{}|{}", r.len(), hex(&r), hex(&e)));
            }
        }
        if r_next != e_next && (is_tramp || err_r != err_e) {
            ds.push(format!(
                "next:{}",
                match (r_next, e_next) {
                    (Some(a), Some(b)) => format!("{a:x}/{b:x}"),
                    (a, b) => format!("{a:?}/{b:?}"),
                }
            ));
        }
        if !ds.is_empty() || !dm.is_empty() || err_r != err_e {
            let _ = writeln!(
                out,
                "{}",
                serde_json::json!({
                    "id": c.id, "pc": format!("{:#x}", c.pc), "er": err_r, "ee": err_e,
                    "slots": ds, "mem": dm
                })
            );
        }
    }
    eprintln!("SKIP two-phase (u32f64/i32f64): {skipped}");
}
