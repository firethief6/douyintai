//! 静态初态与整幅快照的逐页比对：差异为 0 才算等价。
use dysign_pure::medusa::Vm;

fn main() {
    let snap = dysign_pure::medusa::snapshot_bytes();
    let a = Vm::from_snapshot(snap).expect("snapshot");
    let b = Vm::from_static().expect("static");
    let sl: Vec<usize> = (0..64).filter(|i| a.slots[*i] != b.slots[*i]).collect();
    let mut p = 0usize;
    let rd = |p: &mut usize| -> u64 {
        let v = u64::from_le_bytes(snap[*p..*p + 8].try_into().unwrap());
        *p += 8;
        v
    };
    let _pc = rd(&mut p);
    for _ in 0..48 {
        rd(&mut p);
    }
    let n = u32::from_le_bytes(snap[p..p + 4].try_into().unwrap()) as usize;
    p += 4;
    let mut bad = 0usize;
    let mut first = None;
    for _ in 0..n {
        let addr = rd(&mut p);
        let want = &snap[p..p + 4096];
        p += 4096;
        let got = b.mem.read(addr, 4096).unwrap_or_default();
        if got != want {
            bad += 1;
            first.get_or_insert(addr);
        }
    }
    println!(
        "slots_diff={} pages_diff={} first={:x?} pc={:#x}",
        sl.len(),
        bad,
        first,
        a.pc
    );
    assert!(sl.is_empty() && bad == 0);
}
