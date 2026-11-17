use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() {
    let target: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let dir = std::env::temp_dir().join(format!("dy-pool-smoke-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let api = Arc::new(
        dycore::api::Api::new_stored(
            dysign::default_resources(),
            dycore::signer::Backend::Native,
            dir.join("pool.jsonl"),
            target,
        )
        .unwrap(),
    );
    api.start_pool_maintainer();
    let start = Instant::now();
    loop {
        let s = api.identity_status();
        eprintln!("{:?}", s);
        if s.total >= target {
            println!("OK {}", dir.join("pool.jsonl").display());
            return;
        }
        if start.elapsed() > Duration::from_secs(180) {
            panic!("timeout: {:?}", s);
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}
