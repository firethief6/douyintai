use dysign_pure::medusa::{Vm, OUT_ADDR};

fn hexs(s: &str) -> Vec<u8> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
        .collect()
}

fn fnv(d: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in d {
        h = (h ^ *b as u64).wrapping_mul(0x100000001b3);
    }
    h
}

fn main() {
    let snap = std::fs::read(std::env::var("SNAP").unwrap()).unwrap();
    let mut vm = Vm::from_snapshot(&snap).unwrap();
    if let Ok(ps) = std::env::var("PATCH") {
        for spec in ps.split(',').filter(|s| !s.is_empty()) {
            let (a, h) = spec.split_once(':').unwrap();
            let addr = u64::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
            vm.mem.map(addr, &hexs(h));
        }
    }
    if let Ok(c) = std::env::var("CLOCK") {
        vm.set_clock(c.parse().unwrap());
    }
    if let Ok(l) = std::env::var("SETLEN") {
        let n = u16::from_str_radix(&l, 16).unwrap();
        vm.mem.map(0x701e_df74, &n.to_le_bytes());
    }
    let r = vm.run(20_000_000);
    let len = vm.nat.out_len;
    let got = vm.mem.read(OUT_ADDR, len).unwrap_or_default();
    if let Ok(p) = std::env::var("OUT") {
        std::fs::write(p, &got).unwrap();
    }
    let same = std::env::var("REF")
        .ok()
        .map(|p| std::fs::read(p).unwrap() == got)
        .unwrap_or(false);
    println!(
        "ok={} steps={} pc={:#x} out_len={} urand={} fnv={:016x} same_ref={same} head={:02x?}",
        r.is_ok(),
        vm.steps,
        vm.pc,
        len,
        vm.nat.rng.taken,
        fnv(&got),
        &got[..16.min(got.len())]
    );
}
