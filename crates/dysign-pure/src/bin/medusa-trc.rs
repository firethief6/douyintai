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
    let r = a.sign_injected(&q, &Stub::Empty, 1762300000, &mut Urng::new(1));
    eprintln!("res={:?}", r.as_ref().err());
    for pg in [0x5008c000u64, 0x1052f000, 0x701ec000] {
        if let Some(b) = a.mem.page_bytes(pg) {
            for (i, c) in b.chunks(32).enumerate() {
                println!(
                    "{pg:x}+{:x} {}",
                    i * 32,
                    c.iter().map(|x| format!("{x:02x}")).collect::<String>()
                );
            }
        }
    }
}
