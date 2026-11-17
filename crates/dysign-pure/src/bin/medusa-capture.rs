//! 可复验快照：记录 query、stub、入口槽与期望输出，供之后逐字节重放。
//!
//! 采集：`medusa-capture <out.cap> <url 文件> <头块文件> [clock] [seed]`
//! 复验：`medusa-capture --verify <cap>`（MEDUSA_IMPL/MEDUSA_STATIC 生效）
use base64::Engine;
use dysign_pure::medusa::{Stub, Urng, Vm};
use md5::{Digest, Md5};
use std::io::Write;

fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn md5_hex(b: &[u8]) -> String {
    hexs(&Md5::digest(b))
}

fn stub_word(s: &str) -> Stub {
    match s.trim() {
        "missing" => Stub::Missing,
        "empty" => Stub::Empty,
        v => Stub::Hex(
            v.as_bytes()
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
                .collect(),
        ),
    }
}

struct Cap {
    clock: u64,
    seed: u64,
    stub: Stub,
    slots: [u64; 64],
    query: Vec<u8>,
    expected: Vec<u8>,
    base: String,
}

fn run(c: &Cap, snap: &[u8]) -> Result<Vec<u8>, String> {
    if md5_hex(snap) != c.base {
        return Err(format!("基础快照 md5 不符: {}", c.base));
    }
    let mut vm = if std::env::var("MEDUSA_BASE").as_deref() == Ok("snap") {
        Vm::from_snapshot(snap)
    } else {
        Vm::from_static()
    }
    .ok_or("快照损坏")?;
    vm.slots = c.slots;
    for i in 0..64u64 {
        let _ = vm.mem.w64(vm.slots_addr + 8 + 8 * i, c.slots[i as usize]);
    }
    let q = std::str::from_utf8(&c.query).map_err(|e| e.to_string())?;
    vm.set_clock(c.clock);
    vm.sign_injected(q, &c.stub, c.clock, &mut Urng::new(c.seed))
        .map_err(|e| format!("{e:?}"))?;
    let len = vm.nat.out_len;
    if len == 0 || len > 4096 {
        return Err(format!("输出长度异常 {len}"));
    }
    vm.mem
        .read(dysign_pure::medusa::OUT_ADDR, len)
        .map_err(|e| format!("{e:?}"))
}

fn parse(text: &str) -> Result<Cap, String> {
    let mut clock = None;
    let mut seed = None;
    let mut stub = None;
    let mut slots = None;
    let mut query = None;
    let mut expected = None;
    let mut base = None;
    for line in text.lines() {
        let Some((k, v)) = line.split_once(' ') else {
            continue;
        };
        match k {
            "clock" => clock = Some(v.parse().map_err(|_| "clock")?),
            "seed" => seed = Some(v.parse().map_err(|_| "seed")?),
            "stub" => stub = Some(stub_word(v)),
            "base_md5" => base = Some(v.to_string()),
            "query_b64" => {
                query = Some(
                    base64::engine::general_purpose::STANDARD
                        .decode(v)
                        .map_err(|_| "query_b64")?,
                )
            }
            "expected_b64" => {
                expected = Some(
                    base64::engine::general_purpose::STANDARD
                        .decode(v)
                        .map_err(|_| "expected_b64")?,
                )
            }
            "slots" => {
                let mut s = [0u64; 64];
                let mut n = 0;
                for x in v.split(' ') {
                    if n == 64 {
                        return Err("slots 过多".into());
                    }
                    s[n] = u64::from_str_radix(x, 16).map_err(|_| "slot")?;
                    n += 1;
                }
                if n != 64 {
                    return Err(format!("slots 只有 {n}"));
                }
                slots = Some(s);
            }
            _ => {}
        }
    }
    Ok(Cap {
        clock: clock.ok_or("缺 clock")?,
        seed: seed.ok_or("缺 seed")?,
        stub: stub.ok_or("缺 stub")?,
        slots: slots.ok_or("缺 slots")?,
        query: query.ok_or("缺 query_b64")?,
        expected: expected.ok_or("缺 expected_b64")?,
        base: base.ok_or("缺 base_md5")?,
    })
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let snap = dysign_pure::medusa::snapshot_bytes();
    let b64 = base64::engine::general_purpose::STANDARD;
    if a.get(1).map(String::as_str) == Some("--verify") {
        let cap =
            parse(&std::fs::read_to_string(a.get(2).expect("缺 cap")).unwrap()).expect("格式");
        match run(&cap, snap) {
            Ok(got) if got == cap.expected => {
                println!("ok len={} md5={}", got.len(), md5_hex(&got));
            }
            Ok(got) => {
                eprintln!(
                    "MISMATCH want_len={} got_len={} want_md5={} got_md5={}",
                    cap.expected.len(),
                    got.len(),
                    md5_hex(&cap.expected),
                    md5_hex(&got)
                );
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("ERR {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    let out = a.get(1).expect("缺输出路径");
    let url = std::fs::read_to_string(a.get(2).expect("缺 url 文件")).expect("读 url");
    let hdr = std::fs::read(a.get(3).expect("缺头块文件")).expect("读头块");
    let clock = a.get(4).map(|x| x.parse().unwrap()).unwrap_or(1762300000);
    let seed = a.get(5).map(|x| x.parse().unwrap()).unwrap_or(1);
    let query = url.split_once('?').map(|x| x.1).unwrap_or("").to_string();
    let stub = dysign_pure::stub_of(&String::from_utf8_lossy(&hdr));
    let vm = Vm::from_snapshot(snap).expect("快照损坏");
    let slots = vm.slots;
    let mut cap = Cap {
        clock,
        seed,
        stub,
        slots,
        query: query.into_bytes(),
        expected: Vec::new(),
        base: md5_hex(snap),
    };
    cap.expected = run(&cap, snap).expect("采集失败");
    let mut f = std::fs::File::create(out).expect("写 cap");
    let stub_s = match &cap.stub {
        Stub::Missing => "missing".to_string(),
        Stub::Empty => "empty".to_string(),
        Stub::Hex(v) => hexs(v),
    };
    writeln!(f, "medusa-capture 1").unwrap();
    writeln!(f, "base_md5 {}", cap.base).unwrap();
    writeln!(f, "clock {}", cap.clock).unwrap();
    writeln!(f, "seed {}", cap.seed).unwrap();
    writeln!(f, "stub {stub_s}").unwrap();
    writeln!(
        f,
        "slots {}",
        cap.slots
            .iter()
            .map(|x| format!("{x:016x}"))
            .collect::<Vec<_>>()
            .join(" ")
    )
    .unwrap();
    writeln!(f, "query_b64 {}", b64.encode(&cap.query)).unwrap();
    writeln!(f, "expected_len {}", cap.expected.len()).unwrap();
    writeln!(f, "expected_md5 {}", md5_hex(&cap.expected)).unwrap();
    writeln!(f, "expected_b64 {}", b64.encode(&cap.expected)).unwrap();
    println!(
        "ok len={} md5={}",
        cap.expected.len(),
        md5_hex(&cap.expected)
    );
}
