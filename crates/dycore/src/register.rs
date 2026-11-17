//! 本地 device_register：生成整机明文、gzip、ttEncrypt、签名并投递。
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde_json::{json, Value};

use crate::http::Http;
use crate::pool::Identity;
use crate::{Error, Result};

const HOST: &str = "https://log.snssdk.com";
const PATH: &str = "/service/2/device_register/";

/// 型号 -> (品牌, 厂商, 可见高, 宽, dpi, 导航栏高度)
const HANDSETS: &[(&str, &str, &str, u32, u32, u32, u32)] = &[
    ("2201123C", "xiaomi", "Xiaomi", 2400, 1080, 419, 126),
    ("22101316C", "xiaomi", "Xiaomi", 2400, 1080, 409, 126),
    ("V2183A", "vivo", "vivo", 2376, 1080, 394, 121),
    ("PGT-AN00", "oppo", "OPPO", 2412, 1080, 402, 121),
    ("PDVM00", "oppo", "OPPO", 1600, 720, 270, 96),
    ("SM-G991B", "samsung", "samsung", 2400, 1080, 421, 126),
    ("SM-A536B", "samsung", "samsung", 2400, 1080, 405, 126),
    ("FNE-AN00", "honor", "HONOR", 2400, 1080, 395, 126),
    ("RKY-AN00", "honor", "HONOR", 2388, 1080, 394, 126),
    ("CET-AL00", "huawei", "HUAWEI", 2700, 1224, 480, 141),
    ("NOH-AN00", "huawei", "HUAWEI", 2772, 1344, 456, 145),
];

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn hex(bytes: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..bytes * 2)
        .map(|_| char::from_digit(rng.gen_range(0..16), 16).expect("hex"))
        .collect()
}

fn mac() -> String {
    let mut rng = rand::thread_rng();
    let mut out = String::from("02");
    for _ in 0..5 {
        out.push(':');
        out.push_str(&format!("{:02x}", rng.gen_range(0..=255u8)));
    }
    out
}

fn query(header: &Value, ts: u64) -> String {
    let mut ser = url::form_urlencoded::Serializer::new(String::new());
    let text = |key: &str| header[key].as_str().unwrap_or_default().to_string();
    let resolution = text("resolution").replace('x', "*");
    let pairs = [
        ("tt_data", "a".to_string()),
        ("ac", "mobile".to_string()),
        ("channel", text("channel")),
        ("aid", "1128".to_string()),
        ("app_name", "aweme".to_string()),
        ("version_code", header["version_code"].to_string()),
        ("version_name", text("app_version")),
        ("device_platform", "android".to_string()),
        ("os", "android".to_string()),
        ("ssmix", "a".to_string()),
        ("device_type", text("device_model")),
        ("device_brand", text("device_brand")),
        ("language", text("language")),
        ("os_api", header["os_api"].to_string()),
        ("os_version", text("os_version")),
        (
            "manifest_version_code",
            header["manifest_version_code"].to_string(),
        ),
        ("resolution", resolution),
        ("dpi", header["density_dpi"].to_string()),
        (
            "update_version_code",
            header["update_version_code"].to_string(),
        ),
        ("_rticket", (ts * 1000 + 3).to_string()),
        ("package", text("package")),
        (
            "first_launch_timestamp",
            (header["apk_first_install_time"].as_u64().unwrap_or(0) / 1000).to_string(),
        ),
        ("last_deeplink_update_version_code", "0".to_string()),
        ("cpu_support64", "true".to_string()),
        ("host_abi", "arm64-v8a".to_string()),
        ("is_guest_mode", "0".to_string()),
        ("app_type", "normal".to_string()),
        ("minor_status", "0".to_string()),
        ("appTheme", "light".to_string()),
        ("is_preinstall", "0".to_string()),
        ("need_personal_recommend", "1".to_string()),
        ("is_android_pad", "0".to_string()),
        ("is_android_fold", "0".to_string()),
        ("ts", ts.to_string()),
        ("cdid", text("cdid")),
        ("md", "8".to_string()),
    ];
    for (k, v) in pairs {
        ser.append_pair(k, &v);
    }
    ser.finish()
}

