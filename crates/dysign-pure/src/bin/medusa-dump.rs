//! 一次性反汇编辅助：从 base 快照按地址导出机器码。
use dysign_pure::medusa::Vm;
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let vm = Vm::from_base().unwrap();
    for s in a {
        let (ad, len) = s.split_once(':').unwrap();
        let ad = u64::from_str_radix(ad.trim_start_matches("0x"), 16).unwrap();
        let len: u64 = len.parse().unwrap();
        match vm.mem.read(ad, len as usize) {
            Ok(d) => {
                for (i, c) in d.chunks(16).enumerate() {
                    print!("{:#x}:", ad + i as u64 * 16);
                    for w in c.chunks(4) {
                        let v =
                            u32::from_le_bytes([w[0], w[1], w[2], w.get(3).copied().unwrap_or(0)]);
                        print!(" {:08x}", v);
                    }
                    println!();
                }
            }
            Err(e) => println!("{ad:#x}: {e:?}"),
        }
    }
}
