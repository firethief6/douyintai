//! 纯 Rust 复现 32.2.6 的 `X-Helios`（不依赖 `unidroid`/`dysign`）。
//!
//! 只保证 `libmetasec_ml.so` 32.2.6 的行为；`X-Medusa` 与其余信封头在 M1
//! 阶段用固定常量占位，形态与台子一致。

#![allow(static_mut_refs)]

use base64::Engine;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};

pub mod block;
pub mod medusa;
pub mod rng;

/// 身份池里的一行（13 个字段）。M1 只用到与 Helios 无关的字段，保留是为了
/// 后续 Medusa 段和环境段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Device {
    pub device_id: String,
    pub iid: String,
    pub device_token: String,
    pub brand: String,
    pub model: String,
    pub resolution: String,
    pub dpi: String,
    pub android_id: String,
    pub wifi_mac: String,
    pub bt_mac: String,
    pub used: u32,
    pub search_state: String,
    pub status: String,
}

/// `frameSign` 的 lid 与 `ms.bd.c.m` 的取值，来自冷启快照，32.2.6 内恒定。
const SIGN_LID: &str = "1378330313";
const SIGN_M: &str = "664226";

// 以下四枚仍是固定夹具的实拍常量，未复刻推导：
// - X-Argus 由环境 envelope（0x5009fa00 同族 protobuf）派生，改 query 文本即变；
//   该缓冲对 Medusa 已实测为死源（见 docs/research/medusa-segments §4）；
// - X-Gorgon 由 URL query 文本派生，末尾字节随输入变；
// - X-Ladon 由冷启常量与请求文本派生；
// - X-Soter 是固定结构。
// 服务端不校验这四枚，接受性由 `scripts/pure-accept.sh` 兜底。
const X_ARGUS: &str = "VAxrfPtgFSzAPPPdmEYBhwOiBsrrMrDutiOHVFRZoEK+eEJPbrnJwPe41hjDtTI6VrsHNN5lV9FDhX8ezm5D++xZQ6TfInqHDptAFmZG7b486eQAm+3s6nltsMl9v8Gq1R6+P3ldpCdzYDRWM7zfpCBbfwCHxvxkUpqElgA90R7uhn6iWCBcszRZe8c5ZsIU0KS4HkpeHZwRixoSVIFCkZ3IjKS5akjo4exn/R/JPNVYG7TSkeDygVhvKs5Pppspzq4=";
const X_GORGON: &str = "8404c0f60000edd618f0b63f1f9148ab831eedc910552395c050";
const X_LADON: &str = "LfehVhTx6WflX1n8wEin5039pfmAv/XuhtJAgzlkovKUm86q";
const X_MEDUSA: &str = "ZJAKaXL/ihyGFbhf2HuOo0JbSYEHJwAB1iJbaNxbEdQuGmkPK727mJOKWAi3nWI8S968Kos1cvuf6gGiTXB0D02Tx7vBg2Ot4AWXvDICmMf49Kaq27LMDNc2J9w/fI7HRjPgkS/SU97YONySJ5aHNPjQqrbS349J8N4jVTxXlDcqGq5uyfN/1xnGCzXoPUUWkTmU5cXmAQWjRuLSxlGIURtIB2V8nDTwOEjzYiVHMLwjLxjLutgVI+/mS4pf/8CONDOQLU8kK7NJFLW1lwZ9+KZkCP5D5kOGu17gcQORas3RlxWbN3tdRhUFNBZcXsl480APdjc3/ESOHvks6HiBSGN5R/rGNK1s2RB8A7inD0m7vtfT6v71uUq+WWQI7oAJ6i93DlnW6mpHO5pFD7DXdxBB9D4r3Mewje/O5FvTknPiCH3Jddmxgp8f96A0+fUa9kYE0zRBB3l19vZkJ2JfXtkuhuYYvcYkp+izQZtptzhHPn5wFwccEx41aQQWhyZamS4mqsIFkb34234iw99WJTScp+vWedZ/WydSqHCtmuGg1N1GmVFwSyLtIKZB9gvNquS5Rb7qdEZv86kXtzMTvupfXgEMqMsRAwp45gGOGk6Pz8Da2S7EIJV7cK5a1OBdhz14h6WxCeL92X7GtGKFNn7k3i6PqUJ6Oc0o/dA7enCO/4FmyhHvHlPtv4tk7VCH6NgAbYqBPu8L3LZWttNVjwrrihAGe716/WU2ItNffkjwA7cg1NHRjvYhLb1vmrCYv4wyC3LOKDiiIhnyVrW5B1oivNE61hDR9TMW1WkcVdzQa8wrawb0iygDZorchJgGdWxfOULXckovDn+s8Z1bcecpN1b6PnnM9lVyJ63SGhptNEoi8iO4Gagite4dH1Bw+P4A5g2uT6FtlVRKiccBWHT9tMMHwv//B8L//x8i";
const X_SOTER: &str = "AAEAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
/// 无 `aid` 时台子只回这一枚头；取值来自 32.2.6 冷启环境的固定项。
const X_NEPTUNE: &str = "-8|50:51:59:20:21:30:40:47:49:39:22:29";

