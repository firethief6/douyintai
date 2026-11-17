use serde_json::{json, Map, Value};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = dycore::api::Api::new(dysign::default_resources())?;
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "feed".into());
    let mut q = Map::new();
    for a in args {
        if let Some((k, v)) = a.split_once('=') {
            q.insert(k.into(), Value::String(v.into()));
        }
    }
    let n: usize = std::env::var("REPEAT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    if let Ok(w) = std::env::var("WARM") {
        for p in w.split(',') {
            let _ = api.call(p, &json!({}));
        }
    }
    if std::env::var("WARM_SIGN").is_ok() {
        api.warm()?;
        eprintln!("[*] 预热完成");
    }
    let mut last = Value::Null;
    for _ in 0..n {
        let t = std::time::Instant::now();
        last = api.call(&path, &json!(q))?;
        eprintln!("[*] {}ms", t.elapsed().as_millis());
    }
    println!("{}", serde_json::to_string_pretty(&last)?);
    Ok(())
}
