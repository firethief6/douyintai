//! 探针 CLI：`dysign '<url>' '<extra>'` 打印 7 枚信封头。
//! 环境变量：DEVICE_ID / IID 切身份，REPLAY_N 限重放条数，JNI_TRACE=1 打 JNI 调用。
use std::io::Write;

fn b64d(s: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let mut acc = 0u32;
    let mut n = 0;
    for c in s.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => continue,
        };
        acc = (acc << 6) | v as u32;
        n += 6;
        if n >= 8 {
            n -= 8;
            out.push((acc >> n) as u8);
            acc &= (1 << n) - 1;
        }
    }
    out
}

/// 取 `X-Medusa` 的值，并把随进程递增的 [20..24) 归零，供锚点前后对比。
fn medusa(out: &str) -> Vec<u8> {
    let mut v = Vec::new();
    for line in out.split("\r\n") {
        if line == "X-Medusa" {
            v = line.to_string().into_bytes();
            continue;
        }
        if !v.is_empty() {
            v = line.as_bytes().to_vec();
            break;
        }
    }
    if v.len() < 24 {
        return v;
    }
    let d = base64_decode(&v);
    let mut d = d;
    d[20..24].fill(0);
    d
}

fn base64_decode(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut acc = 0u32;
    let mut n = 0;
    for c in s.iter().copied() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => continue,
        };
        acc = (acc << 6) | v as u32;
        n += 6;
        if n >= 8 {
            n -= 8;
            out.push((acc >> n) as u8);
            acc &= (1 << n) - 1;
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://api5-normal-c-lf.amemv.com/aweme/v1/feed/?count=6".into());
    let extra = std::env::args().nth(2).unwrap_or_else(|| "GET".into());
    if std::env::var("DYSIGN_ANCHOR").is_ok() && std::env::var("DYSIGN_ANCHOR_ONE").is_err() {
        let spec = std::env::var("DYSIGN_ANCHOR")?;
        // 先在本进程校验语法，写错立刻报错，不混进“锚点不活”。
        let items = unidroid::profile::parse_anchor_specs(&spec)?;
        let exe = std::env::current_exe()?;
        let run =
            |anchor: Option<&str>| -> Result<std::process::Output, Box<dyn std::error::Error>> {
                let mut c = std::process::Command::new(&exe);
                c.arg(&url).arg(&extra);
                match anchor {
                    Some(a) => {
                        c.env("DYSIGN_ANCHOR_ONE", "1").env("DYSIGN_ANCHOR", a);
                    }
                    None => {
                        c.env_remove("DYSIGN_ANCHOR")
                            .env_remove("DYSIGN_ANCHOR_ONE");
                    }
                }
                Ok(c.output()?)
            };
        let base = medusa(&String::from_utf8_lossy(&run(None)?.stdout));
        let mut dead = 0;
        for item in items
            .iter()
            .map(|a| format!("0x{:x}:0x{:x}:{:x}", a.pc, a.addr, a.len))
        {
            let out = run(Some(&item))?;
            let stderr = String::from_utf8_lossy(&out.stderr);
            let hits: u64 = stderr
                .lines()
                .find_map(|l| l.strip_prefix("[anchor] hits="))
                .and_then(|n| n.trim().parse().ok())
                .unwrap_or(0);
            let now = medusa(&String::from_utf8_lossy(&out.stdout));
            if hits == 0 {
                eprintln!("[anchor] {item}: 锚点不活（PC 未命中）");
                dead += 1;
            } else if now.is_empty() || now == base {
                eprintln!("[anchor] {item}: 锚点不活（改字节后 X-Medusa 未变）");
                dead += 1;
            } else {
                eprintln!("[anchor] {item}: 通过（命中 {hits} 次，X-Medusa 已变）");
            }
        }
        if dead > 0 {
            return Err(format!("{dead} 个锚点不活").into());
        }
        return Ok(());
    }
    let t0 = std::time::Instant::now();
    unidroid::profile::mark("init-start");
    let mut signer = dysign::Signer::new(&dysign::default_resources())?;
    unidroid::profile::mark("init-end");
    eprintln!("[*] 冷启 {}ms", t0.elapsed().as_millis());
    if let Ok(did) = std::env::var("DEVICE_ID") {
        let iid = std::env::var("IID").unwrap_or_default();
        signer.set_identity(&did, &iid)?;
    }
    if let Ok(seq) = std::env::var("DYSIGN_SEQ") {
        let dir = std::env::var("DYSIGN_SEQ_OUT").unwrap_or_else(|_| "/tmp/pure/seq".into());
        std::fs::create_dir_all(&dir)?;
        for (i, line) in std::fs::read_to_string(seq)?.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let (u, h) = line.split_once('\t').unwrap_or((line, ""));
            let u = String::from_utf8_lossy(&b64d(u)).into_owned();
            let h = String::from_utf8_lossy(&b64d(h)).into_owned();
            let t1 = std::time::Instant::now();
            let out = signer.sign(&u, &h)?;
            std::fs::write(format!("{dir}/{i:04}.bin"), out.as_bytes())?;
            eprintln!("[*] seq #{} {}ms", i, t1.elapsed().as_millis());
        }
        unidroid::profile::dump();
        unidroid::profile::watch_dump();
        unidroid::profile::write_dump();
        return Ok(());
    }
    let reps: usize = std::env::var("DYSIGN_REPEAT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let mut out = String::new();
    for i in 0..reps {
        let t1 = std::time::Instant::now();
        if std::env::var("IDENTITY_EACH").is_ok() {
            if let Ok(did) = std::env::var("DEVICE_ID") {
                let iid = std::env::var("IID").unwrap_or_default();
                signer.set_identity(&did, &iid)?;
            }
        }
        unidroid::profile::mark("sign-start");
        out = signer.sign(&url, &extra)?;
        unidroid::profile::mark("sign-end");
        eprintln!(
            "[*] 签名 #{} {}ms, {} 字节",
            i + 1,
            t1.elapsed().as_millis(),
            out.len()
        );
    }
    let mut stdout = std::io::stdout();
    stdout.write_all(out.as_bytes())?;
    stdout.flush()?;
    unidroid::profile::dump();
    unidroid::profile::watch_dump();
    unidroid::profile::write_dump();
    if std::env::var("DYSIGN_ANCHOR_ONE").is_ok() {
        eprintln!("[anchor] hits={}", unidroid::profile::anchor_hits());
    }
    Ok(())
}
