//! 稳定性压测：反复签名并按轮次换查询，报告常驻内存增长；多线程时校验确定性。
//! 用法：medusa-stress <url 文件> <次数> [线程数]（release 构建）
//! 常驻内存只在 Linux 上采集，其它平台输出 0。
use dysign_pure::medusa::{Urng, Vm};
use std::io::Write;

/// 当前进程的常驻内存（KiB）；非 Linux 返回 0。
fn rss_kib() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/proc/self/statm") {
            if let Some(pages) = s
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<u64>().ok())
            {
                return pages * 4;
            }
        }
        0
    }
    #[cfg(not(target_os = "linux"))]
    {
        0
    }
}

/// 每轮换一个查询：改内容，逼出新的解码项。`STRESS_FIXED=1` 时始终用同一个。
fn variant(base: &str, i: usize) -> String {
    if std::env::var_os("STRESS_FIXED").is_some() {
        format!("{base}&stress=00000000")
    } else {
        format!("{base}&stress={i:08}")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = std::fs::read_to_string(args.get(1).expect("缺 url 文件")).expect("读不到 url");
    let q = url.split_once('?').map(|x| x.1).unwrap_or("").to_string();
    let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
    let threads: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    const CLOCK: u64 = 1_762_300_000;

    // 先取单线程参考序列。
    let want: Vec<String> = {
        let mut rng = Urng::new(7);
        (0..n)
            .map(|i| Vm::sign_query(&variant(&q, i), CLOCK, &mut rng).expect("参考签名失败"))
            .collect()
    };

    if threads > 1 {
        let arcs: Vec<(&String, &String)> = want.iter().map(|w| (w, w)).collect();
        let _ = &arcs;
        let handle: Vec<_> = (0..threads)
            .map(|_| {
                let q = q.clone();
                let want = want.clone();
                std::thread::spawn(move || {
                    let mut rng = Urng::new(7);
                    let mut bad = 0usize;
                    for (i, w) in want.iter().enumerate() {
                        let got =
                            Vm::sign_query(&variant(&q, i), CLOCK, &mut rng).expect("签名失败");
                        if &got != w {
                            bad += 1;
                            if bad <= 3 {
                                eprintln!("线程输出不同 # {i}");
                            }
                        }
                    }
                    bad
                })
            })
            .collect();
        let mut bad = 0usize;
        for h in handle {
            bad += h.join().expect("线程 panic");
        }
        println!("threads={threads} n={n} 不一致={bad}");
        std::process::exit(i32::from(bad != 0));
    }

    let base = rss_kib();
    let mut peak = base;
    let mut rng = Urng::new(7);
    for (i, w) in want.iter().enumerate() {
        let got = Vm::sign_query(&variant(&q, i), CLOCK, &mut rng).expect("签名失败");
        assert_eq!(&got, w, "第 {i} 次输出与参考不同");
        if i % 200 == 0 {
            let r = rss_kib();
            peak = peak.max(r);
            print!("\rrss={r}KiB (base={base}) ");
            let _ = std::io::stdout().flush();
        }
    }
    let end = rss_kib();
    let (live, free) = dysign_pure::medusa::native::arena_stats();
    println!(
        "\nn={n} rss base={base}KiB peak={peak}KiB end={end}KiB 增长={}KiB opmiss={} live={live} free={free}",
        end.saturating_sub(base),
        dysign_pure::medusa::native::OP_MISS.load(std::sync::atomic::Ordering::Relaxed)
    );
}
