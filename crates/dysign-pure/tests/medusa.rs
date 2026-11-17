//! Medusa VM KAT：入口快照直跑 + `sign_query` 生产路径。
//!
//! `MEDUSA_SNAP`+`MEDUSA_EXP`+`MEDUSA_URL` 指定时只跑该条引擎级用例。

use base64::Engine;
use dysign_pure::medusa::{Urng, Vm, OUT_ADDR};
use dysign_pure::{Device, PureSigner};
use std::path::PathBuf;

const B64: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::general_purpose::STANDARD;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scripts/fixtures/medusa-vm")
        .join(name)
}

struct Case {
    name: &'static str,
    snap: &'static str,
    exp: &'static str,
    url: &'static str,
}

const CASES: &[Case] = &[
    Case {
        name: "case1",
        snap: "medusa-snap.bin",
        exp: "expected.bin",
        url: "case1.url",
    },
    Case {
        name: "case2",
        snap: "medusa-snap-2.bin",
        exp: "expected-2.bin",
        url: "case2.url",
    },
    Case {
        name: "case3",
        snap: "medusa-snap.bin",
        exp: "expected-3.bin",
        url: "case3.url",
    },
];

const CLOCK: u64 = 1_762_300_000;
const SEED: u64 = 1;

fn query_of(url: &str) -> &str {
    url.split_once('?').expect("URL 没有 query").1
}

fn medusa_of(out: &str) -> String {
    out.split("\r\n")
        .skip_while(|line| *line != "X-Medusa")
        .nth(1)
        .unwrap_or_default()
        .to_string()
}

fn case1_inputs() -> (String, String) {
    let line = std::fs::read_to_string("../../scripts/fixtures/pure-cases.tsv").expect("缺用例");
    let fields: Vec<_> = line.lines().next().expect("用例为空").split('\t').collect();
    let url = String::from_utf8(B64.decode(fields[1]).expect("URL 不是 Base64")).expect("URL");
    let extra =
        String::from_utf8(B64.decode(fields[2]).expect("extra 不是 Base64")).expect("extra");
    (url, extra)
}

fn sign_medusa(url: &str, extra: &str) -> String {
    let signer = PureSigner::new(&Device::default(), CLOCK, SEED);
    medusa_of(&signer.sign(url, extra).expect("签名失败"))
}

fn run_snapshot(name: &str, snap: &[u8], exp: &[u8]) {
    let mut vm = Vm::from_snapshot(snap).expect("快照损坏");
    let r = vm.run(20_000_000);
    eprintln!(
        "[{name}] steps={} pc={:#x} ok={}",
        vm.steps,
        vm.pc,
        r.is_ok()
    );
    r.expect("执行失败");
    assert_eq!(
        vm.mem.read(OUT_ADDR, exp.len()).expect("读不到输出"),
        exp,
        "[{name}] 输出不同"
    );
}

#[test]
fn vm_reproduces_expected_bytes() {
    if let (Ok(snap), Ok(exp)) = (std::env::var("MEDUSA_SNAP"), std::env::var("MEDUSA_EXP")) {
        run_snapshot(
            "override",
            &std::fs::read(snap).expect("缺快照"),
            &std::fs::read(exp).expect("缺期望输出"),
        );
        return;
    }
    let case = &CASES[0];
    run_snapshot(
        case.name,
        &std::fs::read(fixture(case.snap)).expect("缺快照"),
        &std::fs::read(fixture(case.exp)).expect("缺期望输出"),
    );
}

#[test]
fn sign_query_matches_fixtures() {
    let mut rng = Urng::new(SEED);
    if let (Ok(exp), Ok(url)) = (std::env::var("MEDUSA_EXP"), std::env::var("MEDUSA_URL")) {
        let want = B64.encode(std::fs::read(exp).expect("缺期望输出"));
        let url = std::fs::read_to_string(url).expect("缺 URL");
        assert_eq!(
            Vm::sign_query(query_of(&url), CLOCK, &mut rng).expect("执行失败"),
            want
        );
        return;
    }
    for case in CASES {
        let mut rng = Urng::new(SEED);
        let want = B64.encode(std::fs::read(fixture(case.exp)).expect("缺期望输出"));
        let url = std::fs::read_to_string(fixture(case.url)).expect("缺 URL");
        assert_eq!(
            Vm::sign_query(query_of(&url), CLOCK, &mut rng).expect("执行失败"),
            want,
            "[{}] 输出不同",
            case.name
        );
    }
}

