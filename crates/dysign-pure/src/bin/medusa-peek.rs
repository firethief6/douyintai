//! 一次性探针：跑一次签名后打印 bss 运行期代码池样本。
use dysign_pure::medusa::{Stub, Urng, Vm};
fn main() {
    let url = std::fs::read_to_string("scripts/fixtures/medusa-vm/case1.url").unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("").to_string();
    let mut vm = Vm::from_base().unwrap();
    vm.sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1))
        .unwrap();
    for a in [0x10531000u64, 0x10508000, 0x10519000, 0x1052f000] {
        let d = vm.mem.read(a, 32).unwrap_or_default();
        println!("{a:#x} {:02x?}", d);
        let w: Vec<u32> = d
            .chunks(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        println!("  w {:08x?}", w);
    }
}
