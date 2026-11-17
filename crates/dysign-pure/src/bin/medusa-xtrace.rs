use dysign_pure::medusa::{Stub, Urng, Vm};
fn main() {
    let url = std::fs::read_to_string("scripts/fixtures/medusa-vm/case1.url").unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("").to_string();
    let snap = {
        let mut v = Vec::new();
        std::io::Read::read_to_end(
            &mut flate2::read::GzDecoder::new(
                std::fs::File::open(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/resources/medusa-snap.bin.gz"
                ))
                .unwrap(),
            ),
            &mut v,
        )
        .unwrap();
        v
    };
    let mut a = Vm::from_snapshot(&snap).unwrap();
    let _ = a
        .sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1))
        .unwrap();
    let t1 = dysign_pure::medusa::native::trace_take();
    let x1 = dysign_pure::medusa::native::trace_x();
    let p1 = a.mem.page_bytes(0x1052f000).unwrap();
    a.rewind();
    let _ = a.sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1));
    let t2 = dysign_pure::medusa::native::trace_take();
    let r2 = dysign_pure::medusa::native::trace_r();
    let x2 = dysign_pure::medusa::native::trace_x();
    let mut b = Vm::from_snapshot(&snap).unwrap();
    let _ = b.sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1));
    let t3 = dysign_pure::medusa::native::trace_take();
    let r3 = dysign_pure::medusa::native::trace_r();
    let x3 = dysign_pure::medusa::native::trace_x();
    dysign_pure::medusa::native::clear_ops();
    let mut c = Vm::from_snapshot(&snap).unwrap();
    let _ = c.sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1));
    let t4 = dysign_pure::medusa::native::trace_take();
    let k = t3
        .iter()
        .zip(&t4)
        .position(|(x, y)| x != y)
        .unwrap_or(usize::MAX);
    println!(
        "fresh-vs-fresh diff at {k} len3={} len4={}",
        t3.len(),
        t4.len()
    );
    println!("len1={} len2={} len3={}", t1.len(), t2.len(), t3.len());
    let p2 = a.mem.page_bytes(0x1052f000).unwrap();
    let d: Vec<usize> = (0..4096).filter(|i| p1[*i] != p2[*i]).collect();
    println!("page diffs={} first={:?}", d.len(), &d[..d.len().min(8)]);
    if let Some(x) = d.first() {
        println!(
            "bytes1 {:02x?} bytes2 {:02x?}",
            &p1[x.saturating_sub(8)..(x + 8).min(4096)],
            &p2[x.saturating_sub(8)..(x + 8).min(4096)]
        );
    }
    let i = t2
        .iter()
        .zip(&t3)
        .position(|(x, y)| x != y)
        .unwrap_or(usize::MAX);
    println!("first diff at {i}");
    let j = r2
        .iter()
        .zip(&r3)
        .position(|(x, y)| x != y)
        .unwrap_or(usize::MAX);
    println!("first reg diff at {j}");
    let _ = (&x1, &x3);
    if let Some(q) = x2.iter().zip(&x3).position(|(u, v)| u != v) {
        let reg = (0..32).find(|r| x2[q][*r] != x3[q][*r]).unwrap();
        println!(
            "first full-reg diff at {q} reg x{reg} {:#x} vs {:#x} pc={:#x}/{:#x}",
            x2[q][reg],
            x3[q][reg],
            t2[15000 + q],
            t3[15000 + q]
        );
    }
    if j != usize::MAX {
        println!("r2={:x?} r3={:x?}", r2[j], r3[j]);
    }
    if i != usize::MAX {
        println!("r2@i={:x?} r3@i={:x?}", r2[i], r3[i]);
    }
    let lo = i.saturating_sub(12);
    println!("run1 {:#x?}", &t1[lo..(lo + 24).min(t1.len())]);
    println!("run2 {:#x?}", &t2[lo..(lo + 24).min(t2.len())]);
    println!("fresh {:#x?}", &t3[lo..(lo + 24).min(t3.len())]);
}
