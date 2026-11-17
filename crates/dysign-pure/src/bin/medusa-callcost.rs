//! 一次性微测：Native::call 的固定入口成本（目标选 END，构造完即返回）。
use dysign_pure::medusa::native::{Native, END};
use dysign_pure::medusa::Vm;
fn main() {
    let mut vm = Vm::from_base().unwrap();
    let mut rng = dysign_pure::medusa::Urng::new(1);
    let url = std::fs::read_to_string("scripts/fixtures/medusa-vm/case1.url").unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("");
    let _ = vm
        .sign_injected(q, &dysign_pure::medusa::Stub::Empty, 1762300000, &mut rng)
        .unwrap();
    let n = 200_000u32;
    for _ in 0..10_000 {
        let _ = vm.nat.call(
            &mut vm.mem,
            &vm.slots,
            END,
            &mut vm.sp,
            &mut vm.regs,
            false,
            false,
        );
    }
    let t = std::time::Instant::now();
    for _ in 0..n {
        let _ = vm.nat.call(
            &mut vm.mem,
            &vm.slots,
            END,
            &mut vm.sp,
            &mut vm.regs,
            false,
            false,
        );
    }
    println!("{} ns/call", t.elapsed().as_nanos() / n as u128);
    let _ = Native::new();
}