#[derive(Debug)]
pub enum Error {
    NoAid,
    Medusa(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoAid => write!(f, "URL 里没有 aid 参数"),
            Error::Medusa(e) => write!(f, "Medusa 复算失败：{e}"),
        }
    }
}

impl std::error::Error for Error {}

fn is_hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

fn hex_byte(pair: &[u8]) -> Option<u8> {
    fn nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }
    Some(nibble(*pair.first()?)? << 4 | nibble(*pair.get(1)?)?)
}

/// 只按 CRLF 成对解析头块；非法十六进制停在最后一个完整字节对。
pub fn stub_of(extra: &str) -> medusa::Stub {
    let mut lines = extra.split("\r\n");
    while let Some(name) = lines.next() {
        let Some(value) = lines.next() else {
            break;
        };
        if !name.eq_ignore_ascii_case("x-ss-stub") {
            continue;
        }
        if value.is_empty() {
            return medusa::Stub::Empty;
        }
        let bytes = value.as_bytes();
        let mut stub = Vec::with_capacity(16);
        for pair in bytes.as_chunks::<2>().0.iter().take(16) {
            if !is_hex(pair[0]) || !is_hex(pair[1]) {
                break;
            }
            stub.push(hex_byte(pair).unwrap());
        }
        return medusa::Stub::Hex(stub);
    }
    medusa::Stub::Missing
}

/// 两条互不相干的随机流：Helios 用 `rand()`，Medusa 的 urandom 用 splitmix。
struct Streams {
    helios: rng::Type3,
    urandom: medusa::Urng,
}

pub struct PureSigner {
    #[allow(dead_code)]
    device: Device,
    /// Medusa urandom 的种子；Helios 的 PRNG 由时钟秒值播种，不消费这个字段。
    seed: u64,
    clock: u64,
    streams: std::sync::Mutex<Streams>,
}

/// 取 query 里第一个 `aid=<十进制>`。
fn aid_of(url: &str) -> Option<&str> {
    let bytes = url.as_bytes();
    let mut i = 0;
    while i + 4 <= bytes.len() {
        if (bytes[i] == b'?' || bytes[i] == b'&')
            && &bytes[i + 1..i + 4] == b"aid"
            && bytes.get(i + 4) == Some(&b'=')
        {
            let start = i + 5;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start {
                return Some(&url[start..end]);
            }
            return None;
        }
        i += 1;
    }
    None
}

fn hex32(d: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, b) in d.iter().enumerate() {
        out[i * 2] = b"0123456789abcdef"[(b >> 4) as usize];
        out[i * 2 + 1] = b"0123456789abcdef"[(b & 0xf) as usize];
    }
    out
}

