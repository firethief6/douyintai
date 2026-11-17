use dysign_pure::{block, rng, Device, PureSigner};

const HDR: &str = "Host\r\nx";

#[test]
fn prng_vector() {
    let mut r = rng::Type3::new(1762300000);
    let want: [u32; 9] = [
        0x15b8cb42, 0x66140c54, 0x37e435b8, 0x56a1f72d, 0x5e65986a, 0x221f2707, 0x5bdc6c5b,
        0x76e73edc, 0x375afd8b,
    ];
    for (i, w) in want.iter().enumerate() {
        assert_eq!(r.next_u31(), *w, "第 {} 个输出", i + 1);
    }
}

#[test]
fn prng_nth() {
    let mut r = rng::Type3::new(1762300000);
    assert_eq!(r.nth0(4), 0x56a1f72d);
    assert_eq!(r.nth0(4), 0x76e73edc);
}

#[test]
fn block_roundtrip_known() {
    // 28B 明文 → 32B 密文；用 KAT 的 K2 与 H 前缀段。
    let key = *b"b6c1fa38e2a3943738eedfe666349a89";
    let plain: Vec<u8> = b"1762300000-1378330313-664226"
        .iter()
        .copied()
        .chain([4u8; 4])
        .collect();
    let out = block::transform(&plain, &key);
    assert_eq!(out.len(), 32);
    assert_eq!(&out[..8], &[0x34, 0x5e, 0xb1, 0x71, 0x0f, 0x5b, 0xcd, 0x3f]);
}

#[test]
fn helios_kat() {
    let s = PureSigner::new(&Device::default(), 1762300000, 1);
    let url = "https://api5-normal-lq.amemv.com/service/settings/v3/?aid=1128&app_version=32.2.0";
    let out = s.sign(url, HDR).unwrap();
    assert!(
        out.contains("\r\n3D7ndjResXEPW80/HPYAEwLJ/2LRiLa2o0FL5hciWtBgBzjq\r\n"),
        "{out}"
    );
}

#[test]
fn helios_aid_varies() {
    let a = PureSigner::new(&Device::default(), 1762300000, 1)
        .sign("https://x/?aid=1", HDR)
        .unwrap();
    let b = PureSigner::new(&Device::default(), 1762300000, 1)
        .sign("https://x/?aid=7", HDR)
        .unwrap();
    assert!(
        a.contains("3D7ndt5QXiQqBcpp16nNQ5dAC4Hdoa2XbH1BH5wNU76N+OJZ"),
        "{a}"
    );
    assert!(
        b.contains("3D7ndlyx8XKDAnoCeRas4cRsryQdilEPc3V9ypIbA+4V9j8T"),
        "{b}"
    );
}

#[test]
fn missing_aid() {
    let s = PureSigner::new(&Device::default(), 1, 1);
    let out = s.sign("https://x/?app_version=1", "").unwrap();
    assert_eq!(
        out,
        "X-Neptune\r\n-8|50:51:59:20:21:30:40:47:49:39:22:29\r\n"
    );
    assert_eq!(out.len(), 51);
}

#[test]
fn set_device_keeps_stream() {
    let s = PureSigner::new(&Device::default(), 1762300000, 1);
    let a = s.sign("https://x/?aid=1128", HDR).unwrap();
    let mut s2 = PureSigner::new(&Device::default(), 1762300000, 1);
    let a2 = s2.sign("https://x/?aid=1128", HDR).unwrap();
    s2.set_device(Device {
        device_id: "x".into(),
        ..Default::default()
    });
    let b = s2.sign("https://x/?aid=1128", HDR).unwrap();
    let b1 = s.sign("https://x/?aid=1128", HDR).unwrap();
    assert_eq!(a, a2);
    assert_eq!(b, b1, "set_device 不应推进 Helios PRNG 流");
}

#[test]
fn stream_continues_across_signs() {
    let s = PureSigner::new(&Device::default(), 1762300000, 1);
    let a = s.sign("https://x/?aid=1128", HDR).unwrap();
    let b = s.sign("https://x/?aid=1128", HDR).unwrap();
    assert!(
        a.contains("3D7ndjResXEPW80/HPYAEwLJ/2LRiLa2o0FL5hciWtBgBzjq"),
        "{a}"
    );
    assert!(
        b.contains("iTkOeCGd2j90V095CyCy1q2FUQAkwNWHjxzkxjyD3lnG7WO2"),
        "{b}"
    );
}

#[test]
fn strict_surfaces_medusa_failure() {
    let s = PureSigner::new(&Device::default(), 1762300000, 1);
    let long = format!("https://x/?aid=1&pad={}", "x".repeat(70_000));
    assert!(s.sign(&long, HDR).is_ok());
    assert!(s.sign_strict(&long, HDR).is_err());
}
