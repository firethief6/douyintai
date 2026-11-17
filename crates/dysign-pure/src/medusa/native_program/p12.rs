#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102a2f94 => { vm.decode(0x22844befu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f98u64) }
        0x102a2f98 => { vm.decode(0xa29c23b5u32); let d = 17i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f9cu64) }
        0x102a2f9c => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2fa0u64) }
        0x102a2fa0 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2fa4u64) }
        0x102a2fa4 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2fa8u64) }
        0x102a2fa8 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2facu64) }
        0x102a2fac => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2fb0u64) }
        0x102a2fb0 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2fb4u64) }
        0x102a2fb4 => { vm.decode(0xe39c63b5u32); let d = 17i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2fb8u64) }
        0x102a2fb8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2fbcu64) }
        0x102a2fbc => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2fc0u64) }
        0x102a2fc0 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2fc4u64) }
        0x102a2fc4 => { vm.decode(0x2880c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2fc8u64) }
        0x102a2fc8 => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2fccu64) }
        0x102a2fcc => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2fd0u64) }
        0x102a2fd0 => { vm.decode(0x6a09875u32); let d = 13i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2fd4u64) }
        0x102a2fd4 => { vm.decode(0xa39c23b5u32); let d = 17i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2fd8u64) }
        0x102a2fd8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2fdcu64) }
        0x102a2fdc => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2fe0u64) }
        0x102a2fe0 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2fe4u64) }
        0x102a2fe4 => { vm.decode(0x40113835u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2fe8u64) }
        0x102a2fe8 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2fecu64) }
        0x102a2fec => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ff0u64) }
        0x102a2ff0 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2ff4u64) }
        0x102a2ff4 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2ff8u64) }
        0x102a2ff8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ffcu64) }
        0x102a2ffc => { vm.decode(0x2a8ce32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3000u64) }
        0x102a3000 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3004u64) }
        0x102a3004 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3008u64) }
        0x102a3008 => { vm.decode(0x428663afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a300cu64) }
        0x102a300c => { vm.decode(0x429c43b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3010u64) }
        0x102a3010 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3014u64) }
        0x102a3014 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3018u64) }
        0x102a3018 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a301cu64) }
        0x102a301c => { vm.decode(0x64112835u32); let d = 17i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3020u64) }
        0x102a3020 => { vm.decode(0x839c43b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3024u64) }
        0x102a3024 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3028u64) }
        0x102a3028 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a302cu64) }
        0x102a302c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3030u64) }
        0x102a3030 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3034u64) }
        0x102a3034 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3038u64) }
        0x102a3038 => { vm.decode(0x806befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a303cu64) }
        0x102a303c => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3040u64) }
        0x102a3040 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3044u64) }
        0x102a3044 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3048u64) }
        0x102a3048 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a304cu64) }
        0x102a304c => { vm.decode(0x2a8ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3050u64) }
        0x102a3050 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3054u64) }
        0x102a3054 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3058u64) }
        0x102a3058 => { vm.decode(0x40e32bb5u32); let d = 3i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a305cu64) }
        0x102a305c => { vm.decode(0xdd4349u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3060u64) }
        0x102a3060 => { vm.decode(0x4e809875u32); let d = 31i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3064u64) }
        0x102a3064 => { vm.decode(0x30d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a3068u64) }
        0x102a3068 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a306cu64) }
        0x102a306c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3070u64) }
        0x102a3070 => { vm.decode(0x2288e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3074u64) }
        0x102a3074 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3078u64) }
        0x102a3078 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a307cu64) }
        0x102a307c => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3080u64) }
        0x102a3080 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3084u64) }
        0x102a3084 => { vm.decode(0x4080d875u32); let d = 7i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3088u64) }
        0x102a3088 => { vm.decode(0x39c43b5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a308cu64) }
        0x102a308c => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3090u64) }
        0x102a3090 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3094u64) }
        0x102a3094 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3098u64) }
        0x102a3098 => { vm.decode(0xa8ec36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a309cu64) }
        0x102a309c => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a30a0u64) }
        0x102a30a0 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a30a4u64) }
        0x102a30a4 => { vm.decode(0x439c03b5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a30a8u64) }
        0x102a30a8 => { vm.decode(0x42901875u32); let d = 14i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a30acu64) }
        0x102a30ac => { vm.decode(0x89281d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a30b0u64) }
        0x102a30b0 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a30b4u64) }
        0x102a30b4 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a30b8u64) }
        0x102a30b8 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a30bcu64) }
        0x102a30bc => { vm.decode(0x80e52bb5u32); let d = 22i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a30c0u64) }
        0x102a30c0 => { vm.decode(0x60522d35u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a30c4u64) }
        0x102a30c4 => { vm.decode(0x42755162u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a30c8u64) }
        0x102a30c8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a30ccu64) }
        0x102a30cc => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a30d0u64) }
        0x102a30d0 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a30d4u64) }
        0x102a30d4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a30d8u64) }
        0x102a30d8 => { vm.decode(0x2886c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a30dcu64) }
        0x102a30dc => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a30e0u64) }
        0x102a30e0 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a30e4u64) }
        0x102a30e4 => { vm.decode(0x30bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a30e8u64) }
        0x102a30e8 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a30ecu64) }
        0x102a30ec => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a30f0u64) }
        0x102a30f0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a30f4u64) }
        0x102a30f4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a30f8u64) }
        0x102a30f8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a30fcu64) }
        0x102a30fc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3100u64) }
        0x102a3100 => { vm.decode(0x65582835u32); let d = 23i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3104u64) }
        0x102a3104 => { vm.decode(0x2000402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3108u64) }
        0x102a3108 => { vm.decode(0x8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a310cu64) }
        0x102a310c => { vm.decode(0x44480835u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3110u64) }
        0x102a3110 => { vm.decode(0x80a10bb5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3114u64) }
        0x102a3114 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3118u64) }
        0x102a3118 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a311cu64) }
        0x102a311c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3120u64) }
        0x102a3120 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3124u64) }
        0x102a3124 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3128u64) }
        0x102a3128 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a312cu64) }
        0x102a312c => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3130u64) }
        0x102a3130 => { vm.decode(0x81c42bb5u32); let d = 30i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3134u64) }
        0x102a3134 => { vm.decode(0x60526d35u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3138u64) }
        0x102a3138 => { vm.decode(0x62777162u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a313cu64) }
        0x102a313c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3140u64) }
        0x102a3140 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3144u64) }
        0x102a3144 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3148u64) }
        0x102a3148 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a314cu64) }
        0x102a314c => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3150u64) }
        0x102a3150 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3154u64) }
        0x102a3154 => { vm.decode(0x40862bb5u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3158u64) }
        0x102a3158 => { vm.decode(0x40922d35u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a315cu64) }
        0x102a315c => { vm.decode(0x35d1022u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a3160u64) }
        0x102a3160 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3164u64) }
        0x102a3164 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3168u64) }
        0x102a3168 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a316cu64) }
        0x102a316c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3170u64) }
        0x102a3170 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3174u64) }
        0x102a3174 => { vm.decode(0x44482835u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3178u64) }
        0x102a3178 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a317cu64) }
        0x102a317c => { vm.decode(0x1820bb5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3180u64) }
        0x102a3180 => { vm.decode(0x40924d35u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3184u64) }
        0x102a3184 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3188u64) }
        0x102a3188 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a318cu64) }
        0x102a318c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3190u64) }
        0x102a3190 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3194u64) }
        0x102a3194 => { vm.decode(0x40032835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3198u64) }
        0x102a3198 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a319cu64) }
        0x102a319c => { vm.decode(0x81012bb5u32); let d = 24i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a31a0u64) }
        0x102a31a0 => { vm.decode(0x40524d35u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a31a4u64) }
        0x102a31a4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a31a8u64) }
        0x102a31a8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a31acu64) }
        0x102a31ac => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a31b0u64) }
        0x102a31b0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a31b4u64) }
        0x102a31b4 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a31b8u64) }
        0x102a31b8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a31bcu64) }
        0x102a31bc => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a31c0u64) }
        0x102a31c0 => { vm.decode(0x40220bb5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a31c4u64) }
        0x102a31c4 => { vm.decode(0x408a8d35u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a31c8u64) }
        0x102a31c8 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a31ccu64) }
        0x102a31cc => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a31d0u64) }
        0x102a31d0 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a31d4u64) }
        0x102a31d4 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a31d8u64) }
        0x102a31d8 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a31dcu64) }
        0x102a31dc => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a31e0u64) }
        0x102a31e0 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a31e4u64) }
        0x102a31e4 => { vm.decode(0x1002bb5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a31e8u64) }
        0x102a31e8 => { vm.decode(0x401b4d35u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a31ecu64) }
        0x102a31ec => { vm.decode(0x62371062u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a31f0u64) }
        0x102a31f0 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a31f4u64) }
        0x102a31f4 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a31f8u64) }
        0x102a31f8 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a31fcu64) }
        0x102a31fc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3200u64) }
        0x102a3200 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3204u64) }
        0x102a3204 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3208u64) }
        0x102a3208 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a320cu64) }
        0x102a320c => { vm.decode(0x2a8aeb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3210u64) }
        0x102a3210 => { vm.decode(0x80a72bb5u32); let d = 21i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3214u64) }
        0x102a3214 => { vm.decode(0x40db2d35u32); let d = 11i16; let s = 21i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3218u64) }
        0x102a3218 => { vm.decode(0x43453162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a321cu64) }
        0x102a321c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3220u64) }
        0x102a3220 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3224u64) }
        0x102a3224 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3228u64) }
        0x102a3228 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a322cu64) }
        0x102a322c => { vm.decode(0x73140835u32); let d = 12i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3230u64) }
        0x102a3230 => { vm.decode(0x60c36d35u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3234u64) }
        0x102a3234 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3238u64) }
        0x102a3238 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a323cu64) }
        0x102a323c => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3240u64) }
        0x102a3240 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3244u64) }
        0x102a3244 => { vm.decode(0xa8ec32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3248u64) }
        0x102a3248 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a324cu64) }
        0x102a324c => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3250u64) }
        0x102a3250 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3254u64) }
        0x102a3254 => { vm.decode(0x62157062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a3258u64) }
        0x102a3258 => { vm.decode(0x60030d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a325cu64) }
        0x102a325c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3260u64) }
        0x102a3260 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3264u64) }
        0x102a3264 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3268u64) }
        0x102a3268 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a326cu64) }
        0x102a326c => { vm.decode(0xa8ae32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3270u64) }
        0x102a3270 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3274u64) }
        0x102a3274 => { vm.decode(0x28c6befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3278u64) }
        0x102a3278 => { vm.decode(0x472bb5u32); let d = 7i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a327cu64) }
        0x102a327c => { vm.decode(0x609b2d35u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3280u64) }
        0x102a3280 => { vm.decode(0x3757062u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a3284u64) }
        0x102a3284 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3288u64) }
        0x102a3288 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a328cu64) }
        0x102a328c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3290u64) }
        0x102a3290 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3294u64) }
        0x102a3294 => { vm.decode(0xa8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3298u64) }
        0x102a3298 => { vm.decode(0x514d0835u32); let d = 6i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a329cu64) }
        0x102a329c => { vm.decode(0x918ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a32a0u64) }
        0x102a32a0 => { vm.decode(0x52d84349u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a32a4u64) }
        0x102a32a4 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a32a8u64) }
        0x102a32a8 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a32acu64) }
        0x102a32ac => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32b0u64) }
        0x102a32b0 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a32b4u64) }
        0x102a32b4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32b8u64) }
        0x102a32b8 => { vm.decode(0xa88cb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32bcu64) }
        0x102a32bc => { vm.decode(0x53462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a32c0u64) }
        0x102a32c0 => { vm.decode(0x80232bb5u32); let d = 17i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a32c4u64) }
        0x102a32c4 => { vm.decode(0x40db0d35u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a32c8u64) }
        0x102a32c8 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a32ccu64) }
        0x102a32cc => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a32d0u64) }
        0x102a32d0 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a32d4u64) }
        0x102a32d4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32d8u64) }
        0x102a32d8 => { vm.decode(0x888cb6fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32dcu64) }
        0x102a32dc => { vm.decode(0x524e2835u32); let d = 11i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a32e0u64) }
        0x102a32e0 => { vm.decode(0x88ceb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32e4u64) }
        0x102a32e4 => { vm.decode(0x60200a2fu32); let d = 25i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a32e8u64) }
        0x102a32e8 => { vm.decode(0x52581835u32); let d = 10i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a32ecu64) }
        0x102a32ec => { vm.decode(0x9209d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a32f0u64) }
        0x102a32f0 => { vm.decode(0x507c4349u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a32f4u64) }
        0x102a32f4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a32f8u64) }
        0x102a32f8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a32fcu64) }
        0x102a32fc => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3300u64) }
        0x102a3300 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3304u64) }
        0x102a3304 => { vm.decode(0x36552835u32); let d = 25i16; let s = 10i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3308u64) }
        0x102a3308 => { vm.decode(0x546f4349u32); let d = 10i16; let s = 30i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a330cu64) }
        0x102a330c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3310u64) }
        0x102a3310 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3314u64) }
        0x102a3314 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3318u64) }
        0x102a3318 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a331cu64) }
        0x102a331c => { vm.decode(0x2a82c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3320u64) }
        0x102a3320 => { vm.decode(0x330d0835u32); let d = 14i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3324u64) }
        0x102a3324 => { vm.decode(0x608a4befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3328u64) }
        0x102a3328 => { vm.decode(0x41e12bb5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a332cu64) }
        0x102a332c => { vm.decode(0x6413cd35u32); let d = 23i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a3330u64) }
        0x102a3330 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3334u64) }
        0x102a3334 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3338u64) }
        0x102a3338 => { vm.decode(0xc96624f5u32); let d = 23i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a333cu64) }
        0x102a333c => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3340u64) }
        0x102a3340 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3344u64) }
        0x102a3344 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3348u64) }
        0x102a3348 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a334cu64) }
        0x102a334c => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3350u64) }
        0x102a3350 => { vm.decode(0x31549835u32); let d = 4i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3354u64) }
        0x102a3354 => { vm.decode(0x910dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3358u64) }
        0x102a3358 => { vm.decode(0x44ce5349u32); let d = 19i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a335cu64) }
        0x102a335c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3360u64) }
        0x102a3360 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3364u64) }
        0x102a3364 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3368u64) }
        0x102a3368 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a336cu64) }
        0x102a336c => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3370u64) }
        0x102a3370 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3374u64) }
        0x102a3374 => { vm.decode(0x9285cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3378u64) }
        0x102a3378 => { vm.decode(0x44dc5349u32); let d = 19i16; let s = 17i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a337cu64) }
        0x102a337c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3380u64) }
        0x102a3380 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3384u64) }
        0x102a3384 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3388u64) }
        0x102a3388 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a338cu64) }
        0x102a338c => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3390u64) }
        0x102a3390 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3394u64) }
        0x102a3394 => { vm.decode(0x908dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3398u64) }
        0x102a3398 => { vm.decode(0x40de5349u32); let d = 19i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a339cu64) }
        0x102a339c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a33a0u64) }
        0x102a33a0 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a33a4u64) }
        0x102a33a4 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a33a8u64) }
        0x102a33a8 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a33acu64) }
        0x102a33ac => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a33b0u64) }
        0x102a33b0 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a33b4u64) }
        0x102a33b4 => { vm.decode(0x9205cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a33b8u64) }
        0x102a33b8 => { vm.decode(0x1510cb11u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a33bcu64) }
        0x102a33bc => { vm.decode(0x446c5349u32); let d = 18i16; let s = 20i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a33c0u64) }
        0x102a33c0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a33c4u64) }
        0x102a33c4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a33c8u64) }
        0x102a33c8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a33ccu64) }
        0x102a33cc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a33d0u64) }
        0x102a33d0 => { vm.decode(0x882e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a33d4u64) }
        0x102a33d4 => { vm.decode(0x1051835u32); let d = 4i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a33d8u64) }
        0x102a33d8 => { vm.decode(0x900dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a33dcu64) }
        0x102a33dc => { vm.decode(0x446d5349u32); let d = 18i16; let s = 22i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a33e0u64) }
        0x102a33e0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a33e4u64) }
        0x102a33e4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a33e8u64) }
        0x102a33e8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a33ecu64) }
        0x102a33ec => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a33f0u64) }
        0x102a33f0 => { vm.decode(0x1071835u32); let d = 4i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a33f4u64) }
        0x102a33f4 => { vm.decode(0x920dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a33f8u64) }
        0x102a33f8 => { vm.decode(0x107dc349u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a33fcu64) }
        0x102a33fc => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3400u64) }
        0x102a3400 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3404u64) }
        0x102a3404 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3408u64) }
        0x102a3408 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a340cu64) }
        0x102a340c => { vm.decode(0x282e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3410u64) }
        0x102a3410 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3414u64) }
        0x102a3414 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3418u64) }
        0x102a3418 => { vm.decode(0x147cc349u32); let d = 12i16; let s = 21i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a341cu64) }
        0x102a341c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3420u64) }
        0x102a3420 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3424u64) }
        0x102a3424 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3428u64) }
        0x102a3428 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a342cu64) }
        0x102a342c => { vm.decode(0x886c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3430u64) }
        0x102a3430 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3434u64) }
        0x102a3434 => { vm.decode(0x900dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3438u64) }
        0x102a3438 => { vm.decode(0x104ec349u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a343cu64) }
        0x102a343c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3440u64) }
        0x102a3440 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3444u64) }
        0x102a3444 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3448u64) }
        0x102a3448 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a344cu64) }
        0x102a344c => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3450u64) }
        0x102a3450 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3454u64) }
        0x102a3454 => { vm.decode(0xdc4349u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3458u64) }
        0x102a3458 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a345cu64) }
        0x102a345c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3460u64) }
        0x102a3460 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3464u64) }
        0x102a3464 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3468u64) }
        0x102a3468 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a346cu64) }
        0x102a346c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3470u64) }
        0x102a3470 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3474u64) }
        0x102a3474 => { vm.decode(0xcf4349u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3478u64) }
        0x102a3478 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a347cu64) }
        0x102a347c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3480u64) }
        0x102a3480 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3484u64) }
        0x102a3484 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3488u64) }
        0x102a3488 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a348cu64) }
        0x102a348c => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3490u64) }
        0x102a3490 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3494u64) }
        0x102a3494 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3498u64) }
        0x102a3498 => { vm.decode(0x2286c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a349cu64) }
        0x102a349c => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a34a0u64) }
        0x102a34a0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a34a4u64) }
        0x102a34a4 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a34a8u64) }
        0x102a34a8 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a34acu64) }
        0x102a34ac => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a34b0u64) }
        0x102a34b0 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a34b4u64) }
        0x102a34b4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a34b8u64) }
        0x102a34b8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a34bcu64) }
        0x102a34bc => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a34c0u64) }
        0x102a34c0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a34c4u64) }
        0x102a34c4 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a34c8u64) }
        0x102a34c8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a34ccu64) }
        0x102a34cc => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a34d0u64) }
        0x102a34d0 => { vm.decode(0x6ca4349u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a34d4u64) }
        0x102a34d4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a34d8u64) }
        0x102a34d8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a34dcu64) }
        0x102a34dc => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a34e0u64) }
        0x102a34e0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a34e4u64) }
        0x102a34e4 => { vm.decode(0x40432835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a34e8u64) }
        0x102a34e8 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a34ecu64) }
        0x102a34ec => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a34f0u64) }
        0x102a34f0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a34f4u64) }
        0x102a34f4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a34f8u64) }
        0x102a34f8 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a34fcu64) }
        0x102a34fc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3500u64) }
        0x102a3500 => { vm.decode(0x60122835u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3504u64) }
        0x102a3504 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3508u64) }
        0x102a3508 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a350cu64) }
        0x102a350c => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3510u64) }
        0x102a3510 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3514u64) }
        0x102a3514 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3518u64) }
        0x102a3518 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a351cu64) }
        0x102a351c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3520u64) }
        0x102a3520 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3524u64) }
        0x102a3524 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3528u64) }
        0x102a3528 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a352cu64) }
        0x102a352c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3530u64) }
        0x102a3530 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3534u64) }
        0x102a3534 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3538u64) }
        0x102a3538 => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a353cu64) }
        0x102a353c => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3540u64) }
        0x102a3540 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3544u64) }
        0x102a3544 => { vm.decode(0xdadac12du32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3548u64) }
        0x102a3548 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a354cu64) }
        0x102a354c => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3550u64) }
        0x102a3550 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3554u64) }
        0x102a3554 => { vm.decode(0xda5ec12du32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3558u64) }
        0x102a3558 => { vm.decode(0x228843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a355cu64) }
        0x102a355c => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3560u64) }
        0x102a3560 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3564u64) }
        0x102a3564 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3568u64) }
        0x102a3568 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a356cu64) }
        0x102a356c => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3570u64) }
        0x102a3570 => { vm.decode(0x2088e36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3574u64) }
        0x102a3574 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3578u64) }
        0x102a3578 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a357cu64) }
        0x102a357c => { vm.decode(0xe100dd75u32); let d = 22i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3580u64) }
        0x102a3580 => { vm.decode(0xb3089d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3584u64) }
        0x102a3584 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3588u64) }
        0x102a3588 => { vm.decode(0xe340ad75u32); let d = 21i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a358cu64) }
        0x102a358c => { vm.decode(0xe040dd75u32); let d = 23i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3590u64) }
        0x102a3590 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3594u64) }
        0x102a3594 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3598u64) }
        0x102a3598 => { vm.decode(0xa1487d75u32); let d = 11i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a359cu64) }
        0x102a359c => { vm.decode(0x208c43afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a35a0u64) }
        0x102a35a0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35a4u64) }
        0x102a35a4 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a35a8u64) }
        0x102a35a8 => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35acu64) }
        0x102a35ac => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a35b0u64) }
        0x102a35b0 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35b4u64) }
        0x102a35b4 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a35b8u64) }
        0x102a35b8 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35bcu64) }
        0x102a35bc => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a35c0u64) }
        0x102a35c0 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35c4u64) }
        0x102a35c4 => { vm.decode(0xb348dd75u32); let d = 15i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35c8u64) }
        0x102a35c8 => { vm.decode(0xa0405d75u32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35ccu64) }
        0x102a35cc => { vm.decode(0xe1486d75u32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35d0u64) }
        0x102a35d0 => { vm.decode(0xa100cd75u32); let d = 4i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35d4u64) }
        0x102a35d4 => { vm.decode(0xf1089d75u32); let d = 30i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35d8u64) }
        0x102a35d8 => { vm.decode(0xda6ed16du32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a35dcu64) }
        0x102a35dc => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a35e0u64) }
        0x102a35e0 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35e4u64) }
        0x102a35e4 => { vm.decode(0xb3054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a35e8u64) }
        0x102a35e8 => { vm.decode(0xe08434f5u32); let d = 24i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35ecu64) }
        0x102a35ec => { vm.decode(0xa08224f5u32); let d = 12i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35f0u64) }
        0x102a35f0 => { vm.decode(0x68c024f5u32); let d = 11i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35f4u64) }
        0x102a35f4 => { vm.decode(0xe16024f5u32); let d = 1i16; let s = 5i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35f8u64) }
        0x102a35f8 => { vm.decode(0x696634f5u32); let d = 23i16; let s = 5i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a35fcu64) }
        0x102a35fc => { vm.decode(0xa92024f5u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3600u64) }
        0x102a3600 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3604u64) }
        0x102a3604 => { vm.decode(0xe90024f5u32); let d = 2i16; let s = 5i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3608u64) }
        0x102a3608 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a360cu64) }
        0x102a360c => { vm.decode(0x682034f5u32); let d = 2i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3610u64) }
        0x102a3610 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3614u64) }
        0x102a3614 => { vm.decode(0x696424f5u32); let d = 19i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3618u64) }
        0x102a3618 => { vm.decode(0x690024f5u32); let d = 2i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a361cu64) }
        0x102a361c => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3620u64) }
        0x102a3620 => { vm.decode(0x282424f5u32); let d = 18i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3624u64) }
        0x102a3624 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3628u64) }
        0x102a3628 => { vm.decode(0xe88224f5u32); let d = 14i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a362cu64) }
        0x102a362c => { vm.decode(0xe82034f5u32); let d = 2i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3630u64) }
        0x102a3630 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3634u64) }
        0x102a3634 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3638u64) }
        0x102a3638 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a363cu64) }
        0x102a363c => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3640u64) }
        0x102a3640 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3644u64) }
        0x102a3644 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3648u64) }
        0x102a3648 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a364cu64) }
        0x102a364c => { vm.decode(0x474c2835u32); let d = 31i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3650u64) }
        0x102a3650 => { vm.decode(0x46d84349u32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3654u64) }
        0x102a3654 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3658u64) }
        0x102a3658 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a365cu64) }
        0x102a365c => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3660u64) }
        0x102a3660 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3664u64) }
        0x102a3664 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3668u64) }
        0x102a3668 => { vm.decode(0x43442835u32); let d = 13i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a366cu64) }
        0x102a366c => { vm.decode(0x46e84349u32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3670u64) }
        0x102a3670 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3674u64) }
        0x102a3674 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3678u64) }
        0x102a3678 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a367cu64) }
        0x102a367c => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3680u64) }
        0x102a3680 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3684u64) }
        0x102a3684 => { vm.decode(0x404c2835u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3688u64) }
        0x102a3688 => { vm.decode(0x24bc349u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a368cu64) }
        0x102a368c => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3690u64) }
        0x102a3690 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3694u64) }
        0x102a3694 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3698u64) }
        0x102a3698 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a369cu64) }
        0x102a369c => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36a0u64) }
        0x102a36a0 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a36a4u64) }
        0x102a36a4 => { vm.decode(0x45104211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a36a8u64) }
        0x102a36a8 => { vm.decode(0x52494349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a36acu64) }
        0x102a36ac => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a36b0u64) }
        0x102a36b0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a36b4u64) }
        0x102a36b4 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a36b8u64) }
        0x102a36b8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36bcu64) }
        0x102a36bc => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36c0u64) }
        0x102a36c0 => { vm.decode(0x124c0835u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a36c4u64) }
        0x102a36c4 => { vm.decode(0x52cac349u32); let d = 15i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a36c8u64) }
        0x102a36c8 => { vm.decode(0xfa2fc909u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a36ccu64) }
        0x102a36cc => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a36d0u64) }
        0x102a36d0 => { vm.decode(0xcae234f5u32); let d = 15i16; let s = 6i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a36d4u64) }
        0x102a36d4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36d8u64) }
        0x102a36d8 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36dcu64) }
        0x102a36dc => { vm.decode(0x534ca835u32); let d = 15i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a36e0u64) }
        0x102a36e0 => { vm.decode(0x45906811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a36e4u64) }
        0x102a36e4 => { vm.decode(0x2c95349u32); let d = 17i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a36e8u64) }
        0x102a36e8 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a36ecu64) }
        0x102a36ec => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a36f0u64) }
        0x102a36f0 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a36f4u64) }
        0x102a36f4 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36f8u64) }
        0x102a36f8 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a36fcu64) }
        0x102a36fc => { vm.decode(0x44403835u32); let d = 17i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3700u64) }
        0x102a3700 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a3704u64) }
        0x102a3704 => { vm.decode(0x249d349u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3708u64) }
        0x102a3708 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a370cu64) }
        0x102a370c => { vm.decode(0xf000cd75u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3710u64) }
        0x102a3710 => { vm.decode(0x32624f5u32); let d = 20i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3714u64) }
        0x102a3714 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3718u64) }
        0x102a3718 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a371cu64) }
        0x102a371c => { vm.decode(0x5409835u32); let d = 20i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3720u64) }
        0x102a3720 => { vm.decode(0x4598e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a3724u64) }
        0x102a3724 => { vm.decode(0x5249d349u32); let d = 30i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3728u64) }
        0x102a3728 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a372cu64) }
        0x102a372c => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3730u64) }
        0x102a3730 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3734u64) }
        0x102a3734 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3738u64) }
        0x102a3738 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a373cu64) }
        0x102a373c => { vm.decode(0x1244b835u32); let d = 9i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3740u64) }
        0x102a3740 => { vm.decode(0x2f9d349u32); let d = 21i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3744u64) }
        0x102a3744 => { vm.decode(0xc967c909u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3748u64) }
        0x102a3748 => { vm.decode(0xf040ed75u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a374cu64) }
        0x102a374c => { vm.decode(0x836624f5u32); let d = 21i16; let s = 6i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3750u64) }
        0x102a3750 => { vm.decode(0x6082412fu32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3754u64) }
        0x102a3754 => { vm.decode(0xa88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3758u64) }
        0x102a3758 => { vm.decode(0x4540b835u32); let d = 21i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a375cu64) }
        0x102a375c => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a3760u64) }
        0x102a3760 => { vm.decode(0x4249d349u32); let d = 22i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3764u64) }
        0x102a3764 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3768u64) }
        0x102a3768 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a376cu64) }
        0x102a376c => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3770u64) }
        0x102a3770 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3774u64) }
        0x102a3774 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3778u64) }
        0x102a3778 => { vm.decode(0x54c9835u32); let d = 22i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a377cu64) }
        0x102a377c => { vm.decode(0x4510c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a3780u64) }
        0x102a3780 => { vm.decode(0x2495349u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3784u64) }
        0x102a3784 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3788u64) }
        0x102a3788 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a378cu64) }
        0x102a378c => { vm.decode(0x30424f5u32); let d = 16i16; let s = 6i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3790u64) }
        0x102a3790 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3794u64) }
        0x102a3794 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3798u64) }
        0x102a3798 => { vm.decode(0x4401835u32); let d = 16i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a379cu64) }
        0x102a379c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37a0u64) }
        0x102a37a0 => { vm.decode(0x60432835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37a4u64) }
        0x102a37a4 => { vm.decode(0x208663afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37a8u64) }
        0x102a37a8 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37acu64) }
        0x102a37ac => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37b0u64) }
        0x102a37b0 => { vm.decode(0x514ca835u32); let d = 7i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37b4u64) }
        0x102a37b4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37b8u64) }
        0x102a37b8 => { vm.decode(0x574e0835u32); let d = 30i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37bcu64) }
        0x102a37bc => { vm.decode(0x880a2fu32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37c0u64) }
        0x102a37c0 => { vm.decode(0x11050835u32); let d = 4i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37c4u64) }
        0x102a37c4 => { vm.decode(0x84a2fu32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37c8u64) }
        0x102a37c8 => { vm.decode(0x32440835u32); let d = 8i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37ccu64) }
        0x102a37cc => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37d0u64) }
        0x102a37d0 => { vm.decode(0x2028002fu32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37d4u64) }
        0x102a37d4 => { vm.decode(0x600f2835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37d8u64) }
        0x102a37d8 => { vm.decode(0xa8492fu32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37dcu64) }
        0x102a37dc => { vm.decode(0x355f2835u32); let d = 23i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a37e0u64) }
        0x102a37e0 => { vm.decode(0x42058019u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a37e4u64) }
        0x102a37e4 => { vm.decode(0x1b0854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a37e8u64) }
        0x102a37e8 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a37ecu64) }
        0x102a37ec => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a37f0u64) }
        0x102a37f0 => { vm.decode(0x621d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a37f4u64) }
        0x102a37f4 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37f8u64) }
        0x102a37f8 => { vm.decode(0xa80a2fu32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a37fcu64) }
        0x102a37fc => { vm.decode(0x208643efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3800u64) }
        0x102a3800 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3804u64) }
        0x102a3804 => { vm.decode(0x24b2fu32); let d = 12i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3808u64) }
        0x102a3808 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a380cu64) }
        0x102a380c => { vm.decode(0x2082092fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3810u64) }
        0x102a3810 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3814u64) }
        0x102a3814 => { vm.decode(0x288263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3818u64) }
        0x102a3818 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a381cu64) }
        0x102a381c => { vm.decode(0x228663efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3820u64) }
        0x102a3820 => { vm.decode(0x4080092fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3824u64) }
        0x102a3824 => { vm.decode(0x2882e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3828u64) }
        0x102a3828 => { vm.decode(0x6080092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a382cu64) }
        0x102a382c => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3830u64) }
        0x102a3830 => { vm.decode(0x36011835u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3834u64) }
        0x102a3834 => { vm.decode(0x2a82e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3838u64) }
        0x102a3838 => { vm.decode(0x740d8835u32); let d = 18i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a383cu64) }
        0x102a383c => { vm.decode(0x131c8835u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3840u64) }
        0x102a3840 => { vm.decode(0x5459a835u32); let d = 19i16; let s = 13i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3844u64) }
        0x102a3844 => { vm.decode(0x33118835u32); let d = 12i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3848u64) }
        0x102a3848 => { vm.decode(0x74060835u32); let d = 16i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a384cu64) }
        0x102a384c => { vm.decode(0x121d0835u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3850u64) }
        0x102a3850 => { vm.decode(0x22142835u32); let d = 9i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3854u64) }
        0x102a3854 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3858u64) }
        0x102a3858 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a385cu64) }
        0x102a385c => { vm.decode(0xd9ce111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a3860u64) }
        0x102a3860 => { vm.decode(0x52f84349u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a3864u64) }
        0x102a3864 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3868u64) }
        0x102a3868 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a386cu64) }
        0x102a386c => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3870u64) }
        0x102a3870 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3874u64) }
        0x102a3874 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3878u64) }
        0x102a3878 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a387cu64) }
        0x102a387c => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3880u64) }
        0x102a3880 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3884u64) }
        0x102a3884 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3888u64) }
        0x102a3888 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a388cu64) }
        0x102a388c => { vm.decode(0x4a1835u32); let d = 2i16; let s = 10i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3890u64) }
        0x102a3890 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3894u64) }
        0x102a3894 => { vm.decode(0x400f1835u32); let d = 2i16; let s = 12i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3898u64) }
        0x102a3898 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a389cu64) }
        0x102a389c => { vm.decode(0x3473835u32); let d = 13i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38a0u64) }
        0x102a38a0 => { vm.decode(0x4e90d875u32); let d = 31i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38a4u64) }
        0x102a38a4 => { vm.decode(0x4416b835u32); let d = 17i16; let s = 23i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38a8u64) }
        0x102a38a8 => { vm.decode(0x48a00875u32); let d = 22i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38acu64) }
        0x102a38ac => { vm.decode(0x33400835u32); let d = 12i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38b0u64) }
        0x102a38b0 => { vm.decode(0x110d9835u32); let d = 6i16; let s = 4i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38b4u64) }
        0x102a38b4 => { vm.decode(0x4680d875u32); let d = 15i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38b8u64) }
        0x102a38b8 => { vm.decode(0x85c8841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 116400i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a38bcu64) }
        0x102a38bc => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38c0u64) }
        0x102a38c0 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38c4u64) }
        0x102a38c4 => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38c8u64) }
        0x102a38c8 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38ccu64) }
        0x102a38cc => { vm.decode(0x404d2835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38d0u64) }
        0x102a38d0 => { vm.decode(0xf0835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38d4u64) }
        0x102a38d4 => { vm.decode(0x60102835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38d8u64) }
        0x102a38d8 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38dcu64) }
        0x102a38dc => { vm.decode(0x21038835u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38e0u64) }
        0x102a38e0 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38e4u64) }
        0x102a38e4 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38e8u64) }
        0x102a38e8 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38ecu64) }
        0x102a38ec => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a38f0u64) }
        0x102a38f0 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38f4u64) }
        0x102a38f4 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38f8u64) }
        0x102a38f8 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a38fcu64) }
        0x102a38fc => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3900u64) }
        0x102a3900 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3904u64) }
        0x102a3904 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3908u64) }
        0x102a3908 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a390cu64) }
        0x102a390c => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3910u64) }
        0x102a3910 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3914u64) }
        0x102a3914 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3918u64) }
        0x102a3918 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a391cu64) }
        0x102a391c => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3920u64) }
        0x102a3920 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3924u64) }
        0x102a3924 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3928u64) }
        0x102a3928 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a392cu64) }
        0x102a392c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3930u64) }
        0x102a3930 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3934u64) }
        0x102a3934 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3938u64) }
        0x102a3938 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a393cu64) }
        0x102a393c => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3940u64) }
        0x102a3940 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3944u64) }
        0x102a3944 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3948u64) }
        0x102a3948 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a394cu64) }
        0x102a394c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3950u64) }
        0x102a3950 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3954u64) }
        0x102a3954 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3958u64) }
        0x102a3958 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a395cu64) }
        0x102a395c => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3960u64) }
        0x102a3960 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3964u64) }
        0x102a3964 => { vm.decode(0x288ce3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3968u64) }
        0x102a3968 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a396cu64) }
        0x102a396c => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3970u64) }
        0x102a3970 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3974u64) }
        0x102a3974 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3978u64) }
        0x102a3978 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a397cu64) }
        0x102a397c => { vm.decode(0x4002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3980u64) }
        0x102a3980 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3984u64) }
        0x102a3984 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3988u64) }
        0x102a3988 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a398cu64) }
        0x102a398c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3990u64) }
        0x102a3990 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3994u64) }
        0x102a3994 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a3998u64) }
        0x102a3998 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a399cu64) }
        0x102a399c => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a39a0u64) }
        0x102a39a0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a39a4u64) }
        0x102a39a4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a39a8u64) }
        0x102a39a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a39acu64 + 4);
        vm.wr(37, 0x102a39acu64 + 4);
        Ok(0x102a39acu64) }
        0x102a39ac => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a39b0u64) }
        0x102a39b0 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a39b4u64) }
        0x102a39b4 => { vm.decode(0x4828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a39b8u64) }
        0x102a39b8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a39bcu64) }
        0x102a39bc => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a39c0u64) }
        0x102a39c0 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a39c4u64) }
        0x102a39c4 => { vm.decode(0x282e35u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102a39c8u64) }
        0x102a39c8 => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a39ccu64).wrapping_add((imm * 4) as u64) } else { 0x102a39ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a39ccu64) }
        0x102a39cc => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a39d0u64) }
        0x102a39d0 => { vm.decode(0x2280e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a39d4u64) }
        0x102a39fc => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a00u64) }
        0x102a3a00 => { vm.decode(0x40101019u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3a04u64) }
        0x102a3a04 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3a08u64) }
        0x102a3a08 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a3a0cu64) }
        0x102a3a0c => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a3a10u64).wrapping_add((imm * 4) as u64) } else { 0x102a3a10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a3a10u64) }
        0x102a3a10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a3a14u64) } }
        0x102a3a14 => { vm.decode(0x2024002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a18u64) }
        0x102a3a18 => { vm.decode(0x2060f5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a1cu64) }
        0x102a3a1c => { vm.decode(0x22240f5u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a20u64) }
        0x102a3a20 => { vm.decode(0x42040f5u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a24u64) }
        0x102a3a24 => { vm.decode(0x92c403b5u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a28u64) }
        0x102a3a28 => { vm.decode(0x820013b5u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a2cu64) }
        0x102a3a2c => { vm.decode(0x808413b5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a30u64) }
        0x102a3a30 => { vm.decode(0x802204b3u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a34u64) }
        0x102a3a34 => { vm.decode(0x2304b3u32); let d = 9i16; let s = 2i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a38u64) }
        0x102a3a38 => { vm.decode(0x80210433u32); let d = 8i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a3cu64) }
        0x102a3a3c => { vm.decode(0x20620433u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a40u64) }
        0x102a3a40 => { vm.decode(0xa06000b3u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a44u64) }
        0x102a3a44 => { vm.decode(0x6200b3u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a48u64) }
        0x102a3a48 => { vm.decode(0x80600033u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a4cu64) }
        0x102a3a4c => { vm.decode(0x22220f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3a50u64) }
        0x102a3a50 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a3a54u64) }
        0x102a3a54 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3a58u64) }
        0x102a3a58 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a3a5cu64) }
        0x102a3a5c => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a3a60u64).wrapping_add((imm * 4) as u64) } else { 0x102a3a60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a3a60u64) }
        0x102a3a60 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3a64u64) }
        0x102a3a64 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3a68u64) }
        0x102a3a68 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a6cu64) }
        0x102a3a6c => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a70u64) }
        0x102a3a70 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a74u64) }
        0x102a3a74 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a78u64) }
        0x102a3a78 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a7cu64) }
        0x102a3a7c => { vm.decode(0x19001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a80u64) }
        0x102a3a80 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a84u64) }
        0x102a3a84 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a88u64) }
        0x102a3a88 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a8cu64) }
        0x102a3a8c => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a90u64) }
        0x102a3a90 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a94u64) }
        0x102a3a94 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3a98u64) }
        0x102a3a98 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3a9cu64) }
        0x102a3a9c => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3aa0u64) }
        0x102a3aa0 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3aa4u64) }
        0x102a3aa4 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3aa8u64) }
        0x102a3aa8 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a3aacu64) }
        0x102a3aac => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a3ab0u64).wrapping_add((imm * 4) as u64) } else { 0x102a3ab0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a3ab0u64) }
        0x102a3ab0 => { vm.decode(0x652d1019u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3ab4u64) }
        0x102a3ab4 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3ab8u64) }
        0x102a3ab8 => { vm.decode(0x200802beu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3abcu64) }
        0x102a3abc => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ac0u64) }
        0x102a3ac0 => { vm.decode(0x90480437u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102a3ac4u64) }
        0x102a3ac4 => { vm.decode(0x81749935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ac8u64) }
        0x102a3ac8 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3accu64) }
        0x102a3acc => { vm.decode(0x320a023eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3ad0u64) }
        0x102a3ad0 => { vm.decode(0x81709935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ad4u64) }
        0x102a3ad4 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ad8u64) }
        0x102a3ad8 => { vm.decode(0x320a02beu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3adcu64) }
        0x102a3adc => { vm.decode(0x81309935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ae0u64) }
        0x102a3ae0 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ae4u64) }
        0x102a3ae4 => { vm.decode(0x220a027eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3ae8u64) }
        0x102a3ae8 => { vm.decode(0x36a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3aecu64) }
        0x102a3aec => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3af0u64) }
        0x102a3af0 => { vm.decode(0x220a02feu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3af4u64) }
        0x102a3af4 => { vm.decode(0x34a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3af8u64) }
        0x102a3af8 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3afcu64) }
        0x102a3afc => { vm.decode(0x41849019u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3b00u64) }
        0x102a3b00 => { vm.decode(0x38c0019u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3b04u64) }
        0x102a3b04 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3b08u64) }
        0x102a3b08 => { vm.decode(0x380a02feu32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3b0cu64) }
        0x102a3b0c => { vm.decode(0x1a0a027eu32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a3b10u64) }
        0x102a3b10 => { vm.decode(0x13289d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a3b14u64) }
        0x102a3b14 => { vm.decode(0x3004a835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b18u64) }
        0x102a3b18 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b1cu64) }
        0x102a3b1c => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b20u64) }
        0x102a3b20 => { vm.decode(0x42204875u32); let d = 10i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b24u64) }
        0x102a3b24 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a3b28u64) }
        0x102a3b28 => { vm.decode(0x27d12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -30i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a3b2cu64).wrapping_add((imm * 4) as u64) } else { 0x102a3b2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a3b2cu64) }
        0x102a3b2c => { vm.decode(0x19389570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b30u64) }
        0x102a3b30 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b34u64) }
        0x102a3b34 => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b38u64) }
        0x102a3b38 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b3cu64) }
        0x102a3b3c => { vm.decode(0x6882c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b40u64) }
        0x102a3b40 => { vm.decode(0x8100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a3b44u64) }
        0x102a3b44 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b48u64) }
        0x102a3b48 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b4cu64) }
        0x102a3b4c => { vm.decode(0x8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b50u64) }
        0x102a3b50 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3b54u64) }
        0x102a3b54 => { vm.decode(0x28c5bafu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b58u64) }
        0x102a3b58 => { vm.decode(0x4886e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b5cu64) }
        0x102a3b5c => { vm.decode(0x208c5bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b60u64) }
        0x102a3b60 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b64u64) }
        0x102a3b64 => { vm.decode(0x2827bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b68u64) }
        0x102a3b68 => { vm.decode(0x22867bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b6cu64) }
        0x102a3b6c => { vm.decode(0x28a5bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b70u64) }
        0x102a3b70 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b74u64) }
        0x102a3b74 => { vm.decode(0x208a5befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3b78u64) }
        0x102a3b78 => { vm.decode(0x714b0835u32); let d = 6i16; let s = 9i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b7cu64) }
        0x102a3b7c => { vm.decode(0xb10d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b80u64) }
        0x102a3b80 => { vm.decode(0x414b9835u32); let d = 6i16; let s = 14i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b84u64) }
        0x102a3b84 => { vm.decode(0xb3855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b88u64) }
        0x102a3b88 => { vm.decode(0x610f1835u32); let d = 6i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b8cu64) }
        0x102a3b8c => { vm.decode(0xb08d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b90u64) }
        0x102a3b90 => { vm.decode(0x10f1835u32); let d = 6i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b94u64) }
        0x102a3b94 => { vm.decode(0xb00d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3b98u64) }
        0x102a3b98 => { vm.decode(0x274a9835u32); let d = 30i16; let s = 11i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3b9cu64) }
        0x102a3b9c => { vm.decode(0x474aa835u32); let d = 31i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ba0u64) }
        0x102a3ba0 => { vm.decode(0x193095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ba4u64) }
        0x102a3ba4 => { vm.decode(0x40d29119u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a3ba8u64) }
        0x102a3ba8 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3bacu64) }
        0x102a3bac => { vm.decode(0x46449835u32); let d = 24i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3bb0u64) }
        0x102a3bb0 => { vm.decode(0x2208a835u32); let d = 11i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3bb4u64) }
        0x102a3bb4 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a3bb8u64) }
        0x102a3bb8 => { vm.decode(0x420c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 915i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a3bbcu64).wrapping_add((imm * 4) as u64) } else { 0x102a3bbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a3bbcu64) }
        0x102a3bbc => { vm.decode(0x2004875u32); let d = 8i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3bc0u64) }
        0x102a3bc0 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3bc4u64) }
        0x102a3bc4 => { vm.decode(0x54a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3bc8u64) }
        0x102a3bc8 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3bccu64) }
        0x102a3bcc => { vm.decode(0x9b1055f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3bd0u64) }
        0x102a3bd0 => { vm.decode(0x1b3015f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3bd4u64) }
        0x102a3bd4 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3bd8u64) }
        0x102a3bd8 => { vm.decode(0x5290c0f5u32); let d = 12i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3bdcu64) }
        0x102a3bdc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3be0u64) }
        0x102a3be0 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3be4u64) }
        0x102a3be4 => { vm.decode(0xb381470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3be8u64) }
        0x102a3be8 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3becu64) }
        0x102a3bec => { vm.decode(0x2a8e636fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3bf0u64) }
        0x102a3bf0 => { vm.decode(0xa2e234f5u32); let d = 13i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3bf4u64) }
        0x102a3bf4 => { vm.decode(0x82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3bf8u64) }
        0x102a3bf8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3bfcu64) }
        0x102a3bfc => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c00u64) }
        0x102a3c00 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c04u64) }
        0x102a3c04 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c08u64) }
        0x102a3c08 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c0cu64) }
        0x102a3c0c => { vm.decode(0x2088e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c10u64) }
        0x102a3c10 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c14u64) }
        0x102a3c14 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c18u64) }
        0x102a3c18 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c1cu64) }
        0x102a3c1c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c20u64) }
        0x102a3c20 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c24u64) }
        0x102a3c24 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c28u64) }
        0x102a3c28 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c2cu64) }
        0x102a3c2c => { vm.decode(0x3692c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c30u64) }
        0x102a3c30 => { vm.decode(0x50d2a0f5u32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c34u64) }
        0x102a3c34 => { vm.decode(0x220d1122u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a3c38u64) }
        0x102a3c38 => { vm.decode(0x52e000f5u32); let d = 16i16; let s = 5i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c3cu64) }
        0x102a3c3c => { vm.decode(0x70c3a0f5u32); let d = 17i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c40u64) }
        0x102a3c40 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c44u64) }
        0x102a3c44 => { vm.decode(0xa34424f5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c48u64) }
        0x102a3c48 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c4cu64) }
        0x102a3c4c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c50u64) }
        0x102a3c50 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c54u64) }
        0x102a3c54 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c58u64) }
        0x102a3c58 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3c5cu64) }
        0x102a3c5c => { vm.decode(0x54e0a0f5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c60u64) }
        0x102a3c60 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c64u64) }
        0x102a3c64 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c68u64) }
        0x102a3c68 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3c6cu64) }
        0x102a3c6c => { vm.decode(0x62e234f5u32); let d = 13i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c70u64) }
        0x102a3c70 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3c74u64) }
        0x102a3c74 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c78u64) }
        0x102a3c78 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c7cu64) }
        0x102a3c7c => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c80u64) }
        0x102a3c80 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c84u64) }
        0x102a3c84 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c88u64) }
        0x102a3c88 => { vm.decode(0x44901875u32); let d = 6i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3c8cu64) }
        0x102a3c8c => { vm.decode(0xab4434f5u32); let d = 19i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3c90u64) }
        0x102a3c90 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3c94u64) }
        0x102a3c94 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c98u64) }
        0x102a3c98 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3c9cu64) }
        0x102a3c9c => { vm.decode(0x32d200f5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ca0u64) }
        0x102a3ca0 => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ca4u64) }
        0x102a3ca4 => { vm.decode(0xc66cb2adu32); let d = 20i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3ca8u64) }
        0x102a3ca8 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3cacu64) }
        0x102a3cac => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3cb0u64) }
        0x102a3cb0 => { vm.decode(0x56a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3cb4u64) }
        0x102a3cb4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3cb8u64) }
        0x102a3cb8 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3cbcu64) }
        0x102a3cbc => { vm.decode(0x3603a835u32); let d = 25i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3cc0u64) }
        0x102a3cc0 => { vm.decode(0x50b1e0f5u32); let d = 12i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3cc4u64) }
        0x102a3cc4 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3cc8u64) }
        0x102a3cc8 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3cccu64) }
        0x102a3ccc => { vm.decode(0x12624f5u32); let d = 20i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3cd0u64) }
        0x102a3cd0 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3cd4u64) }
        0x102a3cd4 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3cd8u64) }
        0x102a3cd8 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3cdcu64) }
        0x102a3cdc => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3ce0u64) }
        0x102a3ce0 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3ce4u64) }
        0x102a3ce4 => { vm.decode(0x33142835u32); let d = 13i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ce8u64) }
        0x102a3ce8 => { vm.decode(0x3490a0f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3cecu64) }
        0x102a3cec => { vm.decode(0x8b1814f0u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3cf0u64) }
        0x102a3cf0 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3cf4u64) }
        0x102a3cf4 => { vm.decode(0x6a8034f5u32); let d = 10i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3cf8u64) }
        0x102a3cf8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3cfcu64) }
        0x102a3cfc => { vm.decode(0x50462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d00u64) }
        0x102a3d00 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d04u64) }
        0x102a3d04 => { vm.decode(0x20a80a2fu32); let d = 11i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d08u64) }
        0x102a3d08 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d0cu64) }
        0x102a3d0c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d10u64) }
        0x102a3d10 => { vm.decode(0x6080402fu32); let d = 17i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d14u64) }
        0x102a3d14 => { vm.decode(0x4088022fu32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d18u64) }
        0x102a3d18 => { vm.decode(0x24501835u32); let d = 16i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d1cu64) }
        0x102a3d1c => { vm.decode(0x70b1e0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3d20u64) }
        0x102a3d20 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3d24u64) }
        0x102a3d24 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3d28u64) }
        0x102a3d28 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d2cu64) }
        0x102a3d2c => { vm.decode(0x5007a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d30u64) }
        0x102a3d30 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d34u64) }
        0x102a3d34 => { vm.decode(0x688a4bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d38u64) }
        0x102a3d38 => { vm.decode(0x76b3c0f5u32); let d = 15i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3d3cu64) }
        0x102a3d3c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3d40u64) }
        0x102a3d40 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3d44u64) }
        0x102a3d44 => { vm.decode(0x60a8032fu32); let d = 19i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d48u64) }
        0x102a3d48 => { vm.decode(0x43501835u32); let d = 12i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d4cu64) }
        0x102a3d4c => { vm.decode(0x54b260f5u32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3d50u64) }
        0x102a3d50 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3d54u64) }
        0x102a3d54 => { vm.decode(0xeae234f5u32); let d = 15i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3d58u64) }
        0x102a3d58 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d5cu64) }
        0x102a3d5c => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d60u64) }
        0x102a3d60 => { vm.decode(0x20472835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d64u64) }
        0x102a3d64 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d68u64) }
        0x102a3d68 => { vm.decode(0x534f2835u32); let d = 15i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d6cu64) }
        0x102a3d6c => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d70u64) }
        0x102a3d70 => { vm.decode(0x14a020f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3d74u64) }
        0x102a3d74 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3d78u64) }
        0x102a3d78 => { vm.decode(0x6a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3d7cu64) }
        0x102a3d7c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d80u64) }
        0x102a3d80 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d84u64) }
        0x102a3d84 => { vm.decode(0x404c1835u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3d88u64) }
        0x102a3d88 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3d8cu64) }
        0x102a3d8c => { vm.decode(0x56a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3d90u64) }
        0x102a3d90 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3d94u64) }
        0x102a3d94 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3d98u64) }
        0x102a3d98 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3d9cu64) }
        0x102a3d9c => { vm.decode(0x1b301470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3da0u64) }
        0x102a3da0 => { vm.decode(0x34b220f5u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3da4u64) }
        0x102a3da4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3da8u64) }
        0x102a3da8 => { vm.decode(0xeac034f5u32); let d = 11i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3dacu64) }
        0x102a3dac => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3db0u64) }
        0x102a3db0 => { vm.decode(0x25462835u32); let d = 21i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3db4u64) }
        0x102a3db4 => { vm.decode(0x569380f5u32); let d = 11i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3db8u64) }
        0x102a3db8 => { vm.decode(0x42105875u32); let d = 10i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3dbcu64) }
        0x102a3dbc => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3dc0u64) }
        0x102a3dc0 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3dc4u64) }
        0x102a3dc4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3dc8u64) }
        0x102a3dc8 => { vm.decode(0x549160f5u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3dccu64) }
        0x102a3dcc => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3dd0u64) }
        0x102a3dd0 => { vm.decode(0x6aa234f5u32); let d = 14i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3dd4u64) }
        0x102a3dd4 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3dd8u64) }
        0x102a3dd8 => { vm.decode(0x524f2835u32); let d = 11i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ddcu64) }
        0x102a3ddc => { vm.decode(0x16b040f5u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3de0u64) }
        0x102a3de0 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3de4u64) }
        0x102a3de4 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3de8u64) }
        0x102a3de8 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3decu64) }
        0x102a3dec => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3df0u64) }
        0x102a3df0 => { vm.decode(0x14e020f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3df4u64) }
        0x102a3df4 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3df8u64) }
        0x102a3df8 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3dfcu64) }
        0x102a3dfc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e00u64) }
        0x102a3e00 => { vm.decode(0x145ca835u32); let d = 19i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e04u64) }
        0x102a3e04 => { vm.decode(0x7691c0f5u32); let d = 14i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e08u64) }
        0x102a3e08 => { vm.decode(0x40305875u32); let d = 2i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e0cu64) }
        0x102a3e0c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e10u64) }
        0x102a3e10 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e14u64) }
        0x102a3e14 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e18u64) }
        0x102a3e18 => { vm.decode(0x74e1a0f5u32); let d = 18i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e1cu64) }
        0x102a3e1c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e20u64) }
        0x102a3e20 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e24u64) }
        0x102a3e24 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e28u64) }
        0x102a3e28 => { vm.decode(0x135c8835u32); let d = 14i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e2cu64) }
        0x102a3e2c => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e30u64) }
        0x102a3e30 => { vm.decode(0x36c000f5u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e34u64) }
        0x102a3e34 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e38u64) }
        0x102a3e38 => { vm.decode(0x490424f5u32); let d = 18i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e3cu64) }
        0x102a3e3c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e40u64) }
        0x102a3e40 => { vm.decode(0x428e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e44u64) }
        0x102a3e44 => { vm.decode(0x70c1e0f5u32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e48u64) }
        0x102a3e48 => { vm.decode(0xc66c22edu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e4cu64) }
        0x102a3e4c => { vm.decode(0x232624f5u32); let d = 20i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e50u64) }
        0x102a3e50 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e54u64) }
        0x102a3e54 => { vm.decode(0x9b00d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3e58u64) }
        0x102a3e58 => { vm.decode(0x4153835u32); let d = 17i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e5cu64) }
        0x102a3e5c => { vm.decode(0x54e000f5u32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e60u64) }
        0x102a3e60 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e64u64) }
        0x102a3e64 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e68u64) }
        0x102a3e68 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e6cu64) }
        0x102a3e6c => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e70u64) }
        0x102a3e70 => { vm.decode(0x99289cf0u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3e74u64) }
        0x102a3e74 => { vm.decode(0x131c63b5u32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e78u64) }
        0x102a3e78 => { vm.decode(0xb2855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3e7cu64) }
        0x102a3e7c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3e80u64) }
        0x102a3e80 => { vm.decode(0x1b205430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3e84u64) }
        0x102a3e84 => { vm.decode(0x2282c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e88u64) }
        0x102a3e88 => { vm.decode(0x690424f5u32); let d = 18i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3e8cu64) }
        0x102a3e8c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e90u64) }
        0x102a3e90 => { vm.decode(0x6a84cb6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3e94u64) }
        0x102a3e94 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3e98u64) }
        0x102a3e98 => { vm.decode(0x16e000f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3e9cu64) }
        0x102a3e9c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3ea0u64) }
        0x102a3ea0 => { vm.decode(0x410624f5u32); let d = 20i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3ea4u64) }
        0x102a3ea4 => { vm.decode(0x34c080f5u32); let d = 18i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ea8u64) }
        0x102a3ea8 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3eacu64) }
        0x102a3eac => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3eb0u64) }
        0x102a3eb0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3eb4u64) }
        0x102a3eb4 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3eb8u64) }
        0x102a3eb8 => { vm.decode(0x250f1835u32); let d = 22i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ebcu64) }
        0x102a3ebc => { vm.decode(0xb109470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ec0u64) }
        0x102a3ec0 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ec4u64) }
        0x102a3ec4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3ec8u64) }
        0x102a3ec8 => { vm.decode(0xaa2434f5u32); let d = 18i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3eccu64) }
        0x102a3ecc => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ed0u64) }
        0x102a3ed0 => { vm.decode(0x50b3c0f5u32); let d = 13i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3ed4u64) }
        0x102a3ed4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3ed8u64) }
        0x102a3ed8 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3edcu64) }
        0x102a3edc => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3ee0u64) }
        0x102a3ee0 => { vm.decode(0x5010a835u32); let d = 1i16; let s = 13i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ee4u64) }
        0x102a3ee4 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ee8u64) }
        0x102a3ee8 => { vm.decode(0x309240f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3eecu64) }
        0x102a3eec => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3ef0u64) }
        0x102a3ef0 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3ef4u64) }
        0x102a3ef4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3ef8u64) }
        0x102a3ef8 => { vm.decode(0x208063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3efcu64) }
        0x102a3efc => { vm.decode(0x72008835u32); let d = 8i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f00u64) }
        0x102a3f00 => { vm.decode(0x30b240f5u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f04u64) }
        0x102a3f04 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f08u64) }
        0x102a3f08 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f0cu64) }
        0x102a3f0c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f10u64) }
        0x102a3f10 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f14u64) }
        0x102a3f14 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3f18u64) }
        0x102a3f18 => { vm.decode(0x7093c0f5u32); let d = 13i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f1cu64) }
        0x102a3f1c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f20u64) }
        0x102a3f20 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f24u64) }
        0x102a3f24 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f28u64) }
        0x102a3f28 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f2cu64) }
        0x102a3f2c => { vm.decode(0x7300a835u32); let d = 13i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f30u64) }
        0x102a3f30 => { vm.decode(0x309140f5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f34u64) }
        0x102a3f34 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f38u64) }
        0x102a3f38 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f3cu64) }
        0x102a3f3c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f40u64) }
        0x102a3f40 => { vm.decode(0x3043a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f44u64) }
        0x102a3f44 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3f48u64) }
        0x102a3f48 => { vm.decode(0x42806befu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f4cu64) }
        0x102a3f4c => { vm.decode(0x3492c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f50u64) }
        0x102a3f50 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f54u64) }
        0x102a3f54 => { vm.decode(0xeae214f5u32); let d = 15i16; let s = 3i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f58u64) }
        0x102a3f58 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f5cu64) }
        0x102a3f5c => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f60u64) }
        0x102a3f60 => { vm.decode(0x730ca835u32); let d = 15i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f64u64) }
        0x102a3f64 => { vm.decode(0x50c100f5u32); let d = 16i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f68u64) }
        0x102a3f68 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f6cu64) }
        0x102a3f6c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f70u64) }
        0x102a3f70 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f74u64) }
        0x102a3f74 => { vm.decode(0x20113835u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f78u64) }
        0x102a3f78 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3f7cu64) }
        0x102a3f7c => { vm.decode(0x2801875u32); let d = 12i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f80u64) }
        0x102a3f80 => { vm.decode(0x148100f5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3f84u64) }
        0x102a3f84 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3f88u64) }
        0x102a3f88 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3f8cu64) }
        0x102a3f8c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f90u64) }
        0x102a3f90 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f94u64) }
        0x102a3f94 => { vm.decode(0x200c0835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3f98u64) }
        0x102a3f98 => { vm.decode(0x228e43afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3f9cu64) }
        0x102a3f9c => { vm.decode(0x50e100f5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3fa0u64) }
        0x102a3fa0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3fa4u64) }
        0x102a3fa4 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3fa8u64) }
        0x102a3fa8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3facu64) }
        0x102a3fac => { vm.decode(0x433835u32); let d = 1i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3fb0u64) }
        0x102a3fb0 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3fb4u64) }
        0x102a3fb4 => { vm.decode(0x70e180f5u32); let d = 16i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3fb8u64) }
        0x102a3fb8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3fbcu64) }
        0x102a3fbc => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3fc0u64) }
        0x102a3fc0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3fc4u64) }
        0x102a3fc4 => { vm.decode(0x25503835u32); let d = 21i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3fc8u64) }
        0x102a3fc8 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3fccu64) }
        0x102a3fcc => { vm.decode(0x10e000f5u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3fd0u64) }
        0x102a3fd0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3fd4u64) }
        0x102a3fd4 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3fd8u64) }
        0x102a3fd8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3fdcu64) }
        0x102a3fdc => { vm.decode(0x244a3835u32); let d = 19i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3fe0u64) }
        0x102a3fe0 => { vm.decode(0x4a80c875u32); let d = 31i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3fe4u64) }
        0x102a3fe4 => { vm.decode(0x549200f5u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a3fe8u64) }
        0x102a3fe8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a3fecu64) }
        0x102a3fec => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a3ff0u64) }
        0x102a3ff0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a3ff4u64) }
        0x102a3ff4 => { vm.decode(0x71140835u32); let d = 4i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a3ff8u64) }
        0x102a3ff8 => { vm.decode(0x938dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a3ffcu64) }
        0x102a3ffc => { vm.decode(0x52bb5u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4000u64) }
        0x102a4000 => { vm.decode(0x24d3022u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a4004u64) }
        0x102a4004 => { vm.decode(0x2209875u32); let d = 9i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4008u64) }
        0x102a4008 => { vm.decode(0xb181570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a400cu64) }
        0x102a400c => { vm.decode(0x401b2d35u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4010u64) }
        0x102a4010 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4014u64) }
        0x102a4014 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4018u64) }
        0x102a4018 => { vm.decode(0x282e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a401cu64) }
        0x102a401c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4020u64) }
        0x102a4020 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4024u64) }
        0x102a4024 => { vm.decode(0x74160835u32); let d = 16i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4028u64) }
        0x102a4028 => { vm.decode(0xd39c53b5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a402cu64) }
        0x102a402c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4030u64) }
        0x102a4030 => { vm.decode(0xe8c034f5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4034u64) }
        0x102a4034 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4038u64) }
        0x102a4038 => { vm.decode(0x88ee36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a403cu64) }
        0x102a403c => { vm.decode(0x554d2835u32); let d = 23i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4040u64) }
        0x102a4040 => { vm.decode(0x6080482fu32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4044u64) }
        0x102a4044 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4048u64) }
        0x102a4048 => { vm.decode(0x51581835u32); let d = 6i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a404cu64) }
        0x102a404c => { vm.decode(0xb085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4050u64) }
        0x102a4050 => { vm.decode(0x639c73b5u32); let d = 25i16; let s = 30i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4054u64) }
        0x102a4054 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4058u64) }
        0x102a4058 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a405cu64) }
        0x102a405c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4060u64) }
        0x102a4060 => { vm.decode(0x54591835u32); let d = 18i16; let s = 22i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4064u64) }
        0x102a4064 => { vm.decode(0x28886bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4068u64) }
        0x102a4068 => { vm.decode(0x30f300f5u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a406cu64) }
        0x102a406c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4070u64) }
        0x102a4070 => { vm.decode(0xab0614f5u32); let d = 22i16; let s = 3i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4074u64) }
        0x102a4074 => { vm.decode(0xe39c73b5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4078u64) }
        0x102a4078 => { vm.decode(0x9b3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a407cu64) }
        0x102a407c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4080u64) }
        0x102a4080 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4084u64) }
        0x102a4084 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4088u64) }
        0x102a4088 => { vm.decode(0x55021835u32); let d = 20i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a408cu64) }
        0x102a408c => { vm.decode(0x40b00875u32); let d = 6i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4090u64) }
        0x102a4090 => { vm.decode(0x721c13b5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4094u64) }
        0x102a4094 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4098u64) }
        0x102a4098 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a409cu64) }
        0x102a409c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40a0u64) }
        0x102a40a0 => { vm.decode(0x2a8cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40a4u64) }
        0x102a40a4 => { vm.decode(0x374c0835u32); let d = 30i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a40a8u64) }
        0x102a40a8 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40acu64) }
        0x102a40ac => { vm.decode(0x229c33b5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a40b0u64) }
        0x102a40b0 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a40b4u64) }
        0x102a40b4 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a40b8u64) }
        0x102a40b8 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40bcu64) }
        0x102a40bc => { vm.decode(0x700b3835u32); let d = 3i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a40c0u64) }
        0x102a40c0 => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a40c4u64) }
        0x102a40c4 => { vm.decode(0x886befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40c8u64) }
        0x102a40c8 => { vm.decode(0x23dc13b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a40ccu64) }
        0x102a40cc => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a40d0u64) }
        0x102a40d0 => { vm.decode(0xa0e234f5u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a40d4u64) }
        0x102a40d4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40d8u64) }
        0x102a40d8 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40dcu64) }
        0x102a40dc => { vm.decode(0x7048a835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a40e0u64) }
        0x102a40e0 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a40e4u64) }
        0x102a40e4 => { vm.decode(0x22824befu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40e8u64) }
        0x102a40e8 => { vm.decode(0xa29c73b5u32); let d = 25i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a40ecu64) }
        0x102a40ec => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a40f0u64) }
        0x102a40f0 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a40f4u64) }
        0x102a40f4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a40f8u64) }
        0x102a40f8 => { vm.decode(0x76433835u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a40fcu64) }
        0x102a40fc => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4100u64) }
        0x102a4100 => { vm.decode(0x92dc13b5u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4104u64) }
        0x102a4104 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4108u64) }
        0x102a4108 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a410cu64) }
        0x102a410c => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4110u64) }
        0x102a4110 => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4114u64) }
        0x102a4114 => { vm.decode(0x7004a835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4118u64) }
        0x102a4118 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a411cu64) }
        0x102a411c => { vm.decode(0x93dc53b5u32); let d = 15i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4120u64) }
        0x102a4120 => { vm.decode(0x46805875u32); let d = 14i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4124u64) }
        0x102a4124 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4128u64) }
        0x102a4128 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a412cu64) }
        0x102a412c => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4130u64) }
        0x102a4130 => { vm.decode(0x5044a835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4134u64) }
        0x102a4134 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4138u64) }
        0x102a4138 => { vm.decode(0xb21c03b5u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a413cu64) }
        0x102a413c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4140u64) }
        0x102a4140 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4144u64) }
        0x102a4144 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4148u64) }
        0x102a4148 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a414cu64) }
        0x102a414c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4150u64) }
        0x102a4150 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4154u64) }
        0x102a4154 => { vm.decode(0x608043efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4158u64) }
        0x102a4158 => { vm.decode(0xd21c03b5u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a415cu64) }
        0x102a415c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4160u64) }
        0x102a4160 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4164u64) }
        0x102a4164 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4168u64) }
        0x102a4168 => { vm.decode(0x20542835u32); let d = 1i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a416cu64) }
        0x102a416c => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4170u64) }
        0x102a4170 => { vm.decode(0x321c43b5u32); let d = 2i16; let s = 12i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4174u64) }
        0x102a4174 => { vm.decode(0x4820c875u32); let d = 19i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4178u64) }
        0x102a4178 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a417cu64) }
        0x102a417c => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4180u64) }
        0x102a4180 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4184u64) }
        0x102a4184 => { vm.decode(0x20152835u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4188u64) }
        0x102a4188 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a418cu64) }
        0x102a418c => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4190u64) }
        0x102a4190 => { vm.decode(0x931c43b5u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4194u64) }
        0x102a4194 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4198u64) }
        0x102a4198 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a419cu64) }
        0x102a419c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41a0u64) }
        0x102a41a0 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41a4u64) }
        0x102a41a4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a41a8u64) }
        0x102a41a8 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a41acu64) }
        0x102a41ac => { vm.decode(0x1830bb5u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a41b0u64) }
        0x102a41b0 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a41b4u64) }
        0x102a41b4 => { vm.decode(0x30f1022u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a41b8u64) }
        0x102a41b8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a41bcu64) }
        0x102a41bc => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a41c0u64) }
        0x102a41c0 => { vm.decode(0x208cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41c4u64) }
        0x102a41c4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a41c8u64) }
        0x102a41c8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41ccu64) }
        0x102a41cc => { vm.decode(0x142835u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a41d0u64) }
        0x102a41d0 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a41d4u64) }
        0x102a41d4 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a41d8u64) }
        0x102a41d8 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a41dcu64) }
        0x102a41dc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a41e0u64) }
        0x102a41e0 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a41e4u64) }
        0x102a41e4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41e8u64) }
        0x102a41e8 => { vm.decode(0x2a8ceb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41ecu64) }
        0x102a41ec => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a41f0u64) }
        0x102a41f0 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a41f4u64) }
        0x102a41f4 => { vm.decode(0x28884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a41f8u64) }
        0x102a41f8 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a41fcu64) }
        0x102a41fc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4200u64) }
        0x102a4200 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4204u64) }
        0x102a4204 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4208u64) }
        0x102a4208 => { vm.decode(0x230bb5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a420cu64) }
        0x102a420c => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a4210u64) }
        0x102a4210 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4214u64) }
        0x102a4214 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4218u64) }
        0x102a4218 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a421cu64) }
        0x102a421c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4220u64) }
        0x102a4220 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4224u64) }
        0x102a4224 => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4228u64) }
        0x102a4228 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a422cu64) }
        0x102a422c => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4230u64) }
        0x102a4230 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a4234u64) }
        0x102a4234 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4238u64) }
        0x102a4238 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a423cu64) }
        0x102a423c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4240u64) }
        0x102a4240 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4244u64) }
        0x102a4244 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4248u64) }
        0x102a4248 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a424cu64) }
        0x102a424c => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4250u64) }
        0x102a4250 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4254u64) }
        0x102a4254 => { vm.decode(0x208263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4258u64) }
        0x102a4258 => { vm.decode(0x65012835u32); let d = 21i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a425cu64) }
        0x102a425c => { vm.decode(0x80010bb5u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4260u64) }
        0x102a4260 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4264u64) }
        0x102a4264 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4268u64) }
        0x102a4268 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a426cu64) }
        0x102a426c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4270u64) }
        0x102a4270 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4274u64) }
        0x102a4274 => { vm.decode(0x45192835u32); let d = 23i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4278u64) }
        0x102a4278 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a427cu64) }
        0x102a427c => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4280u64) }
        0x102a4280 => { vm.decode(0xa001875u32); let d = 24i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4284u64) }
        0x102a4284 => { vm.decode(0x62357062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a4288u64) }
        0x102a4288 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a428cu64) }
        0x102a428c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4290u64) }
        0x102a4290 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4294u64) }
        0x102a4294 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4298u64) }
        0x102a4298 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a429cu64) }
        0x102a429c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a42a0u64) }
        0x102a42a0 => { vm.decode(0x40e12bb5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a42a4u64) }
        0x102a42a4 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a42a8u64) }
        0x102a42a8 => { vm.decode(0x21d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a42acu64) }
        0x102a42ac => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a42b0u64) }
        0x102a42b0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a42b4u64) }
        0x102a42b4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a42b8u64) }
        0x102a42b8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a42bcu64) }
        0x102a42bc => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a42c0u64) }
        0x102a42c0 => { vm.decode(0x64090835u32); let d = 18i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a42c4u64) }
        0x102a42c4 => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a42c8u64) }
        0x102a42c8 => { vm.decode(0x1840bb5u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a42ccu64) }
        0x102a42cc => { vm.decode(0x40126d35u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a42d0u64) }
        0x102a42d0 => { vm.decode(0x42471062u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a42d4u64) }
        0x102a42d4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a42d8u64) }
        0x102a42d8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a42dcu64) }
        0x102a42dc => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a42e0u64) }
        0x102a42e0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a42e4u64) }
        0x102a42e4 => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a42e8u64) }
        0x102a42e8 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a42ecu64) }
        0x102a42ec => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a42f0u64) }
        0x102a42f0 => { vm.decode(0x1472bb5u32); let d = 15i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a42f4u64) }
        0x102a42f4 => { vm.decode(0x60926d35u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a42f8u64) }
        0x102a42f8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a42fcu64) }
        0x102a42fc => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4300u64) }
        0x102a4300 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4304u64) }
        0x102a4304 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4308u64) }
        0x102a4308 => { vm.decode(0x65120835u32); let d = 20i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a430cu64) }
        0x102a430c => { vm.decode(0x41800bb5u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4310u64) }
        0x102a4310 => { vm.decode(0x400b4d35u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4314u64) }
        0x102a4314 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4318u64) }
        0x102a4318 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a431cu64) }
        0x102a431c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4320u64) }
        0x102a4320 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4324u64) }
        0x102a4324 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4328u64) }
        0x102a4328 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a432cu64) }
        0x102a432c => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4330u64) }
        0x102a4330 => { vm.decode(0xa70bb5u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4334u64) }
        0x102a4334 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4338u64) }
        0x102a4338 => { vm.decode(0x4480c875u32); let d = 7i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a433cu64) }
        0x102a433c => { vm.decode(0x63751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a4340u64) }
        0x102a4340 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4344u64) }
        0x102a4344 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4348u64) }
        0x102a4348 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a434cu64) }
        0x102a434c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4350u64) }
        0x102a4350 => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4354u64) }
        0x102a4354 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4358u64) }
        0x102a4358 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a435cu64) }
        0x102a435c => { vm.decode(0x2a82cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4360u64) }
        0x102a4360 => { vm.decode(0xa62bb5u32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4364u64) }
        0x102a4364 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4368u64) }
        0x102a4368 => { vm.decode(0x43453062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a436cu64) }
        0x102a436c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4370u64) }
        0x102a4370 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4374u64) }
        0x102a4374 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4378u64) }
        0x102a4378 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a437cu64) }
        0x102a437c => { vm.decode(0x2284e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4380u64) }
        0x102a4380 => { vm.decode(0x36042835u32); let d = 25i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4384u64) }
        0x102a4384 => { vm.decode(0x408b4d35u32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4388u64) }
        0x102a4388 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a438cu64) }
        0x102a438c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4390u64) }
        0x102a4390 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4394u64) }
        0x102a4394 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4398u64) }
        0x102a4398 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a439cu64) }
        0x102a439c => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a43a0u64) }
        0x102a43a0 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a43a4u64) }
        0x102a43a4 => { vm.decode(0x232bb5u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a43a8u64) }
        0x102a43a8 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a43acu64) }
        0x102a43ac => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a43b0u64) }
        0x102a43b0 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a43b4u64) }
        0x102a43b4 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a43b8u64) }
        0x102a43b8 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a43bcu64) }
        0x102a43bc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a43c0u64) }
        0x102a43c0 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a43c4u64) }
        0x102a43c4 => { vm.decode(0x370c2835u32); let d = 31i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a43c8u64) }
        0x102a43c8 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a43ccu64) }
        0x102a43cc => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a43d0u64) }
        0x102a43d0 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a43d4u64) }
        0x102a43d4 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a43d8u64) }
        0x102a43d8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a43dcu64) }
        0x102a43dc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a43e0u64) }
        0x102a43e0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a43e4u64) }
        0x102a43e4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a43e8u64) }
        0x102a43e8 => { vm.decode(0x88ac36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a43ecu64) }
        0x102a43ec => { vm.decode(0x114d0835u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a43f0u64) }
        0x102a43f0 => { vm.decode(0x9305db0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a43f4u64) }
        0x102a43f4 => { vm.decode(0x524a4349u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a43f8u64) }
        0x102a43f8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a43fcu64) }
        0x102a43fc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4400u64) }
        0x102a4400 => { vm.decode(0xc109875u32); let d = 17i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4404u64) }
        0x102a4404 => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4408u64) }
        0x102a4408 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a440cu64) }
        0x102a440c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4410u64) }
        0x102a4410 => { vm.decode(0x2884cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4414u64) }
        0x102a4414 => { vm.decode(0x340e2835u32); let d = 19i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4418u64) }
        0x102a4418 => { vm.decode(0x1c52bb5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a441cu64) }
        0x102a441c => { vm.decode(0x600b6d35u32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4420u64) }
        0x102a4420 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4424u64) }
        0x102a4424 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4428u64) }
        0x102a4428 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a442cu64) }
        0x102a442c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4430u64) }
        0x102a4430 => { vm.decode(0x2880eb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4434u64) }
        0x102a4434 => { vm.decode(0x350e0835u32); let d = 22i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4438u64) }
        0x102a4438 => { vm.decode(0x2884eb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a443cu64) }
        0x102a443c => { vm.decode(0x80b2fu32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4440u64) }
        0x102a4440 => { vm.decode(0x32152835u32); let d = 9i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4444u64) }
        0x102a4444 => { vm.decode(0x19209c70u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4448u64) }
        0x102a4448 => { vm.decode(0x504d4349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a444cu64) }
        0x102a444c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4450u64) }
        0x102a4450 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4454u64) }
        0x102a4454 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4458u64) }
        0x102a4458 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a445cu64) }
        0x102a445c => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4460u64) }
        0x102a4460 => { vm.decode(0x15442835u32); let d = 21i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4464u64) }
        0x102a4464 => { vm.decode(0x544e4349u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4468u64) }
        0x102a4468 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a446cu64) }
        0x102a446c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4470u64) }
        0x102a4470 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4474u64) }
        0x102a4474 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4478u64) }
        0x102a4478 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a447cu64) }
        0x102a447c => { vm.decode(0x16440835u32); let d = 24i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4480u64) }
        0x102a4480 => { vm.decode(0x41c20bb5u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a4484u64) }
        0x102a4484 => { vm.decode(0x608b4d35u32); let d = 10i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a4488u64) }
        0x102a4488 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a448cu64) }
        0x102a448c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4490u64) }
        0x102a4490 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4494u64) }
        0x102a4494 => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4498u64) }
        0x102a4498 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a449cu64) }
        0x102a449c => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a44a0u64) }
        0x102a44a0 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a44a4u64) }
        0x102a44a4 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a44a8u64) }
        0x102a44a8 => { vm.decode(0x105c9835u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a44acu64) }
        0x102a44ac => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a44b0u64) }
        0x102a44b0 => { vm.decode(0x407f5349u32); let d = 18i16; let s = 15i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a44b4u64) }
        0x102a44b4 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a44b8u64) }
        0x102a44b8 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a44bcu64) }
        0x102a44bc => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a44c0u64) }
        0x102a44c0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a44c4u64) }
        0x102a44c4 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a44c8u64) }
        0x102a44c8 => { vm.decode(0x4c1835u32); let d = 2i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a44ccu64) }
        0x102a44cc => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a44d0u64) }
        0x102a44d0 => { vm.decode(0x406f5349u32); let d = 18i16; let s = 14i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a44d4u64) }
        0x102a44d4 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a44d8u64) }
        0x102a44d8 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a44dcu64) }
        0x102a44dc => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a44e0u64) }
        0x102a44e0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a44e4u64) }
        0x102a44e4 => { vm.decode(0x205d1835u32); let d = 2i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a44e8u64) }
        0x102a44e8 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a44ecu64) }
        0x102a44ec => { vm.decode(0x406e5349u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a44f0u64) }
        0x102a44f0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a44f4u64) }
        0x102a44f4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a44f8u64) }
        0x102a44f8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a44fcu64) }
        0x102a44fc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4500u64) }
        0x102a4500 => { vm.decode(0x1d1835u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4504u64) }
        0x102a4504 => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4508u64) }
        0x102a4508 => { vm.decode(0xd105a11u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a450cu64) }
        0x102a450c => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4510u64) }
        0x102a4510 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4514u64) }
        0x102a4514 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4518u64) }
        0x102a4518 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a451cu64) }
        0x102a451c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4520u64) }
        0x102a4520 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4524u64) }
        0x102a4524 => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4528u64) }
        0x102a4528 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a452cu64) }
        0x102a452c => { vm.decode(0x5d98e911u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4530u64) }
        0x102a4530 => { vm.decode(0x447d4349u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4534u64) }
        0x102a4534 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4538u64) }
        0x102a4538 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a453cu64) }
        0x102a453c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4540u64) }
        0x102a4540 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4544u64) }
        0x102a4544 => { vm.decode(0x201e0835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4548u64) }
        0x102a4548 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a454cu64) }
        0x102a454c => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4550u64) }
        0x102a4550 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4554u64) }
        0x102a4554 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4558u64) }
        0x102a4558 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a455cu64) }
        0x102a455c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4560u64) }
        0x102a4560 => { vm.decode(0x2280e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4564u64) }
        0x102a4564 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4568u64) }
        0x102a4568 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a456cu64) }
        0x102a456c => { vm.decode(0x407c4349u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4570u64) }
        0x102a4570 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4574u64) }
        0x102a4574 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4578u64) }
        0x102a4578 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a457cu64) }
        0x102a457c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4580u64) }
        0x102a4580 => { vm.decode(0x2a84e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4584u64) }
        0x102a4584 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4588u64) }
        0x102a4588 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a458cu64) }
        0x102a458c => { vm.decode(0x407d4349u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4590u64) }
        0x102a4590 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4594u64) }
        0x102a4594 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4598u64) }
        0x102a4598 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a459cu64) }
        0x102a459c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45a0u64) }
        0x102a45a0 => { vm.decode(0x20572835u32); let d = 1i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a45a4u64) }
        0x102a45a4 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a45a8u64) }
        0x102a45a8 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a45acu64) }
        0x102a45ac => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a45b0u64) }
        0x102a45b0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a45b4u64) }
        0x102a45b4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a45b8u64) }
        0x102a45b8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45bcu64) }
        0x102a45bc => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45c0u64) }
        0x102a45c0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a45c4u64) }
        0x102a45c4 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a45c8u64) }
        0x102a45c8 => { vm.decode(0xdf4349u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a45ccu64) }
        0x102a45cc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a45d0u64) }
        0x102a45d0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a45d4u64) }
        0x102a45d4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a45d8u64) }
        0x102a45d8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45dcu64) }
        0x102a45dc => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a45e0u64) }
        0x102a45e0 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a45e4u64) }
        0x102a45e4 => { vm.decode(0xd2ca916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a45e8u64) }
        0x102a45e8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a45ecu64) }
        0x102a45ec => { vm.decode(0x628ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45f0u64) }
        0x102a45f0 => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a45f4u64) }
        0x102a45f4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a45f8u64) }
        0x102a45f8 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a45fcu64) }
        0x102a45fc => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4600u64) }
        0x102a4600 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4604u64) }
        0x102a4604 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4608u64) }
        0x102a4608 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a460cu64) }
        0x102a460c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4610u64) }
        0x102a4610 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4614u64) }
        0x102a4614 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4618u64) }
        0x102a4618 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a461cu64) }
        0x102a461c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4620u64) }
        0x102a4620 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4624u64) }
        0x102a4624 => { vm.decode(0x2fb4349u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4628u64) }
        0x102a4628 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a462cu64) }
        0x102a462c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4630u64) }
        0x102a4630 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4634u64) }
        0x102a4634 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4638u64) }
        0x102a4638 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a463cu64) }
        0x102a463c => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4640u64) }
        0x102a4640 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4644u64) }
        0x102a4644 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4648u64) }
        0x102a4648 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a464cu64) }
        0x102a464c => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4650u64) }
        0x102a4650 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4654u64) }
        0x102a4654 => { vm.decode(0x60112835u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4658u64) }
        0x102a4658 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a465cu64) }
        0x102a465c => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4660u64) }
        0x102a4660 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4664u64) }
        0x102a4664 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4668u64) }
        0x102a4668 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a466cu64) }
        0x102a466c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4670u64) }
        0x102a4670 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4674u64) }
        0x102a4674 => { vm.decode(0x2020022fu32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4678u64) }
        0x102a4678 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a467cu64) }
        0x102a467c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4680u64) }
        0x102a4680 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4684u64) }
        0x102a4684 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4688u64) }
        0x102a4688 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a468cu64) }
        0x102a468c => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4690u64) }
        0x102a4690 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4694u64) }
        0x102a4694 => { vm.decode(0x208c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4698u64) }
        0x102a4698 => { vm.decode(0xda5ac12du32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a469cu64) }
        0x102a469c => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46a0u64) }
        0x102a46a0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46a4u64) }
        0x102a46a4 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46a8u64) }
        0x102a46a8 => { vm.decode(0xdacec12du32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46acu64) }
        0x102a46ac => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46b0u64) }
        0x102a46b0 => { vm.decode(0xdadec12du32); let d = 15i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46b4u64) }
        0x102a46b4 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46b8u64) }
        0x102a46b8 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46bcu64) }
        0x102a46bc => { vm.decode(0x228c63afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46c0u64) }
        0x102a46c0 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46c4u64) }
        0x102a46c4 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46c8u64) }
        0x102a46c8 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46ccu64) }
        0x102a46cc => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46d0u64) }
        0x102a46d0 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46d4u64) }
        0x102a46d4 => { vm.decode(0xb148ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46d8u64) }
        0x102a46d8 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46dcu64) }
        0x102a46dc => { vm.decode(0xe3408d75u32); let d = 21i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46e0u64) }
        0x102a46e0 => { vm.decode(0xe000cd75u32); let d = 20i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46e4u64) }
        0x102a46e4 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46e8u64) }
        0x102a46e8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46ecu64) }
        0x102a46ec => { vm.decode(0xa1087d75u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a46f0u64) }
        0x102a46f0 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46f4u64) }
        0x102a46f4 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a46f8u64) }
        0x102a46f8 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a46fcu64) }
        0x102a46fc => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4700u64) }
        0x102a4700 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4704u64) }
        0x102a4704 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4708u64) }
        0x102a4708 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a470cu64) }
        0x102a470c => { vm.decode(0xd25ec1adu32); let d = 22i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4710u64) }
        0x102a4710 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4714u64) }
        0x102a4714 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4718u64) }
        0x102a4718 => { vm.decode(0xf3084d75u32); let d = 24i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a471cu64) }
        0x102a471c => { vm.decode(0xb2405d75u32); let d = 3i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4720u64) }
        0x102a4720 => { vm.decode(0xa000cd75u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4724u64) }
        0x102a4724 => { vm.decode(0xa140ed75u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4728u64) }
        0x102a4728 => { vm.decode(0xf1009d75u32); let d = 22i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a472cu64) }
        0x102a472c => { vm.decode(0xda6e812du32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a4730u64) }
        0x102a4730 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4734u64) }
        0x102a4734 => { vm.decode(0x2225b4f5u32); let d = 16i16; let s = 12i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4738u64) }
        0x102a4738 => { vm.decode(0x62c5b4f5u32); let d = 25i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a473cu64) }
        0x102a473c => { vm.decode(0xa283a4f5u32); let d = 12i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4740u64) }
        0x102a4740 => { vm.decode(0x6a81a4f5u32); let d = 10i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4744u64) }
        0x102a4744 => { vm.decode(0x2361a4f5u32); let d = 1i16; let s = 20i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4748u64) }
        0x102a4748 => { vm.decode(0x6327a4f5u32); let d = 20i16; let s = 22i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a474cu64) }
        0x102a474c => { vm.decode(0xab21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4750u64) }
        0x102a4750 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4754u64) }
        0x102a4754 => { vm.decode(0xeb01a4f5u32); let d = 2i16; let s = 19i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4758u64) }
        0x102a4758 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a475cu64) }
        0x102a475c => { vm.decode(0xaa65b4f5u32); let d = 19i16; let s = 13i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4760u64) }
        0x102a4760 => { vm.decode(0xeae3b4f5u32); let d = 15i16; let s = 15i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4764u64) }
        0x102a4764 => { vm.decode(0x6b01a4f5u32); let d = 2i16; let s = 18i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4768u64) }
        0x102a4768 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a476cu64) }
        0x102a476c => { vm.decode(0xa267a4f5u32); let d = 21i16; let s = 5i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4770u64) }
        0x102a4770 => { vm.decode(0x2a25a4f5u32); let d = 18i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4774u64) }
        0x102a4774 => { vm.decode(0xe2c3a4f5u32); let d = 13i16; let s = 3i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4778u64) }
        0x102a4778 => { vm.decode(0x2b01b4f5u32); let d = 2i16; let s = 24i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a477cu64) }
        0x102a477c => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4780u64) }
        0x102a4780 => { vm.decode(0x45906011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4784u64) }
        0x102a4784 => { vm.decode(0x42494349u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4788u64) }
        0x102a4788 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a478cu64) }
        0x102a478c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4790u64) }
        0x102a4790 => { vm.decode(0x4a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4794u64) }
        0x102a4794 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4798u64) }
        0x102a4798 => { vm.decode(0x208ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a479cu64) }
        0x102a479c => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a47a0u64) }
        0x102a47a0 => { vm.decode(0x42eb4349u32); let d = 3i16; let s = 14i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a47a4u64) }
        0x102a47a4 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a47a8u64) }
        0x102a47a8 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a47acu64) }
        0x102a47ac => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a47b0u64) }
        0x102a47b0 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47b4u64) }
        0x102a47b4 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47b8u64) }
        0x102a47b8 => { vm.decode(0x430d0835u32); let d = 14i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a47bcu64) }
        0x102a47bc => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a47c0u64) }
        0x102a47c0 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a47c4u64) }
        0x102a47c4 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a47c8u64) }
        0x102a47c8 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a47ccu64) }
        0x102a47cc => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47d0u64) }
        0x102a47d0 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47d4u64) }
        0x102a47d4 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a47d8u64) }
        0x102a47d8 => { vm.decode(0x25bc349u32); let d = 4i16; let s = 11i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a47dcu64) }
        0x102a47dc => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a47e0u64) }
        0x102a47e0 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a47e4u64) }
        0x102a47e4 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a47e8u64) }
        0x102a47e8 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47ecu64) }
        0x102a47ec => { vm.decode(0x2886e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a47f0u64) }
        0x102a47f0 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a47f4u64) }
        0x102a47f4 => { vm.decode(0xd104311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a47f8u64) }
        0x102a47f8 => { vm.decode(0x2f8c349u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a47fcu64) }
        0x102a47fc => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4800u64) }
        0x102a4800 => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4804u64) }
        0x102a4804 => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4808u64) }
        0x102a4808 => { vm.decode(0x2082012fu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a480cu64) }
        0x102a480c => { vm.decode(0x2880cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4810u64) }
        0x102a4810 => { vm.decode(0x6102a835u32); let d = 5i16; let s = 5i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4814u64) }
        0x102a4814 => { vm.decode(0x52ca4349u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4818u64) }
        0x102a4818 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a481cu64) }
        0x102a481c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4820u64) }
        0x102a4820 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4824u64) }
        0x102a4824 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4828u64) }
        0x102a4828 => { vm.decode(0x284eb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a482cu64) }
        0x102a482c => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4830u64) }
        0x102a4830 => { vm.decode(0x5907811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4834u64) }
        0x102a4834 => { vm.decode(0x124a5349u32); let d = 24i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4838u64) }
        0x102a4838 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a483cu64) }
        0x102a483c => { vm.decode(0xe1084d75u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4840u64) }
        0x102a4840 => { vm.decode(0x38434f5u32); let d = 24i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4844u64) }
        0x102a4844 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4848u64) }
        0x102a4848 => { vm.decode(0x80eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a484cu64) }
        0x102a484c => { vm.decode(0x16021835u32); let d = 24i16; let s = 8i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4850u64) }
        0x102a4850 => { vm.decode(0x518f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4854u64) }
        0x102a4854 => { vm.decode(0x2ca5349u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4858u64) }
        0x102a4858 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a485cu64) }
        0x102a485c => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4860u64) }
        0x102a4860 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4864u64) }
        0x102a4864 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4868u64) }
        0x102a4868 => { vm.decode(0x280eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a486cu64) }
        0x102a486c => { vm.decode(0x44023835u32); let d = 17i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4870u64) }
        0x102a4870 => { vm.decode(0x46f9d349u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4874u64) }
        0x102a4874 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4878u64) }
        0x102a4878 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a487cu64) }
        0x102a487c => { vm.decode(0xcb6624f5u32); let d = 23i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a4880u64) }
        0x102a4880 => { vm.decode(0x60aa412fu32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4884u64) }
        0x102a4884 => { vm.decode(0x884cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4888u64) }
        0x102a4888 => { vm.decode(0x4206b835u32); let d = 9i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a488cu64) }
        0x102a488c => { vm.decode(0x52f9d349u32); let d = 31i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a4890u64) }
        0x102a4890 => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a4894u64) }
        0x102a4894 => { vm.decode(0xf348fd75u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a4898u64) }
        0x102a4898 => { vm.decode(0xcbe634f5u32); let d = 31i16; let s = 6i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a489cu64) }
        0x102a489c => { vm.decode(0x60aa4b2fu32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48a0u64) }
        0x102a48a0 => { vm.decode(0x288ae36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48a4u64) }
        0x102a48a4 => { vm.decode(0x774db835u32); let d = 31i16; let s = 7i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a48a8u64) }
        0x102a48a8 => { vm.decode(0x4d9ce911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a48acu64) }
        0x102a48ac => { vm.decode(0x4279d349u32); let d = 22i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a48b0u64) }
        0x102a48b0 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a48b4u64) }
        0x102a48b4 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a48b8u64) }
        0x102a48b8 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a48bcu64) }
        0x102a48bc => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48c0u64) }
        0x102a48c0 => { vm.decode(0x288ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48c4u64) }
        0x102a48c4 => { vm.decode(0x254d9835u32); let d = 22i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a48c8u64) }
        0x102a48c8 => { vm.decode(0x4d10c311u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a48ccu64) }
        0x102a48cc => { vm.decode(0x5279d349u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a48d0u64) }
        0x102a48d0 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a48d4u64) }
        0x102a48d4 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a48d8u64) }
        0x102a48d8 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a48dcu64) }
        0x102a48dc => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48e0u64) }
        0x102a48e0 => { vm.decode(0x228ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48e4u64) }
        0x102a48e4 => { vm.decode(0x374d9835u32); let d = 30i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a48e8u64) }
        0x102a48e8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48ecu64) }
        0x102a48ec => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a48f0u64) }
        0x102a48f0 => { vm.decode(0x208e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48f4u64) }
        0x102a48f4 => { vm.decode(0x20a8092fu32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48f8u64) }
        0x102a48f8 => { vm.decode(0xa80cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a48fcu64) }
        0x102a48fc => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4900u64) }
        0x102a4900 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4904u64) }
        0x102a4904 => { vm.decode(0x350d2835u32); let d = 23i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4908u64) }
        0x102a4908 => { vm.decode(0x2082022fu32); let d = 5i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a490cu64) }
        0x102a490c => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4910u64) }
        0x102a4910 => { vm.decode(0x2002432fu32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4914u64) }
        0x102a4914 => { vm.decode(0x62408835u32); let d = 8i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4918u64) }
        0x102a4918 => { vm.decode(0x2008402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a491cu64) }
        0x102a491c => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a4920u64) }
        0x102a4920 => { vm.decode(0x404f2835u32); let d = 3i16; let s = 3i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4924u64) }
        0x102a4924 => { vm.decode(0x2082402fu32); let d = 5i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4928u64) }
        0x102a4928 => { vm.decode(0x45408835u32); let d = 20i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a492cu64) }
        0x102a492c => { vm.decode(0x42859019u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a4930u64) }
        0x102a4930 => { vm.decode(0x1b0855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4934u64) }
        0x102a4934 => { vm.decode(0x44a09875u32); let d = 7i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4938u64) }
        0x102a4938 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a493cu64) }
        0x102a493c => { vm.decode(0xa8032fu32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4940u64) }
        0x102a4940 => { vm.decode(0x2082032fu32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4944u64) }
        0x102a4944 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4948u64) }
        0x102a4948 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a494cu64) }
        0x102a494c => { vm.decode(0x8824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4950u64) }
        0x102a4950 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4954u64) }
        0x102a4954 => { vm.decode(0x2082492fu32); let d = 13i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4958u64) }
        0x102a4958 => { vm.decode(0x88a6bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a495cu64) }
        0x102a495c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4960u64) }
        0x102a4960 => { vm.decode(0x228e6befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4964u64) }
        0x102a4964 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4968u64) }
        0x102a4968 => { vm.decode(0x6880eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a496cu64) }
        0x102a496c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4970u64) }
        0x102a4970 => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4974u64) }
        0x102a4974 => { vm.decode(0x535ca835u32); let d = 15i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4978u64) }
        0x102a4978 => { vm.decode(0x4a88c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a497cu64) }
        0x102a497c => { vm.decode(0x145c8835u32); let d = 18i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4980u64) }
        0x102a4980 => { vm.decode(0x5353a835u32); let d = 13i16; let s = 13i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4984u64) }
        0x102a4984 => { vm.decode(0x1459a835u32); let d = 19i16; let s = 12i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4988u64) }
        0x102a4988 => { vm.decode(0x725f2835u32); let d = 11i16; let s = 11i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a498cu64) }
        0x102a498c => { vm.decode(0x35062835u32); let d = 21i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4990u64) }
        0x102a4990 => { vm.decode(0x6110a835u32); let d = 5i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4994u64) }
        0x102a4994 => { vm.decode(0x2162835u32); let d = 9i16; let s = 2i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4998u64) }
        0x102a4998 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a499cu64) }
        0x102a499c => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49a0u64) }
        0x102a49a0 => { vm.decode(0x459cf011u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a49a4u64) }
        0x102a49a4 => { vm.decode(0x524b4349u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a49a8u64) }
        0x102a49a8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a49acu64) }
        0x102a49ac => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a49b0u64) }
        0x102a49b0 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a49b4u64) }
        0x102a49b4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a49b8u64) }
        0x102a49b8 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49bcu64) }
        0x102a49bc => { vm.decode(0xa8a43afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a49c0u64) }
        0x102a49c0 => { vm.decode(0xa8082fu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a49c4u64) }
        0x102a49c4 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49c8u64) }
        0x102a49c8 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49ccu64) }
        0x102a49cc => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a49d0u64) }
        0x102a49d0 => { vm.decode(0x60099835u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49d4u64) }
        0x102a49d4 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a49d8u64) }
        0x102a49d8 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49dcu64) }
        0x102a49dc => { vm.decode(0x604e1835u32); let d = 2i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49e0u64) }
        0x102a49e0 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a49e4u64) }
        0x102a49e4 => { vm.decode(0x200f1835u32); let d = 2i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49e8u64) }
        0x102a49e8 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a49ecu64) }
        0x102a49ec => { vm.decode(0x274b9835u32); let d = 30i16; let s = 15i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49f0u64) }
        0x102a49f0 => { vm.decode(0x37482835u32); let d = 31i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49f4u64) }
        0x102a49f4 => { vm.decode(0x46059835u32); let d = 24i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49f8u64) }
        0x102a49f8 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a49fcu64) }
        0x102a49fc => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a4a00u64) }
        0x102a4a00 => { vm.decode(0xa78329f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -913i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a4a04u64).wrapping_add((imm * 4) as u64) } else { 0x102a4a04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a4a04u64) }
        0x102a4a04 => { vm.decode(0x6005875u32); let d = 8i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a08u64) }
        0x102a4a08 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a0cu64) }
        0x102a4a0c => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a10u64) }
        0x102a4a10 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a14u64) }
        0x102a4a14 => { vm.decode(0x401e2835u32); let d = 3i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a18u64) }
        0x102a4a18 => { vm.decode(0x205f0835u32); let d = 2i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a1cu64) }
        0x102a4a1c => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a20u64) }
        0x102a4a20 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a24u64) }
        0x102a4a24 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a28u64) }
        0x102a4a28 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a2cu64) }
        0x102a4a2c => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a30u64) }
        0x102a4a30 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a34u64) }
        0x102a4a34 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a38u64) }
        0x102a4a38 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a3cu64) }
        0x102a4a3c => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a40u64) }
        0x102a4a40 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a44u64) }
        0x102a4a44 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a48u64) }
        0x102a4a48 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a4cu64) }
        0x102a4a4c => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a50u64) }
        0x102a4a50 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a54u64) }
        0x102a4a54 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a58u64) }
        0x102a4a58 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a5cu64) }
        0x102a4a5c => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a60u64) }
        0x102a4a60 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a64u64) }
        0x102a4a64 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a68u64) }
        0x102a4a68 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a6cu64) }
        0x102a4a6c => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a70u64) }
        0x102a4a70 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a74u64) }
        0x102a4a74 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a78u64) }
        0x102a4a78 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a7cu64) }
        0x102a4a7c => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a80u64) }
        0x102a4a80 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a84u64) }
        0x102a4a84 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4a88u64) }
        0x102a4a88 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a8cu64) }
        0x102a4a8c => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a90u64) }
        0x102a4a90 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a94u64) }
        0x102a4a94 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4a98u64) }
        0x102a4a98 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4a9cu64) }
        0x102a4a9c => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4aa0u64) }
        0x102a4aa0 => { vm.decode(0x949119u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a4aa4u64) }
        0x102a4aa4 => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4aa8u64) }
        0x102a4aa8 => { vm.decode(0x82c42099u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a4aacu64) }
        0x102a4aac => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a4ab0u64) }
        0x102a4ab0 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4ab4u64) }
        0x102a4ab4 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4ab8u64) }
        0x102a4ab8 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4abcu64) }
        0x102a4abc => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4ac0u64) }
        0x102a4ac0 => { vm.decode(0x2886e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4ac4u64) }
        0x102a4ac4 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4ac8u64) }
        0x102a4ac8 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4accu64) }
        0x102a4acc => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a4ad0u64) }
        0x102a4ad0 => { vm.decode(0xc8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a4ad4u64).wrapping_add((imm * 4) as u64) } else { 0x102a4ad4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a4ad4u64) }
        0x102a4ad4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a4ad8u64) } }
        0x102a4ad8 => { vm.decode(0x10a080a3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102a4adcu64) }
        0x102a4adc => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a4ae0u64) }
        0x102a4ae0 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4ae4u64) }
        0x102a4ae4 => { vm.decode(0x22200f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4ae8u64) }
        0x102a4ae8 => { vm.decode(0xdd9e77b3u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a4aecu64) }
        0x102a4aec => { vm.decode(0xc77ef59u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a4af0u64) }
        0x102a4af0 => { vm.decode(0x42848019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a4af4u64) }
        0x102a4af4 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a4af8u64) }
        0x102a4af8 => { vm.decode(0x2a012fu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4afcu64) }
        0x102a4afc => { vm.decode(0x134443b5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4b00u64) }
        0x102a4b00 => { vm.decode(0x7ddc77b3u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a4b04u64) }
        0x102a4b04 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b08u64) }
        0x102a4b08 => { vm.decode(0x918403b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a4b0cu64) }
        0x102a4b0c => { vm.decode(0xfd9e7733u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a4b10u64) }
        0x102a4b10 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4b14u64) }
        0x102a4b14 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a4b18u64) }
        0x102a4b18 => { vm.decode(0xa7d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a4b1cu64).wrapping_add((imm * 4) as u64) } else { 0x102a4b1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a4b1cu64) }
        0x102a4b1c => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4b20u64) }
        0x102a4b20 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b24u64) }
        0x102a4b24 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b28u64) }
        0x102a4b28 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4b2cu64) }
        0x102a4b2c => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b30u64) }
        0x102a4b30 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4b34u64) }
        0x102a4b34 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b38u64) }
        0x102a4b38 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b3cu64) }
        0x102a4b3c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a4b40u64) }
        0x102a4b40 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a4b44u64) }
        0x102a4b44 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b48u64) }
        0x102a4b48 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a4b4cu64) }
        0x102a4b4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a4b50u64) } }
        0x102a4b50 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a4b54u64) }
        0x102a90c4 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a90c8u64) }
        0x102a90c8 => { vm.decode(0xa80084f5u32); let d = 2i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a90ccu64) }
        0x102a90cc => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a90d0u64) }
        0x102a90d0 => { vm.decode(0xaa0024f5u32); let d = 2i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a90d4u64) }
        0x102a90d4 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a90d8u64) }
        0x102a90d8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a90dcu64) }
        0x102a90dc => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a90e0u64) }
        0x102a90e0 => { vm.decode(0x10408191u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a90e4u64) }
        0x102a90e4 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a90e8u64) }
        0x102a90e8 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a90ecu64) }
        0x102a90ec => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a90f0u64) }
        0x102a90f0 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a90f4u64) }
        0x102a90f4 => { vm.decode(0x10108019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a90f8u64) }
        0x102a90f8 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a90fcu64) }
        0x102a90fc => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9100u64) }
        0x102a9100 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9104u64) }
        0x102a9104 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9108u64) }
        0x102a9108 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a910cu64) }
        0x102a910c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9110u64) }
        0x102a9110 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9114u64) }
        0x102a9114 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9118u64 + 4);
        vm.wr(37, 0x102a9118u64 + 4);
        Ok(0x102a9118u64) }
        0x102a9118 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a911cu64) }
        0x102a911c => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9120u64) }
        0x102a9120 => { vm.decode(0x21994298u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 50403i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9124u64) }
        0x102a9124 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a9128u64) }
        0x102a9128 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a912cu64) }
        0x102a912c => { vm.decode(0xc8368c59u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12886i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9130u64) }
        0x102a9130 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9134u64) }
        0x102a9134 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9138u64) }
        0x102a9138 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a913cu64) }
        0x102a913c => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9140u64) }
        0x102a9140 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a9144u64) }
        0x102a9144 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9148u64).wrapping_add((imm * 4) as u64) } else { 0x102a9148u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9148u64) }
        0x102a9148 => { vm.decode(0x70499519u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a914cu64) }
        0x102a9154 => { vm.decode(0x8024735u32); let d = 1i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9158u64) }
        0x102a9158 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102a915cu64) }
        0x102a915c => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9160u64) }
        0x102a9160 => { vm.decode(0x64104675u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9164u64) }
        0x102a9164 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9168u64) }
        0x102a9168 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a916cu64) }
        0x102a916c => { vm.decode(0x6b2114f5u32); let d = 2i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9170u64) }
        0x102a9170 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a9174u64) }
        0x102a9174 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9178u64) }
        0x102a9178 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a917cu64) }
        0x102a917c => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a9180u64) }
        0x102a9180 => { vm.decode(0x4848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 39i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9184u64).wrapping_add((imm * 4) as u64) } else { 0x102a9184u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9184u64) }
        0x102a9184 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9188u64) } }
        0x102a9188 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a918cu64) }
        0x102a918c => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9190u64) }
        0x102a9190 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9194u64) }
        0x102a9194 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9198u64) }
        0x102a9198 => { vm.decode(0x9b115430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a919cu64) }
        0x102a919c => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a91a0u64) }
        0x102a91a0 => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a91a4u64) }
        0x102a91a4 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a91a8u64) }
        0x102a91a8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a91acu64) }
        0x102a91ac => { vm.decode(0x14000011u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a91b0u64) }
        0x102a91b0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a91b4u64) }
        0x102a91b4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a91b8u64) }
        0x102a91b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a91bcu64 + 4);
        vm.wr(37, 0x102a91bcu64 + 4);
        Ok(0x102a91bcu64) }
        0x102a91bc => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a91c0u64) }
        0x102a91c0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a91c4u64) }
        0x102a91c4 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a91c8u64) }
        0x102a91c8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a91ccu64) }
        0x102a91cc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a91d0u64) }
        0x102a91d0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a91d4u64).wrapping_add((imm * 4) as u64) } else { 0x102a91d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a91d4u64) }
        0x102a91d4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a91d8u64) }
        0x102a91d8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a91dcu64) }
        0x102a91dc => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a91e0u64) }
        0x102a91e0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a91e4u64) }
        0x102a91e4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a91e8u64) }
        0x102a91e8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a91ecu64).wrapping_add((imm * 4) as u64) } else { 0x102a91ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a91ecu64) }
        0x102a91ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a91f0u64) } }
        0x102a91f0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a91f4u64) }
        0x102a91f4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a91f8u64) }
        0x102a91f8 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a91fcu64).wrapping_add((imm * 4) as u64) } else { 0x102a91fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a91fcu64) }
        0x102a91fc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9200u64) }
        0x102a9200 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9204u64) }
        0x102a9204 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9208u64) }
        0x102a9208 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a920cu64) }
        0x102a920c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9210u64) }
        0x102a9210 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9214u64).wrapping_add((imm * 4) as u64) } else { 0x102a9214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9214u64) }
        0x102a9214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9218u64) } }
        0x102a9218 => { vm.decode(0xa620551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 142804i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a921cu64) }
        0x102a921c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9220u64) } }
        0x102a9220 => { vm.decode(0x9b1015b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9224u64) }
        0x102a9224 => { vm.decode(0x6a8063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9228u64) }
        0x102a9228 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a922cu64) }
        0x102a922c => { vm.decode(0x10001059u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9230u64) }
        0x102a9230 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9234u64) }
        0x102a9234 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9238u64) }
        0x102a9238 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a923cu64) }
        0x102a923c => { vm.decode(0x742d0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9240u64) }
        0x102a9240 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9244u64) }
        0x102a9244 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9248u64) }
        0x102a9248 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a924cu64) }
        0x102a924c => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9250u64) }
        0x102a9250 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9254u64) }
        0x102a9254 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9258u64 + 4);
        vm.wr(37, 0x102a9258u64 + 4);
        Ok(0x102a9258u64) }
        0x102a9258 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a925cu64) }
        0x102a925c => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9260u64) }
        0x102a9260 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9264u64) }
        0x102a9264 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9268u64) }
        0x102a9268 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a926cu64) }
        0x102a926c => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9270u64) }
        0x102a9270 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9274u64) }
        0x102a9274 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9278u64) }
        0x102a9278 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a927cu64) }
        0x102a927c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9280u64 + 4);
        vm.wr(37, 0x102a9280u64 + 4);
        Ok(0x102a9280u64) }
        0x102a9280 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9284u64) }
        0x102a9284 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9288u64) }
        0x102a9288 => { vm.decode(0xc8368cd9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12885i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a928cu64) }
        0x102a928c => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9290u64) }
        0x102a9290 => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9294u64) }
        0x102a9294 => { vm.decode(0x12108019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9298u64) }
        0x102a9298 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a929cu64) }
        0x102a929c => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a92a0u64) }
        0x102a92a0 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a92a4u64) }
        0x102a92a4 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a92a8u64).wrapping_add((imm * 4) as u64) } else { 0x102a92a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a92a8u64) }
        0x102a92a8 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a92acu64) }
        0x102a92b4 => { vm.decode(0xa424735u32); let d = 1i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a92b8u64) }
        0x102a92b8 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102a92bcu64) }
        0x102a92bc => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a92c0u64) }
        0x102a92c0 => { vm.decode(0x74114675u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a92c4u64) }
        0x102a92c4 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a92c8u64) }
        0x102a92c8 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a92ccu64) }
        0x102a92cc => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a92d0u64) }
        0x102a92d0 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a92d4u64) }
        0x102a92d4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a92d8u64) }
        0x102a92d8 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a92dcu64) }
        0x102a92dc => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a92e0u64) }
        0x102a92e0 => { vm.decode(0x4840827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 38i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a92e4u64).wrapping_add((imm * 4) as u64) } else { 0x102a92e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a92e4u64) }
        0x102a92e4 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a92e8u64) }
        0x102a92e8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a92ecu64) }
        0x102a92ec => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a92f0u64) }
        0x102a92f0 => { vm.decode(0x9b195430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a92f4u64) }
        0x102a92f4 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a92f8u64) }
        0x102a92f8 => { vm.decode(0x8b111530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a92fcu64) }
        0x102a92fc => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9300u64) }
        0x102a9300 => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a9304u64) }
        0x102a9304 => { vm.decode(0x416114f5u32); let d = 1i16; let s = 16i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9308u64) }
        0x102a9308 => { vm.decode(0x5c000011u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a930cu64) }
        0x102a930c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9310u64) }
        0x102a9310 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9314u64) }
        0x102a9314 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9318u64 + 4);
        vm.wr(37, 0x102a9318u64 + 4);
        Ok(0x102a9318u64) }
        0x102a9318 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a931cu64) }
        0x102a931c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a9320u64) }
        0x102a9320 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9324u64) }
        0x102a9324 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9328u64) }
        0x102a9328 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a932cu64) }
        0x102a932c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9330u64).wrapping_add((imm * 4) as u64) } else { 0x102a9330u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9330u64) }
        0x102a9330 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9334u64) }
        0x102a9334 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9338u64) }
        0x102a9338 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a933cu64) }
        0x102a933c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9340u64) }
        0x102a9340 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9344u64) }
        0x102a9344 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9348u64).wrapping_add((imm * 4) as u64) } else { 0x102a9348u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9348u64) }
        0x102a9348 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a934cu64) } }
        0x102a934c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9350u64) }
        0x102a9350 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9354u64) }
        0x102a9354 => { vm.decode(0x739cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9358u64).wrapping_add((imm * 4) as u64) } else { 0x102a9358u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9358u64) }
        0x102a9358 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a935cu64) }
        0x102a935c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9360u64) }
        0x102a9360 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9364u64) }
        0x102a9364 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9368u64) }
        0x102a9368 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a936cu64) }
        0x102a936c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9370u64).wrapping_add((imm * 4) as u64) } else { 0x102a9370u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9370u64) }
        0x102a9370 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9374u64) } }
        0x102a9374 => { vm.decode(0x2780551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 143156i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a9378u64) }
        0x102a9378 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a937cu64) } }
        0x102a937c => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9380u64) }
        0x102a9380 => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9384u64) }
        0x102a9384 => { vm.decode(0x40008119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9388u64) }
        0x102a9388 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a938cu64) }
        0x102a938c => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9390u64) }
        0x102a9390 => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9394u64) }
        0x102a9394 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9398u64) }
        0x102a9398 => { vm.decode(0x12008019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a939cu64) }
        0x102a939c => { vm.decode(0x604b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a93a0u64) }
        0x102a93a0 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a93a4u64) }
        0x102a93a4 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a93a8u64) }
        0x102a93a8 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a93acu64) }
        0x102a93ac => { vm.decode(0x8019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a93b0u64) }
        0x102a93b0 => { vm.decode(0x8a020071u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a93b4u64).wrapping_add((imm * 4) as u64) } else { 0x102a93b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a93b4u64) }
        0x102a93b4 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a93b8u64) }
        0x102a93e0 => { vm.decode(0x4c0400a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 416i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a93e4u64).wrapping_add((imm * 4) as u64) } else { 0x102a93e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a93e4u64) }
        0x102a93e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a93e8u64) } }
        0x102a93e8 => { vm.decode(0x12404171u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 196i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a93ecu64).wrapping_add((imm * 4) as u64) } else { 0x102a93ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a93ecu64) }
        0x102a93ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a93f0u64) } }
        0x102a93f0 => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a93f4u64) }
        0x102a93f4 => { vm.decode(0x2420131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 193i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a93f8u64).wrapping_add((imm * 4) as u64) } else { 0x102a93f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a93f8u64) }
        0x102a93f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a93fcu64) } }
        0x102a96fc => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9700u64) }
        0x102a9700 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9704u64) }
        0x102a9704 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9708u64) }
        0x102a9708 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a970cu64) }
        0x102a970c => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9710u64) }
        0x102a9710 => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9714u64) }
        0x102a9714 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9718u64) }
        0x102a9718 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a971cu64) }
        0x102a971c => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9720u64) }
        0x102a9720 => { vm.decode(0xc1422b75u32); let d = 16i16; let s = 20i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102a9724u64) }
        0x102a9724 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a9728u64) }
        0x102a9728 => { vm.decode(0x24040e35u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x102a972cu64) }
        0x102a972c => { vm.decode(0x840042f5u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102a9730u64) }
        0x102a9730 => { vm.decode(0x8b614909u32); let d = 18i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a9734u64) }
        0x102a9734 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9738u64) }
        0x102a9738 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a973cu64) }
        0x102a973c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9740u64) }
        0x102a9740 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9744u64) }
        0x102a9744 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9748u64 + 4);
        vm.wr(37, 0x102a9748u64 + 4);
        Ok(0x102a9748u64) }
        0x102a9748 => { vm.decode(0xc009875u32); let d = 17i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a974cu64) }
        0x102a974c => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9750u64) }
        0x102a9750 => { vm.decode(0x6b4504f5u32); let d = 19i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9754u64) }
        0x102a9754 => { vm.decode(0x9b009530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9758u64) }
        0x102a9758 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a975cu64) }
        0x102a975c => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9760u64) }
        0x102a9760 => { vm.decode(0xf6b37cf1u32); let d = 20i16; let s = 21i16; let s2 = -1i16; let imm = -225i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9764u64).wrapping_add((imm * 4) as u64) } else { 0x102a9764u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9764u64) }
        0x102a9764 => { vm.decode(0xa8208b35u32); let d = 3i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9768u64) }
        0x102a9a64 => { vm.decode(0x9b2054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9a68u64) }
        0x102a9a68 => { vm.decode(0x6a8863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9a6cu64) }
        0x102a9a6c => { vm.decode(0x2824412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9a70u64) }
        0x102a9a70 => { vm.decode(0x10000119u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9a74u64) }
        0x102a9a74 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9a78u64) }
        0x102a9a78 => { vm.decode(0x28843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9a7cu64) }
        0x102a9a7c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9a80u64) }
        0x102a9a80 => { vm.decode(0x740f1119u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9a84u64) }
        0x102a9a84 => { vm.decode(0x8b1115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9a88u64) }
        0x102a9a88 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9a8cu64) }
        0x102a9a8c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9a90u64) }
        0x102a9a90 => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9a94u64) }
        0x102a9a94 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9a98u64) }
        0x102a9a98 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9a9cu64 + 4);
        vm.wr(37, 0x102a9a9cu64 + 4);
        Ok(0x102a9a9cu64) }
        0x102a9a9c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9aa0u64) }
        0x102a9aa0 => { vm.decode(0x2220416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9aa4u64) }
        0x102a9aa4 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9aa8u64) }
        0x102a9aa8 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9aacu64) }
        0x102a9aac => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9ab0u64) }
        0x102a9ab0 => { vm.decode(0x8b2114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9ab4u64) }
        0x102a9ab4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9ab8u64) }
        0x102a9ab8 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9abcu64) }
        0x102a9abc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9ac0u64) }
        0x102a9ac0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9ac4u64) }
        0x102a9ac4 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9ac8u64) }
        0x102a9ac8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9accu64 + 4);
        vm.wr(37, 0x102a9accu64 + 4);
        Ok(0x102a9accu64) }
        0x102a9acc => { vm.decode(0x8001875u32); let d = 16i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9ad0u64) }
        0x102a9ad0 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9ad4u64) }
        0x102a9ad4 => { vm.decode(0x4836ac59u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12884i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9ad8u64) }
        0x102a9ad8 => { vm.decode(0x12048019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9adcu64) }
        0x102a9adc => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9ae0u64) }
        0x102a9ae0 => { vm.decode(0x8a0604f5u32); let d = 22i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9ae4u64) }
        0x102a9ae4 => { vm.decode(0x760d0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9ae8u64) }
        0x102a9ae8 => { vm.decode(0x26000486u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a9aecu64) }
        0x102a9aec => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9af0u64).wrapping_add((imm * 4) as u64) } else { 0x102a9af0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9af0u64) }
        0x102a9af0 => { vm.decode(0x2112236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9af4u64) }
        0x102a9afc => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b00u64) }
        0x102a9b00 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102a9b04u64) }
        0x102a9b04 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9b08u64) }
        0x102a9b08 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b0cu64) }
        0x102a9b0c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9b10u64) }
        0x102a9b10 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9b14u64) }
        0x102a9b14 => { vm.decode(0x492114f5u32); let d = 2i16; let s = 16i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9b18u64) }
        0x102a9b18 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9b1cu64) }
        0x102a9b1c => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b20u64) }
        0x102a9b20 => { vm.decode(0x26000486u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b24u64) }
        0x102a9b24 => { vm.decode(0x4848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 39i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9b28u64).wrapping_add((imm * 4) as u64) } else { 0x102a9b28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9b28u64) }
        0x102a9b28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9b2cu64) } }
        0x102a9b2c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9b30u64) }
        0x102a9b30 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9b34u64) }
        0x102a9b34 => { vm.decode(0x9b195430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9b38u64) }
        0x102a9b38 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9b3cu64) }
        0x102a9b3c => { vm.decode(0x8b115530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9b40u64) }
        0x102a9b40 => { vm.decode(0xc1c50bb5u32); let d = 30i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b44u64) }
        0x102a9b44 => { vm.decode(0x436114f5u32); let d = 1i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9b48u64) }
        0x102a9b48 => { vm.decode(0x2004023eu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a9b4cu64) }
        0x102a9b4c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9b50u64) }
        0x102a9b50 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9b54u64) }
        0x102a9b54 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9b58u64 + 4);
        vm.wr(37, 0x102a9b58u64 + 4);
        Ok(0x102a9b58u64) }
        0x102a9b58 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9b5cu64) }
        0x102a9b5c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b60u64) }
        0x102a9b60 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b64u64) }
        0x102a9b64 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b68u64) }
        0x102a9b68 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9b6cu64) }
        0x102a9b6c => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9b70u64).wrapping_add((imm * 4) as u64) } else { 0x102a9b70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9b70u64) }
        0x102a9b70 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b74u64) }
        0x102a9b74 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b78u64) }
        0x102a9b78 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a9b7cu64) }
        0x102a9b7c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9b80u64) }
        0x102a9b80 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9b84u64) }
        0x102a9b84 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9b88u64) }
        0x102a9b88 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9b8cu64).wrapping_add((imm * 4) as u64) } else { 0x102a9b8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9b8cu64) }
        0x102a9b8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9b90u64) } }
        0x102a9b90 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9b94u64) }
        0x102a9b94 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9b98u64) }
        0x102a9b98 => { vm.decode(0x731c36a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9b9cu64).wrapping_add((imm * 4) as u64) } else { 0x102a9b9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9b9cu64) }
        0x102a9b9c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9ba0u64) }
        0x102a9ba0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9ba4u64) }
        0x102a9ba4 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a9ba8u64) }
        0x102a9ba8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a9bacu64) }
        0x102a9bac => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9bb0u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
