//! MPEG-CENC（cenc-aes-ctr）就地解密：按 senc 的每样本 IV / subsample 表解 protected 区，
//! 再把 `encv`/`enca` 还原成 `frma` 声明的原始编码、把加密盒改名成 `free`。
//! 只在回环媒体代理里用，密钥不落盘。

use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};

type Ctr = ctr::Ctr128BE<Aes128>;

#[derive(Clone, Copy)]
struct BoxRef {
    at: usize,
    size: usize,
    head: usize,
}

fn boxed(buf: &[u8], start: usize, end: usize) -> Vec<BoxRef> {
    let mut out = Vec::new();
    let mut at = start;
    while at + 8 <= end {
        let size = u32::from_be_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]]) as usize;
        let head = match size {
            0 => end - at,
            1 if at + 16 <= end => {
                let big = u64::from_be_bytes(buf[at + 8..at + 16].try_into().unwrap()) as usize;
                if big < 16 {
                    break;
                }
                big
            }
            1 => break,
            n => n,
        };
        if size < 8 || at + head > end {
            break;
        }
        out.push(BoxRef {
            at,
            size: head,
            head,
        });
        at += head;
    }
    out
}

fn child(buf: &[u8], parent: &BoxRef, want: &[u8; 4]) -> Option<BoxRef> {
    let start = parent.at + if parent.head == 16 { 16 } else { 8 };
    boxed(buf, start, parent.at + parent.size)
        .into_iter()
        .find(|b| &buf[b.at + 4..b.at + 8] == want)
}

/// 在 `[start, end)` 里按类型签名找子盒。sample entry 头部有保留 0 字段，
/// 不能从 entry 体起始位置按盒子链遍历，只能逐个位置校验。
fn scan(buf: &[u8], start: usize, end: usize, want: &[u8; 4]) -> Option<BoxRef> {
    let end = end.min(buf.len());
    let mut at = start;
    while at + 8 <= end {
        if &buf[at + 4..at + 8] == want {
            let size = u32::from_be_bytes(buf[at..at + 4].try_into().unwrap()) as usize;
            if size >= 8 && at + size <= end {
                return Some(BoxRef { at, size, head: 8 });
            }
        }
        at += 1;
    }
    None
}

fn kids(buf: &[u8], parent: &BoxRef, want: &[u8; 4]) -> Vec<BoxRef> {
    let start = parent.at + if parent.head == 16 { 16 } else { 8 };
    boxed(buf, start, parent.at + parent.size)
        .into_iter()
        .filter(|b| &buf[b.at + 4..b.at + 8] == want)
        .collect()
}

fn u16_at(buf: &[u8], at: usize) -> Option<usize> {
    (at + 2 <= buf.len()).then(|| u16::from_be_bytes(buf[at..at + 2].try_into().unwrap()) as usize)
}

fn u32_at(buf: &[u8], at: usize) -> Option<usize> {
    (at + 4 <= buf.len()).then(|| u32::from_be_bytes(buf[at..at + 4].try_into().unwrap()) as usize)
}

fn rename(buf: &mut [u8], at: usize, to: &[u8; 4]) {
    if at + 8 <= buf.len() {
        buf[at + 4..at + 8].copy_from_slice(to);
    }
}