#[test]
fn sign_query_is_clock_sensitive() {
    let mut rng = Urng::new(SEED);
    let url = std::fs::read_to_string(fixture(CASES[0].url)).expect("缺 URL");
    let q = query_of(&url);
    let a = B64
        .decode(Vm::sign_query(q, CLOCK, &mut rng).expect("执行失败"))
        .unwrap();
    let b = B64
        .decode(Vm::sign_query(q, CLOCK + 1, &mut rng).expect("执行失败"))
        .unwrap();
    assert_ne!(a, b, "改时钟秒值输出不变");
}

#[test]
fn stub_changes_medusa_and_empty_equals_missing() {
    let (url, extra) = case1_inputs();
    let default = "D41D8CD98F00B204E9800998ECF8427E";
    let real = extra.replace(default, "00E4834440C330EAA45AAC5A26FC1098");
    let empty = extra.replace(default, "");
    let missing = extra.replacen(&format!("X-SS-STUB\r\n{default}\r\n"), "", 1);

    let base = sign_medusa(&url, &extra);
    let real = sign_medusa(&url, &real);
    let empty = sign_medusa(&url, &empty);
    let missing = sign_medusa(&url, &missing);
    assert_ne!(base, real, "换 stub 后 X-Medusa 不变");
    assert_eq!(empty, missing, "空 stub 与缺失 stub 必须同值");
}

#[test]
fn repeat_same_input_is_stable() {
    let (url, extra) = case1_inputs();
    let a = sign_medusa(&url, &extra);
    let b = sign_medusa(&url, &extra);
    assert_eq!(a, b, "同一输入两次签名结果不同");
    assert!(!a.is_empty(), "X-Medusa 为空");
}