fn plain(ip: &str) -> (Value, &'static str, String, u64) {
    let mut rng = rand::thread_rng();
    let handset = HANDSETS[rng.gen_range(0..HANDSETS.len())];
    let (model, brand, maker, height, width, dpi, nav) = handset;
    let install = now_ms() - rng.gen_range(60_000..900_000);
    let cdid = uuid();
    let header = json!({

            "display_name": "抖音", "update_version_code": 32209900,
            "manifest_version_code": 320201, "app_version_minor": "", "aid": 1128,
            "channel": "douyin_sem_baidu_yj_and_nonbrand_4",
            "package": "com.ss.android.ugc.aweme", "app_version": "32.2.0",
            "version_code": 320200, "sdk_version": "3.7.3-rc.50-douyin",
            "sdk_target_version": 29, "git_hash": "ea27dc1", "os": "Android",
            "os_version": "13", "os_api": 33, "device_model": model,
            "device_brand": brand, "device_manufacturer": maker,
            "device_category": "phone", "cpu_abi": "arm64-v8a",
            "release_build": "b4e8c50_20241118_13966264-a59e-11ef-a06d-e63c58e947e6",
            "density_dpi": dpi, "display_density": "mdpi",
            "resolution": format!("{}x{}", height - nav, width),
            "language": "zh", "timezone": 8, "access": "mobile", "not_request_sender": 0,
            "rom": "eng.builder.20234395.811666",
            "rom_version": "redroid_x86_64_only-userdebug 13 TQ3A.230805.001.S1 eng.frank.20240527.135352 test-keys",
            "cdid": cdid, "sig_hash": "aea615ab910015038f73c47e45d21466",
            "openudid": uuid::Uuid::new_v4().simple().to_string().chars().take(16).collect::<String>(),
            "clientudid": uuid(), "region": "CN", "tz_name": "Asia/Shanghai",
            "tz_offset": 28800, "sim_serial_number": [], "oaid_may_support": false,
            "req_id": uuid(), "device_platform": "android",
            "custom": {"client_ipv4": ip}, "apk_first_install_time": install,
            "is_system_app": 0, "sdk_flavor": "china", "guest_mode": 0
    });
    let body = json!({"magic_tag": "ss_app_log", "header": header, "_gen_time": now_ms()});
    (body, model, cdid, install)
}

/// 注册一条并在返回前填好可入池的完整字段。网络与签名错误原样上抛。
pub fn register(http: &Http, resources: &std::path::Path) -> Result<Identity> {
    let mut picked = http.pick_proxy();
    let ip = http.public_ip_via(picked.as_deref());
    let (body, model, cdid, install) = plain(&ip);
    let header = &body["header"];
    let plain = serde_json::to_vec(&body)?;
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(6));
    gz.write_all(&plain)?;
    let compressed = gz.finish()?;
    let encrypted = dysign::encrypt::tt_encrypt(resources, &compressed).map_err(Error::from)?;
    let ts = now_ms() / 1000;
    let url = format!("{HOST}{PATH}?{}", query(header, ts));
    let text = http.register_request(&url, &encrypted, picked.as_deref())?;
    let value: Value = serde_json::from_slice(&text)?;
    let device_id = match &value["device_id"] {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => String::new(),
    };
    let iid = match &value["install_id"] {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => String::new(),
    };
    if device_id.is_empty() || device_id == "0" || iid.is_empty() {
        return Err(Error::msg(format!(
            "注册未签发：{}",
            String::from_utf8_lossy(&text)
                .chars()
                .take(160)
                .collect::<String>()
        )));
    }
    picked = picked.or_else(|| http.pick_proxy());
    http.bind_proxy(&device_id, picked.as_deref());
    let brand = header["device_brand"].as_str().unwrap_or_default();
    Ok(Identity {
        device_id,
        iid,
        device_token: value["device_token"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        brand: brand.to_string(),
        model: model.to_string(),
        resolution: header["resolution"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        dpi: header["density_dpi"].to_string(),
        android_id: hex(8),
        wifi_mac: mac(),
        bt_mac: mac(),
        cdid,
        search_state: "available".into(),
        status: "available".into(),
        first_launch: (install / 1000).to_string(),
        ..Identity::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_has_self_consistent_device_fields() {
        let (body, model, cdid, install) = plain("203.0.113.7");
        let h = &body["header"];
        assert_eq!(h["device_model"], model);
        assert!(HANDSETS.iter().any(|(m, ..)| *m == model));
        assert_eq!(h["cdid"], cdid);
        assert_eq!(h["custom"]["client_ipv4"], "203.0.113.7");
        assert!(install > 0);
        let q = query(h, 1_700_000_000);
        assert!(q.contains("md=8"));
        assert!(!q.contains("device_id="));
        assert!(!q.contains("iid="));
        assert!(q.contains("resolution="));
    }
}
