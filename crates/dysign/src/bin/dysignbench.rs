//! 签名耗时基准：`signbench [次数]`。
//! 先照 `api.warm` 的口径签一次 feed，再签 N 条不同 URL，打印每条耗时与返回字节数。
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| run().map_err(|e| e.to_string()))?
        .join()
        .unwrap()?)
}

fn run() -> Result<(), String> {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let did = std::env::var("DEVICE_ID").unwrap_or_default();
    let iid = std::env::var("IID").unwrap_or_default();
    let t0 = Instant::now();
    let mut s = dysign::Signer::new(&dysign::default_resources()).map_err(|e| e.to_string())?;
    eprintln!("[*] 冷启 {}ms", t0.elapsed().as_millis());
    s.set_identity(&did, &iid).map_err(|e| e.to_string())?;
    let gap: u64 = std::env::var("GAP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for k in 0..std::env::var("WARM_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
    {
        if gap > 0 {
            std::thread::sleep(std::time::Duration::from_millis(gap));
        }
        let warm = Instant::now();
        let w = s
            .sign("https://api5-normal-lq.amemv.com/aweme/v1/feed/", "")
            .map_err(|e| e.to_string())?;
        eprintln!(
            "[*] 预热#{k} {}ms, {} 字节",
            warm.elapsed().as_millis(),
            w.len()
        );
    }
    let mut ms = Vec::new();
    if std::env::var("MODE").as_deref() == Ok("identity") {
        let t = Instant::now();
        for _ in 0..n {
            s.set_identity(&did, &iid).map_err(|e| e.to_string())?;
        }
        eprintln!(
            "[*] identity x{n} 共 {}ms, 单次 {}us",
            t.elapsed().as_millis(),
            t.elapsed().as_micros() / n as u128
        );
        return Ok(());
    }
    let token = std::env::var("DEVICE_TOKEN").unwrap_or_default();
    let extra_base = if token.is_empty() {
        "x-ss-stub\r\n00E4834440C330EAA45AAC5A26FC1098".to_string()
    } else {
        format!("x-ss-stub\r\n00E4834440C330EAA45AAC5A26FC1098\r\nx-tt-dt\r\n{token}")
    };
    for i in 0..n {
        let ts = 1793782400u64 + i as u64;
        let url = format!(
            "https://api5-normal-c-lf.amemv.com/aweme/v2/comment/list/?_rticket={}&aid=6383&aweme_id=7671015412280069410&count=50&cursor={}&device_id={did}&device_platform=android&iid={iid}&os=android&ts={}",
            ts * 1000,
            i * 50,
            ts
        );
        if std::env::var("IDENTITY_EACH").is_ok() {
            s.set_identity(&did, &iid).map_err(|e| e.to_string())?;
        }
        let t = Instant::now();
        let out = s.sign(&url, &extra_base).map_err(|e| e.to_string())?;
        let el = t.elapsed().as_millis();
        ms.push(el);
        eprintln!("[*] #{i} {el}ms {} 字节", out.len());
    }
    ms.sort();
    let p = |q: f64| ms[((ms.len() as f64 * q) as usize).min(ms.len() - 1)];
    eprintln!(
        "[*] n={} p50={} p90={} max={}",
        ms.len(),
        p(0.5),
        p(0.9),
        ms[ms.len() - 1]
    );
    Ok(())
}
