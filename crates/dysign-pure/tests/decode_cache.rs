//! 解码缓存占用：单独成文件，避免其它测试的并发解码干扰计数断言。

use dysign_pure::medusa::native::arena_stats;
use dysign_pure::{Device, PureSigner};

fn sign_medusa(url: &str, extra: &str) -> String {
    let signer = PureSigner::new(&Device::default(), 1_762_300_000, 1);
    signer.sign(url, extra).expect("签名失败")
}

fn case1_inputs() -> (String, String) {
    use base64::Engine;
    const B64: base64::engine::general_purpose::GeneralPurpose =
        base64::engine::general_purpose::STANDARD;
    let line = std::fs::read_to_string("../../scripts/fixtures/pure-cases.tsv").expect("缺用例");
    let fields: Vec<_> = line.lines().next().expect("用例为空").split('\t').collect();
    (
        String::from_utf8(B64.decode(fields[1]).expect("URL 不是 Base64")).unwrap(),
        String::from_utf8(B64.decode(fields[2]).expect("extra 不是 Base64")).unwrap(),
    )
}

/// 复用线程时解码项必须回收：默认路径靠换主回收，`MEDUSA_OPCLEAR` 路径靠整表批量回收。
#[test]
fn decode_cache_stays_bounded() {
    let (url, extra) = case1_inputs();
    for _ in 0..12 {
        sign_medusa(&url, &extra);
    }
    let warm = arena_stats().0;
    for _ in 0..60 {
        sign_medusa(&url, &extra);
    }
    let after = arena_stats().0;
    assert!(after <= warm + 64, "解码项随调用次数增长：{warm} → {after}");

    std::env::set_var("MEDUSA_OPCLEAR", "1");
    for _ in 0..5 {
        sign_medusa(&url, &extra);
    }
    let (warm, _) = arena_stats();
    for _ in 0..20 {
        sign_medusa(&url, &extra);
    }
    let (live, free) = arena_stats();
    std::env::remove_var("MEDUSA_OPCLEAR");
    assert!(
        live <= warm + 64,
        "整表清空后解码项仍有增长：{warm} → {live}"
    );
    // 清空后同一轮会把回收项全部取回，末态只看得到零星剩余。
    assert!(free >= 1, "回收表没有被填充");
}
