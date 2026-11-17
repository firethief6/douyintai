//! 请求参数与 UA 装配：照真机字段逐项对齐（见上游 docs/30-data.md）。
use md5::Digest;
use std::collections::BTreeMap;

use crate::pool::Identity;

pub const VERSION_CODE: &str = "320206";
pub const VERSION_NAME: &str = "32.2.6";
pub const PACKAGE: &str = "com.ss.android.ugc.aweme.mobile";
pub const APP_NAME: &str = "douyinos";

pub fn os_of(brand: &str) -> (&'static str, &'static str) {
    match brand.to_ascii_lowercase().as_str() {
        "huawei" | "honor" => ("11", "30"),
        "samsung" => ("13", "33"),
        _ => ("13", "33"),
    }
}

pub fn build_ua(id: &Identity) -> String {
    let brand = id.brand.to_ascii_lowercase();
    let build = match brand.as_str() {
        "honor" => format!("HONOR{}", id.model),
        "xiaomi" | "redmi" | "poco" => "TKQ1.221114.001".into(),
        "samsung" => "TP1A.220624.014".into(),
        "huawei" => "HUAWEIPGT-AN00".into(),
        _ => "TQ3A.230805.001.S1".into(),
    };
    let (os, _) = os_of(&id.brand);
    format!("{PACKAGE}/{VERSION_CODE} (Linux; U; Android {os}; zh_CN; {}; Build/{build}; Cronet/TTNetVersion:3578d129 2024-11-05 QuicVersion:d9628e3d 2024-10-11)", id.model)
}

/// 每条 APP 主 API 都带的公共参数。
/// 时间戳对齐到 60s 桶：桶内同参数请求生成同一 URL，签名可复用。
pub const TICK_MS: u128 = 60_000;

pub fn tick_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / TICK_MS
        * TICK_MS
}

pub fn common(id: &Identity, aid: &str, first_launch: &str) -> BTreeMap<String, String> {
    let (os, os_api) = os_of(&id.brand);
    let now_ms = tick_ms().to_string();
    let mut d = BTreeMap::new();
    let resolution = id.resolution.replace('x', "*");
    let fixed = [
        ("device_platform", "android"),
        ("aid", aid),
        ("channel", "googlePlay"),
        ("app_name", APP_NAME),
        ("version_code", VERSION_CODE),
        ("version_name", VERSION_NAME),
        ("device_id", id.device_id.as_str()),
        ("iid", id.iid.as_str()),
        ("os", "android"),
        ("os_api", os_api),
        ("os_version", os),
        ("device_type", id.model.as_str()),
        ("device_brand", id.brand.as_str()),
        ("manifest_version_code", VERSION_CODE),
        ("update_version_code", "32229906"),
        ("resolution", resolution.as_str()),
        ("dpi", id.dpi.as_str()),
        ("language", "zh"),
        ("package", PACKAGE),
        ("cpu_support64", "true"),
        ("host_abi", "arm64-v8a"),
        ("ssmix", "a"),
        ("ac", "mobile"),
        ("is_guest_mode", "0"),
        ("app_type", "normal"),
        ("minor_status", "0"),
        ("appTheme", "light"),
        ("is_preinstall", "0"),
        ("need_personal_recommend", "1"),
        ("is_android_pad", "0"),
        ("is_android_fold", "0"),
        ("last_deeplink_update_version_code", "0"),
        ("ts", ""),
        ("_rticket", ""),
        ("first_launch_timestamp", first_launch),
    ];
    for (k, v) in fixed {
        d.insert(k.into(), v.into());
    }
    d.insert(
        "ts".into(),
        (now_ms.parse::<u128>().unwrap_or(0) / 1000).to_string(),
    );
    d.insert("_rticket".into(), now_ms);
    // 真实设备的 cdid 是固定的；由 device_id 派生，保证同身份生成的 URL 稳定、签名可复用。
    let cdid = if id.cdid.is_empty() {
        let h: String = format!("{:x}", md5::Md5::digest(id.device_id.as_bytes()))
            .chars()
            .take(32)
            .collect();
        format!(
            "{}-{}-{}-{}-{}",
            &h[0..8],
            &h[8..12],
            &h[12..16],
            &h[16..20],
            &h[20..32]
        )
    } else {
        id.cdid.clone()
    };
    d.insert("cdid".into(), cdid);
    d
}

/// 视频面（详情/作品/话题/合集）用高清 aid，播放器档位才全。
pub fn media(id: &Identity, first_launch: &str) -> BTreeMap<String, String> {
    common(id, "6383", first_launch)
}

/// `x-tt-dt` 等身份头。
pub fn device_token(id: &Identity) -> &str {
    &id.device_token
}
