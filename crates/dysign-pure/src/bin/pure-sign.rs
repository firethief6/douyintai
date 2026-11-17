//! 探针 CLI：`pure-sign '<url>' '<extra>'`，输出与台子同形的 7 头。
//! 环境变量：DEVICE_ID 选池内身份，DYSIGN_FIXED_TIME 秒，DYSIGN_RANDOM_SEED 种子，
//! DYSIGN_POOL 指定身份池路径。
use dysign_pure::{Device, PureSigner};
use std::io::Write;

fn b64d(s: &str) -> String {
    use base64::Engine;
    String::from_utf8_lossy(
        &base64::engine::general_purpose::STANDARD
            .decode(s)
            .unwrap_or_default(),
    )
    .into_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).unwrap_or_default();
    let extra = std::env::args().nth(2).unwrap_or_default();
    let pool = std::env::var("DYSIGN_POOL").unwrap_or_else(|_| {
        dirs::config_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("douyintai/identity-pool.jsonl")
            .to_string_lossy()
            .into_owned()
    });
    let want = std::env::var("DEVICE_ID").unwrap_or_default();
    let text = std::fs::read_to_string(&pool)?;
    let mut device = None;
    for line in text.lines() {
        let d: Device = serde_json::from_str(line)?;
        if d.device_id == want {
            device = Some(d);
            break;
        }
    }
    let device = device.ok_or_else(|| format!("池里没有 device_id={want}"))?;
    let clock: u64 = std::env::var("DYSIGN_FIXED_TIME")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let seed: u64 = std::env::var("DYSIGN_RANDOM_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let signer = PureSigner::new(&device, clock, seed);
    if let Ok(seq) = std::env::var("DYSIGN_SEQ") {
        let dir = std::env::var("DYSIGN_SEQ_OUT").unwrap_or_else(|_| "/tmp/pure/pseq".into());
        std::fs::create_dir_all(&dir)?;
        for (i, line) in std::fs::read_to_string(seq)?.lines().enumerate() {
            let (u, h) = line.split_once('\t').unwrap_or((line, ""));
            let out = signer.sign(&b64d(u), &b64d(h))?;
            std::fs::write(format!("{dir}/{i:04}.bin"), out)?;
        }
        return Ok(());
    }
    let out = signer.sign(&url, &extra)?;
    let mut so = std::io::stdout();
    so.write_all(out.as_bytes())?;
    so.flush()?;
    Ok(())
}