#[test]
fn concurrent_signers_match_serial() {
    let (url, extra) = case1_inputs();
    let want: Vec<String> = (0..12).map(|_| sign_medusa(&url, &extra)).collect();
    assert!(want.iter().all(|w| *w == want[0]), "串行结果就不一致");
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let (url, extra) = (url.clone(), extra.clone());
            std::thread::spawn(move || {
                (0..12)
                    .map(|_| sign_medusa(&url, &extra))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    for h in hs {
        assert_eq!(h.join().expect("线程 panic"), want, "并发结果与串行不同");
    }
}

#[test]
fn query_shapes_are_all_accepted() {
    let (url, extra) = case1_inputs();
    let base = query_of(&url).to_string();
    let shapes = [
        base.clone(),
        format!("{base}&padding=0123456789abcdef"),
        format!("{base}&cjk=%E7%8C%AB"),
        format!("{base}&x=1&y=2&z=3"),
    ];
    let outs: Vec<String> = shapes
        .iter()
        .map(|q| {
            let s = PureSigner::new(&Device::default(), CLOCK, SEED);
            medusa_of(
                &s.sign(&format!("https://x/?{q}"), &extra)
                    .expect("签名失败"),
            )
        })
        .collect();
    for (q, o) in shapes.iter().zip(&outs) {
        assert!(!o.is_empty(), "形状 {q} 的 X-Medusa 为空");
    }
    assert_ne!(outs[0], outs[1], "加长查询后 X-Medusa 不变");
}

#[test]
fn over_long_query_is_rejected() {
    let (url, extra) = case1_inputs();
    let qlen = query_of(&url).len();
    // 追加而不是替换 query，保住 URL 里的 aid。
    let ok = format!("{url}&{}", "a".repeat(0xffff - qlen - 1));
    let bad = format!("{url}&{}", "a".repeat(0x1_0000 - qlen - 1));
    assert_eq!(query_of(&ok).len(), 0xffff);
    assert_eq!(query_of(&bad).len(), 0x1_0000);
    let mut rng = Urng::new(SEED);
    assert!(
        Vm::sign_query(query_of(&ok), CLOCK, &mut rng).is_ok(),
        "未超长却被拒"
    );
    assert!(
        Vm::sign_query(query_of(&bad), CLOCK, &mut rng).is_err(),
        "超长没有被拒"
    );
    // 产品路径不因超长中断：退回夹具常量，其余头照常。
    let signer = PureSigner::new(&Device::default(), CLOCK, SEED);
    let out = signer.sign(&bad, &extra).expect("超长不该让签名失败");
    assert!(out.contains("X-Medusa\r\n"), "超长时缺 X-Medusa 头");
}

#[test]
fn truncated_snapshot_is_rejected() {
    let snap = std::fs::read(fixture(CASES[0].snap)).expect("缺快照");
    for cut in [0, 7, 384, 392, snap.len() / 2, snap.len() - 1] {
        assert!(
            Vm::from_snapshot(&snap[..cut]).is_none(),
            "截断到 {cut} 字节仍被接受"
        );
    }
}

#[test]
fn bad_page_address_is_rejected() {
    let snap = std::fs::read(fixture(CASES[0].snap)).expect("缺快照");
    // 头部是 pc + 48 个槽，紧跟页数；第一页地址在 8 + 48*8 + 4。
    let at = 8 + 48 * 8 + 4;
    let addr = u64::from_le_bytes(snap[at..at + 8].try_into().unwrap());
    assert_eq!(addr & 0xfff, 0, "夹具页地址本该对齐");

    let mut misaligned = snap.clone();
    misaligned[at..at + 8].copy_from_slice(&(addr + 1).to_le_bytes());
    assert!(
        Vm::from_snapshot(&misaligned).is_none(),
        "未对齐页地址被接受"
    );

    let mut off_region = snap;
    off_region[at..at + 8].copy_from_slice(&0xdead_0000u64.to_le_bytes());
    assert!(
        Vm::from_snapshot(&off_region).is_none(),
        "区间外页地址被接受"
    );
}

#[test]
fn step_limit_exhaustion_is_reported() {
    let snap = std::fs::read(fixture(CASES[0].snap)).expect("缺快照");
    let mut vm = Vm::from_snapshot(&snap).expect("快照损坏");
    assert!(
        matches!(vm.run(1), Err(dysign_pure::medusa::Error::Steps)),
        "步数用尽没有报 Steps"
    );
    assert_eq!(vm.steps, 1, "步数用尽时步计应停在限额");
}

#[test]
fn idle_pool_rebuild_matches_warm() {
    let (url, extra) = case1_inputs();
    let want = sign_medusa(&url, &extra);
    dysign_pure::medusa::set_idle_secs(0);
    let a = sign_medusa(&url, &extra);
    let b = sign_medusa(&url, &extra);
    dysign_pure::medusa::set_idle_secs(300);
    assert!(!want.is_empty(), "X-Medusa 为空");
    assert_eq!(a, want, "重建实例后输出不同");
    assert_eq!(b, want, "重建实例后输出不同（第二次）");
}

#[cfg(target_os = "linux")]
fn rss_kib() -> u64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        .map(|pages: u64| pages * 4)
        .unwrap_or(0)
}

/// 变长输入连签：常驻内存增长必须有界，且显式回收后回落。
#[cfg(target_os = "linux")]
#[test]
fn rss_growth_is_bounded_and_released() {
    let (url, extra) = case1_inputs();
    let q = query_of(&url).to_string();
    for _ in 0..20 {
        sign_medusa(&url, &extra);
    }
    let base = rss_kib();
    for i in 0..300 {
        let u = url.replacen(&q, &format!("{q}&n={i}"), 1);
        sign_medusa(&u, &extra);
    }
    let grew = rss_kib().saturating_sub(base);
    assert!(grew < 24 * 1024, "300 次签名 RSS 增长 {grew}KiB 超过 24MiB");
    let before = rss_kib();
    dysign_pure::medusa::release_pool();
    let after = rss_kib();
    assert!(
        after <= before,
        "release_pool 后 RSS 未回落: {before}→{after}KiB"
    );
}
