//! Helios 的 34 轮自定义块算法（RC5 风格，密钥是 md5 的 32 字节 hex 文本）。

const MASK: u64 = u64::MAX;

fn ror64(x: u64, n: u32) -> u64 {
    x.rotate_right(n) & MASK
}

fn expand(key: &[u8; 32]) -> [u64; 35] {
    let mut st = [0u64; 4];
    for (i, s) in st.iter_mut().enumerate() {
        let mut b = [0u8; 8];
        b.copy_from_slice(&key[i * 8..i * 8 + 8]);
        *s = u64::from_le_bytes(b);
    }
    let mut table = [0u64; 35];
    table[0] = st[0];
    for i in 0..34u64 {
        let b = ror64(st[1], 8).wrapping_add(st[0]) ^ i;
        let a = b ^ ror64(st[0], 61);
        st = [a, st[2], st[3], b];
        table[i as usize + 1] = a;
    }
    table
}

/// `data` 必须是 16 字节对齐的长度；输出与输入等长。
pub fn transform(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let table = expand(key);
    let mut out = Vec::with_capacity(data.len());
    for blk in data.as_chunks::<16>().0 {
        let mut a = u64::from_le_bytes(blk[0..8].try_into().unwrap());
        let mut b = u64::from_le_bytes(blk[8..16].try_into().unwrap());
        for t in table[..34].iter() {
            let x = ror64(b, 8).wrapping_add(a);
            b = t ^ x;
            a = b ^ ror64(a, 61);
        }
        out.extend_from_slice(&a.to_le_bytes());
        out.extend_from_slice(&b.to_le_bytes());
    }
    out
}
