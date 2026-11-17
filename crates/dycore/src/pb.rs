//! 极简 protobuf 解码（无 .proto）：按 wire type 走树，长度不吻合就退回字节串。

#[derive(Debug, Clone)]
pub enum Value {
    Varint(u64),
    Fixed64([u8; 8]),
    Bytes(Vec<u8>),
    Message(Vec<(u32, Value)>),
    Fixed32([u8; 4]),
}

pub type Field = (u32, Value);

fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let mut v = 0u64;
    let mut s = 0;
    loop {
        let c = *b.get(i)?;
        i += 1;
        v |= ((c & 0x7F) as u64) << s;
        s += 7;
        if c & 0x80 == 0 {
            return Some((v, i));
        }
        if s > 63 {
            return None;
        }
    }
}

pub fn decode(b: &[u8]) -> Vec<Field> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let Some((key, next)) = varint(b, i) else {
            if std::env::var("DEBUG_PB").is_ok() {
                eprintln!("pb break key @{i}");
            }
            break;
        };
        i = next;
        let f = (key >> 3) as u32;
        let start = i - key_len(key);
        match key & 7 {
            0 => {
                let Some((v, next)) = varint(b, i) else {
                    if std::env::var("DEBUG_PB").is_ok() {
                        eprintln!("pb break varint @{i}");
                    }
                    break;
                };
                i = next;
                out.push((f, Value::Varint(v)));
                dbg_size(b, start, i, f, out.last().unwrap());
            }
            1 => {
                if i + 8 > b.len() {
                    break;
                }
                let mut a = [0u8; 8];
                a.copy_from_slice(&b[i..i + 8]);
                i += 8;
                out.push((f, Value::Fixed64(a)));
                dbg_size(b, start, i, f, out.last().unwrap());
            }
            2 => {
                let Some((len, next)) = varint(b, i) else {
                    if std::env::var("DEBUG_PB").is_ok() {
                        eprintln!("pb break len @{i}");
                    }
                    break;
                };
                i = next;
                let len = len as usize;
                if i + len > b.len() {
                    break;
                }
                let chunk = b[i..i + len].to_vec();
                i += len;
                let sub = if chunk.is_empty() {
                    Vec::new()
                } else {
                    decode(&chunk)
                };
                let fits = !sub.is_empty() && sub.iter().map(size).sum::<usize>() == len;
                out.push((
                    f,
                    if fits {
                        Value::Message(sub)
                    } else {
                        Value::Bytes(chunk)
                    },
                ));
                dbg_size(b, start, i, f, out.last().unwrap());
            }
            5 => {
                if i + 4 > b.len() {
                    break;
                }
                let mut a = [0u8; 4];
                a.copy_from_slice(&b[i..i + 4]);
                i += 4;
                out.push((f, Value::Fixed32(a)));
                dbg_size(b, start, i, f, out.last().unwrap());
            }
            3 => {
                let end = skip_group(b, i, f);
                if std::env::var("DEBUG_PB").is_ok() {
                    eprintln!("pb group f{f} @{i} -> {end}");
                }
                if end == 0 {
                    break;
                }
                out.push((f, Value::Bytes(b[i..end].to_vec())));
                i = end;
                dbg_size(b, start, i, f, out.last().unwrap());
            }
            w => {
                if std::env::var("DEBUG_PB").is_ok() {
                    eprintln!(
                        "pb break wire {w} @{i}/{} next={:02x?}",
                        b.len(),
                        &b[i..b.len().min(i + 12)]
                    );
                }
                break;
            }
        }
    }
    out
}

/// 跳过 group（wire type 3/4）：返回结束位置（指向对应 END_GROUP 之后）。
fn skip_group(b: &[u8], mut i: usize, field: u32) -> usize {
    let dbg = std::env::var("DEBUG_PB").is_ok();
    while i < b.len() {
        let Some((key, next)) = varint(b, i) else {
            return 0;
        };
        i = next;
        let (f, w) = ((key >> 3) as u32, key & 7);
        match w {
            0 => {
                let Some((_, next)) = varint(b, i) else {
                    return 0;
                };
                i = next;
            }
            1 => i += 8,
            2 => {
                let Some((len, next)) = varint(b, i) else {
                    return 0;
                };
                i = next + len as usize;
            }
            3 => {
                if dbg {
                    eprintln!("  nest group f{f} @{i}");
                }
                i = skip_group(b, i, f);
                if i == 0 {
                    return 0;
                }
            }
            4 => {
                if dbg {
                    eprintln!("  end group f{f} want {field}");
                }
                if f == field {
                    return i;
                }
            }
            5 => i += 4,
            _ => return 0,
        }
    }
    0
}

fn key_len(key: u64) -> usize {
    let mut k = 1;
    let mut v = key >> 7;
    while v != 0 {
        k += 1;
        v >>= 7;
    }
    k
}

