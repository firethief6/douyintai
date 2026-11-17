//! `ttEncrypt` 信封体：跑 `libEncryptor.so` 的 JNI 方法，供 device_register 使用。
use std::path::Path;

use unidroid::{Emulator, Handle, JniCall, JniHandler, JniOp, JniValue};

const CLASS: &str = "com/bytedance/frameworks/encryptor/EncryptorUtil";
const METHOD: &str = "ttEncrypt([BI)[B";

struct Env;

impl JniHandler for Env {
    fn member(&mut self, emu: &mut Emulator, call: &JniCall<'_>) -> Option<JniValue> {
        match (call.op, call.class, call.name) {
            (JniOp::Call, "java/lang/Class", "getClassLoader") => {
                Some(emu.new_object("java/lang/ClassLoader", "loader").into())
            }
            (JniOp::Call, "java/lang/Class", "getName") => Some(emu.new_string(CLASS).into()),
            (JniOp::Call, "java/lang/Object", "getClass") => Some(
                emu.new_object("java/lang/Class", CLASS.replace('/', "."))
                    .into(),
            ),
            _ => None,
        }
    }
}

/// 对明文做 ttEncrypt，返回信封体。
pub fn tt_encrypt(res: &Path, plain: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut emu = Emulator::builder()
        .rootfs(res.join("rootfs"))
        .process_name("com.ss.android.ugc.aweme")
        .system_property("ro.product.model", "2201123C")
        .system_property("ro.product.brand", "Xiaomi")
        .system_property("ro.build.version.sdk", "33")
        .build()?;
    let lib = emu.load_library(res.join("libEncryptor.so"))?;
    let mut env = Env;
    emu.call_jni_onload(lib, &mut env)?;
    let input = emu.new_byte_array(plain);
    let out = emu.call_static_jni_method(
        &mut env,
        CLASS,
        METHOD,
        &[JniValue::Object(input), JniValue::Int(plain.len() as i32)],
    )?;
    let handle: Handle = match out {
        JniValue::Object(h) => h,
        other => return Err(format!("ttEncrypt 返回类型异常：{other:?}").into()),
    };
    let bytes = emu.byte_array(handle)?;
    emu.release(input);
    emu.release(handle);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn header_and_length_are_fixed() {
        let out = super::tt_encrypt(&super::super::default_resources(), b"hello").unwrap();
        assert_eq!(&out[..6], &[0x74, 0x63, 0x05, 0x10, 0x00, 0x00]);
        assert_eq!(out.len(), 102 + 16);
    }
}
