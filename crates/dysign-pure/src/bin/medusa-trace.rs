//! 单个 ALU 字在真实引擎上的 pc 轨迹（用于定位分发后的 handler 入口）。
use dysign_pure::medusa::{Vm, PROG_LO};

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn main() {
    let snap = std::fs::read(std::env::args().nth(1).expect("缺快照")).expect("读快照失败");
    let mut vm = Vm::from_snapshot(&snap).expect("快照损坏");
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let (hp, hw) = line.trim().split_once('\t').expect("pc<TAB>word");
    let pc = u64::from_str_radix(hp.trim_start_matches("0x"), 16).unwrap();
    let word = u32::from_str_radix(hw.trim_start_matches("0x"), 16).unwrap();
    let base = vm.slots_addr;
    let _ = vm.mem.w64(vm.sp + 0x38, base);
    let _ = vm.mem.w64(base, pc);
    for i in 0..64u64 {
        let v = mix(0x51ed_0000 + i);
        let _ = vm.mem.w64(base + 8 + 8 * i, v);
        let _ = vm.mem.w64(base + 0x148 + 8 * i, v);
    }
    let _ = vm.mem.w64(base + 8 + 8 * 36, PROG_LO);
    vm.regs[6] = base;
    vm.regs[27] = base + 0x140;
    vm.regs[0] = pc;
    let _ = word;
    let _ = vm.nat.call(
        &mut vm.mem,
        &vm.slots,
        0x1004_e710,
        &mut vm.sp,
        &mut vm.regs,
        false,
        true,
    );
}