impl PureSigner {
    pub fn new(device: &Device, clock: u64, seed: u64) -> Self {
        PureSigner {
            device: device.clone(),
            seed,
            clock,
            streams: std::sync::Mutex::new(Streams {
                helios: rng::Type3::new(clock as u32),
                urandom: medusa::Urng::new(seed),
            }),
        }
    }

    pub fn set_device(&mut self, device: Device) {
        self.device = device;
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// 16 字节对齐的 0x0N 填充。
    fn padded_plain(t: u32) -> Vec<u8> {
        let mut plain = format!("{t}-{SIGN_LID}-{SIGN_M}").into_bytes();
        let pad = ((plain.len() + 16) & !15) - plain.len();
        plain.extend_from_slice(&vec![pad as u8; pad]);
        plain
    }

    /// 返回 CRLF 分隔的 7 枚头。Medusa 复算失败时退回夹具常量，签名不中断。
    pub fn sign(&self, url: &str, extra: &str) -> Result<String, Error> {
        self.sign_inner(url, extra, false)
    }

    /// 同 `sign`，但 Medusa 复算失败直接报错，供上层换实现重签。
    pub fn sign_strict(&self, url: &str, extra: &str) -> Result<String, Error> {
        self.sign_inner(url, extra, true)
    }

    fn sign_inner(&self, url: &str, extra: &str, strict: bool) -> Result<String, Error> {
        // 台子（真机 Java 封装层同口径）在缺 aid 或头块不是 CRLF 分隔时只回 X-Neptune。
        let Some(aid) = aid_of(url).filter(|_| extra.contains("\r\n")) else {
            return Ok(format!("X-Neptune\r\n{X_NEPTUNE}\r\n"));
        };
        let t = self.clock as u32;
        let mut st = self.streams.lock().unwrap_or_else(|e| e.into_inner());
        let mut u32_v = 0u32;
        let mut prefix = 0u32;
        for i in 1..=9 {
            let v = st.helios.next_u31();
            if i == 4 {
                u32_v = v;
            }
            if i == 8 {
                prefix = v;
            }
        }

        let mut k1_input = u32_v.to_le_bytes().to_vec();
        k1_input.extend_from_slice(aid.as_bytes());
        let k1 = hex32(&Md5::digest(&k1_input));
        let mut k2_input = prefix.to_le_bytes().to_vec();
        k2_input.extend_from_slice(aid.as_bytes());
        let k2 = hex32(&Md5::digest(&k2_input));

        let plain = Self::padded_plain(t);
        let _h1 = block::transform(&plain, &k1);
        let h2 = block::transform(&plain, &k2);

        let mut out = prefix.to_le_bytes().to_vec();
        out.extend_from_slice(&h2);
        let helios = base64::engine::general_purpose::STANDARD.encode(out);

        // Medusa 只吃 `?` 之后的 query。
        let medusa = match url.split_once('?') {
            Some((_, q)) => {
                match medusa::Vm::sign_query_with_stub(
                    q,
                    &stub_of(extra),
                    self.clock,
                    &mut st.urandom,
                ) {
                    Ok(v) => v,
                    Err(e) if strict => return Err(Error::Medusa(format!("{e:?}"))),
                    Err(_) => X_MEDUSA.to_string(),
                }
            }
            None => X_MEDUSA.to_string(),
        };

        let mut s = String::new();
        for (name, value) in [
            ("X-Argus", X_ARGUS.to_string()),
            ("X-Gorgon", X_GORGON.to_string()),
            ("X-Helios", helios),
            ("X-Khronos", String::new()),
            ("X-Ladon", X_LADON.to_string()),
            ("X-Medusa", medusa),
            ("X-Soter", X_SOTER.to_string()),
        ] {
            s.push_str(name);
            s.push_str("\r\n");
            if name == "X-Khronos" {
                s.push_str(&self.clock.to_string());
            } else {
                s.push_str(&value);
            }
            s.push_str("\r\n");
        }
        Ok(s)
    }
}
