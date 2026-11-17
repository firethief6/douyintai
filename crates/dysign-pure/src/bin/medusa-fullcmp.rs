use dysign_pure::medusa::{Urng, Vm, QUERY_LEN, QUERY_PTR};
fn main() {
    let snap = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let url = std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("");
    let mut r = Vm::from_snapshot(&snap).unwrap();
    let mut n = Vm::from_snapshot(&snap).unwrap();
    for vm in [&mut r, &mut n] {
        vm.set_clock(1762300000);
        vm.mem.map(0x5080_a000, q.as_bytes());
        vm.mem.map(QUERY_PTR, &0x5080_a000u64.to_le_bytes());
        vm.mem.map(QUERY_LEN, &(q.len() as u16).to_le_bytes());
        vm.nat.rng = Urng::new(1);
    }
    let a = r.run(20_000_000);
    let b = n.run_aot(20_000_000);
    println!(
        "r={a:?} n={b:?} pr={:#x} pn={:#x} sr={} sn={} lens={}/{}",
        r.pc, n.pc, r.steps, n.steps, r.nat.out_len, n.nat.out_len
    );
    let ar = r.mem.read(0x503c_f600, 0x700).unwrap();
    let an = n.mem.read(0x503c_f600, 0x700).unwrap();
    let d: Vec<usize> = ar
        .iter()
        .zip(&an)
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(i, _)| i)
        .collect();
    println!("diffcount={} first={:?}", d.len(), &d[..d.len().min(8)]);
    let qd = d.iter().filter(|i| (0x18..0x111).contains(*i)).count();
    let env = d.iter().filter(|i| (0x111..0x2b3).contains(*i)).count();
    let tail = d.iter().filter(|i| **i >= 0x2b3).count();
    println!("query={qd} env={env} tail={tail}");
}
