//! 签名并发基准：`signbench [并发数]`，打印单次耗时与总墙钟。
use std::sync::Arc;
use std::time::Instant;

fn main() {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

fn run() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let did = std::env::var("DEVICE_ID").unwrap_or_default();
    let iid = std::env::var("IID").unwrap_or_default();
    let t0 = Instant::now();
    let s = Arc::new(dycore::signer::Signer::spawn_from_env(dysign::default_resources()).unwrap());
    let tok0 = std::env::var("DEVICE_TOKEN").unwrap_or_default();
    let warm_extra = format!("x-ss-stub\r\n00E4834440C330EAA45AAC5A26FC1098\r\nx-tt-dt\r\n{tok0}");
    let warm_url = |i: usize| {
        format!(
        "https://api5-normal-c-lf.amemv.com/aweme/v2/comment/list/?aid=6383&aweme_id=1&count=50&cursor={i}&device_id={did}&device_platform=android&iid={iid}&os=android&ts=1793782000"
    )
    };
    let workers = std::env::var("DYSIGN_WORKERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for i in 0..workers.max(1) * 2 {
        let _ = s.sign(&did, &iid, &warm_url(900 + i), &warm_extra);
    }
    eprintln!(
        "[*] 池就绪 {}ms，实现 {}",
        t0.elapsed().as_millis(),
        s.info().backend.label()
    );
    if std::env::var("MODE").as_deref() == Ok("serial") {
        let mut ms = Vec::new();
        for i in 0..n {
            let ts = 1793783000u64 + i as u64;
            let url = std::env::var("URL").unwrap_or_else(|_| {
                format!(
                "https://api5-normal-c-lf.amemv.com/aweme/v2/comment/list/?aid=6383&aweme_id=7671015412280069410&count=50&cursor={}&device_id={did}&device_platform=android&iid={iid}&os=android&ts={ts}",
                i * 50
            )
            });
            let t = Instant::now();
            let _ = s.sign(&did, &iid, &url, &warm_extra);
            ms.push(t.elapsed().as_millis());
        }
        ms.sort();
        let p = |q: f64| ms[((ms.len() as f64 * q) as usize).min(ms.len() - 1)];
        eprintln!(
            "[*] serial n={n} p50={} p90={} max={} 计数 {:?}",
            p(0.5),
            p(0.9),
            ms[ms.len() - 1],
            s.info()
        );
        return;
    }
    if std::env::var("MODE").as_deref() == Ok("same") {
        let h = Arc::new(dycore::http::Http::new((*s).clone()).unwrap());
        let id = dycore::pool::Identity {
            device_id: did.clone(),
            iid: iid.clone(),
            device_token: std::env::var("DEVICE_TOKEN").unwrap_or_default(),
            brand: "xiaomi".into(),
            model: "2201123C".into(),
            resolution: "1080x2400".into(),
            dpi: "419".into(),
            android_id: String::new(),
            wifi_mac: String::new(),
            bt_mac: String::new(),
            used: 0,
            cdid: String::new(),
            search_state: String::new(),
            status: String::new(),
            ..Default::default()
        };
        let mut p: std::collections::BTreeMap<String, String> = Default::default();
        for (k, v) in [
            ("count", "20"),
            ("aid", "6383"),
            ("device_id", did.as_str()),
            ("iid", iid.as_str()),
            ("device_platform", "android"),
            ("os", "android"),
            ("ts", "1793782400"),
            ("_rticket", "1793782400000"),
        ] {
            p.insert(k.into(), v.into());
        }
        let t = Instant::now();
        let mut hs = Vec::new();
        for _ in 0..n {
            let h = h.clone();
            let id = id.clone();
            let p = p.clone();
            hs.push(std::thread::spawn(move || {
                let t = Instant::now();
                let r = h.presign(
                    &id,
                    "api5-normal-c-lf.amemv.com",
                    "/aweme/v2/feed/",
                    &p,
                    b"{\"count\":20}",
                );
                eprintln!("    {r:?}");
                t.elapsed().as_millis()
            }));
        }
        let mut ms: Vec<u128> = hs.into_iter().map(|h| h.join().unwrap()).collect();
        ms.sort();
        eprintln!(
            "[*] same n={n} 墙钟={}ms max={}",
            t.elapsed().as_millis(),
            ms[ms.len() - 1]
        );
        return;
    }
    let t = Instant::now();
    let mut hs = Vec::new();
    for i in 0..n {
        let s = s.clone();
        let (did, iid) = (did.clone(), iid.clone());
        hs.push(std::thread::spawn(move || {
            let ts = 1793782400u64 + i as u64;
            let url = format!(
                "https://api5-normal-c-lf.amemv.com/aweme/v2/comment/list/?_rticket={}&aid=6383&aweme_id=7671015412280069410&count=50&cursor={}&device_id={did}&device_platform=android&iid={iid}&os=android&ts={ts}",
                ts * 1000,
                i * 50
            );
            let t = Instant::now();
            let tok = std::env::var("DEVICE_TOKEN").unwrap_or_default();
            let extra = format!("x-ss-stub\r\n00E4834440C330EAA45AAC5A26FC1098\r\nx-tt-dt\r\n{tok}");
            let out = s.sign(&did, &iid, &url, &extra);
            let r = (t.elapsed().as_millis(), out.map(|b| b.len()).unwrap_or(0));
            eprintln!("    # {i} {}ms {}B", r.0, r.1);
            r
        }));
    }
    let mut ms: Vec<u128> = hs.into_iter().map(|h| h.join().unwrap().0).collect();
    let wall = t.elapsed().as_millis();
    ms.sort();
    let p = |q: f64| ms[((ms.len() as f64 * q) as usize).min(ms.len() - 1)];
    eprintln!(
        "[*] n={n} 墙钟={wall}ms p50={} p90={} max={}",
        p(0.5),
        p(0.9),
        ms[ms.len() - 1]
    );
}
