//! 对单个 VM 字在真实引擎上做离线语义派生：受控槽值 → 目标槽、源槽、运算。
//! 用法：medusa-derive <snapshot> < words.tsv（每行一个十六进制字值）。
//! 输出每行：word<TAB>pc<TAB>dest<TAB>srcs(逗号)<TAB>matches(逗号)<TAB>ok
use dysign_pure::medusa::{Vm, PROG_LO};

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

struct Run {
    dest: Option<(u8, usize)>,
    changed: Vec<(u8, usize, u64)>,
    pc: u64,
}

fn rd(vm: &Vm, base: u64, off: u64, i: usize) -> u64 {
    vm.mem.ld(base + off + 8 * i as u64, 8).unwrap_or(0)
}

fn one(vm: &mut Vm, base: u64, pc: u64, vals: &[u64; 64]) -> Run {
    vm.slots_addr = base;
    vm.pc = pc;
    let mut want = *vals;
    want[35] = 0;
    want[36] = PROG_LO;
    want[37] = vm.pc.wrapping_add(4);
    let _ = vm.mem.w64(vm.sp + 0x38, base);
    let _ = vm.mem.w64(base, vm.pc);
    for (i, value) in want.iter().enumerate() {
        let _ = vm.mem.w64(base + 8 + 8 * i as u64, *value);
    }
    vm.regs[6] = base;
    vm.regs[27] = base + 0x140;
    vm.regs[0] = vm.pc;
    let sp0 = vm.sp;
    let r = vm.nat.call(
        &mut vm.mem,
        &vm.slots,
        0x1004_e710,
        &mut vm.sp,
        &mut vm.regs,
        false,
        true,
    );
    if std::env::var("DD3").is_ok() {
        eprintln!(
            "sp {:#x}->{:#x} re={:?}",
            sp0,
            vm.sp,
            r.as_ref().map(|x| x.as_ref().map(|r| (r.pc, r.slots_addr)))
        );
    }
    match r {
        Ok(Some(re)) => {
            vm.slots_addr = re.slots_addr;
            vm.pc = re.pc;
            vm.sp = sp0;
        }
        Ok(None) => {
            vm.pc = want[37];
        }
        Err(_) => {}
    }
    let mut changed = Vec::new();
    for (i, value) in want.iter().enumerate() {
        if (35..=37).contains(&i) {
            continue;
        }
        if rd(vm, base, 8, i) != *value {
            changed.push((0u8, i, rd(vm, base, 8, i)));
        }
    }
    let mut dest = None;
    let a_only = changed.iter().filter(|(_, i, _)| *i < 64).count();
    if a_only == 1 {
        dest = changed
            .iter()
            .find(|(_, i, _)| *i < 64)
            .map(|(f, i, _)| (*f, *i));
    }
    Run {
        dest,
        changed,
        pc: vm.pc,
    }
}

fn cands(out: u64, x: u64, y: u64, two: bool) -> Vec<(&'static str, i64)> {
    let mut v = Vec::new();
    if two {
        let ops: &[(&str, u64)] = &[
            ("xor", x ^ y),
            ("and", x & y),
            ("or", x | y),
            ("nor", !(x | y)),
            ("add", x.wrapping_add(y)),
            (
                "add32",
                (x as u32).wrapping_add(y as u32) as i32 as i64 as u64,
            ),
            ("sub", x.wrapping_sub(y)),
            (
                "sub32",
                (x as u32).wrapping_sub(y as u32) as i32 as i64 as u64,
            ),
            ("mul", x.wrapping_mul(y)),
        ];
        for (n, val) in ops {
            if *val == out {
                v.push((*n, 0));
            }
        }
        let sh = y as u32;
        let shifts: &[(&str, u64)] = &[
            ("shl64r", x.wrapping_shl(sh)),
            ("shr64r", x.wrapping_shr(sh)),
            ("shr32r", (x as u32).wrapping_shr(sh) as u64),
            ("sar32r", ((x as u32 as i32) >> (sh & 31)) as u64),
        ];
        for (n, val) in shifts {
            if *val == out {
                v.push((*n, 0));
            }
        }
    } else {
        if out == x {
            v.push(("mov", 0));
        }
        v.push(("lea", out.wrapping_sub(x) as i64));
        v.push(("lea32", 0u64.wrapping_sub(out.wrapping_sub(x)) as i64));
        let ux = x as u32;
        for sh in 0..64u32 {
            if x.wrapping_shl(sh) == out {
                v.push(("shl64i", sh as i64));
            }
            if x.wrapping_shr(sh) == out {
                v.push(("shr64i", sh as i64));
            }
            if (ux.wrapping_shl(sh) as i32 as i64 as u64) == out {
                v.push(("shl32i", sh as i64));
            }
            if (ux.wrapping_shr(sh) as u64) == out {
                v.push(("shr32i", sh as i64));
            }
            if (((ux as i32) >> (sh & 31)) as u64) == out {
                v.push(("sar32i", sh as i64));
            }
        }
    }
    v
}

