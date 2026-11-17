//! 逐字对拍：同一快照上，AOT 表与原生引擎各跑一侧，逐步比较 pc 与全部槽。
//!
//! 两侧都先用 replay 跑到目标字，再逐字执行；AOT 侧出程序区时改走引擎的
//! 绑定/逐字路径，保证对照的是同一段控制流。
//!
//! 用法：medusa-worddiff <snap> <url-file> <clock> <pc> [steps]
//! `ONE=1` 只在引擎侧跑目标字（配合 TRAPT 看轨迹）；`MEMD=1` 打印引擎侧写过的槽。
use dysign_pure::medusa::{
    native_program, Error, Urng, Vm, DEFAULT_CALL, NATIVE_CALL, PROG_HI, PROG_LO,
};

const STUB: [u8; 16] = [
    0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e,
];

fn setup(snap: &[u8], url: &str, clock: u64) -> Vm {
    let query = url.split_once('?').expect("URL 没有 query").1.to_string();
    let mut vm = Vm::from_snapshot(snap).expect("快照损坏");
    let mut rng = Urng::new(12345);
    vm.set_clock(clock);
    vm.mem.map(0x5080_a000, query.as_bytes());
    vm.mem.map(0x701e_df78, &0x5080_a000u64.to_le_bytes());
    vm.mem.map(0x701e_df74, &(query.len() as u16).to_le_bytes());
    vm.mem.map(0x503f_f520, &STUB);
    vm.mem.map(0x503f_f4ec, &(STUB.len() as u32).to_le_bytes());
    vm.mem.map(0x503f_f4f0, &0x503f_f520u64.to_le_bytes());
    vm.nat.rng = std::mem::take(&mut rng);
    vm
}

fn reach(vm: &mut Vm, target: u64) -> bool {
    for n in 0..2_000_000u64 {
        if vm.pc == target {
            return true;
        }
        if let Err(e) = vm.run(n + 1) {
            if !matches!(e, Error::Steps) {
                eprintln!("停在 {:#x}: {e:?}", vm.pc);
                return false;
            }
        }
    }
    false
}

fn changed(vm: &Vm, before: &[u64; 64]) -> Vec<String> {
    (0..64)
        .filter(|i| vm.slots[*i] != before[*i])
        .map(|i| format!("s{i}->{:#x}", vm.slots[i]))
        .collect()
}

fn slot_diff(a: &Vm, b: &Vm) -> String {
    let d: Vec<String> = (0..64)
        .filter(|i| a.slots[*i] != b.slots[*i])
        .map(|i| format!("s{i}: {:#x} vs {:#x}", a.slots[i], b.slots[i]))
        .collect();
    if d.is_empty() {
        "same".into()
    } else {
        d.join(", ")
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let snap_path = args.next().expect("缺快照");
    let url_path = args.next().expect("缺 URL 文件");
    let clock: u64 = args.next().expect("缺时钟").parse().expect("时钟不是数字");
    let target = u64::from_str_radix(args.next().expect("缺目标 pc").trim_start_matches("0x"), 16)
        .expect("pc 不是十六进制");
    let steps: usize = args
        .next()
        .map(|s| s.parse().expect("步数不是数字"))
        .unwrap_or(64);

    let snap = std::fs::read(snap_path).expect("读快照失败");
    let url = std::fs::read_to_string(url_path).expect("读 URL 失败");
    let mut aot = setup(&snap, &url, clock);
    let mut eng = setup(&snap, &url, clock);
    if !reach(&mut aot, target) || !reach(&mut eng, target) {
        return;
    }
    // ONE=1：只在引擎侧跑目标字；配合外部 `TRAPT=1` 看逐指令轨迹。
    if std::env::var_os("ONE").is_some() {
        let _ = eng.dbg_engine_word(target);
        return;
    }
    println!("PRE {}", slot_diff(&aot, &eng));
    for _ in 0..steps {
        if aot.pc != eng.pc {
            println!("pc 分叉 {:#x} vs {:#x}", aot.pc, eng.pc);
            return;
        }
        let pc = aot.pc;
        let before_a = aot.slots;
        let before_b = eng.slots;
        let res_a = if (PROG_LO..PROG_HI).contains(&pc) {
            if aot.slots[35] == 2 {
                aot.slots[35] = 3;
            }
            let w = aot.mem.code_u32(pc);
            aot.decode(w);
            aot.dbg_materialize();
            let r = native_program::exec(&mut aot, pc);
            if let Ok(next) = &r {
                aot.pc = *next;
            }
            match aot.jump_flag {
                1 => aot.jump_call = true,
                2 => aot.jump_call = false,
                _ => {}
            }
            aot.jump_flag = 0;
            if r.is_ok() {
                aot.dbg_transition_aot();
                let mut guard = 0;
                while !(PROG_LO..PROG_HI).contains(&aot.pc) && guard < 64 {
                    guard += 1;
                    let npc = aot.pc;
                    let call = npc == NATIVE_CALL || npc == DEFAULT_CALL || aot.jump_call;
                    let r2 = if call {
                        aot.dbg_native_bound(npc).map(|_| true)
                    } else {
                        aot.dbg_engine_word(npc)
                    };
                    if !r2.unwrap_or(false) {
                        break;
                    }
                }
            }
            r
        } else {
            aot.dbg_engine_word(pc).map(|_| aot.pc)
        };
        let res_b = eng.dbg_engine_word(pc);
        if std::env::var_os("MEMD").is_some() {
            let written: Vec<usize> = (0..64)
                .filter(|i| {
                    eng.mem
                        .r64(eng.slots_addr + 8 + 8 * *i as u64)
                        .unwrap_or(u64::MAX)
                        != before_b[*i]
                })
                .collect();
            if !written.is_empty() {
                println!("MB {written:?}");
            }
        }
        println!(
            "CH next={:#x}/{:#x} a:[{}] b:[{}]",
            aot.pc,
            eng.pc,
            changed(&aot, &before_a).join(","),
            changed(&eng, &before_b).join(",")
        );
        println!(
            "{pc:#x} aot={:?} eng={:?} {}",
            res_a.as_ref().map(|p| *p),
            res_b.is_ok(),
            slot_diff(&aot, &eng)
        );
        if res_a.is_err() || res_b.is_err() {
            break;
        }
    }
}
