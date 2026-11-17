//! 热进程单线程延迟测量：同一查询重复签名，打印 p50/p99 与阶段构成。
//! 用法：MEDUSA_MC=1 medusa-bench <url 文件> [次数]（release 构建，串行，无其它负载）
//! 阶段：0 池取用/回滚、1 输入注入、2 AOT 执行、3 输出读取与编码。
use dysign_pure::medusa::{phases, Urng, Vm};
use std::time::Instant;

fn us(ns: u128) -> String {
    format!("{:.1}", ns as f64 / 1000.0)
}

fn median(v: &mut [u128]) -> u128 {
    v.sort_unstable();
    v[(v.len() - 1) / 2]
}

fn main() {
    let url = std::fs::read_to_string(std::env::args().nth(1).expect("缺 url 文件")).unwrap();
    let q = url.split_once('?').map(|x| x.1).unwrap_or("").to_string();
    let n: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);
    let mut rng = Urng::new(1);
    for _ in 0..20 {
        let _ = Vm::sign_query(&q, 1762300000, &mut rng).unwrap();
    }
    phases::take();
    let mut us_v = Vec::with_capacity(n);
    for _ in 0..n {
        let t = Instant::now();
        let _ = Vm::sign_query(&q, 1762300000, &mut rng).unwrap();
        us_v.push(t.elapsed().as_micros());
    }
    us_v.sort_unstable();
    println!(
        "n={} qlen={} p50={}us p99={}us max={}us",
        n,
        q.len(),
        us_v[(us_v.len() * 50 / 100).min(us_v.len() - 1)],
        us_v[(us_v.len() * 99 / 100).min(us_v.len() - 1)],
        us_v[us_v.len() - 1]
    );
    dysign_pure::medusa::native::dump_hist();
    let s = phases::take();
    if s.len() == n {
        let names = ["pool", "inject", "run", "encode", "nat", "aot", "merge"];
        let mut cols: Vec<Vec<u128>> = (0..names.len())
            .map(|i| s.iter().map(|v| v[i]).collect())
            .collect();
        let mut total: Vec<u128> = (0..n).map(|i| s[i].iter().sum()).collect();
        let sum_med = median(&mut total);
        let run_med = median(&mut cols[2]);
        let mut parts = String::new();
        for (i, col) in cols.iter_mut().enumerate() {
            let m = median(col);
            let base = if i >= 4 { run_med } else { sum_med };
            let share = if base == 0 {
                0.0
            } else {
                m as f64 * 100.0 / base as f64
            };
            parts.push_str(&format!(" {}={}us({:.1}%)", names[i], us(m), share));
        }
        println!("PHASES sum_p50={}us{}", us(sum_med), parts);
        let cnt = |i: usize| median(&mut s.iter().map(|v| v[i]).collect::<Vec<_>>());
        println!(
            "COUNTS eng={} natcall={} tramp={} aot={}",
            cnt(7),
            cnt(8),
            cnt(9),
            cnt(11)
        );
        let natv = phases::take_nat();
        let mut pg: std::collections::BTreeMap<u64, (u128, u64)> = Default::default();
        let mut all = 0u128;
        for (a, ns, _) in &natv {
            let e = pg.entry(a >> 12).or_default();
            e.0 += *ns;
            e.1 += 1;
            all += *ns;
        }
        let mut pv: Vec<_> = pg.into_iter().collect();
        pv.sort_by_key(|x| std::cmp::Reverse(x.1 .0));
        for (p, (ns, n)) in pv.iter().take(12) {
            println!(
                "NATPAGE {:#x} {:.1}% calls={} avg={}ns",
                p << 12,
                *ns as f64 * 100.0 / all.max(1) as f64,
                n,
                ns / *n as u128
            );
        }
        let mut av: std::collections::BTreeMap<u64, (u128, u64, u64)> = Default::default();
        for (a, ns, st) in &natv {
            let e = av.entry(*a).or_default();
            e.2 += *st;
            e.0 += *ns;
            e.1 += 1;
        }
        let mut avv: Vec<_> = av.into_iter().collect();
        avv.sort_by_key(|x| std::cmp::Reverse(x.1 .0));
        for (a, (ns, n, st)) in avv.iter().take(20) {
            println!(
                "NATADDR {:#x} {:.1}% calls={} avg={}ns steps={} per={}",
                a,
                *ns as f64 * 100.0 / all.max(1) as f64,
                n,
                ns / *n as u128,
                st,
                st / *n
            );
        }
        println!(
            "OPMISS {}",
            dysign_pure::medusa::native::OP_MISS.load(std::sync::atomic::Ordering::Relaxed)
        );
        let run_col = &cols[2];
        println!(
            "RUN p50={}us p99={}us",
            us(run_col[run_col.len() * 50 / 100]),
            us(run_col[(run_col.len() * 99 / 100).min(run_col.len() - 1)])
        );
    } else {
        println!("PHASES samples={}", s.len());
    }
}