fn main() {
    let snap = std::fs::read(std::env::args().nth(1).expect("缺快照")).expect("读快照失败");
    let mut vm = Vm::from_snapshot(&snap).expect("快照损坏");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (hp, hw) = line.split_once('\t').expect("需要 pc<TAB>word");
        let pc = u64::from_str_radix(hp.trim_start_matches("0x"), 16).expect("坏 pc");
        let word = u32::from_str_radix(hw.trim_start_matches("0x"), 16).expect("坏字值");
        vm.pc = pc;
        let mut vals = [0u64; 64];
        for (i, value) in vals.iter_mut().enumerate() {
            *value = mix(0x51ed_0000 + i as u64);
        }
        let base0 = vm.slots_addr;
        let r0 = one(&mut vm, base0, pc, &vals);
        if std::env::var("DD").is_ok() {
            eprintln!(
                "{pc:x} {word:08x} changed={:?} pc2={:#x}",
                r0.changed, r0.pc
            );
        }
        let Some((df, d)) = r0.dest else {
            println!("{pc:x}\t{word:08x}\t-\t-\t\tmulti {:?}", r0.changed);
            continue;
        };
        let mut srcs = Vec::new();
        for k in 0..64 {
            if k == 35 || k == 36 || k == 37 {
                continue;
            }
            let mut v2 = vals;
            v2[k] ^= 0xdead_beef;
            let r = one(&mut vm, base0, pc, &v2);
            let c0 = r0
                .changed
                .iter()
                .filter(|(f, _, _)| *f == df)
                .find(|(_, i, _)| *i == d)
                .map(|(_, _, v)| *v)
                .unwrap_or(0);
            let nv = r
                .changed
                .iter()
                .filter(|(f, i, _)| *f == df && *i == d)
                .map(|(_, _, v)| *v)
                .next();
            if std::env::var("DD2").is_ok() {
                eprintln!(
                    "probe k={} c0={:#x} nv={:?} dest={:?} ch={:?}",
                    k,
                    c0,
                    nv,
                    r.dest,
                    r.changed
                        .iter()
                        .filter(|(f, _, _)| *f == 1)
                        .collect::<Vec<_>>()
                );
            }
            if r.changed
                .iter()
                .any(|(f, i, val)| *f == df && *i == d && *val != c0)
            {
                srcs.push(k);
            }
        }
        if srcs.len() > 2 {
            println!("{pc:x}\t{word:08x}\td={df}:{d}\tsrcs={srcs:?}\t\ttoomany");
            continue;
        }
        // 三组样本推运算。
        let mut common: Option<Vec<(&'static str, i64)>> = None;
        let mut samples = vec![(
            vals,
            r0.changed
                .iter()
                .find(|(_, i, _)| *i == d)
                .map(|(_, _, v)| *v)
                .unwrap_or(0),
        )];
        for t in 0..3u64 {
            let mut v = vals;
            for k in &srcs {
                v[*k] = mix(0x9000 + t * 97 + *k as u64);
                if t == 1 {
                    v[*k] |= 0x8000_0001;
                }
            }
            if let Some(s) = srcs.first() {
                v[*s] = mix(0x7000 + t);
                if t == 1 {
                    v[*s] = 0x8000_0001;
                }
            }
            let r = one(&mut vm, base0, pc, &v);
            if r.dest != Some((df, d)) {
                samples.push((v, u64::MAX));
                continue;
            }
            samples.push((
                v,
                r.changed
                    .iter()
                    .filter(|(f, _, _)| *f == df)
                    .find(|(_, i, _)| *i == d)
                    .map(|(_, _, v)| *v)
                    .unwrap_or(0),
            ));
        }
        for (v, out) in &samples {
            if *out == u64::MAX {
                common = Some(Vec::new());
                break;
            }
            let x = v[srcs.first().copied().unwrap_or(0)];
            let y = v[srcs.get(1).copied().unwrap_or(0)];
            let c = cands(*out, x, y, srcs.len() == 2);
            common = Some(match common {
                None => c,
                Some(prev) => prev
                    .into_iter()
                    .filter(|(n, sh)| c.iter().any(|(m, s2)| m == n && s2 == sh))
                    .collect(),
            });
        }
        let ok = if r0.pc == pc + 4 { "flow" } else { "flowbad" };
        println!(
            "{pc:x}\t{word:08x}\td={df}:{d}\tsrcs={srcs:?}\t{}\t{ok}",
            common
                .unwrap_or_default()
                .iter()
                .map(|(n, sh)| if *sh == 0 {
                    n.to_string()
                } else {
                    format!("{n}:{sh}")
                })
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