fn dbg_size(b: &[u8], start: usize, end: usize, f: u32, v: &(u32, Value)) {
    if std::env::var("DEBUG_PB").is_ok() && b.len() > 500 {
        let want = size(v);
        if want != end - start {
            eprintln!(
                "size mismatch f{f} got={} want={want} start={start} end={end} len={}",
                end - start,
                b.len()
            );
        }
    }
}

fn vlen(mut v: u64) -> usize {
    let mut n = 1;
    v >>= 7;
    while v != 0 {
        n += 1;
        v >>= 7;
    }
    n
}

fn size(f: &Field) -> usize {
    let n = match &f.1 {
        Value::Varint(v) => vlen(*v),
        Value::Fixed64(_) => 8,
        Value::Fixed32(_) => 4,
        Value::Bytes(b) => b.len() + vlen(b.len() as u64),
        Value::Message(m) => {
            let n: usize = m.iter().map(size).sum();
            n + vlen(n as u64)
        }
    };
    n + vlen((f.0 as u64) << 3)
}

pub fn fields(msg: &[Field], no: u32) -> impl Iterator<Item = &Value> {
    msg.iter().filter(move |(f, _)| *f == no).map(|(_, v)| v)
}

pub fn one(msg: &[Field], no: u32) -> Option<&Value> {
    fields(msg, no).next()
}

pub fn as_bytes(v: &Value) -> Option<&[u8]> {
    match v {
        Value::Bytes(b) => Some(b),
        _ => None,
    }
}

pub fn as_u64(v: &Value) -> Option<u64> {
    match v {
        Value::Varint(n) => Some(*n),
        _ => None,
    }
}

fn wire(v: &Value) -> u64 {
    match v {
        Value::Varint(_) => 0,
        Value::Fixed64(_) => 1,
        Value::Bytes(_) | Value::Message(_) => 2,
        Value::Fixed32(_) => 5,
    }
}

fn put_varint(mut v: u64, out: &mut Vec<u8>) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

/// 还原字段的原始 wire 字节。length-delimited 的字符串若恰好能按 protobuf
/// 解析，decode 会把它收成 Message；按已知字符串字段取文本时必须还原，
/// 不能只取最后一个嵌套字节串。
pub fn raw(v: &Value) -> Vec<u8> {
    match v {
        Value::Varint(n) => {
            let mut out = Vec::new();
            put_varint(*n, &mut out);
            out
        }
        Value::Fixed64(a) => a.to_vec(),
        Value::Fixed32(a) => a.to_vec(),
        Value::Bytes(b) => b.clone(),
        Value::Message(m) => {
            let mut out = Vec::new();
            for (f, v) in m {
                put_varint(((*f as u64) << 3) | wire(v), &mut out);
                out.extend(raw(v));
            }
            out
        }
    }
}

pub fn text(v: &Value) -> String {
    match v {
        Value::Varint(n) => n.to_string(),
        _ => String::from_utf8_lossy(&raw(v)).into_owned(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decodes_wire_types_without_external_fixture() {
        let b = [
            0x08, 0x96, 0x01, // field 1: varint 150
            0x15, 1, 2, 3, 4, // field 2: fixed32
            0x2a, 0x05, 0x08, 0x01, 0x12, 0x01, b'x', // field 5: nested message
            0x31, 1, 2, 3, 4, 5, 6, 7, 8, // field 6: fixed64
            0x3a, 0x02, b'z', b'z', // field 7: bytes
        ];
        let top = super::decode(&b);
        assert_eq!(top.len(), 5);
        assert!(matches!(
            super::one(&top, 1),
            Some(super::Value::Varint(150))
        ));
        assert!(matches!(
            super::one(&top, 2),
            Some(super::Value::Fixed32(_))
        ));
        let Some(super::Value::Message(message)) = super::one(&top, 5) else {
            panic!("field 5 should be a message");
        };
        assert_eq!(message.len(), 2);
        assert!(matches!(
            super::one(&top, 6),
            Some(super::Value::Fixed64(_))
        ));
        assert_eq!(
            super::as_bytes(super::one(&top, 7).unwrap()),
            Some(&b"zz"[..])
        );
    }

    #[test]
    fn text_keeps_strings_that_look_like_messages() {
        let b = b"\x0a\x0b98108133007";
        let top = super::decode(b);
        let Some(super::Value::Message(text)) = super::one(&top, 1) else {
            panic!("numeric string should decode as a message");
        };
        assert_eq!(
            super::text(&super::Value::Message(text.to_vec())),
            "98108133007"
        );
    }

    #[test]
    fn incomplete_input_stops_at_boundary() {
        let top = super::decode(&[0x08, 0x80]);
        assert!(top.is_empty());
    }
}
