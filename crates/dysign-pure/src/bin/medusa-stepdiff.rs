//! 临时：同一输入下 replay 与 native 逐步对拍，打印首个分叉。
use dysign_pure::medusa::{Urng, Vm, QUERY_LEN, QUERY_PTR};

fn main() {
    let snap = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let url = std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("");
    let mut r = Vm::from_snapshot(&snap).unwrap();
    let mut n = Vm::from_snapshot(&snap).unwrap();
    for vm in [&mut r, &mut n] {
        vm.set_clock(
            std::env::var("CLOCK")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1762300000),
        );
        vm.mem.map(0x5080_a000, q.as_bytes());
        vm.mem.map(QUERY_PTR, &0x5080_a000u64.to_le_bytes());
        vm.mem.map(QUERY_LEN, &(q.len() as u16).to_le_bytes());
        let stub = [
            0xd4u8, 0x1d, 0x8c, 0xd9, 0x8f, 0, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
            0x42, 0x7e,
        ];
        vm.mem.map(0x503f_f520, &stub);
        vm.mem.map(0x503f_f4ec, &(stub.len() as u32).to_le_bytes());
        vm.mem.map(0x503f_f4f0, &0x503f_f520u64.to_le_bytes());
        vm.nat.rng = Urng::new(1);
    }
    println!(
        "init m18r={:02x?} m18n={:02x?}",
        r.mem.read(0x50082010, 16).unwrap_or_default(),
        n.mem.read(0x50082010, 16).unwrap_or_default()
    );
    println!(
        "init s29r={:#x} s29n={:#x} pcr={:#x} pcn={:#x}",
        r.slots[29], n.slots[29], r.pc, n.pc
    );
    let _ = r.run(1);
    let _ = n.run_aot(1);
    let mut pages = std::collections::BTreeSet::new();
    let mut ring: Vec<u64> = Vec::new();
    for step in 1..600_000u64 {
        let pc = r.pc;
        if pc != n.pc {
            println!(
                "diverge pre step={step} pcr={pc:#x} pcn={:#x} s29r={:#x} s29n={:#x} m35r={} m35n={} s36r={:#x} s36n={:#x}",
                n.pc, r.slots[29], n.slots[29], r.slots[35], n.slots[35], r.slots[36], n.slots[36]
            );
        }
        let before18 = r.mem.ld(0x50082018, 1).unwrap_or(0);
        ring.push(pc);
        if ring.len() > 12 {
            ring.remove(0);
        }
        let a = r.run(step + 1);
        let b = n.run_aot(step + 1);
        if let Err(e) = &a {
            if !matches!(e, dysign_pure::medusa::Error::Steps) {
                println!("replay err step={step} pc={pc:#x} {e:?}");
                return;
            }
        }
        if let Err(e) = &b {
            if !matches!(e, dysign_pure::medusa::Error::Steps) {
                println!("native err step={step} pc={pc:#x} {e:?}");
                return;
            }
        }
        let slots_differ = (0..64usize).any(|i| r.slots[i] != n.slots[i] && !matches!(i, 35..=37));
        if r.pc != n.pc || slots_differ {
            println!(
                "diverge step={step} pc={pc:#x} pcr={:#x} pcn={:#x} s29r={:#x} s29n={:#x} m35r={} m35n={} s36r={:#x} s36n={:#x} s37r={:#x} s37n={:#x}",
                r.pc, n.pc, r.slots[29], n.slots[29], r.slots[35], n.slots[35], r.slots[36], n.slots[36], r.slots[37], n.slots[37]
            );
            for i in 0..64 {
                if r.slots[i] != n.slots[i] {
                    println!(" slot{i} r={:#x} n={:#x}", r.slots[i], n.slots[i]);
                }
            }
            return;
        }
        if matches!(a, Ok(())) || matches!(b, Ok(())) || r.pc == 0x2_0000 || n.pc == 0x2_0000 {
            println!("end step={step} rpc={:#x} npc={:#x}", r.pc, n.pc);
            return;
        }

        let after18 = r.mem.ld(0x50082018, 1).unwrap_or(0);
        if before18 != after18 {
            println!("W18 step={step} pc={pc:#x} {:#x}->{:#x}", before18, after18);
        }
        pages.clear();
        pages.extend(r.mem.dirty_pages());
        pages.extend(n.mem.dirty_pages());
        for p in &pages {
            let mut ra = r.mem.read(*p, 4096).unwrap_or_default();
            let mut na = n.mem.read(*p, 4096).unwrap_or_default();
            if *p == (r.slots_addr & !0xfff) {
                ra.fill(0);
                na.fill(0);
            }
            for (base, len) in [(0u64, 0u64), (r.sp, 0x200)] {
                if base >= *p && base + len <= *p + 4096 {
                    let b0 = (base - *p) as usize;
                    for k in b0..b0 + len as usize {
                        ra[k] = 0;
                        na[k] = 0;
                    }
                }
            }
            if ra != na {
                let off = ra.iter().zip(&na).position(|(x, y)| x != y).unwrap();
                println!(
                    " ring={:x?}",
                    ring.iter().map(|x| format!("{x:#x}")).collect::<Vec<_>>()
                );
                println!(
                    " word={:02x?} rv={:02x?} nv={:02x?}",
                    r.mem.read(pc, 4).unwrap_or_default(),
                    r.mem.read(p + off as u64 - 4, 8).unwrap_or_default(),
                    n.mem.read(p + off as u64 - 4, 8).unwrap_or_default()
                );
                println!("mem diverge step={step} pc={pc:#x} addr={:#x} r={:02x} n={:02x} s29r={:#x} s29n={:#x} s0r={:#x} s0n={:#x}", p + off as u64, ra[off], na[off], r.slots[29], n.slots[29], r.slots[0], n.slots[0]);

                return;
            }
        }
    }
    println!(
        "no divergence in 200k steps: rpc={:#x} npc={:#x} stepsr={} stepsn={}",
        r.pc, n.pc, r.steps, n.steps
    );
    let ar = r.mem.read(0x503c_f600, 0x619).unwrap();
    let an = n.mem.read(0x503c_f600, 0x619).unwrap();
    println!(
        "buf same={} first={:?} prodr={:#x} prodn={:#x}",
        ar == an,
        ar.iter().zip(&an).position(|(x, y)| x != y),
        r.nat.out_len,
        n.nat.out_len
    );
}