/// 解密所有带 `senc` 的轨道。`key` 为 16 字节内容密钥。
pub fn decrypt(buf: &mut [u8], key: &[u8; 16]) -> Result<usize, String> {
    let end = buf.len();
    let moov = boxed(buf, 0, end)
        .into_iter()
        .find(|b| &buf[b.at + 4..b.at + 8] == b"moov")
        .ok_or("缺少 moov")?;
    let mut samples = 0usize;
    let mut fixes: Vec<(usize, [u8; 4])> = Vec::new();
    for trak in kids(buf, &moov, b"trak") {
        let Some(mdia) = child(buf, &trak, b"mdia") else {
            continue;
        };
        let Some(minf) = child(buf, &mdia, b"minf") else {
            continue;
        };
        let Some(stbl) = child(buf, &minf, b"stbl") else {
            continue;
        };
        let Some(stsd) = child(buf, &stbl, b"stsd") else {
            continue;
        };
        let entries = boxed(buf, stsd.at + 16, stsd.at + stsd.size);
        let Some(entry) = entries.first() else {
            continue;
        };
        let mut kind = [0u8; 4];
        kind.copy_from_slice(&buf[entry.at + 4..entry.at + 8]);
        if &kind != b"encv" && &kind != b"enca" {
            continue;
        }
        let Some(senc) = child(buf, &stbl, b"senc") else {
            continue;
        };
        let Some(stsz) = child(buf, &stbl, b"stsz") else {
            continue;
        };
        let Some(stsc) = child(buf, &stbl, b"stsc") else {
            continue;
        };
        let Some(stco) = child(buf, &stbl, b"stco") else {
            continue;
        };

        let flags = u32_at(buf, senc.at + 8).ok_or("senc 头损坏")? & 0x00ff_ffff;
        let count = u32_at(buf, senc.at + 12).ok_or("senc 头损坏")?;
        let fixed = u32_at(buf, stsz.at + 12).ok_or("stsz 头损坏")?;
        let sample_count = u32_at(buf, stsz.at + 16).ok_or("stsz 头损坏")?;
        if sample_count != count {
            return Err(format!("senc/stsz 样本数不一致 {count}/{sample_count}"));
        }
        let sizes: Vec<usize> = if fixed == 0 {
            (0..count)
                .map(|i| u32_at(buf, stsz.at + 20 + i * 4).unwrap_or(0))
                .collect()
        } else {
            vec![fixed; count]
        };
        let chunks: Vec<usize> = (0..u32_at(buf, stco.at + 12).ok_or("stco 头损坏")?)
            .map(|i| u32_at(buf, stco.at + 16 + i * 4).unwrap_or(0))
            .collect();
        let map: Vec<(usize, usize)> = (0..u32_at(buf, stsc.at + 12).ok_or("stsc 头损坏")?)
            .map(|i| {
                (
                    u32_at(buf, stsc.at + 16 + i * 12).unwrap_or(0),
                    u32_at(buf, stsc.at + 20 + i * 12).unwrap_or(0),
                )
            })
            .collect();

        let mut iv_at = senc.at + 16;
        let mut sample = 0usize;
        let mut entry_i = 0usize;
        for (chunk_i, chunk) in chunks.iter().enumerate() {
            while entry_i + 1 < map.len() && map[entry_i + 1].0 <= chunk_i + 1 {
                entry_i += 1;
            }
            let per_chunk = map.get(entry_i).map(|m| m.1).unwrap_or(0);
            let mut at = *chunk;
            for _ in 0..per_chunk {
                if sample >= count {
                    break;
                }
                let len = sizes[sample];
                if iv_at + 8 > buf.len() {
                    return Err("senc IV 越界".into());
                }
                let mut iv = [0u8; 16];
                iv[..8].copy_from_slice(&buf[iv_at..iv_at + 8]);
                iv_at += 8;
                let mut ctr = Ctr::new(key.into(), (&iv).into());
                if flags & 2 == 0 {
                    if at + len > buf.len() {
                        return Err("样本越界".into());
                    }
                    ctr.apply_keystream(&mut buf[at..at + len]);
                    at += len;
                } else {
                    let subs = u16_at(buf, iv_at).ok_or("senc subsample 越界")?;
                    iv_at += 2;
                    for _ in 0..subs {
                        let clear = u16_at(buf, iv_at).ok_or("senc subsample 越界")?;
                        iv_at += 2;
                        let prot = u32_at(buf, iv_at).ok_or("senc subsample 越界")?;
                        iv_at += 4;
                        at += clear;
                        if at + prot > buf.len() {
                            return Err("subsample 越界".into());
                        }
                        ctr.apply_keystream(&mut buf[at..at + prot]);
                        at += prot;
                    }
                }
                sample += 1;
            }
        }
        if sample != count {
            return Err(format!("只解出 {sample}/{count} 个样本"));
        }
        samples += sample;

        // 解密后按 frma 还原编码；加密相关盒改名 free（大小不变，偏移不动）。
        let sinf = scan(buf, entry.at + 8, entry.at + entry.size, b"sinf");
        let format = sinf
            .and_then(|sinf| child(buf, &sinf, b"frma"))
            .and_then(|frma| {
                (frma.at + 12 <= buf.len()).then(|| {
                    let mut f = [0u8; 4];
                    f.copy_from_slice(&buf[frma.at + 8..frma.at + 12]);
                    f
                })
            })
            .unwrap_or(if &kind == b"encv" { *b"avc1" } else { *b"mp4a" });
        fixes.push((entry.at, format));
        if let Some(s) = sinf {
            fixes.push((s.at, *b"free"));
        }
        for name in [b"senc", b"saiz", b"saio"] {
            for b in kids(buf, &stbl, name) {
                fixes.push((b.at, *b"free"));
            }
        }
    }
    for (at, to) in fixes {
        rename(buf, at, &to);
    }
    Ok(samples)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bx(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut v = ((body.len() + 8) as u32).to_be_bytes().to_vec();
        v.extend_from_slice(kind);
        v.extend_from_slice(body);
        v
    }

    /// 真片回归：设 `CENC_SAMPLE` 指向加密样本与 `CENC_KEY` 为 hex key。
    #[test]
    #[ignore]
    fn decrypts_real_episode() {
        let Ok(path) = std::env::var("CENC_SAMPLE") else {
            return;
        };
        let key = std::env::var("CENC_KEY").unwrap();
        let mut buf = std::fs::read(&path).unwrap();
        let kb: [u8; 16] = (0..16)
            .map(|i| u8::from_str_radix(&key[i * 2..i * 2 + 2], 16).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let n = decrypt(&mut buf, &kb).unwrap();
        assert!(n > 0);
        let out = std::env::var("CENC_OUT").unwrap();
        std::fs::write(out, &buf).unwrap();
    }

    /// 造一个单样本、单 subsample 的加密片；`codec` 是 frma 原始编码，`conf` 是配置盒类型。
    fn fixture(
        codec: &[u8; 4],
        conf: &[u8; 4],
        key: &[u8; 16],
        iv: &[u8; 8],
        clear: &[u8],
        plain: &[u8],
    ) -> (Vec<u8>, usize, usize) {
        let mut enc = plain.to_vec();
        let mut b = [0u8; 16];
        b[..8].copy_from_slice(iv);
        Ctr::new(key.into(), (&b).into()).apply_keystream(&mut enc);

        let mut stbl = Vec::new();
        stbl.extend(bx(b"stsd", &{
            let mut v = vec![0, 0, 0, 0, 0, 0, 0, 1];
            let mut sinf = bx(b"frma", codec);
            sinf.extend(bx(
                b"schm",
                &[0, 0, 0, 0, b'c', b'e', b'n', b'c', 0, 1, 0, 0],
            ));
            // 加密样本条目的固定头是 78B（visual sample entry），其后才是子盒。
            let mut entry = vec![0u8; 78];
            entry.extend(bx(conf, &[0u8; 8]));
            entry.extend(bx(b"sinf", &sinf));
            v.extend(bx(b"encv", &entry));
            v
        }));
        stbl.extend(bx(b"stsz", &{
            let mut v = 0u32.to_be_bytes().to_vec();
            v.extend(0u32.to_be_bytes());
            v.extend(1u32.to_be_bytes());
            v.extend(((clear.len() + enc.len()) as u32).to_be_bytes());
            v
        }));
        stbl.extend(bx(b"stsc", &{
            let mut v = 0u32.to_be_bytes().to_vec();
            v.extend(1u32.to_be_bytes());
            v.extend(1u32.to_be_bytes());
            v.extend(1u32.to_be_bytes());
            v.extend(1u32.to_be_bytes());
            v
        }));
        stbl.extend(bx(b"stco", &{
            let mut v = 0u32.to_be_bytes().to_vec();
            v.extend(1u32.to_be_bytes());
            v.extend(0u32.to_be_bytes());
            v
        }));
        stbl.extend(bx(b"senc", &{
            let mut v = vec![0u8, 0, 0, 2];
            v.extend(1u32.to_be_bytes());
            v.extend(*iv);
            v.extend(1u16.to_be_bytes());
            v.extend((clear.len() as u16).to_be_bytes());
            v.extend((enc.len() as u32).to_be_bytes());
            v
        }));
        let trak = bx(b"trak", &bx(b"mdia", &bx(b"minf", &bx(b"stbl", &stbl))));
        let mut file = bx(b"moov", &trak);
        let mdat_at = file.len() + 8;
        let stco_at = file.windows(4).rposition(|w| w == b"stco").unwrap();
        file[stco_at + 12..stco_at + 16].copy_from_slice(&(mdat_at as u32).to_be_bytes());
        file.extend(bx(b"mdat", &[clear.to_vec(), enc].concat()));
        let entry_at = file.windows(4).position(|w| w == b"encv").unwrap();
        (file, mdat_at, entry_at)
    }

    fn assert_decrypted(codec: &[u8; 4], conf: &[u8; 4]) {
        let key = [7u8; 16];
        let iv = [3u8; 8];
        let clear = b"\0\0\0\x05\x65";
        let plain = vec![0x41u8; 32];
        let (mut file, mdat_at, entry_at) = fixture(codec, conf, &key, &iv, clear, &plain);

        let n = decrypt(&mut file, &key).unwrap();
        assert_eq!(n, 1);
        let data = &file[mdat_at..];
        assert_eq!(&data[..5], clear);
        assert_eq!(&data[5..], &plain[..]);
        // 样本编码按 frma 还原，加密盒全部改名 free。
        assert_eq!(&file[entry_at..entry_at + 4], codec);
        assert!(file.windows(4).any(|w| w == conf));
        assert!(!file.windows(4).any(|w| w == b"encv"));
        assert!(!file.windows(4).any(|w| w == b"sinf"));
        assert!(file.windows(4).any(|w| w == b"free"));
    }

    #[test]
    fn decrypts_subsamples_and_restores_avc() {
        assert_decrypted(b"avc1", b"avcC");
    }

    /// 现在线上的密文都是 AVC；HEVC 密文暂无样本，这里锁住 hvc1 的还原路径。
    #[test]
    fn decrypts_subsamples_and_restores_hevc() {
        assert_decrypted(b"hvc1", b"hvcC");
    }
}
