use dysign_pure::medusa::Vm;
fn fixture(name: &str) -> String {
    format!(
        "{}/../../scripts/fixtures/medusa-vm/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}
#[test]
fn dump() {
    let snap = std::fs::read(fixture("medusa-snap.bin")).expect("缺快照");
    let mut vm = Vm::from_snapshot(&snap).expect("快照损坏");
    for _ in 0..400 {
        let pc = vm.pc;
        if (dysign_pure::medusa::PROG_LO..dysign_pure::medusa::PROG_HI).contains(&pc) {
            let mut line = format!("{} {:#x}", vm.steps, pc);
            for i in 0..40 {
                line.push_str(&format!(" {}", vm.slots[i]));
            }
            eprintln!("{}", line);
        }
        if let Err(e) = vm.step() {
            eprintln!("ERR {:#x} {:?}", pc, e);
            break;
        }
    }
}
