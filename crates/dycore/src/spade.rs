//! 播放凭证 `player_access_key`（aK）→ CENC 内容密钥。
//!
//! aK 尾部字节异或后会解出一个算法版本标记：空标记走 v1，前缀为 `app_v2`/`web_v2`
//! 走 v2。v1 是纯字节变换（异或、popcount、位置相关），无 KEK、无 AES，已用 5 组
//! 运行时真值验证（见 `tests`）。v2 是另一套布局，要 `MD5(KEK)` 派生密钥后走
//! AES-GCM-256；缺 KEK 就无法解码，因此 v2 一律识别成 `V2` 错误而不是静默的空值。
//!
//! 现状：抖音短剧的 aK 全部是 v1，v2 只留判定和错误标记，没有实现解码。

fn popcount(v: usize) -> i32 {
    (v as u32).count_ones() as i32
}

fn starts_with_zero(a: &[u8], b: &[u8], n: usize) -> bool {
    for k in 0..n {
        let ca = *a.get(k).unwrap_or(&0);
        let cb = *b.get(k).unwrap_or(&0);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
    }
    true
}

/// aK 的算法版本；`V1` 可直接解，`V2` 需要 KEK 走 AES-GCM。
#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    V1,
    V2(&'static str),
}

/// 只判版本，不校验其余字段。
pub fn version(spade: &[u8]) -> Option<Version> {
    let len = spade.len();
    if len < 3 {
        return None;
    }
    let n = spade[0] ^ spade[1] ^ spade[2];
    let n = n as i32 - 0x30;
    if n < 1 || len < n as usize + 2 {
        return None;
    }
    let n = n as usize;
    let tail = len - n;
    let s1: Vec<u8> = (0..n)
        .map(|i| spade[tail - 2] ^ spade[tail - 1] ^ spade[i + tail])
        .collect();
    if starts_with_zero(&s1, b"app_v2", n) {
        return Some(Version::V2("app_v2"));
    }
    if starts_with_zero(&s1, b"web_v2", n) {
        return Some(Version::V2("web_v2"));
    }
    Some(Version::V1)
}

/// 解 v1 的 aK 原始字节（37B）；失败给出原因，v2 返回 `Err("app_v2")` 之类。
pub fn unwrap_v1_checked(spade: &[u8], flag: bool) -> Result<String, &'static str> {
    let len = spade.len();
    if len < 3 {
        return Err("aK 过短");
    }
    let b5 = spade[0] ^ spade[1] ^ spade[2];
    let type_len = b5 as i32 - 0x30;
    if type_len < 1 {
        return Err("aK 版本长度非法");
    }
    let work = (len as i32 - b5 as i32 + 0x2f) as usize;
    if work < 1 || 1 + work > len {
        return Err("aK 长度非法");
    }
    match version(spade) {
        None => return Err("aK 版本标记越界"),
        Some(Version::V2(tag)) => return Err(tag),
        Some(Version::V1) => {}
    }
    let mut dest = spade[1..1 + work].to_vec();

    let (mut b14, mut b16) = (0x55u8, 0xfau8);
    for (i, slot) in dest.iter_mut().enumerate() {
        let b6 = *slot;
        let mut b3 = b6;
        let mut b7 = b14;
        if i & 1 == 1 {
            b3 = b16;
            b7 = b6;
            b16 = b14;
        }
        let c4 = if flag {
            popcount(i) + 0x15
        } else {
            -0x15 - popcount(i)
        };
        *slot = (c4 + (b16 as i32 ^ b6 as i32)) as u8;
        b14 = b7;
        b16 = b3;
    }

    let b0 = dest[0];
    let nibble = if (0x30..=0x39).contains(&b0) {
        b0 - 0x30
    } else if (0x61..=0x7a).contains(&b0) {
        b0 - 0x57
    } else {
        return Err("aK 首字节不是 hex");
    } as usize;
    let end = work.checked_sub(nibble).ok_or("aK 长度前缀越界")?;
    if end < 2 {
        return Err("aK 长度前缀越界");
    }
    let key = String::from_utf8_lossy(&dest[1..end]).to_string();
    if key.len() != 32 || !key.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("解出的密钥不是 32 位 hex");
    }
    Ok(key.to_ascii_lowercase())
}

/// 同 `unwrap_v1_checked`，失败折叠成 `None`。
pub fn unwrap_v1(spade: &[u8], flag: bool) -> Option<String> {
    unwrap_v1_checked(spade, flag).ok()
}

/// base64 aK → 32 位 hex 密钥；v2 返回带版本名的错误。
pub fn decode(ak: &str) -> Result<String, String> {
    use base64::Engine;
    // 调试用：`SPADE_FORCE_V2=1` 时按 v2 失败，便于在没有 v2 样本时验证提示。
    if std::env::var("SPADE_FORCE_V2").is_ok_and(|v| v == "1") {
        return Err("app_v2 版本未实现".to_string());
    }
    let raw = base64::engine::general_purpose::STANDARD
        .decode(ak.trim())
        .map_err(|_| "aK 不是合法 base64".to_string())?;
    unwrap_v1_checked(&raw, false).map_err(|e| {
        if e.ends_with("_v2") {
            format!("{e} 版本未实现")
        } else {
            e.to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRUTH: &[(&str, &str)] = &[
        (
            "93bc1df253ba1bf7618b19c448b806f64d810afc45b715fe5eb925f26cbf12f541ba098282",
            "287216bfa89e662a0f748120c305e199",
        ),
        (
            "a1bc2ff164b91df04dbf00f17dba00f47cbb35c27b922aec67a628eb52972fdc7ea036a7a7",
            "d742b28967e4e6c92b699f4375add27f",
        ),
        (
            "9cbc12fb588721cd69b739f977b70be7429c08e074990ec847843dcc738638cf76b33ebcbc",
            "113fb30d9767d80e3edbb905b052204f",
        ),
        (
            "a3bc2df760ba2bc27b8b34c04a8907f557be19f249be35c748ba03f64fbc1ff161ba29a2a2",
            "b5674a8384f25d757585dadf343274d5",
        ),
        (
            "9cbc12f45aba11f76fb814f444bd15f459b225cb5e8c15dd459a0fd170ac0ad358aa108b8b",
            "121846f9d5829130ebc0397023feaf8c",
        ),
    ];

    #[test]
    fn matches_runtime_truth() {
        for (spade, want) in TRUTH {
            let raw = (0..spade.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&spade[i..i + 2], 16).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(unwrap_v1(&raw, false).as_deref(), Some(*want));
        }
    }

    #[test]
    fn decodes_base64() {
        assert_eq!(
            decode("nLwS8V+KD8VCgj7ScJ8h43GUO99qjzvycbg+90WMC8N3uz+1tQ==").as_deref(),
            Ok("176e9851e9669d7a34999e1377bc56bb")
        );
    }

    /// 尾部标签是 `app_v2` 时判成 v2，并给出可读原因而不是空值。
    #[test]
    fn flags_v2_without_decoding() {
        for tag in [b"app_v2", b"web_v2"] {
            let mut spade = vec![0u8; 31];
            spade[0] = 0x30 + 6;
            spade.extend_from_slice(tag);
            assert_eq!(
                version(&spade),
                Some(Version::V2(std::str::from_utf8(tag).unwrap()))
            );
            let err = decode(&{
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(&spade)
            })
            .unwrap_err();
            assert!(err.contains("未实现"), "{err}");
        }
    }
}
