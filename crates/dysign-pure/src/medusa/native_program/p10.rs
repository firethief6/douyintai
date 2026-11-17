#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x1029ca70 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca74u64) }
        0x1029ca74 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca78u64) }
        0x1029ca78 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca7cu64) }
        0x1029ca7c => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca80u64) }
        0x1029ca80 => { vm.decode(0x2088e36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca84u64) }
        0x1029ca84 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca88u64) }
        0x1029ca88 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca8cu64) }
        0x1029ca8c => { vm.decode(0xe100dd75u32); let d = 22i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca90u64) }
        0x1029ca90 => { vm.decode(0xb3089d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca94u64) }
        0x1029ca94 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca98u64) }
        0x1029ca98 => { vm.decode(0xe340ad75u32); let d = 21i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca9cu64) }
        0x1029ca9c => { vm.decode(0xe040dd75u32); let d = 23i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029caa0u64) }
        0x1029caa0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029caa4u64) }
        0x1029caa4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029caa8u64) }
        0x1029caa8 => { vm.decode(0xa1487d75u32); let d = 11i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029caacu64) }
        0x1029caac => { vm.decode(0x208c43afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cab0u64) }
        0x1029cab0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029cab4u64) }
        0x1029cab4 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cab8u64) }
        0x1029cab8 => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029cabcu64) }
        0x1029cabc => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cac0u64) }
        0x1029cac0 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029cac4u64) }
        0x1029cac4 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cac8u64) }
        0x1029cac8 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029caccu64) }
        0x1029cacc => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cad0u64) }
        0x1029cad0 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029cad4u64) }
        0x1029cad4 => { vm.decode(0xb348dd75u32); let d = 15i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cad8u64) }
        0x1029cad8 => { vm.decode(0xa0405d75u32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cadcu64) }
        0x1029cadc => { vm.decode(0xe1486d75u32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cae0u64) }
        0x1029cae0 => { vm.decode(0xa100cd75u32); let d = 4i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cae4u64) }
        0x1029cae4 => { vm.decode(0xf1089d75u32); let d = 30i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cae8u64) }
        0x1029cae8 => { vm.decode(0xda6ed16du32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029caecu64) }
        0x1029caec => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029caf0u64) }
        0x1029caf0 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029caf4u64) }
        0x1029caf4 => { vm.decode(0xb3054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029caf8u64) }
        0x1029caf8 => { vm.decode(0xe08434f5u32); let d = 24i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cafcu64) }
        0x1029cafc => { vm.decode(0xa08224f5u32); let d = 12i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb00u64) }
        0x1029cb00 => { vm.decode(0x68c024f5u32); let d = 11i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb04u64) }
        0x1029cb04 => { vm.decode(0xe16024f5u32); let d = 1i16; let s = 5i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb08u64) }
        0x1029cb08 => { vm.decode(0x696634f5u32); let d = 23i16; let s = 5i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb0cu64) }
        0x1029cb0c => { vm.decode(0xa92024f5u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb10u64) }
        0x1029cb10 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cb14u64) }
        0x1029cb14 => { vm.decode(0xe90024f5u32); let d = 2i16; let s = 5i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb18u64) }
        0x1029cb18 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cb1cu64) }
        0x1029cb1c => { vm.decode(0x682034f5u32); let d = 2i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb20u64) }
        0x1029cb20 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cb24u64) }
        0x1029cb24 => { vm.decode(0x696424f5u32); let d = 19i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb28u64) }
        0x1029cb28 => { vm.decode(0x690024f5u32); let d = 2i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb2cu64) }
        0x1029cb2c => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cb30u64) }
        0x1029cb30 => { vm.decode(0x282424f5u32); let d = 18i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb34u64) }
        0x1029cb34 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb38u64) }
        0x1029cb38 => { vm.decode(0xe88224f5u32); let d = 14i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb3cu64) }
        0x1029cb3c => { vm.decode(0xe82034f5u32); let d = 2i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb40u64) }
        0x1029cb40 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cb44u64) }
        0x1029cb44 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cb48u64) }
        0x1029cb48 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cb4cu64) }
        0x1029cb4c => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cb50u64) }
        0x1029cb50 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb54u64) }
        0x1029cb54 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb58u64) }
        0x1029cb58 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb5cu64) }
        0x1029cb5c => { vm.decode(0x474c2835u32); let d = 31i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cb60u64) }
        0x1029cb60 => { vm.decode(0x46d84349u32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cb64u64) }
        0x1029cb64 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cb68u64) }
        0x1029cb68 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cb6cu64) }
        0x1029cb6c => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb70u64) }
        0x1029cb70 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb74u64) }
        0x1029cb74 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb78u64) }
        0x1029cb78 => { vm.decode(0x43442835u32); let d = 13i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cb7cu64) }
        0x1029cb7c => { vm.decode(0x46e84349u32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cb80u64) }
        0x1029cb80 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cb84u64) }
        0x1029cb84 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cb88u64) }
        0x1029cb88 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cb8cu64) }
        0x1029cb8c => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb90u64) }
        0x1029cb90 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cb94u64) }
        0x1029cb94 => { vm.decode(0x404c2835u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cb98u64) }
        0x1029cb98 => { vm.decode(0x24bc349u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cb9cu64) }
        0x1029cb9c => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cba0u64) }
        0x1029cba0 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cba4u64) }
        0x1029cba4 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cba8u64) }
        0x1029cba8 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbacu64) }
        0x1029cbac => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbb0u64) }
        0x1029cbb0 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cbb4u64) }
        0x1029cbb4 => { vm.decode(0x45104211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cbb8u64) }
        0x1029cbb8 => { vm.decode(0x52494349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cbbcu64) }
        0x1029cbbc => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cbc0u64) }
        0x1029cbc0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cbc4u64) }
        0x1029cbc4 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cbc8u64) }
        0x1029cbc8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbccu64) }
        0x1029cbcc => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbd0u64) }
        0x1029cbd0 => { vm.decode(0x124c0835u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cbd4u64) }
        0x1029cbd4 => { vm.decode(0x52cac349u32); let d = 15i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cbd8u64) }
        0x1029cbd8 => { vm.decode(0xfa2fc909u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cbdcu64) }
        0x1029cbdc => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cbe0u64) }
        0x1029cbe0 => { vm.decode(0xcae234f5u32); let d = 15i16; let s = 6i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cbe4u64) }
        0x1029cbe4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbe8u64) }
        0x1029cbe8 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cbecu64) }
        0x1029cbec => { vm.decode(0x534ca835u32); let d = 15i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cbf0u64) }
        0x1029cbf0 => { vm.decode(0x45906811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cbf4u64) }
        0x1029cbf4 => { vm.decode(0x2c95349u32); let d = 17i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cbf8u64) }
        0x1029cbf8 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cbfcu64) }
        0x1029cbfc => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc00u64) }
        0x1029cc00 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cc04u64) }
        0x1029cc04 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc08u64) }
        0x1029cc08 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc0cu64) }
        0x1029cc0c => { vm.decode(0x44403835u32); let d = 17i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cc10u64) }
        0x1029cc10 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cc14u64) }
        0x1029cc14 => { vm.decode(0x249d349u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cc18u64) }
        0x1029cc18 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cc1cu64) }
        0x1029cc1c => { vm.decode(0xf000cd75u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc20u64) }
        0x1029cc20 => { vm.decode(0x32624f5u32); let d = 20i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cc24u64) }
        0x1029cc24 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc28u64) }
        0x1029cc28 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc2cu64) }
        0x1029cc2c => { vm.decode(0x5409835u32); let d = 20i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cc30u64) }
        0x1029cc30 => { vm.decode(0x4598e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cc34u64) }
        0x1029cc34 => { vm.decode(0x5249d349u32); let d = 30i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cc38u64) }
        0x1029cc38 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cc3cu64) }
        0x1029cc3c => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc40u64) }
        0x1029cc40 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cc44u64) }
        0x1029cc44 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc48u64) }
        0x1029cc48 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc4cu64) }
        0x1029cc4c => { vm.decode(0x1244b835u32); let d = 9i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cc50u64) }
        0x1029cc50 => { vm.decode(0x2f9d349u32); let d = 21i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cc54u64) }
        0x1029cc54 => { vm.decode(0xc967c909u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cc58u64) }
        0x1029cc58 => { vm.decode(0xf040ed75u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc5cu64) }
        0x1029cc5c => { vm.decode(0x836624f5u32); let d = 21i16; let s = 6i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cc60u64) }
        0x1029cc60 => { vm.decode(0x6082412fu32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc64u64) }
        0x1029cc64 => { vm.decode(0xa88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc68u64) }
        0x1029cc68 => { vm.decode(0x4540b835u32); let d = 21i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cc6cu64) }
        0x1029cc6c => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cc70u64) }
        0x1029cc70 => { vm.decode(0x4249d349u32); let d = 22i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cc74u64) }
        0x1029cc74 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cc78u64) }
        0x1029cc78 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc7cu64) }
        0x1029cc7c => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cc80u64) }
        0x1029cc80 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc84u64) }
        0x1029cc84 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cc88u64) }
        0x1029cc88 => { vm.decode(0x54c9835u32); let d = 22i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cc8cu64) }
        0x1029cc8c => { vm.decode(0x4510c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cc90u64) }
        0x1029cc90 => { vm.decode(0x2495349u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cc94u64) }
        0x1029cc94 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cc98u64) }
        0x1029cc98 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cc9cu64) }
        0x1029cc9c => { vm.decode(0x30424f5u32); let d = 16i16; let s = 6i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cca0u64) }
        0x1029cca0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cca4u64) }
        0x1029cca4 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cca8u64) }
        0x1029cca8 => { vm.decode(0x4401835u32); let d = 16i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccacu64) }
        0x1029ccac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccb0u64) }
        0x1029ccb0 => { vm.decode(0x60432835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccb4u64) }
        0x1029ccb4 => { vm.decode(0x208663afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccb8u64) }
        0x1029ccb8 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccbcu64) }
        0x1029ccbc => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccc0u64) }
        0x1029ccc0 => { vm.decode(0x514ca835u32); let d = 7i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccc4u64) }
        0x1029ccc4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccc8u64) }
        0x1029ccc8 => { vm.decode(0x574e0835u32); let d = 30i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccccu64) }
        0x1029cccc => { vm.decode(0x880a2fu32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccd0u64) }
        0x1029ccd0 => { vm.decode(0x11050835u32); let d = 4i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccd4u64) }
        0x1029ccd4 => { vm.decode(0x84a2fu32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccd8u64) }
        0x1029ccd8 => { vm.decode(0x32440835u32); let d = 8i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccdcu64) }
        0x1029ccdc => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cce0u64) }
        0x1029cce0 => { vm.decode(0x2028002fu32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cce4u64) }
        0x1029cce4 => { vm.decode(0x600f2835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cce8u64) }
        0x1029cce8 => { vm.decode(0xa8492fu32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ccecu64) }
        0x1029ccec => { vm.decode(0x355f2835u32); let d = 23i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ccf0u64) }
        0x1029ccf0 => { vm.decode(0x42058019u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029ccf4u64) }
        0x1029ccf4 => { vm.decode(0x1b0854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ccf8u64) }
        0x1029ccf8 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ccfcu64) }
        0x1029ccfc => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029cd00u64) }
        0x1029cd00 => { vm.decode(0x621d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029cd04u64) }
        0x1029cd04 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd08u64) }
        0x1029cd08 => { vm.decode(0xa80a2fu32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd0cu64) }
        0x1029cd0c => { vm.decode(0x208643efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd10u64) }
        0x1029cd10 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd14u64) }
        0x1029cd14 => { vm.decode(0x24b2fu32); let d = 12i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd18u64) }
        0x1029cd18 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd1cu64) }
        0x1029cd1c => { vm.decode(0x2082092fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd20u64) }
        0x1029cd20 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd24u64) }
        0x1029cd24 => { vm.decode(0x288263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd28u64) }
        0x1029cd28 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd2cu64) }
        0x1029cd2c => { vm.decode(0x228663efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd30u64) }
        0x1029cd30 => { vm.decode(0x4080092fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd34u64) }
        0x1029cd34 => { vm.decode(0x2882e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd38u64) }
        0x1029cd38 => { vm.decode(0x6080092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd3cu64) }
        0x1029cd3c => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd40u64) }
        0x1029cd40 => { vm.decode(0x36011835u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd44u64) }
        0x1029cd44 => { vm.decode(0x2a82e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd48u64) }
        0x1029cd48 => { vm.decode(0x740d8835u32); let d = 18i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd4cu64) }
        0x1029cd4c => { vm.decode(0x131c8835u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd50u64) }
        0x1029cd50 => { vm.decode(0x5459a835u32); let d = 19i16; let s = 13i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd54u64) }
        0x1029cd54 => { vm.decode(0x33118835u32); let d = 12i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd58u64) }
        0x1029cd58 => { vm.decode(0x74060835u32); let d = 16i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd5cu64) }
        0x1029cd5c => { vm.decode(0x121d0835u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd60u64) }
        0x1029cd60 => { vm.decode(0x22142835u32); let d = 9i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd64u64) }
        0x1029cd64 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd68u64) }
        0x1029cd68 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd6cu64) }
        0x1029cd6c => { vm.decode(0xd9ce111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cd70u64) }
        0x1029cd70 => { vm.decode(0x52f84349u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029cd74u64) }
        0x1029cd74 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cd78u64) }
        0x1029cd78 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cd7cu64) }
        0x1029cd7c => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029cd80u64) }
        0x1029cd80 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd84u64) }
        0x1029cd84 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd88u64) }
        0x1029cd88 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd8cu64) }
        0x1029cd8c => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cd90u64) }
        0x1029cd90 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd94u64) }
        0x1029cd94 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cd98u64) }
        0x1029cd98 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cd9cu64) }
        0x1029cd9c => { vm.decode(0x4a1835u32); let d = 2i16; let s = 10i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cda0u64) }
        0x1029cda0 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cda4u64) }
        0x1029cda4 => { vm.decode(0x400f1835u32); let d = 2i16; let s = 12i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cda8u64) }
        0x1029cda8 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cdacu64) }
        0x1029cdac => { vm.decode(0x3473835u32); let d = 13i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdb0u64) }
        0x1029cdb0 => { vm.decode(0x4e90d875u32); let d = 31i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdb4u64) }
        0x1029cdb4 => { vm.decode(0x4416b835u32); let d = 17i16; let s = 23i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdb8u64) }
        0x1029cdb8 => { vm.decode(0x48a00875u32); let d = 22i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdbcu64) }
        0x1029cdbc => { vm.decode(0x33400835u32); let d = 12i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdc0u64) }
        0x1029cdc0 => { vm.decode(0x110d9835u32); let d = 6i16; let s = 4i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdc4u64) }
        0x1029cdc4 => { vm.decode(0x4680d875u32); let d = 15i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdc8u64) }
        0x1029cdc8 => { vm.decode(0x80acd01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 89024i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1029cdccu64) }
        0x1029cdcc => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdd0u64) }
        0x1029cdd0 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdd4u64) }
        0x1029cdd4 => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdd8u64) }
        0x1029cdd8 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cddcu64) }
        0x1029cddc => { vm.decode(0x404d2835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cde0u64) }
        0x1029cde0 => { vm.decode(0xf0835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cde4u64) }
        0x1029cde4 => { vm.decode(0x60102835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cde8u64) }
        0x1029cde8 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdecu64) }
        0x1029cdec => { vm.decode(0x21038835u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cdf0u64) }
        0x1029cdf0 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdf4u64) }
        0x1029cdf4 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdf8u64) }
        0x1029cdf8 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cdfcu64) }
        0x1029cdfc => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce00u64) }
        0x1029ce00 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce04u64) }
        0x1029ce04 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce08u64) }
        0x1029ce08 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce0cu64) }
        0x1029ce0c => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce10u64) }
        0x1029ce10 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce14u64) }
        0x1029ce14 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce18u64) }
        0x1029ce18 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce1cu64) }
        0x1029ce1c => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce20u64) }
        0x1029ce20 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce24u64) }
        0x1029ce24 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce28u64) }
        0x1029ce28 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce2cu64) }
        0x1029ce2c => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce30u64) }
        0x1029ce30 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce34u64) }
        0x1029ce34 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce38u64) }
        0x1029ce38 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce3cu64) }
        0x1029ce3c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce40u64) }
        0x1029ce40 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce44u64) }
        0x1029ce44 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce48u64) }
        0x1029ce48 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce4cu64) }
        0x1029ce4c => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce50u64) }
        0x1029ce50 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce54u64) }
        0x1029ce54 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce58u64) }
        0x1029ce58 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce5cu64) }
        0x1029ce5c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce60u64) }
        0x1029ce60 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029ce64u64) }
        0x1029ce64 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce68u64) }
        0x1029ce68 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce6cu64) }
        0x1029ce6c => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce70u64) }
        0x1029ce70 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce74u64) }
        0x1029ce74 => { vm.decode(0x288ce3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce78u64) }
        0x1029ce78 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ce7cu64) }
        0x1029ce7c => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce80u64) }
        0x1029ce80 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce84u64) }
        0x1029ce84 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce88u64) }
        0x1029ce88 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ce8cu64) }
        0x1029ce8c => { vm.decode(0x4002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029ce90u64) }
        0x1029ce90 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029ce94u64) }
        0x1029ce94 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ce98u64) }
        0x1029ce98 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029ce9cu64) }
        0x1029ce9c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cea0u64) }
        0x1029cea0 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cea4u64) }
        0x1029cea4 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029cea8u64) }
        0x1029cea8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ceacu64) }
        0x1029ceac => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ceb0u64) }
        0x1029ceb0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ceb4u64) }
        0x1029ceb4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ceb8u64) }
        0x1029ceb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1029cebcu64 + 4);
        vm.wr(37, 0x1029cebcu64 + 4);
        Ok(0x1029cebcu64) }
        0x1029cebc => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029cec0u64) }
        0x1029cec0 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029cec4u64) }
        0x1029cec4 => { vm.decode(0x4828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029cec8u64) }
        0x1029cec8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ceccu64) }
        0x1029cecc => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ced0u64) }
        0x1029ced0 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ced4u64) }
        0x1029ced4 => { vm.decode(0x282e35u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x1029ced8u64) }
        0x1029ced8 => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029cedcu64).wrapping_add((imm * 4) as u64) } else { 0x1029cedcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029cedcu64) }
        0x1029cedc => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cee0u64) }
        0x1029cee0 => { vm.decode(0x2280e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cee4u64) }
        0x1029cf0c => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf10u64) }
        0x1029cf10 => { vm.decode(0x40101019u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029cf14u64) }
        0x1029cf14 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029cf18u64) }
        0x1029cf18 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029cf1cu64) }
        0x1029cf1c => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029cf20u64).wrapping_add((imm * 4) as u64) } else { 0x1029cf20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029cf20u64) }
        0x1029cf20 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029cf24u64) } }
        0x1029cf24 => { vm.decode(0x2024002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf28u64) }
        0x1029cf28 => { vm.decode(0x2060f5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf2cu64) }
        0x1029cf2c => { vm.decode(0x22240f5u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf30u64) }
        0x1029cf30 => { vm.decode(0x42040f5u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf34u64) }
        0x1029cf34 => { vm.decode(0x92c403b5u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf38u64) }
        0x1029cf38 => { vm.decode(0x820013b5u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf3cu64) }
        0x1029cf3c => { vm.decode(0x808413b5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf40u64) }
        0x1029cf40 => { vm.decode(0x802204b3u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf44u64) }
        0x1029cf44 => { vm.decode(0x2304b3u32); let d = 9i16; let s = 2i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf48u64) }
        0x1029cf48 => { vm.decode(0x80210433u32); let d = 8i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf4cu64) }
        0x1029cf4c => { vm.decode(0x20620433u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf50u64) }
        0x1029cf50 => { vm.decode(0xa06000b3u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf54u64) }
        0x1029cf54 => { vm.decode(0x6200b3u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf58u64) }
        0x1029cf58 => { vm.decode(0x80600033u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf5cu64) }
        0x1029cf5c => { vm.decode(0x22220f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029cf60u64) }
        0x1029cf60 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029cf64u64) }
        0x1029cf64 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029cf68u64) }
        0x1029cf68 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029cf6cu64) }
        0x1029cf6c => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029cf70u64).wrapping_add((imm * 4) as u64) } else { 0x1029cf70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029cf70u64) }
        0x1029cf70 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029cf74u64) }
        0x1029cf74 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029cf78u64) }
        0x1029cf78 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf7cu64) }
        0x1029cf7c => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cf80u64) }
        0x1029cf80 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf84u64) }
        0x1029cf84 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cf88u64) }
        0x1029cf88 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf8cu64) }
        0x1029cf8c => { vm.decode(0x19001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cf90u64) }
        0x1029cf90 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf94u64) }
        0x1029cf94 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cf98u64) }
        0x1029cf98 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cf9cu64) }
        0x1029cf9c => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cfa0u64) }
        0x1029cfa0 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cfa4u64) }
        0x1029cfa4 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cfa8u64) }
        0x1029cfa8 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cfacu64) }
        0x1029cfac => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cfb0u64) }
        0x1029cfb0 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cfb4u64) }
        0x1029cfb4 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029cfb8u64) }
        0x1029cfb8 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029cfbcu64) }
        0x1029cfbc => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029cfc0u64).wrapping_add((imm * 4) as u64) } else { 0x1029cfc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029cfc0u64) }
        0x1029cfc0 => { vm.decode(0x652d1019u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029cfc4u64) }
        0x1029cfc4 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029cfc8u64) }
        0x1029cfc8 => { vm.decode(0x200802beu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029cfccu64) }
        0x1029cfcc => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cfd0u64) }
        0x1029cfd0 => { vm.decode(0x90480437u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x1029cfd4u64) }
        0x1029cfd4 => { vm.decode(0x81749935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cfd8u64) }
        0x1029cfd8 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cfdcu64) }
        0x1029cfdc => { vm.decode(0x320a023eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029cfe0u64) }
        0x1029cfe0 => { vm.decode(0x81709935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cfe4u64) }
        0x1029cfe4 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cfe8u64) }
        0x1029cfe8 => { vm.decode(0x320a02beu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029cfecu64) }
        0x1029cfec => { vm.decode(0x81309935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cff0u64) }
        0x1029cff0 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029cff4u64) }
        0x1029cff4 => { vm.decode(0x220a027eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029cff8u64) }
        0x1029cff8 => { vm.decode(0x36a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029cffcu64) }
        0x1029cffc => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d000u64) }
        0x1029d000 => { vm.decode(0x220a02feu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029d004u64) }
        0x1029d004 => { vm.decode(0x34a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d008u64) }
        0x1029d008 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d00cu64) }
        0x1029d00c => { vm.decode(0x41849019u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029d010u64) }
        0x1029d010 => { vm.decode(0x38c0019u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029d014u64) }
        0x1029d014 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029d018u64) }
        0x1029d018 => { vm.decode(0x380a02feu32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029d01cu64) }
        0x1029d01c => { vm.decode(0x1a0a027eu32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029d020u64) }
        0x1029d020 => { vm.decode(0x13289d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d024u64) }
        0x1029d024 => { vm.decode(0x3004a835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d028u64) }
        0x1029d028 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d02cu64) }
        0x1029d02c => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d030u64) }
        0x1029d030 => { vm.decode(0x42204875u32); let d = 10i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d034u64) }
        0x1029d034 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029d038u64) }
        0x1029d038 => { vm.decode(0x27d12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -30i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029d03cu64).wrapping_add((imm * 4) as u64) } else { 0x1029d03cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029d03cu64) }
        0x1029d03c => { vm.decode(0x19389570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d040u64) }
        0x1029d040 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d044u64) }
        0x1029d044 => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d048u64) }
        0x1029d048 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d04cu64) }
        0x1029d04c => { vm.decode(0x6882c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d050u64) }
        0x1029d050 => { vm.decode(0x8100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029d054u64) }
        0x1029d054 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d058u64) }
        0x1029d058 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d05cu64) }
        0x1029d05c => { vm.decode(0x8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d060u64) }
        0x1029d060 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029d064u64) }
        0x1029d064 => { vm.decode(0x28c7bafu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d068u64) }
        0x1029d068 => { vm.decode(0x4886e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d06cu64) }
        0x1029d06c => { vm.decode(0x208c5bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d070u64) }
        0x1029d070 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d074u64) }
        0x1029d074 => { vm.decode(0x867bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d078u64) }
        0x1029d078 => { vm.decode(0x22865bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d07cu64) }
        0x1029d07c => { vm.decode(0x28a5bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d080u64) }
        0x1029d080 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d084u64) }
        0x1029d084 => { vm.decode(0x228a7bafu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d088u64) }
        0x1029d088 => { vm.decode(0x714b0835u32); let d = 6i16; let s = 9i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d08cu64) }
        0x1029d08c => { vm.decode(0xb10d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d090u64) }
        0x1029d090 => { vm.decode(0x414b9835u32); let d = 6i16; let s = 14i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d094u64) }
        0x1029d094 => { vm.decode(0xb3855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d098u64) }
        0x1029d098 => { vm.decode(0x610f1835u32); let d = 6i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d09cu64) }
        0x1029d09c => { vm.decode(0xb08d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0a0u64) }
        0x1029d0a0 => { vm.decode(0x10f1835u32); let d = 6i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0a4u64) }
        0x1029d0a4 => { vm.decode(0xb00d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0a8u64) }
        0x1029d0a8 => { vm.decode(0x274a9835u32); let d = 30i16; let s = 11i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0acu64) }
        0x1029d0ac => { vm.decode(0x474aa835u32); let d = 31i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0b0u64) }
        0x1029d0b0 => { vm.decode(0x193095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0b4u64) }
        0x1029d0b4 => { vm.decode(0x40d29119u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029d0b8u64) }
        0x1029d0b8 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0bcu64) }
        0x1029d0bc => { vm.decode(0x46449835u32); let d = 24i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0c0u64) }
        0x1029d0c0 => { vm.decode(0x2208a835u32); let d = 11i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0c4u64) }
        0x1029d0c4 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029d0c8u64) }
        0x1029d0c8 => { vm.decode(0x420c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 915i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029d0ccu64).wrapping_add((imm * 4) as u64) } else { 0x1029d0ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029d0ccu64) }
        0x1029d0cc => { vm.decode(0x2004875u32); let d = 8i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d0d0u64) }
        0x1029d0d0 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0d4u64) }
        0x1029d0d4 => { vm.decode(0x54a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0d8u64) }
        0x1029d0d8 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0dcu64) }
        0x1029d0dc => { vm.decode(0x9b1055f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0e0u64) }
        0x1029d0e0 => { vm.decode(0x1b3015f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0e4u64) }
        0x1029d0e4 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0e8u64) }
        0x1029d0e8 => { vm.decode(0x5290c0f5u32); let d = 12i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0ecu64) }
        0x1029d0ec => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d0f0u64) }
        0x1029d0f0 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d0f4u64) }
        0x1029d0f4 => { vm.decode(0xb381470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d0f8u64) }
        0x1029d0f8 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d0fcu64) }
        0x1029d0fc => { vm.decode(0x2a8e636fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d100u64) }
        0x1029d100 => { vm.decode(0xa2e234f5u32); let d = 13i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d104u64) }
        0x1029d104 => { vm.decode(0x82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d108u64) }
        0x1029d108 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d10cu64) }
        0x1029d10c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d110u64) }
        0x1029d110 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d114u64) }
        0x1029d114 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d118u64) }
        0x1029d118 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d11cu64) }
        0x1029d11c => { vm.decode(0x2088e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d120u64) }
        0x1029d120 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d124u64) }
        0x1029d124 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d128u64) }
        0x1029d128 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d12cu64) }
        0x1029d12c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d130u64) }
        0x1029d130 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d134u64) }
        0x1029d134 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d138u64) }
        0x1029d138 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d13cu64) }
        0x1029d13c => { vm.decode(0x3692c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d140u64) }
        0x1029d140 => { vm.decode(0x50d2a0f5u32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d144u64) }
        0x1029d144 => { vm.decode(0x220d1122u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d148u64) }
        0x1029d148 => { vm.decode(0x52e000f5u32); let d = 16i16; let s = 5i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d14cu64) }
        0x1029d14c => { vm.decode(0x70c3a0f5u32); let d = 17i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d150u64) }
        0x1029d150 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d154u64) }
        0x1029d154 => { vm.decode(0xa34424f5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d158u64) }
        0x1029d158 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d15cu64) }
        0x1029d15c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d160u64) }
        0x1029d160 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d164u64) }
        0x1029d164 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d168u64) }
        0x1029d168 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d16cu64) }
        0x1029d16c => { vm.decode(0x54e0a0f5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d170u64) }
        0x1029d170 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d174u64) }
        0x1029d174 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d178u64) }
        0x1029d178 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d17cu64) }
        0x1029d17c => { vm.decode(0x62e234f5u32); let d = 13i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d180u64) }
        0x1029d180 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d184u64) }
        0x1029d184 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d188u64) }
        0x1029d188 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d18cu64) }
        0x1029d18c => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d190u64) }
        0x1029d190 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d194u64) }
        0x1029d194 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d198u64) }
        0x1029d198 => { vm.decode(0x44901875u32); let d = 6i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d19cu64) }
        0x1029d19c => { vm.decode(0xab4434f5u32); let d = 19i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1a0u64) }
        0x1029d1a0 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d1a4u64) }
        0x1029d1a4 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d1a8u64) }
        0x1029d1a8 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d1acu64) }
        0x1029d1ac => { vm.decode(0x32d200f5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d1b0u64) }
        0x1029d1b0 => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d1b4u64) }
        0x1029d1b4 => { vm.decode(0xc66cb2adu32); let d = 20i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d1b8u64) }
        0x1029d1b8 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d1bcu64) }
        0x1029d1bc => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d1c0u64) }
        0x1029d1c0 => { vm.decode(0x56a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d1c4u64) }
        0x1029d1c4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d1c8u64) }
        0x1029d1c8 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1ccu64) }
        0x1029d1cc => { vm.decode(0x3603a835u32); let d = 25i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d1d0u64) }
        0x1029d1d0 => { vm.decode(0x50b1e0f5u32); let d = 12i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d1d4u64) }
        0x1029d1d4 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d1d8u64) }
        0x1029d1d8 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d1dcu64) }
        0x1029d1dc => { vm.decode(0x12624f5u32); let d = 20i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1e0u64) }
        0x1029d1e0 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1e4u64) }
        0x1029d1e4 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1e8u64) }
        0x1029d1e8 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d1ecu64) }
        0x1029d1ec => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d1f0u64) }
        0x1029d1f0 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d1f4u64) }
        0x1029d1f4 => { vm.decode(0x33142835u32); let d = 13i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d1f8u64) }
        0x1029d1f8 => { vm.decode(0x3490a0f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d1fcu64) }
        0x1029d1fc => { vm.decode(0x8b1814f0u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d200u64) }
        0x1029d200 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d204u64) }
        0x1029d204 => { vm.decode(0x6a8034f5u32); let d = 10i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d208u64) }
        0x1029d208 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d20cu64) }
        0x1029d20c => { vm.decode(0x50462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d210u64) }
        0x1029d210 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d214u64) }
        0x1029d214 => { vm.decode(0x20a80a2fu32); let d = 11i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d218u64) }
        0x1029d218 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d21cu64) }
        0x1029d21c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d220u64) }
        0x1029d220 => { vm.decode(0x6080402fu32); let d = 17i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d224u64) }
        0x1029d224 => { vm.decode(0x4088022fu32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d228u64) }
        0x1029d228 => { vm.decode(0x24501835u32); let d = 16i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d22cu64) }
        0x1029d22c => { vm.decode(0x70b1e0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d230u64) }
        0x1029d230 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d234u64) }
        0x1029d234 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d238u64) }
        0x1029d238 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d23cu64) }
        0x1029d23c => { vm.decode(0x5007a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d240u64) }
        0x1029d240 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d244u64) }
        0x1029d244 => { vm.decode(0x688a4bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d248u64) }
        0x1029d248 => { vm.decode(0x76b3c0f5u32); let d = 15i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d24cu64) }
        0x1029d24c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d250u64) }
        0x1029d250 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d254u64) }
        0x1029d254 => { vm.decode(0x60a8032fu32); let d = 19i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d258u64) }
        0x1029d258 => { vm.decode(0x43501835u32); let d = 12i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d25cu64) }
        0x1029d25c => { vm.decode(0x54b260f5u32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d260u64) }
        0x1029d260 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d264u64) }
        0x1029d264 => { vm.decode(0xeae234f5u32); let d = 15i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d268u64) }
        0x1029d268 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d26cu64) }
        0x1029d26c => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d270u64) }
        0x1029d270 => { vm.decode(0x20472835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d274u64) }
        0x1029d274 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d278u64) }
        0x1029d278 => { vm.decode(0x534f2835u32); let d = 15i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d27cu64) }
        0x1029d27c => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d280u64) }
        0x1029d280 => { vm.decode(0x14a020f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d284u64) }
        0x1029d284 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d288u64) }
        0x1029d288 => { vm.decode(0x6a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d28cu64) }
        0x1029d28c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d290u64) }
        0x1029d290 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d294u64) }
        0x1029d294 => { vm.decode(0x404c1835u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d298u64) }
        0x1029d298 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d29cu64) }
        0x1029d29c => { vm.decode(0x56a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d2a0u64) }
        0x1029d2a0 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d2a4u64) }
        0x1029d2a4 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d2a8u64) }
        0x1029d2a8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d2acu64) }
        0x1029d2ac => { vm.decode(0x1b301470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d2b0u64) }
        0x1029d2b0 => { vm.decode(0x34b220f5u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d2b4u64) }
        0x1029d2b4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d2b8u64) }
        0x1029d2b8 => { vm.decode(0xeac034f5u32); let d = 11i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d2bcu64) }
        0x1029d2bc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d2c0u64) }
        0x1029d2c0 => { vm.decode(0x25462835u32); let d = 21i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d2c4u64) }
        0x1029d2c4 => { vm.decode(0x569380f5u32); let d = 11i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d2c8u64) }
        0x1029d2c8 => { vm.decode(0x42105875u32); let d = 10i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d2ccu64) }
        0x1029d2cc => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d2d0u64) }
        0x1029d2d0 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d2d4u64) }
        0x1029d2d4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d2d8u64) }
        0x1029d2d8 => { vm.decode(0x549160f5u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d2dcu64) }
        0x1029d2dc => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d2e0u64) }
        0x1029d2e0 => { vm.decode(0x6aa234f5u32); let d = 14i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d2e4u64) }
        0x1029d2e4 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d2e8u64) }
        0x1029d2e8 => { vm.decode(0x524f2835u32); let d = 11i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d2ecu64) }
        0x1029d2ec => { vm.decode(0x16b040f5u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d2f0u64) }
        0x1029d2f0 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d2f4u64) }
        0x1029d2f4 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d2f8u64) }
        0x1029d2f8 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d2fcu64) }
        0x1029d2fc => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d300u64) }
        0x1029d300 => { vm.decode(0x14e020f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d304u64) }
        0x1029d304 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d308u64) }
        0x1029d308 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d30cu64) }
        0x1029d30c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d310u64) }
        0x1029d310 => { vm.decode(0x145ca835u32); let d = 19i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d314u64) }
        0x1029d314 => { vm.decode(0x7691c0f5u32); let d = 14i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d318u64) }
        0x1029d318 => { vm.decode(0x40305875u32); let d = 2i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d31cu64) }
        0x1029d31c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d320u64) }
        0x1029d320 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d324u64) }
        0x1029d324 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d328u64) }
        0x1029d328 => { vm.decode(0x74e1a0f5u32); let d = 18i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d32cu64) }
        0x1029d32c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d330u64) }
        0x1029d330 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d334u64) }
        0x1029d334 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d338u64) }
        0x1029d338 => { vm.decode(0x135c8835u32); let d = 14i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d33cu64) }
        0x1029d33c => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d340u64) }
        0x1029d340 => { vm.decode(0x36c000f5u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d344u64) }
        0x1029d344 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d348u64) }
        0x1029d348 => { vm.decode(0x490424f5u32); let d = 18i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d34cu64) }
        0x1029d34c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d350u64) }
        0x1029d350 => { vm.decode(0x428e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d354u64) }
        0x1029d354 => { vm.decode(0x70c1e0f5u32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d358u64) }
        0x1029d358 => { vm.decode(0xc66c22edu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d35cu64) }
        0x1029d35c => { vm.decode(0x232624f5u32); let d = 20i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d360u64) }
        0x1029d360 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d364u64) }
        0x1029d364 => { vm.decode(0x9b00d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d368u64) }
        0x1029d368 => { vm.decode(0x4153835u32); let d = 17i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d36cu64) }
        0x1029d36c => { vm.decode(0x54e000f5u32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d370u64) }
        0x1029d370 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d374u64) }
        0x1029d374 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d378u64) }
        0x1029d378 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d37cu64) }
        0x1029d37c => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d380u64) }
        0x1029d380 => { vm.decode(0x99289cf0u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d384u64) }
        0x1029d384 => { vm.decode(0x131c63b5u32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d388u64) }
        0x1029d388 => { vm.decode(0xb2855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d38cu64) }
        0x1029d38c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d390u64) }
        0x1029d390 => { vm.decode(0x1b205430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d394u64) }
        0x1029d394 => { vm.decode(0x2282c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d398u64) }
        0x1029d398 => { vm.decode(0x690424f5u32); let d = 18i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d39cu64) }
        0x1029d39c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d3a0u64) }
        0x1029d3a0 => { vm.decode(0x6a84cb6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d3a4u64) }
        0x1029d3a4 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d3a8u64) }
        0x1029d3a8 => { vm.decode(0x16e000f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d3acu64) }
        0x1029d3ac => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d3b0u64) }
        0x1029d3b0 => { vm.decode(0x410624f5u32); let d = 20i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d3b4u64) }
        0x1029d3b4 => { vm.decode(0x34c080f5u32); let d = 18i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d3b8u64) }
        0x1029d3b8 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d3bcu64) }
        0x1029d3bc => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d3c0u64) }
        0x1029d3c0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d3c4u64) }
        0x1029d3c4 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d3c8u64) }
        0x1029d3c8 => { vm.decode(0x250f1835u32); let d = 22i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d3ccu64) }
        0x1029d3cc => { vm.decode(0xb109470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d3d0u64) }
        0x1029d3d0 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d3d4u64) }
        0x1029d3d4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d3d8u64) }
        0x1029d3d8 => { vm.decode(0xaa2434f5u32); let d = 18i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d3dcu64) }
        0x1029d3dc => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d3e0u64) }
        0x1029d3e0 => { vm.decode(0x50b3c0f5u32); let d = 13i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d3e4u64) }
        0x1029d3e4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d3e8u64) }
        0x1029d3e8 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d3ecu64) }
        0x1029d3ec => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d3f0u64) }
        0x1029d3f0 => { vm.decode(0x5010a835u32); let d = 1i16; let s = 13i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d3f4u64) }
        0x1029d3f4 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d3f8u64) }
        0x1029d3f8 => { vm.decode(0x309240f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d3fcu64) }
        0x1029d3fc => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d400u64) }
        0x1029d400 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d404u64) }
        0x1029d404 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d408u64) }
        0x1029d408 => { vm.decode(0x208063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d40cu64) }
        0x1029d40c => { vm.decode(0x72008835u32); let d = 8i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d410u64) }
        0x1029d410 => { vm.decode(0x30b240f5u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d414u64) }
        0x1029d414 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d418u64) }
        0x1029d418 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d41cu64) }
        0x1029d41c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d420u64) }
        0x1029d420 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d424u64) }
        0x1029d424 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d428u64) }
        0x1029d428 => { vm.decode(0x7093c0f5u32); let d = 13i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d42cu64) }
        0x1029d42c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d430u64) }
        0x1029d430 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d434u64) }
        0x1029d434 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d438u64) }
        0x1029d438 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d43cu64) }
        0x1029d43c => { vm.decode(0x7300a835u32); let d = 13i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d440u64) }
        0x1029d440 => { vm.decode(0x309140f5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d444u64) }
        0x1029d444 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d448u64) }
        0x1029d448 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d44cu64) }
        0x1029d44c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d450u64) }
        0x1029d450 => { vm.decode(0x3043a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d454u64) }
        0x1029d454 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d458u64) }
        0x1029d458 => { vm.decode(0x42806befu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d45cu64) }
        0x1029d45c => { vm.decode(0x3492c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d460u64) }
        0x1029d460 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d464u64) }
        0x1029d464 => { vm.decode(0xeae214f5u32); let d = 15i16; let s = 3i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d468u64) }
        0x1029d468 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d46cu64) }
        0x1029d46c => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d470u64) }
        0x1029d470 => { vm.decode(0x730ca835u32); let d = 15i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d474u64) }
        0x1029d474 => { vm.decode(0x50c100f5u32); let d = 16i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d478u64) }
        0x1029d478 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d47cu64) }
        0x1029d47c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d480u64) }
        0x1029d480 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d484u64) }
        0x1029d484 => { vm.decode(0x20113835u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d488u64) }
        0x1029d488 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d48cu64) }
        0x1029d48c => { vm.decode(0x2801875u32); let d = 12i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d490u64) }
        0x1029d490 => { vm.decode(0x148100f5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d494u64) }
        0x1029d494 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d498u64) }
        0x1029d498 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d49cu64) }
        0x1029d49c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4a0u64) }
        0x1029d4a0 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4a4u64) }
        0x1029d4a4 => { vm.decode(0x200c0835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d4a8u64) }
        0x1029d4a8 => { vm.decode(0x228e43afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4acu64) }
        0x1029d4ac => { vm.decode(0x50e100f5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d4b0u64) }
        0x1029d4b0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d4b4u64) }
        0x1029d4b4 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d4b8u64) }
        0x1029d4b8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4bcu64) }
        0x1029d4bc => { vm.decode(0x433835u32); let d = 1i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d4c0u64) }
        0x1029d4c0 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d4c4u64) }
        0x1029d4c4 => { vm.decode(0x70e180f5u32); let d = 16i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d4c8u64) }
        0x1029d4c8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d4ccu64) }
        0x1029d4cc => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d4d0u64) }
        0x1029d4d0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4d4u64) }
        0x1029d4d4 => { vm.decode(0x25503835u32); let d = 21i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d4d8u64) }
        0x1029d4d8 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4dcu64) }
        0x1029d4dc => { vm.decode(0x10e000f5u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d4e0u64) }
        0x1029d4e0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d4e4u64) }
        0x1029d4e4 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d4e8u64) }
        0x1029d4e8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d4ecu64) }
        0x1029d4ec => { vm.decode(0x244a3835u32); let d = 19i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d4f0u64) }
        0x1029d4f0 => { vm.decode(0x4a80c875u32); let d = 31i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d4f4u64) }
        0x1029d4f4 => { vm.decode(0x549200f5u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d4f8u64) }
        0x1029d4f8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d4fcu64) }
        0x1029d4fc => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d500u64) }
        0x1029d500 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d504u64) }
        0x1029d504 => { vm.decode(0x71140835u32); let d = 4i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d508u64) }
        0x1029d508 => { vm.decode(0x938dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d50cu64) }
        0x1029d50c => { vm.decode(0x52bb5u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d510u64) }
        0x1029d510 => { vm.decode(0x24d3022u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d514u64) }
        0x1029d514 => { vm.decode(0x2209875u32); let d = 9i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d518u64) }
        0x1029d518 => { vm.decode(0xb181570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d51cu64) }
        0x1029d51c => { vm.decode(0x401b2d35u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d520u64) }
        0x1029d520 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d524u64) }
        0x1029d524 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d528u64) }
        0x1029d528 => { vm.decode(0x282e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d52cu64) }
        0x1029d52c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d530u64) }
        0x1029d530 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d534u64) }
        0x1029d534 => { vm.decode(0x74160835u32); let d = 16i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d538u64) }
        0x1029d538 => { vm.decode(0xd39c53b5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d53cu64) }
        0x1029d53c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d540u64) }
        0x1029d540 => { vm.decode(0xe8c034f5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d544u64) }
        0x1029d544 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d548u64) }
        0x1029d548 => { vm.decode(0x88ee36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d54cu64) }
        0x1029d54c => { vm.decode(0x554d2835u32); let d = 23i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d550u64) }
        0x1029d550 => { vm.decode(0x6080482fu32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d554u64) }
        0x1029d554 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d558u64) }
        0x1029d558 => { vm.decode(0x51581835u32); let d = 6i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d55cu64) }
        0x1029d55c => { vm.decode(0xb085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d560u64) }
        0x1029d560 => { vm.decode(0x639c73b5u32); let d = 25i16; let s = 30i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d564u64) }
        0x1029d564 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d568u64) }
        0x1029d568 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d56cu64) }
        0x1029d56c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d570u64) }
        0x1029d570 => { vm.decode(0x54591835u32); let d = 18i16; let s = 22i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d574u64) }
        0x1029d574 => { vm.decode(0x28886bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d578u64) }
        0x1029d578 => { vm.decode(0x30f300f5u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d57cu64) }
        0x1029d57c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d580u64) }
        0x1029d580 => { vm.decode(0xab0614f5u32); let d = 22i16; let s = 3i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d584u64) }
        0x1029d584 => { vm.decode(0xe39c73b5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d588u64) }
        0x1029d588 => { vm.decode(0x9b3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d58cu64) }
        0x1029d58c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d590u64) }
        0x1029d590 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d594u64) }
        0x1029d594 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d598u64) }
        0x1029d598 => { vm.decode(0x55021835u32); let d = 20i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d59cu64) }
        0x1029d59c => { vm.decode(0x40b00875u32); let d = 6i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d5a0u64) }
        0x1029d5a0 => { vm.decode(0x721c13b5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d5a4u64) }
        0x1029d5a4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d5a8u64) }
        0x1029d5a8 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d5acu64) }
        0x1029d5ac => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5b0u64) }
        0x1029d5b0 => { vm.decode(0x2a8cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5b4u64) }
        0x1029d5b4 => { vm.decode(0x374c0835u32); let d = 30i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d5b8u64) }
        0x1029d5b8 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5bcu64) }
        0x1029d5bc => { vm.decode(0x229c33b5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d5c0u64) }
        0x1029d5c0 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d5c4u64) }
        0x1029d5c4 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d5c8u64) }
        0x1029d5c8 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5ccu64) }
        0x1029d5cc => { vm.decode(0x700b3835u32); let d = 3i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d5d0u64) }
        0x1029d5d0 => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d5d4u64) }
        0x1029d5d4 => { vm.decode(0x886befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5d8u64) }
        0x1029d5d8 => { vm.decode(0x23dc13b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d5dcu64) }
        0x1029d5dc => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d5e0u64) }
        0x1029d5e0 => { vm.decode(0xa0e234f5u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d5e4u64) }
        0x1029d5e4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5e8u64) }
        0x1029d5e8 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5ecu64) }
        0x1029d5ec => { vm.decode(0x7048a835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d5f0u64) }
        0x1029d5f0 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d5f4u64) }
        0x1029d5f4 => { vm.decode(0x22824befu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d5f8u64) }
        0x1029d5f8 => { vm.decode(0xa29c73b5u32); let d = 25i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d5fcu64) }
        0x1029d5fc => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d600u64) }
        0x1029d600 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d604u64) }
        0x1029d604 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d608u64) }
        0x1029d608 => { vm.decode(0x76433835u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d60cu64) }
        0x1029d60c => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d610u64) }
        0x1029d610 => { vm.decode(0x92dc13b5u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d614u64) }
        0x1029d614 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d618u64) }
        0x1029d618 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d61cu64) }
        0x1029d61c => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d620u64) }
        0x1029d620 => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d624u64) }
        0x1029d624 => { vm.decode(0x7004a835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d628u64) }
        0x1029d628 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d62cu64) }
        0x1029d62c => { vm.decode(0x93dc53b5u32); let d = 15i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d630u64) }
        0x1029d630 => { vm.decode(0x46805875u32); let d = 14i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d634u64) }
        0x1029d634 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d638u64) }
        0x1029d638 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d63cu64) }
        0x1029d63c => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d640u64) }
        0x1029d640 => { vm.decode(0x5044a835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d644u64) }
        0x1029d644 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d648u64) }
        0x1029d648 => { vm.decode(0xb21c03b5u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d64cu64) }
        0x1029d64c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d650u64) }
        0x1029d650 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d654u64) }
        0x1029d654 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d658u64) }
        0x1029d658 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d65cu64) }
        0x1029d65c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d660u64) }
        0x1029d660 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d664u64) }
        0x1029d664 => { vm.decode(0x608043efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d668u64) }
        0x1029d668 => { vm.decode(0xd21c03b5u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d66cu64) }
        0x1029d66c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d670u64) }
        0x1029d670 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d674u64) }
        0x1029d674 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d678u64) }
        0x1029d678 => { vm.decode(0x20542835u32); let d = 1i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d67cu64) }
        0x1029d67c => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d680u64) }
        0x1029d680 => { vm.decode(0x321c43b5u32); let d = 2i16; let s = 12i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d684u64) }
        0x1029d684 => { vm.decode(0x4820c875u32); let d = 19i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d688u64) }
        0x1029d688 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d68cu64) }
        0x1029d68c => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d690u64) }
        0x1029d690 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d694u64) }
        0x1029d694 => { vm.decode(0x20152835u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d698u64) }
        0x1029d698 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d69cu64) }
        0x1029d69c => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6a0u64) }
        0x1029d6a0 => { vm.decode(0x931c43b5u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d6a4u64) }
        0x1029d6a4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d6a8u64) }
        0x1029d6a8 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d6acu64) }
        0x1029d6ac => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6b0u64) }
        0x1029d6b0 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6b4u64) }
        0x1029d6b4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d6b8u64) }
        0x1029d6b8 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d6bcu64) }
        0x1029d6bc => { vm.decode(0x1830bb5u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d6c0u64) }
        0x1029d6c0 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d6c4u64) }
        0x1029d6c4 => { vm.decode(0x30f1022u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d6c8u64) }
        0x1029d6c8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d6ccu64) }
        0x1029d6cc => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d6d0u64) }
        0x1029d6d0 => { vm.decode(0x208cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6d4u64) }
        0x1029d6d4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d6d8u64) }
        0x1029d6d8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6dcu64) }
        0x1029d6dc => { vm.decode(0x142835u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d6e0u64) }
        0x1029d6e0 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d6e4u64) }
        0x1029d6e4 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d6e8u64) }
        0x1029d6e8 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d6ecu64) }
        0x1029d6ec => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d6f0u64) }
        0x1029d6f0 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d6f4u64) }
        0x1029d6f4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6f8u64) }
        0x1029d6f8 => { vm.decode(0x2a8ceb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d6fcu64) }
        0x1029d6fc => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d700u64) }
        0x1029d700 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d704u64) }
        0x1029d704 => { vm.decode(0x28884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d708u64) }
        0x1029d708 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029d70cu64) }
        0x1029d70c => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029d710u64) }
        0x1029d710 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d714u64) }
        0x1029d714 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d718u64) }
        0x1029d718 => { vm.decode(0x230bb5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d71cu64) }
        0x1029d71c => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d720u64) }
        0x1029d720 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d724u64) }
        0x1029d724 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d728u64) }
        0x1029d728 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d72cu64) }
        0x1029d72c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d730u64) }
        0x1029d730 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d734u64) }
        0x1029d734 => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d738u64) }
        0x1029d738 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d73cu64) }
        0x1029d73c => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d740u64) }
        0x1029d740 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d744u64) }
        0x1029d744 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d748u64) }
        0x1029d748 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d74cu64) }
        0x1029d74c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d750u64) }
        0x1029d750 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d754u64) }
        0x1029d754 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d758u64) }
        0x1029d758 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d75cu64) }
        0x1029d75c => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d760u64) }
        0x1029d760 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d764u64) }
        0x1029d764 => { vm.decode(0x208263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d768u64) }
        0x1029d768 => { vm.decode(0x65012835u32); let d = 21i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d76cu64) }
        0x1029d76c => { vm.decode(0x80010bb5u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d770u64) }
        0x1029d770 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d774u64) }
        0x1029d774 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d778u64) }
        0x1029d778 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d77cu64) }
        0x1029d77c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d780u64) }
        0x1029d780 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d784u64) }
        0x1029d784 => { vm.decode(0x45192835u32); let d = 23i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d788u64) }
        0x1029d788 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d78cu64) }
        0x1029d78c => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d790u64) }
        0x1029d790 => { vm.decode(0xa001875u32); let d = 24i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d794u64) }
        0x1029d794 => { vm.decode(0x62357062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d798u64) }
        0x1029d798 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d79cu64) }
        0x1029d79c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d7a0u64) }
        0x1029d7a0 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d7a4u64) }
        0x1029d7a4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7a8u64) }
        0x1029d7a8 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d7acu64) }
        0x1029d7ac => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d7b0u64) }
        0x1029d7b0 => { vm.decode(0x40e12bb5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d7b4u64) }
        0x1029d7b4 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d7b8u64) }
        0x1029d7b8 => { vm.decode(0x21d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d7bcu64) }
        0x1029d7bc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d7c0u64) }
        0x1029d7c0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d7c4u64) }
        0x1029d7c4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d7c8u64) }
        0x1029d7c8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7ccu64) }
        0x1029d7cc => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7d0u64) }
        0x1029d7d0 => { vm.decode(0x64090835u32); let d = 18i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d7d4u64) }
        0x1029d7d4 => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7d8u64) }
        0x1029d7d8 => { vm.decode(0x1840bb5u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d7dcu64) }
        0x1029d7dc => { vm.decode(0x40126d35u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d7e0u64) }
        0x1029d7e0 => { vm.decode(0x42471062u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d7e4u64) }
        0x1029d7e4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d7e8u64) }
        0x1029d7e8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d7ecu64) }
        0x1029d7ec => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d7f0u64) }
        0x1029d7f0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7f4u64) }
        0x1029d7f4 => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d7f8u64) }
        0x1029d7f8 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d7fcu64) }
        0x1029d7fc => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d800u64) }
        0x1029d800 => { vm.decode(0x1472bb5u32); let d = 15i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d804u64) }
        0x1029d804 => { vm.decode(0x60926d35u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d808u64) }
        0x1029d808 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d80cu64) }
        0x1029d80c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d810u64) }
        0x1029d810 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d814u64) }
        0x1029d814 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d818u64) }
        0x1029d818 => { vm.decode(0x65120835u32); let d = 20i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d81cu64) }
        0x1029d81c => { vm.decode(0x41800bb5u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d820u64) }
        0x1029d820 => { vm.decode(0x400b4d35u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d824u64) }
        0x1029d824 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d828u64) }
        0x1029d828 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d82cu64) }
        0x1029d82c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d830u64) }
        0x1029d830 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d834u64) }
        0x1029d834 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d838u64) }
        0x1029d838 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d83cu64) }
        0x1029d83c => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d840u64) }
        0x1029d840 => { vm.decode(0xa70bb5u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d844u64) }
        0x1029d844 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d848u64) }
        0x1029d848 => { vm.decode(0x4480c875u32); let d = 7i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d84cu64) }
        0x1029d84c => { vm.decode(0x63751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d850u64) }
        0x1029d850 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d854u64) }
        0x1029d854 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d858u64) }
        0x1029d858 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d85cu64) }
        0x1029d85c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d860u64) }
        0x1029d860 => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d864u64) }
        0x1029d864 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d868u64) }
        0x1029d868 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d86cu64) }
        0x1029d86c => { vm.decode(0x2a82cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d870u64) }
        0x1029d870 => { vm.decode(0xa62bb5u32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d874u64) }
        0x1029d874 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d878u64) }
        0x1029d878 => { vm.decode(0x43453062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d87cu64) }
        0x1029d87c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d880u64) }
        0x1029d880 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d884u64) }
        0x1029d884 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d888u64) }
        0x1029d888 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d88cu64) }
        0x1029d88c => { vm.decode(0x2284e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d890u64) }
        0x1029d890 => { vm.decode(0x36042835u32); let d = 25i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d894u64) }
        0x1029d894 => { vm.decode(0x408b4d35u32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d898u64) }
        0x1029d898 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d89cu64) }
        0x1029d89c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d8a0u64) }
        0x1029d8a0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d8a4u64) }
        0x1029d8a4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8a8u64) }
        0x1029d8a8 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8acu64) }
        0x1029d8ac => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d8b0u64) }
        0x1029d8b0 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d8b4u64) }
        0x1029d8b4 => { vm.decode(0x232bb5u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d8b8u64) }
        0x1029d8b8 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d8bcu64) }
        0x1029d8bc => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d8c0u64) }
        0x1029d8c0 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d8c4u64) }
        0x1029d8c4 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d8c8u64) }
        0x1029d8c8 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d8ccu64) }
        0x1029d8cc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8d0u64) }
        0x1029d8d0 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8d4u64) }
        0x1029d8d4 => { vm.decode(0x370c2835u32); let d = 31i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d8d8u64) }
        0x1029d8d8 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8dcu64) }
        0x1029d8dc => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d8e0u64) }
        0x1029d8e0 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d8e4u64) }
        0x1029d8e4 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029d8e8u64) }
        0x1029d8e8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d8ecu64) }
        0x1029d8ec => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d8f0u64) }
        0x1029d8f0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d8f4u64) }
        0x1029d8f4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8f8u64) }
        0x1029d8f8 => { vm.decode(0x88ac36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d8fcu64) }
        0x1029d8fc => { vm.decode(0x114d0835u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d900u64) }
        0x1029d900 => { vm.decode(0x9305db0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d904u64) }
        0x1029d904 => { vm.decode(0x524a4349u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d908u64) }
        0x1029d908 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d90cu64) }
        0x1029d90c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d910u64) }
        0x1029d910 => { vm.decode(0xc109875u32); let d = 17i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d914u64) }
        0x1029d914 => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d918u64) }
        0x1029d918 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d91cu64) }
        0x1029d91c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d920u64) }
        0x1029d920 => { vm.decode(0x2884cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d924u64) }
        0x1029d924 => { vm.decode(0x340e2835u32); let d = 19i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d928u64) }
        0x1029d928 => { vm.decode(0x1c52bb5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d92cu64) }
        0x1029d92c => { vm.decode(0x600b6d35u32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d930u64) }
        0x1029d930 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d934u64) }
        0x1029d934 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d938u64) }
        0x1029d938 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d93cu64) }
        0x1029d93c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d940u64) }
        0x1029d940 => { vm.decode(0x2880eb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d944u64) }
        0x1029d944 => { vm.decode(0x350e0835u32); let d = 22i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d948u64) }
        0x1029d948 => { vm.decode(0x2884eb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d94cu64) }
        0x1029d94c => { vm.decode(0x80b2fu32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d950u64) }
        0x1029d950 => { vm.decode(0x32152835u32); let d = 9i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d954u64) }
        0x1029d954 => { vm.decode(0x19209c70u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d958u64) }
        0x1029d958 => { vm.decode(0x504d4349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d95cu64) }
        0x1029d95c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d960u64) }
        0x1029d960 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d964u64) }
        0x1029d964 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d968u64) }
        0x1029d968 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d96cu64) }
        0x1029d96c => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d970u64) }
        0x1029d970 => { vm.decode(0x15442835u32); let d = 21i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d974u64) }
        0x1029d974 => { vm.decode(0x544e4349u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d978u64) }
        0x1029d978 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d97cu64) }
        0x1029d97c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d980u64) }
        0x1029d980 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d984u64) }
        0x1029d984 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d988u64) }
        0x1029d988 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d98cu64) }
        0x1029d98c => { vm.decode(0x16440835u32); let d = 24i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d990u64) }
        0x1029d990 => { vm.decode(0x41c20bb5u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029d994u64) }
        0x1029d994 => { vm.decode(0x608b4d35u32); let d = 10i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029d998u64) }
        0x1029d998 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d99cu64) }
        0x1029d99c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d9a0u64) }
        0x1029d9a0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d9a4u64) }
        0x1029d9a4 => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d9a8u64) }
        0x1029d9a8 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d9acu64) }
        0x1029d9ac => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d9b0u64) }
        0x1029d9b0 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d9b4u64) }
        0x1029d9b4 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d9b8u64) }
        0x1029d9b8 => { vm.decode(0x105c9835u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d9bcu64) }
        0x1029d9bc => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d9c0u64) }
        0x1029d9c0 => { vm.decode(0x407f5349u32); let d = 18i16; let s = 15i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d9c4u64) }
        0x1029d9c4 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d9c8u64) }
        0x1029d9c8 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d9ccu64) }
        0x1029d9cc => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d9d0u64) }
        0x1029d9d0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d9d4u64) }
        0x1029d9d4 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d9d8u64) }
        0x1029d9d8 => { vm.decode(0x4c1835u32); let d = 2i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d9dcu64) }
        0x1029d9dc => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d9e0u64) }
        0x1029d9e0 => { vm.decode(0x406f5349u32); let d = 18i16; let s = 14i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029d9e4u64) }
        0x1029d9e4 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029d9e8u64) }
        0x1029d9e8 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029d9ecu64) }
        0x1029d9ec => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029d9f0u64) }
        0x1029d9f0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029d9f4u64) }
        0x1029d9f4 => { vm.decode(0x205d1835u32); let d = 2i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029d9f8u64) }
        0x1029d9f8 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029d9fcu64) }
        0x1029d9fc => { vm.decode(0x406e5349u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029da00u64) }
        0x1029da00 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029da04u64) }
        0x1029da04 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029da08u64) }
        0x1029da08 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029da0cu64) }
        0x1029da0c => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da10u64) }
        0x1029da10 => { vm.decode(0x1d1835u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029da14u64) }
        0x1029da14 => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029da18u64) }
        0x1029da18 => { vm.decode(0xd105a11u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029da1cu64) }
        0x1029da1c => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029da20u64) }
        0x1029da20 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029da24u64) }
        0x1029da24 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029da28u64) }
        0x1029da28 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029da2cu64) }
        0x1029da2c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da30u64) }
        0x1029da30 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da34u64) }
        0x1029da34 => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029da38u64) }
        0x1029da38 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029da3cu64) }
        0x1029da3c => { vm.decode(0x5d98e911u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029da40u64) }
        0x1029da40 => { vm.decode(0x447d4349u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029da44u64) }
        0x1029da44 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029da48u64) }
        0x1029da48 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029da4cu64) }
        0x1029da4c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029da50u64) }
        0x1029da50 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da54u64) }
        0x1029da54 => { vm.decode(0x201e0835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029da58u64) }
        0x1029da58 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029da5cu64) }
        0x1029da5c => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029da60u64) }
        0x1029da60 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029da64u64) }
        0x1029da64 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029da68u64) }
        0x1029da68 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029da6cu64) }
        0x1029da6c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da70u64) }
        0x1029da70 => { vm.decode(0x2280e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da74u64) }
        0x1029da74 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029da78u64) }
        0x1029da78 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029da7cu64) }
        0x1029da7c => { vm.decode(0x407c4349u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029da80u64) }
        0x1029da80 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029da84u64) }
        0x1029da84 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029da88u64) }
        0x1029da88 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029da8cu64) }
        0x1029da8c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da90u64) }
        0x1029da90 => { vm.decode(0x2a84e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029da94u64) }
        0x1029da94 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029da98u64) }
        0x1029da98 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029da9cu64) }
        0x1029da9c => { vm.decode(0x407d4349u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029daa0u64) }
        0x1029daa0 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029daa4u64) }
        0x1029daa4 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029daa8u64) }
        0x1029daa8 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029daacu64) }
        0x1029daac => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dab0u64) }
        0x1029dab0 => { vm.decode(0x20572835u32); let d = 1i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dab4u64) }
        0x1029dab4 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dab8u64) }
        0x1029dab8 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dabcu64) }
        0x1029dabc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dac0u64) }
        0x1029dac0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dac4u64) }
        0x1029dac4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dac8u64) }
        0x1029dac8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029daccu64) }
        0x1029dacc => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dad0u64) }
        0x1029dad0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dad4u64) }
        0x1029dad4 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dad8u64) }
        0x1029dad8 => { vm.decode(0xdf4349u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dadcu64) }
        0x1029dadc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dae0u64) }
        0x1029dae0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dae4u64) }
        0x1029dae4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dae8u64) }
        0x1029dae8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029daecu64) }
        0x1029daec => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029daf0u64) }
        0x1029daf0 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029daf4u64) }
        0x1029daf4 => { vm.decode(0xd2ca916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029daf8u64) }
        0x1029daf8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dafcu64) }
        0x1029dafc => { vm.decode(0x628ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db00u64) }
        0x1029db00 => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029db04u64) }
        0x1029db04 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db08u64) }
        0x1029db08 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029db0cu64) }
        0x1029db0c => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db10u64) }
        0x1029db10 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029db14u64) }
        0x1029db14 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029db18u64) }
        0x1029db18 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029db1cu64) }
        0x1029db1c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029db20u64) }
        0x1029db20 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029db24u64) }
        0x1029db24 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db28u64) }
        0x1029db28 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db2cu64) }
        0x1029db2c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029db30u64) }
        0x1029db30 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db34u64) }
        0x1029db34 => { vm.decode(0x2fb4349u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029db38u64) }
        0x1029db38 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029db3cu64) }
        0x1029db3c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029db40u64) }
        0x1029db40 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029db44u64) }
        0x1029db44 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db48u64) }
        0x1029db48 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029db4cu64) }
        0x1029db4c => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db50u64) }
        0x1029db50 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029db54u64) }
        0x1029db54 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029db58u64) }
        0x1029db58 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029db5cu64) }
        0x1029db5c => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029db60u64) }
        0x1029db60 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db64u64) }
        0x1029db64 => { vm.decode(0x60112835u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029db68u64) }
        0x1029db68 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db6cu64) }
        0x1029db6c => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029db70u64) }
        0x1029db70 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029db74u64) }
        0x1029db74 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029db78u64) }
        0x1029db78 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029db7cu64) }
        0x1029db7c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029db80u64) }
        0x1029db80 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db84u64) }
        0x1029db84 => { vm.decode(0x2020022fu32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db88u64) }
        0x1029db88 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db8cu64) }
        0x1029db8c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029db90u64) }
        0x1029db90 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029db94u64) }
        0x1029db94 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029db98u64) }
        0x1029db98 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029db9cu64) }
        0x1029db9c => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dba0u64) }
        0x1029dba0 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dba4u64) }
        0x1029dba4 => { vm.decode(0x208c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dba8u64) }
        0x1029dba8 => { vm.decode(0xda5ac12du32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbacu64) }
        0x1029dbac => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbb0u64) }
        0x1029dbb0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbb4u64) }
        0x1029dbb4 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbb8u64) }
        0x1029dbb8 => { vm.decode(0xdacec12du32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbbcu64) }
        0x1029dbbc => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbc0u64) }
        0x1029dbc0 => { vm.decode(0xdadec12du32); let d = 15i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbc4u64) }
        0x1029dbc4 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbc8u64) }
        0x1029dbc8 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbccu64) }
        0x1029dbcc => { vm.decode(0x228c63afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbd0u64) }
        0x1029dbd0 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbd4u64) }
        0x1029dbd4 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dbd8u64) }
        0x1029dbd8 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dbdcu64) }
        0x1029dbdc => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbe0u64) }
        0x1029dbe0 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbe4u64) }
        0x1029dbe4 => { vm.decode(0xb148ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbe8u64) }
        0x1029dbe8 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbecu64) }
        0x1029dbec => { vm.decode(0xe3408d75u32); let d = 21i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbf0u64) }
        0x1029dbf0 => { vm.decode(0xe000cd75u32); let d = 20i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbf4u64) }
        0x1029dbf4 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbf8u64) }
        0x1029dbf8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dbfcu64) }
        0x1029dbfc => { vm.decode(0xa1087d75u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc00u64) }
        0x1029dc00 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dc04u64) }
        0x1029dc04 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc08u64) }
        0x1029dc08 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dc0cu64) }
        0x1029dc0c => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc10u64) }
        0x1029dc10 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dc14u64) }
        0x1029dc14 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc18u64) }
        0x1029dc18 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dc1cu64) }
        0x1029dc1c => { vm.decode(0xd25ec1adu32); let d = 22i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc20u64) }
        0x1029dc20 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dc24u64) }
        0x1029dc24 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc28u64) }
        0x1029dc28 => { vm.decode(0xf3084d75u32); let d = 24i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc2cu64) }
        0x1029dc2c => { vm.decode(0xb2405d75u32); let d = 3i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc30u64) }
        0x1029dc30 => { vm.decode(0xa000cd75u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc34u64) }
        0x1029dc34 => { vm.decode(0xa140ed75u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc38u64) }
        0x1029dc38 => { vm.decode(0xf1009d75u32); let d = 22i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc3cu64) }
        0x1029dc3c => { vm.decode(0xda6e812du32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029dc40u64) }
        0x1029dc40 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dc44u64) }
        0x1029dc44 => { vm.decode(0x2225b4f5u32); let d = 16i16; let s = 12i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc48u64) }
        0x1029dc48 => { vm.decode(0x62c5b4f5u32); let d = 25i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc4cu64) }
        0x1029dc4c => { vm.decode(0xa283a4f5u32); let d = 12i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc50u64) }
        0x1029dc50 => { vm.decode(0x6a81a4f5u32); let d = 10i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc54u64) }
        0x1029dc54 => { vm.decode(0x2361a4f5u32); let d = 1i16; let s = 20i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc58u64) }
        0x1029dc58 => { vm.decode(0x6327a4f5u32); let d = 20i16; let s = 22i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc5cu64) }
        0x1029dc5c => { vm.decode(0xab21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc60u64) }
        0x1029dc60 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dc64u64) }
        0x1029dc64 => { vm.decode(0xeb01a4f5u32); let d = 2i16; let s = 19i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc68u64) }
        0x1029dc68 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dc6cu64) }
        0x1029dc6c => { vm.decode(0xaa65b4f5u32); let d = 19i16; let s = 13i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc70u64) }
        0x1029dc70 => { vm.decode(0xeae3b4f5u32); let d = 15i16; let s = 15i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc74u64) }
        0x1029dc74 => { vm.decode(0x6b01a4f5u32); let d = 2i16; let s = 18i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc78u64) }
        0x1029dc78 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dc7cu64) }
        0x1029dc7c => { vm.decode(0xa267a4f5u32); let d = 21i16; let s = 5i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc80u64) }
        0x1029dc80 => { vm.decode(0x2a25a4f5u32); let d = 18i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc84u64) }
        0x1029dc84 => { vm.decode(0xe2c3a4f5u32); let d = 13i16; let s = 3i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc88u64) }
        0x1029dc88 => { vm.decode(0x2b01b4f5u32); let d = 2i16; let s = 24i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dc8cu64) }
        0x1029dc8c => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dc90u64) }
        0x1029dc90 => { vm.decode(0x45906011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029dc94u64) }
        0x1029dc94 => { vm.decode(0x42494349u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dc98u64) }
        0x1029dc98 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dc9cu64) }
        0x1029dc9c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dca0u64) }
        0x1029dca0 => { vm.decode(0x4a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dca4u64) }
        0x1029dca4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dca8u64) }
        0x1029dca8 => { vm.decode(0x208ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dcacu64) }
        0x1029dcac => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dcb0u64) }
        0x1029dcb0 => { vm.decode(0x42eb4349u32); let d = 3i16; let s = 14i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dcb4u64) }
        0x1029dcb4 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dcb8u64) }
        0x1029dcb8 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dcbcu64) }
        0x1029dcbc => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dcc0u64) }
        0x1029dcc0 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dcc4u64) }
        0x1029dcc4 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dcc8u64) }
        0x1029dcc8 => { vm.decode(0x430d0835u32); let d = 14i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dcccu64) }
        0x1029dccc => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dcd0u64) }
        0x1029dcd0 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dcd4u64) }
        0x1029dcd4 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dcd8u64) }
        0x1029dcd8 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dcdcu64) }
        0x1029dcdc => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dce0u64) }
        0x1029dce0 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dce4u64) }
        0x1029dce4 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dce8u64) }
        0x1029dce8 => { vm.decode(0x25bc349u32); let d = 4i16; let s = 11i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dcecu64) }
        0x1029dcec => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dcf0u64) }
        0x1029dcf0 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dcf4u64) }
        0x1029dcf4 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dcf8u64) }
        0x1029dcf8 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dcfcu64) }
        0x1029dcfc => { vm.decode(0x2886e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd00u64) }
        0x1029dd00 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd04u64) }
        0x1029dd04 => { vm.decode(0xd104311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029dd08u64) }
        0x1029dd08 => { vm.decode(0x2f8c349u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dd0cu64) }
        0x1029dd0c => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dd10u64) }
        0x1029dd10 => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dd14u64) }
        0x1029dd14 => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dd18u64) }
        0x1029dd18 => { vm.decode(0x2082012fu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd1cu64) }
        0x1029dd1c => { vm.decode(0x2880cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd20u64) }
        0x1029dd20 => { vm.decode(0x6102a835u32); let d = 5i16; let s = 5i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd24u64) }
        0x1029dd24 => { vm.decode(0x52ca4349u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dd28u64) }
        0x1029dd28 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dd2cu64) }
        0x1029dd2c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dd30u64) }
        0x1029dd30 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dd34u64) }
        0x1029dd34 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd38u64) }
        0x1029dd38 => { vm.decode(0x284eb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd3cu64) }
        0x1029dd3c => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd40u64) }
        0x1029dd40 => { vm.decode(0x5907811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029dd44u64) }
        0x1029dd44 => { vm.decode(0x124a5349u32); let d = 24i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dd48u64) }
        0x1029dd48 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dd4cu64) }
        0x1029dd4c => { vm.decode(0xe1084d75u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dd50u64) }
        0x1029dd50 => { vm.decode(0x38434f5u32); let d = 24i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dd54u64) }
        0x1029dd54 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd58u64) }
        0x1029dd58 => { vm.decode(0x80eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd5cu64) }
        0x1029dd5c => { vm.decode(0x16021835u32); let d = 24i16; let s = 8i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd60u64) }
        0x1029dd60 => { vm.decode(0x518f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029dd64u64) }
        0x1029dd64 => { vm.decode(0x2ca5349u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dd68u64) }
        0x1029dd68 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dd6cu64) }
        0x1029dd6c => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dd70u64) }
        0x1029dd70 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dd74u64) }
        0x1029dd74 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd78u64) }
        0x1029dd78 => { vm.decode(0x280eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd7cu64) }
        0x1029dd7c => { vm.decode(0x44023835u32); let d = 17i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd80u64) }
        0x1029dd80 => { vm.decode(0x46f9d349u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dd84u64) }
        0x1029dd84 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dd88u64) }
        0x1029dd88 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dd8cu64) }
        0x1029dd8c => { vm.decode(0xcb6624f5u32); let d = 23i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dd90u64) }
        0x1029dd90 => { vm.decode(0x60aa412fu32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd94u64) }
        0x1029dd94 => { vm.decode(0x884cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dd98u64) }
        0x1029dd98 => { vm.decode(0x4206b835u32); let d = 9i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dd9cu64) }
        0x1029dd9c => { vm.decode(0x52f9d349u32); let d = 31i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dda0u64) }
        0x1029dda0 => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dda4u64) }
        0x1029dda4 => { vm.decode(0xf348fd75u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dda8u64) }
        0x1029dda8 => { vm.decode(0xcbe634f5u32); let d = 31i16; let s = 6i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ddacu64) }
        0x1029ddac => { vm.decode(0x60aa4b2fu32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddb0u64) }
        0x1029ddb0 => { vm.decode(0x288ae36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddb4u64) }
        0x1029ddb4 => { vm.decode(0x774db835u32); let d = 31i16; let s = 7i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ddb8u64) }
        0x1029ddb8 => { vm.decode(0x4d9ce911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ddbcu64) }
        0x1029ddbc => { vm.decode(0x4279d349u32); let d = 22i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ddc0u64) }
        0x1029ddc0 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ddc4u64) }
        0x1029ddc4 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ddc8u64) }
        0x1029ddc8 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ddccu64) }
        0x1029ddcc => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddd0u64) }
        0x1029ddd0 => { vm.decode(0x288ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddd4u64) }
        0x1029ddd4 => { vm.decode(0x254d9835u32); let d = 22i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ddd8u64) }
        0x1029ddd8 => { vm.decode(0x4d10c311u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029dddcu64) }
        0x1029dddc => { vm.decode(0x5279d349u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029dde0u64) }
        0x1029dde0 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029dde4u64) }
        0x1029dde4 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dde8u64) }
        0x1029dde8 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ddecu64) }
        0x1029ddec => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddf0u64) }
        0x1029ddf0 => { vm.decode(0x228ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddf4u64) }
        0x1029ddf4 => { vm.decode(0x374d9835u32); let d = 30i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ddf8u64) }
        0x1029ddf8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ddfcu64) }
        0x1029ddfc => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de00u64) }
        0x1029de00 => { vm.decode(0x208e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de04u64) }
        0x1029de04 => { vm.decode(0x20a8092fu32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de08u64) }
        0x1029de08 => { vm.decode(0xa80cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de0cu64) }
        0x1029de0c => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de10u64) }
        0x1029de10 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de14u64) }
        0x1029de14 => { vm.decode(0x350d2835u32); let d = 23i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de18u64) }
        0x1029de18 => { vm.decode(0x2082022fu32); let d = 5i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de1cu64) }
        0x1029de1c => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de20u64) }
        0x1029de20 => { vm.decode(0x2002432fu32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de24u64) }
        0x1029de24 => { vm.decode(0x62408835u32); let d = 8i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de28u64) }
        0x1029de28 => { vm.decode(0x2008402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de2cu64) }
        0x1029de2c => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029de30u64) }
        0x1029de30 => { vm.decode(0x404f2835u32); let d = 3i16; let s = 3i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de34u64) }
        0x1029de34 => { vm.decode(0x2082402fu32); let d = 5i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de38u64) }
        0x1029de38 => { vm.decode(0x45408835u32); let d = 20i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de3cu64) }
        0x1029de3c => { vm.decode(0x42859019u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029de40u64) }
        0x1029de40 => { vm.decode(0x1b0855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029de44u64) }
        0x1029de44 => { vm.decode(0x44a09875u32); let d = 7i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de48u64) }
        0x1029de48 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029de4cu64) }
        0x1029de4c => { vm.decode(0xa8032fu32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de50u64) }
        0x1029de50 => { vm.decode(0x2082032fu32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de54u64) }
        0x1029de54 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de58u64) }
        0x1029de58 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de5cu64) }
        0x1029de5c => { vm.decode(0x8824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de60u64) }
        0x1029de60 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de64u64) }
        0x1029de64 => { vm.decode(0x2082492fu32); let d = 13i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de68u64) }
        0x1029de68 => { vm.decode(0x88a6bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de6cu64) }
        0x1029de6c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de70u64) }
        0x1029de70 => { vm.decode(0x228e6befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de74u64) }
        0x1029de74 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de78u64) }
        0x1029de78 => { vm.decode(0x6880eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de7cu64) }
        0x1029de7c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de80u64) }
        0x1029de80 => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de84u64) }
        0x1029de84 => { vm.decode(0x535ca835u32); let d = 15i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de88u64) }
        0x1029de88 => { vm.decode(0x4a88c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029de8cu64) }
        0x1029de8c => { vm.decode(0x145c8835u32); let d = 18i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de90u64) }
        0x1029de90 => { vm.decode(0x5353a835u32); let d = 13i16; let s = 13i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de94u64) }
        0x1029de94 => { vm.decode(0x1459a835u32); let d = 19i16; let s = 12i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de98u64) }
        0x1029de98 => { vm.decode(0x725f2835u32); let d = 11i16; let s = 11i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029de9cu64) }
        0x1029de9c => { vm.decode(0x35062835u32); let d = 21i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dea0u64) }
        0x1029dea0 => { vm.decode(0x6110a835u32); let d = 5i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dea4u64) }
        0x1029dea4 => { vm.decode(0x2162835u32); let d = 9i16; let s = 2i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dea8u64) }
        0x1029dea8 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029deacu64) }
        0x1029deac => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029deb0u64) }
        0x1029deb0 => { vm.decode(0x459cf011u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029deb4u64) }
        0x1029deb4 => { vm.decode(0x524b4349u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029deb8u64) }
        0x1029deb8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029debcu64) }
        0x1029debc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029dec0u64) }
        0x1029dec0 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029dec4u64) }
        0x1029dec4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dec8u64) }
        0x1029dec8 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029deccu64) }
        0x1029decc => { vm.decode(0xa8a43afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ded0u64) }
        0x1029ded0 => { vm.decode(0xa8082fu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ded4u64) }
        0x1029ded4 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ded8u64) }
        0x1029ded8 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dedcu64) }
        0x1029dedc => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dee0u64) }
        0x1029dee0 => { vm.decode(0x60099835u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dee4u64) }
        0x1029dee4 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dee8u64) }
        0x1029dee8 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029deecu64) }
        0x1029deec => { vm.decode(0x604e1835u32); let d = 2i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029def0u64) }
        0x1029def0 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029def4u64) }
        0x1029def4 => { vm.decode(0x200f1835u32); let d = 2i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029def8u64) }
        0x1029def8 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029defcu64) }
        0x1029defc => { vm.decode(0x274b9835u32); let d = 30i16; let s = 15i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df00u64) }
        0x1029df00 => { vm.decode(0x37482835u32); let d = 31i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df04u64) }
        0x1029df04 => { vm.decode(0x46059835u32); let d = 24i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df08u64) }
        0x1029df08 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df0cu64) }
        0x1029df0c => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029df10u64) }
        0x1029df10 => { vm.decode(0xa78329f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -913i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029df14u64).wrapping_add((imm * 4) as u64) } else { 0x1029df14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029df14u64) }
        0x1029df14 => { vm.decode(0x6005875u32); let d = 8i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df18u64) }
        0x1029df18 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df1cu64) }
        0x1029df1c => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df20u64) }
        0x1029df20 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df24u64) }
        0x1029df24 => { vm.decode(0x401e2835u32); let d = 3i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df28u64) }
        0x1029df28 => { vm.decode(0x205f0835u32); let d = 2i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df2cu64) }
        0x1029df2c => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df30u64) }
        0x1029df30 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df34u64) }
        0x1029df34 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df38u64) }
        0x1029df38 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df3cu64) }
        0x1029df3c => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df40u64) }
        0x1029df40 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df44u64) }
        0x1029df44 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df48u64) }
        0x1029df48 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df4cu64) }
        0x1029df4c => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df50u64) }
        0x1029df50 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df54u64) }
        0x1029df54 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df58u64) }
        0x1029df58 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df5cu64) }
        0x1029df5c => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df60u64) }
        0x1029df60 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df64u64) }
        0x1029df64 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df68u64) }
        0x1029df68 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df6cu64) }
        0x1029df6c => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df70u64) }
        0x1029df70 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df74u64) }
        0x1029df74 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df78u64) }
        0x1029df78 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df7cu64) }
        0x1029df7c => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df80u64) }
        0x1029df80 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df84u64) }
        0x1029df84 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df88u64) }
        0x1029df88 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df8cu64) }
        0x1029df8c => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df90u64) }
        0x1029df90 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029df94u64) }
        0x1029df94 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029df98u64) }
        0x1029df98 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029df9cu64) }
        0x1029df9c => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfa0u64) }
        0x1029dfa0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dfa4u64) }
        0x1029dfa4 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfa8u64) }
        0x1029dfa8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dfacu64) }
        0x1029dfac => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfb0u64) }
        0x1029dfb0 => { vm.decode(0x949119u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029dfb4u64) }
        0x1029dfb4 => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfb8u64) }
        0x1029dfb8 => { vm.decode(0x82c42099u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029dfbcu64) }
        0x1029dfbc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029dfc0u64) }
        0x1029dfc0 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dfc4u64) }
        0x1029dfc4 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfc8u64) }
        0x1029dfc8 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfccu64) }
        0x1029dfcc => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dfd0u64) }
        0x1029dfd0 => { vm.decode(0x2886e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dfd4u64) }
        0x1029dfd4 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029dfd8u64) }
        0x1029dfd8 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029dfdcu64) }
        0x1029dfdc => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029dfe0u64) }
        0x1029dfe0 => { vm.decode(0xc8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029dfe4u64).wrapping_add((imm * 4) as u64) } else { 0x1029dfe4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029dfe4u64) }
        0x1029dfe4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029dfe8u64) } }
        0x1029dfe8 => { vm.decode(0x10a080a3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x1029dfecu64) }
        0x1029dfec => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029dff0u64) }
        0x1029dff0 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029dff4u64) }
        0x1029dff4 => { vm.decode(0x22200f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029dff8u64) }
        0x1029dff8 => { vm.decode(0xdd9e77b3u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029dffcu64) }
        0x1029dffc => { vm.decode(0xc77ef59u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e000u64) }
        0x1029e000 => { vm.decode(0x42848019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e004u64) }
        0x1029e004 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029e008u64) }
        0x1029e008 => { vm.decode(0x2a012fu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e00cu64) }
        0x1029e00c => { vm.decode(0x134443b5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029e010u64) }
        0x1029e010 => { vm.decode(0x7ddc77b3u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029e014u64) }
        0x1029e014 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e018u64) }
        0x1029e018 => { vm.decode(0x918403b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029e01cu64) }
        0x1029e01c => { vm.decode(0xfd9e7733u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029e020u64) }
        0x1029e020 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029e024u64) }
        0x1029e024 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029e028u64) }
        0x1029e028 => { vm.decode(0xa7d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029e02cu64).wrapping_add((imm * 4) as u64) } else { 0x1029e02cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029e02cu64) }
        0x1029e02c => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029e030u64) }
        0x1029e030 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e034u64) }
        0x1029e034 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e038u64) }
        0x1029e038 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029e03cu64) }
        0x1029e03c => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e040u64) }
        0x1029e040 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e044u64) }
        0x1029e044 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e048u64) }
        0x1029e048 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e04cu64) }
        0x1029e04c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029e050u64) }
        0x1029e050 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e054u64) }
        0x1029e054 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e058u64) }
        0x1029e058 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1029e05cu64) }
        0x1029e05c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029e060u64) } }
        0x1029e060 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029e064u64) }
        0x1029e16c => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e170u64) }
        0x1029e170 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e174u64) }
        0x1029e174 => { vm.decode(0x40528119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e178u64) }
        0x1029e178 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e17cu64) }
        0x1029e17c => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e180u64) }
        0x1029e180 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e184u64) }
        0x1029e184 => { vm.decode(0x642d8019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029e188u64) }
        0x1029e188 => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029e18cu64) }
        0x1029e18c => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029e190u64) }
        0x1029e190 => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029e194u64).wrapping_add((imm * 4) as u64) } else { 0x1029e194u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029e194u64) }
        0x1029e194 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029e198u64) } }
        0x1029e198 => { vm.decode(0x2dc8541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 102204i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1029e19cu64) }
        0x1029e19c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029e1a0u64) } }
        0x1029e1a0 => { vm.decode(0x1d984011u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029e1a4u64) }
        0x1029f2bc => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f2c0u64) }
        0x1029f2c0 => { vm.decode(0x2a00212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f2c4u64) }
        0x1029f2c4 => { vm.decode(0x41159u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 328i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f2c8u64) }
        0x1029f2c8 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f2ccu64) }
        0x1029f2cc => { vm.decode(0x8b21c909u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029f2d0u64) }
        0x1029f2d0 => { vm.decode(0x9319db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f2d4u64) }
        0x1029f2d4 => { vm.decode(0x89299cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f2d8u64) }
        0x1029f2d8 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f2dcu64) }
        0x1029f2dc => { vm.decode(0x9219db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f2e0u64) }
        0x1029f2e0 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f2e4u64) }
        0x1029f2e4 => { vm.decode(0x286432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f2e8u64) }
        0x1029f2e8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f2ecu64) }
        0x1029f2ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1029f2f0u64 + 4);
        vm.wr(37, 0x1029f2f0u64 + 4);
        Ok(0x1029f2f0u64) }
        0x1029f2f0 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f2f4u64) }
        0x1029f2f4 => { vm.decode(0x631d1162u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029f2f8u64) }
        0x1029f2f8 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029f2fcu64) }
        0x1029f2fc => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029f300u64) }
        0x1029f300 => { vm.decode(0x2a88436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f304u64) }
        0x1029f304 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f308u64) }
        0x1029f308 => { vm.decode(0x10002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f30cu64) }
        0x1029f30c => { vm.decode(0x20220033u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029f310u64) }
        0x1029f310 => { vm.decode(0x8001236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f314u64) }
        0x1029f314 => { vm.decode(0xa0006000u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 33i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029f318u64) }
        0x1029f318 => { vm.decode(0x3222871u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1079i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029f31cu64).wrapping_add((imm * 4) as u64) } else { 0x1029f31cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029f31cu64) }
        0x1029f31c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029f320u64) } }
        0x1029f320 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f324u64) }
        0x1029f324 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f328u64) }
        0x1029f328 => { vm.decode(0x2276u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f32cu64) }
        0x1029f32c => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f330u64) }
        0x1029f330 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029f334u64) }
        0x1029f334 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f338u64) }
        0x1029f338 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029f33cu64) }
        0x1029f33c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029f340u64) }
        0x1029f340 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f344u64) }
        0x1029f344 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029f348u64) }
        0x1029f348 => { vm.decode(0x740d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f34cu64) }
        0x1029f34c => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f350u64) }
        0x1029f350 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f354u64) }
        0x1029f354 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f358u64) }
        0x1029f358 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f35cu64) }
        0x1029f35c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1029f360u64 + 4);
        vm.wr(37, 0x1029f360u64 + 4);
        Ok(0x1029f360u64) }
        0x1029f360 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f364u64) }
        0x1029f364 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f368u64) }
        0x1029f368 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f36cu64) }
        0x1029f36c => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f370u64) }
        0x1029f370 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f374u64) }
        0x1029f374 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f378u64) }
        0x1029f378 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f37cu64) }
        0x1029f37c => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f380u64) }
        0x1029f380 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f384u64) }
        0x1029f384 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f388u64) }
        0x1029f388 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f38cu64) }
        0x1029f38c => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f390u64) }
        0x1029f390 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f394u64) }
        0x1029f394 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f398u64) }
        0x1029f398 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f39cu64) }
        0x1029f39c => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f3a0u64) }
        0x1029f3a0 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f3a4u64) }
        0x1029f3a4 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029f3a8u64) }
        0x1029f3a8 => { vm.decode(0x8c8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029f3acu64).wrapping_add((imm * 4) as u64) } else { 0x1029f3acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029f3acu64) }
        0x1029f3ac => { vm.decode(0x642d1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f3b0u64) }
        0x1029f3b0 => { vm.decode(0x204002beu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f3b4u64) }
        0x1029f3b4 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3b8u64) }
        0x1029f3b8 => { vm.decode(0xa0480037u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x1029f3bcu64) }
        0x1029f3bc => { vm.decode(0x81740935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3c0u64) }
        0x1029f3c0 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f3c4u64) }
        0x1029f3c4 => { vm.decode(0x1242023eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f3c8u64) }
        0x1029f3c8 => { vm.decode(0x81700935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3ccu64) }
        0x1029f3cc => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f3d0u64) }
        0x1029f3d0 => { vm.decode(0x124202beu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f3d4u64) }
        0x1029f3d4 => { vm.decode(0x81300935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3d8u64) }
        0x1029f3d8 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f3dcu64) }
        0x1029f3dc => { vm.decode(0x242027eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f3e0u64) }
        0x1029f3e0 => { vm.decode(0x32a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3e4u64) }
        0x1029f3e4 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f3e8u64) }
        0x1029f3e8 => { vm.decode(0x24202feu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f3ecu64) }
        0x1029f3ec => { vm.decode(0x30a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f3f0u64) }
        0x1029f3f0 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f3f4u64) }
        0x1029f3f4 => { vm.decode(0x61041019u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f3f8u64) }
        0x1029f3f8 => { vm.decode(0x1849019u32); let d = 11i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f3fcu64) }
        0x1029f3fc => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f400u64) }
        0x1029f400 => { vm.decode(0x184202feu32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f404u64) }
        0x1029f404 => { vm.decode(0x3842027eu32); let d = 13i16; let s = 17i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029f408u64) }
        0x1029f408 => { vm.decode(0x1168ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f40cu64) }
        0x1029f40c => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f410u64) }
        0x1029f410 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f414u64) }
        0x1029f414 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f418u64) }
        0x1029f418 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f41cu64) }
        0x1029f41c => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029f420u64) }
        0x1029f420 => { vm.decode(0x27d32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -29i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029f424u64).wrapping_add((imm * 4) as u64) } else { 0x1029f424u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029f424u64) }
        0x1029f424 => { vm.decode(0x8209875u32); let d = 17i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f428u64) }
        0x1029f428 => { vm.decode(0x4a8aebafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f42cu64) }
        0x1029f42c => { vm.decode(0x886c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f430u64) }
        0x1029f430 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f434u64) }
        0x1029f434 => { vm.decode(0x4880c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f438u64) }
        0x1029f438 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029f43cu64) }
        0x1029f43c => { vm.decode(0x631d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029f440u64) }
        0x1029f440 => { vm.decode(0x2a82c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f444u64) }
        0x1029f444 => { vm.decode(0x228073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f448u64) }
        0x1029f448 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f44cu64) }
        0x1029f44c => { vm.decode(0x650d1059u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f450u64) }
        0x1029f450 => { vm.decode(0x228c7bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f454u64) }
        0x1029f454 => { vm.decode(0x88ee3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f458u64) }
        0x1029f458 => { vm.decode(0x825befu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f45cu64) }
        0x1029f45c => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f460u64) }
        0x1029f460 => { vm.decode(0x20865bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f464u64) }
        0x1029f464 => { vm.decode(0x8e7bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f468u64) }
        0x1029f468 => { vm.decode(0x40807bafu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f46cu64) }
        0x1029f46c => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f470u64) }
        0x1029f470 => { vm.decode(0x62805bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f474u64) }
        0x1029f474 => { vm.decode(0x701a2835u32); let d = 3i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f478u64) }
        0x1029f478 => { vm.decode(0x1b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f47cu64) }
        0x1029f47c => { vm.decode(0x1a3835u32); let d = 3i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f480u64) }
        0x1029f480 => { vm.decode(0x1b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f484u64) }
        0x1029f484 => { vm.decode(0x404f3835u32); let d = 3i16; let s = 14i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f488u64) }
        0x1029f488 => { vm.decode(0x1b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f48cu64) }
        0x1029f48c => { vm.decode(0x23073835u32); let d = 13i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f490u64) }
        0x1029f490 => { vm.decode(0x4a80d875u32); let d = 31i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f494u64) }
        0x1029f494 => { vm.decode(0x407a835u32); let d = 17i16; let s = 6i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f498u64) }
        0x1029f498 => { vm.decode(0x48804875u32); let d = 22i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f49cu64) }
        0x1029f49c => { vm.decode(0x23428835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4a0u64) }
        0x1029f4a0 => { vm.decode(0xda9119u32); let d = 3i16; let s = 10i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029f4a4u64) }
        0x1029f4a4 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f4a8u64) }
        0x1029f4a8 => { vm.decode(0x114c9835u32); let d = 6i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4acu64) }
        0x1029f4ac => { vm.decode(0x4680c875u32); let d = 15i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4b0u64) }
        0x1029f4b0 => { vm.decode(0x6208a835u32); let d = 11i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4b4u64) }
        0x1029f4b4 => { vm.decode(0xd94c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029f4b8u64) }
        0x1029f4b8 => { vm.decode(0x30102200u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029f4bcu64) }
        0x1029f4bc => { vm.decode(0x460408a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 930i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029f4c0u64).wrapping_add((imm * 4) as u64) } else { 0x1029f4c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029f4c0u64) }
        0x1029f4c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029f4c4u64) } }
        0x1029f4c4 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4c8u64) }
        0x1029f4c8 => { vm.decode(0x148080f5u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4ccu64) }
        0x1029f4cc => { vm.decode(0x769000f5u32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4d0u64) }
        0x1029f4d0 => { vm.decode(0x1b3815f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f4d4u64) }
        0x1029f4d4 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4d8u64) }
        0x1029f4d8 => { vm.decode(0xa0c875u32); let d = 5i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4dcu64) }
        0x1029f4dc => { vm.decode(0xb1054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f4e0u64) }
        0x1029f4e0 => { vm.decode(0x72b1c0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4e4u64) }
        0x1029f4e4 => { vm.decode(0x9b385570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f4e8u64) }
        0x1029f4e8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f4ecu64) }
        0x1029f4ec => { vm.decode(0x6204875u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f4f0u64) }
        0x1029f4f0 => { vm.decode(0x70b360f5u32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f4f4u64) }
        0x1029f4f4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f4f8u64) }
        0x1029f4f8 => { vm.decode(0x1b3014f0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f4fcu64) }
        0x1029f4fc => { vm.decode(0x2a844b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f500u64) }
        0x1029f500 => { vm.decode(0xa0e294f5u32); let d = 13i16; let s = 9i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f504u64) }
        0x1029f504 => { vm.decode(0xa86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f508u64) }
        0x1029f508 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f50cu64) }
        0x1029f50c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f510u64) }
        0x1029f510 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f514u64) }
        0x1029f514 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f518u64) }
        0x1029f518 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f51cu64) }
        0x1029f51c => { vm.decode(0x2088c32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f520u64) }
        0x1029f520 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f524u64) }
        0x1029f524 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f528u64) }
        0x1029f528 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f52cu64) }
        0x1029f52c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f530u64) }
        0x1029f530 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f534u64) }
        0x1029f534 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f538u64) }
        0x1029f538 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f53cu64) }
        0x1029f53c => { vm.decode(0x52d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f540u64) }
        0x1029f540 => { vm.decode(0x70f3a0f5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f544u64) }
        0x1029f544 => { vm.decode(0x40908875u32); let d = 7i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f548u64) }
        0x1029f548 => { vm.decode(0x52c180f5u32); let d = 16i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f54cu64) }
        0x1029f54c => { vm.decode(0x9b201430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f550u64) }
        0x1029f550 => { vm.decode(0x10e2a0f5u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f554u64) }
        0x1029f554 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f558u64) }
        0x1029f558 => { vm.decode(0xa14484f5u32); let d = 17i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f55cu64) }
        0x1029f55c => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f560u64) }
        0x1029f560 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f564u64) }
        0x1029f564 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f568u64) }
        0x1029f568 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f56cu64) }
        0x1029f56c => { vm.decode(0x4a909875u32); let d = 31i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f570u64) }
        0x1029f570 => { vm.decode(0x54e120f5u32); let d = 18i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f574u64) }
        0x1029f574 => { vm.decode(0x1b305530u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f578u64) }
        0x1029f578 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f57cu64) }
        0x1029f57c => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f580u64) }
        0x1029f580 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f584u64) }
        0x1029f584 => { vm.decode(0x60e294f5u32); let d = 13i16; let s = 9i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f588u64) }
        0x1029f588 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f58cu64) }
        0x1029f58c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f590u64) }
        0x1029f590 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f594u64) }
        0x1029f594 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f598u64) }
        0x1029f598 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f59cu64) }
        0x1029f59c => { vm.decode(0x18434f5u32); let d = 24i16; let s = 4i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5a0u64) }
        0x1029f5a0 => { vm.decode(0x4c909875u32); let d = 23i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f5a4u64) }
        0x1029f5a4 => { vm.decode(0xa94494f5u32); let d = 19i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5a8u64) }
        0x1029f5a8 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f5acu64) }
        0x1029f5ac => { vm.decode(0xc64c72edu32); let d = 20i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f5b0u64) }
        0x1029f5b0 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f5b4u64) }
        0x1029f5b4 => { vm.decode(0x76b140f5u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f5b8u64) }
        0x1029f5b8 => { vm.decode(0xc6cc72edu32); let d = 21i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f5bcu64) }
        0x1029f5bc => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f5c0u64) }
        0x1029f5c0 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f5c4u64) }
        0x1029f5c4 => { vm.decode(0x728380f5u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f5c8u64) }
        0x1029f5c8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f5ccu64) }
        0x1029f5cc => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5d0u64) }
        0x1029f5d0 => { vm.decode(0x330b8835u32); let d = 14i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f5d4u64) }
        0x1029f5d4 => { vm.decode(0x7481a0f5u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f5d8u64) }
        0x1029f5d8 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f5dcu64) }
        0x1029f5dc => { vm.decode(0x22492fu32); let d = 12i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f5e0u64) }
        0x1029f5e0 => { vm.decode(0x81e224f5u32); let d = 13i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5e4u64) }
        0x1029f5e4 => { vm.decode(0x690484f5u32); let d = 18i16; let s = 9i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5e8u64) }
        0x1029f5e8 => { vm.decode(0x96424f5u32); let d = 19i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5ecu64) }
        0x1029f5ec => { vm.decode(0x4002422fu32); let d = 20i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f5f0u64) }
        0x1029f5f0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f5f4u64) }
        0x1029f5f4 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f5f8u64) }
        0x1029f5f8 => { vm.decode(0x321c0835u32); let d = 10i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f5fcu64) }
        0x1029f5fc => { vm.decode(0x9309d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f600u64) }
        0x1029f600 => { vm.decode(0x549120f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f604u64) }
        0x1029f604 => { vm.decode(0xb285530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f608u64) }
        0x1029f608 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f60cu64) }
        0x1029f60c => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f610u64) }
        0x1029f610 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f614u64) }
        0x1029f614 => { vm.decode(0x524e0835u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f618u64) }
        0x1029f618 => { vm.decode(0xb081570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f61cu64) }
        0x1029f61c => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f620u64) }
        0x1029f620 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f624u64) }
        0x1029f624 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f628u64) }
        0x1029f628 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f62cu64) }
        0x1029f62c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f630u64) }
        0x1029f630 => { vm.decode(0x4031835u32); let d = 16i16; let s = 12i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f634u64) }
        0x1029f634 => { vm.decode(0x50b060f5u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f638u64) }
        0x1029f638 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f63cu64) }
        0x1029f63c => { vm.decode(0x20a294f5u32); let d = 12i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f640u64) }
        0x1029f640 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f644u64) }
        0x1029f644 => { vm.decode(0x103b835u32); let d = 5i16; let s = 12i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f648u64) }
        0x1029f648 => { vm.decode(0x1938dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f64cu64) }
        0x1029f64c => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f650u64) }
        0x1029f650 => { vm.decode(0x32c200f5u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f654u64) }
        0x1029f654 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f658u64) }
        0x1029f658 => { vm.decode(0x814424f5u32); let d = 17i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f65cu64) }
        0x1029f65c => { vm.decode(0x4008412fu32); let d = 18i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f660u64) }
        0x1029f660 => { vm.decode(0x53408835u32); let d = 12i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f664u64) }
        0x1029f664 => { vm.decode(0xa105875u32); let d = 24i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f668u64) }
        0x1029f668 => { vm.decode(0x8b1855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f66cu64) }
        0x1029f66c => { vm.decode(0x5481a0f5u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f670u64) }
        0x1029f670 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f674u64) }
        0x1029f674 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f678u64) }
        0x1029f678 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f67cu64) }
        0x1029f67c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f680u64) }
        0x1029f680 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f684u64) }
        0x1029f684 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f688u64) }
        0x1029f688 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f68cu64) }
        0x1029f68c => { vm.decode(0x28826bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f690u64) }
        0x1029f690 => { vm.decode(0x70a220f5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f694u64) }
        0x1029f694 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f698u64) }
        0x1029f698 => { vm.decode(0xa04084f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f69cu64) }
        0x1029f69c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6a0u64) }
        0x1029f6a0 => { vm.decode(0x20043835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f6a4u64) }
        0x1029f6a4 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f6a8u64) }
        0x1029f6a8 => { vm.decode(0x56b100f5u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f6acu64) }
        0x1029f6ac => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f6b0u64) }
        0x1029f6b0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f6b4u64) }
        0x1029f6b4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6b8u64) }
        0x1029f6b8 => { vm.decode(0x2844befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6bcu64) }
        0x1029f6bc => { vm.decode(0x349220f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f6c0u64) }
        0x1029f6c0 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f6c4u64) }
        0x1029f6c4 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f6c8u64) }
        0x1029f6c8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6ccu64) }
        0x1029f6cc => { vm.decode(0x35462835u32); let d = 21i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f6d0u64) }
        0x1029f6d0 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f6d4u64) }
        0x1029f6d4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f6d8u64) }
        0x1029f6d8 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f6dcu64) }
        0x1029f6dc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6e0u64) }
        0x1029f6e0 => { vm.decode(0x54b3a0f5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f6e4u64) }
        0x1029f6e4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f6e8u64) }
        0x1029f6e8 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f6ecu64) }
        0x1029f6ec => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f6f0u64) }
        0x1029f6f0 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f6f4u64) }
        0x1029f6f4 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f6f8u64) }
        0x1029f6f8 => { vm.decode(0x76b000f5u32); let d = 10i16; let s = 13i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f6fcu64) }
        0x1029f6fc => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f700u64) }
        0x1029f700 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f704u64) }
        0x1029f704 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f708u64) }
        0x1029f708 => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f70cu64) }
        0x1029f70c => { vm.decode(0x14b3a0f5u32); let d = 11i16; let s = 19i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f710u64) }
        0x1029f710 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f714u64) }
        0x1029f714 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f718u64) }
        0x1029f718 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f71cu64) }
        0x1029f71c => { vm.decode(0x33462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f720u64) }
        0x1029f720 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f724u64) }
        0x1029f724 => { vm.decode(0x16b000f5u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f728u64) }
        0x1029f728 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f72cu64) }
        0x1029f72c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f730u64) }
        0x1029f730 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f734u64) }
        0x1029f734 => { vm.decode(0x54b220f5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f738u64) }
        0x1029f738 => { vm.decode(0x4c004875u32); let d = 18i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f73cu64) }
        0x1029f73c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f740u64) }
        0x1029f740 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f744u64) }
        0x1029f744 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f748u64) }
        0x1029f748 => { vm.decode(0x354e0835u32); let d = 22i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f74cu64) }
        0x1029f74c => { vm.decode(0x4a8243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f750u64) }
        0x1029f750 => { vm.decode(0x569080f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f754u64) }
        0x1029f754 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f758u64) }
        0x1029f758 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f75cu64) }
        0x1029f75c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f760u64) }
        0x1029f760 => { vm.decode(0x228e63efu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f764u64) }
        0x1029f764 => { vm.decode(0x54b320f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f768u64) }
        0x1029f768 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f76cu64) }
        0x1029f76c => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f770u64) }
        0x1029f770 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f774u64) }
        0x1029f774 => { vm.decode(0x36462835u32); let d = 25i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f778u64) }
        0x1029f778 => { vm.decode(0x349080f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f77cu64) }
        0x1029f77c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f780u64) }
        0x1029f780 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f784u64) }
        0x1029f784 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f788u64) }
        0x1029f788 => { vm.decode(0x11472835u32); let d = 5i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f78cu64) }
        0x1029f78c => { vm.decode(0x1928dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f790u64) }
        0x1029f790 => { vm.decode(0xd31c53b5u32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f794u64) }
        0x1029f794 => { vm.decode(0x9b2055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f798u64) }
        0x1029f798 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f79cu64) }
        0x1029f79c => { vm.decode(0x2082e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f7a0u64) }
        0x1029f7a0 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f7a4u64) }
        0x1029f7a4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f7a8u64) }
        0x1029f7a8 => { vm.decode(0x4210c875u32); let d = 11i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f7acu64) }
        0x1029f7ac => { vm.decode(0x88eeb6fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f7b0u64) }
        0x1029f7b0 => { vm.decode(0x134f0835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f7b4u64) }
        0x1029f7b4 => { vm.decode(0x16b180f5u32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f7b8u64) }
        0x1029f7b8 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f7bcu64) }
        0x1029f7bc => { vm.decode(0x408434f5u32); let d = 24i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f7c0u64) }
        0x1029f7c0 => { vm.decode(0x549100f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f7c4u64) }
        0x1029f7c4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f7c8u64) }
        0x1029f7c8 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f7ccu64) }
        0x1029f7cc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f7d0u64) }
        0x1029f7d0 => { vm.decode(0xa82c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f7d4u64) }
        0x1029f7d4 => { vm.decode(0x14052835u32); let d = 17i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f7d8u64) }
        0x1029f7d8 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f7dcu64) }
        0x1029f7dc => { vm.decode(0x8b10d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f7e0u64) }
        0x1029f7e0 => { vm.decode(0x5490a0f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f7e4u64) }
        0x1029f7e4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f7e8u64) }
        0x1029f7e8 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f7ecu64) }
        0x1029f7ec => { vm.decode(0x230d875u32); let d = 9i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f7f0u64) }
        0x1029f7f0 => { vm.decode(0x8938dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f7f4u64) }
        0x1029f7f4 => { vm.decode(0x749380f5u32); let d = 11i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f7f8u64) }
        0x1029f7f8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f7fcu64) }
        0x1029f7fc => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f800u64) }
        0x1029f800 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f804u64) }
        0x1029f804 => { vm.decode(0x511c0835u32); let d = 6i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f808u64) }
        0x1029f808 => { vm.decode(0x928ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f80cu64) }
        0x1029f80c => { vm.decode(0x549280f5u32); let d = 11i16; let s = 20i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f810u64) }
        0x1029f810 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f814u64) }
        0x1029f814 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f818u64) }
        0x1029f818 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f81cu64) }
        0x1029f81c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f820u64) }
        0x1029f820 => { vm.decode(0x570d0835u32); let d = 30i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f824u64) }
        0x1029f824 => { vm.decode(0x349200f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f828u64) }
        0x1029f828 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f82cu64) }
        0x1029f82c => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f830u64) }
        0x1029f830 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f834u64) }
        0x1029f834 => { vm.decode(0x51070835u32); let d = 4i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f838u64) }
        0x1029f838 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f83cu64) }
        0x1029f83c => { vm.decode(0x44805875u32); let d = 6i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f840u64) }
        0x1029f840 => { vm.decode(0x54b300f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f844u64) }
        0x1029f844 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f848u64) }
        0x1029f848 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f84cu64) }
        0x1029f84c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f850u64) }
        0x1029f850 => { vm.decode(0xa86e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f854u64) }
        0x1029f854 => { vm.decode(0x53050835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f858u64) }
        0x1029f858 => { vm.decode(0x6305875u32); let d = 8i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f85cu64) }
        0x1029f85c => { vm.decode(0x74b380f5u32); let d = 11i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f860u64) }
        0x1029f860 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f864u64) }
        0x1029f864 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f868u64) }
        0x1029f868 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f86cu64) }
        0x1029f86c => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f870u64) }
        0x1029f870 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f874u64) }
        0x1029f874 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f878u64) }
        0x1029f878 => { vm.decode(0x74a100f5u32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f87cu64) }
        0x1029f87c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f880u64) }
        0x1029f880 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f884u64) }
        0x1029f884 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f888u64) }
        0x1029f888 => { vm.decode(0x628a6befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f88cu64) }
        0x1029f88c => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f890u64) }
        0x1029f890 => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f894u64) }
        0x1029f894 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f898u64) }
        0x1029f898 => { vm.decode(0x54a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f89cu64) }
        0x1029f89c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f8a0u64) }
        0x1029f8a0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f8a4u64) }
        0x1029f8a4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8a8u64) }
        0x1029f8a8 => { vm.decode(0x201d0835u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f8acu64) }
        0x1029f8ac => { vm.decode(0x9189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f8b0u64) }
        0x1029f8b0 => { vm.decode(0x44808875u32); let d = 7i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f8b4u64) }
        0x1029f8b4 => { vm.decode(0x14a000f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f8b8u64) }
        0x1029f8b8 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f8bcu64) }
        0x1029f8bc => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f8c0u64) }
        0x1029f8c0 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8c4u64) }
        0x1029f8c4 => { vm.decode(0x228063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8c8u64) }
        0x1029f8c8 => { vm.decode(0x25040835u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f8ccu64) }
        0x1029f8cc => { vm.decode(0x228c4bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8d0u64) }
        0x1029f8d0 => { vm.decode(0x34a100f5u32); let d = 2i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f8d4u64) }
        0x1029f8d4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f8d8u64) }
        0x1029f8d8 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f8dcu64) }
        0x1029f8dc => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8e0u64) }
        0x1029f8e0 => { vm.decode(0x552835u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f8e4u64) }
        0x1029f8e4 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f8e8u64) }
        0x1029f8e8 => { vm.decode(0x148180f5u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f8ecu64) }
        0x1029f8ec => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f8f0u64) }
        0x1029f8f0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f8f4u64) }
        0x1029f8f4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f8f8u64) }
        0x1029f8f8 => { vm.decode(0x250f0835u32); let d = 22i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f8fcu64) }
        0x1029f8fc => { vm.decode(0x74a180f5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f900u64) }
        0x1029f900 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f904u64) }
        0x1029f904 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f908u64) }
        0x1029f908 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f90cu64) }
        0x1029f90c => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f910u64) }
        0x1029f910 => { vm.decode(0x25042835u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f914u64) }
        0x1029f914 => { vm.decode(0x6909875u32); let d = 13i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f918u64) }
        0x1029f918 => { vm.decode(0x10e180f5u32); let d = 16i16; let s = 19i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f91cu64) }
        0x1029f91c => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f920u64) }
        0x1029f920 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f924u64) }
        0x1029f924 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f928u64) }
        0x1029f928 => { vm.decode(0x20123835u32); let d = 1i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f92cu64) }
        0x1029f92c => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f930u64) }
        0x1029f930 => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029f934u64) }
        0x1029f934 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029f938u64) }
        0x1029f938 => { vm.decode(0xb0014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f93cu64) }
        0x1029f93c => { vm.decode(0x44820d35u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029f940u64) }
        0x1029f940 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029f944u64) }
        0x1029f944 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029f948u64) }
        0x1029f948 => { vm.decode(0x282c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f94cu64) }
        0x1029f94c => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f950u64) }
        0x1029f950 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f954u64) }
        0x1029f954 => { vm.decode(0x54b3835u32); let d = 23i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f958u64) }
        0x1029f958 => { vm.decode(0xa21c23b5u32); let d = 16i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f95cu64) }
        0x1029f95c => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f960u64) }
        0x1029f960 => { vm.decode(0x210424f5u32); let d = 16i16; let s = 5i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f964u64) }
        0x1029f964 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f968u64) }
        0x1029f968 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f96cu64) }
        0x1029f96c => { vm.decode(0x20003835u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f970u64) }
        0x1029f970 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f974u64) }
        0x1029f974 => { vm.decode(0x60004a2fu32); let d = 25i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f978u64) }
        0x1029f978 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f97cu64) }
        0x1029f97c => { vm.decode(0x50423835u32); let d = 1i16; let s = 10i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f980u64) }
        0x1029f980 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f984u64) }
        0x1029f984 => { vm.decode(0x39c73b5u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f988u64) }
        0x1029f988 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f98cu64) }
        0x1029f98c => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f990u64) }
        0x1029f990 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f994u64) }
        0x1029f994 => { vm.decode(0x74183835u32); let d = 19i16; let s = 17i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f998u64) }
        0x1029f998 => { vm.decode(0x88a63afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f99cu64) }
        0x1029f99c => { vm.decode(0x50d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f9a0u64) }
        0x1029f9a0 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f9a4u64) }
        0x1029f9a4 => { vm.decode(0x230414f5u32); let d = 16i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f9a8u64) }
        0x1029f9a8 => { vm.decode(0xa21c73b5u32); let d = 24i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f9acu64) }
        0x1029f9ac => { vm.decode(0xe20c875u32); let d = 25i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f9b0u64) }
        0x1029f9b0 => { vm.decode(0x1b00d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f9b4u64) }
        0x1029f9b4 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f9b8u64) }
        0x1029f9b8 => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f9bcu64) }
        0x1029f9bc => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f9c0u64) }
        0x1029f9c0 => { vm.decode(0x175b1835u32); let d = 30i16; let s = 24i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f9c4u64) }
        0x1029f9c4 => { vm.decode(0xa31c73b5u32); let d = 24i16; let s = 15i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f9c8u64) }
        0x1029f9c8 => { vm.decode(0x1b105570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f9ccu64) }
        0x1029f9cc => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f9d0u64) }
        0x1029f9d0 => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f9d4u64) }
        0x1029f9d4 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f9d8u64) }
        0x1029f9d8 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f9dcu64) }
        0x1029f9dc => { vm.decode(0x36001835u32); let d = 24i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f9e0u64) }
        0x1029f9e0 => { vm.decode(0xa8843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f9e4u64) }
        0x1029f9e4 => { vm.decode(0x135c13b5u32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029f9e8u64) }
        0x1029f9e8 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029f9ecu64) }
        0x1029f9ec => { vm.decode(0x68a234f5u32); let d = 14i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029f9f0u64) }
        0x1029f9f0 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029f9f4u64) }
        0x1029f9f4 => { vm.decode(0x1007a835u32); let d = 1i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029f9f8u64) }
        0x1029f9f8 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029f9fcu64) }
        0x1029f9fc => { vm.decode(0x2884befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa00u64) }
        0x1029fa00 => { vm.decode(0x235c13b5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa04u64) }
        0x1029fa04 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa08u64) }
        0x1029fa08 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa0cu64) }
        0x1029fa0c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa10u64) }
        0x1029fa10 => { vm.decode(0x2a80c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa14u64) }
        0x1029fa14 => { vm.decode(0x33008835u32); let d = 12i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa18u64) }
        0x1029fa18 => { vm.decode(0x22844befu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa1cu64) }
        0x1029fa1c => { vm.decode(0xa29c23b5u32); let d = 17i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa20u64) }
        0x1029fa20 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa24u64) }
        0x1029fa24 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa28u64) }
        0x1029fa28 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa2cu64) }
        0x1029fa2c => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa30u64) }
        0x1029fa30 => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa34u64) }
        0x1029fa34 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fa38u64) }
        0x1029fa38 => { vm.decode(0xe39c63b5u32); let d = 17i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa3cu64) }
        0x1029fa3c => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa40u64) }
        0x1029fa40 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa44u64) }
        0x1029fa44 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa48u64) }
        0x1029fa48 => { vm.decode(0x2880c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa4cu64) }
        0x1029fa4c => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa50u64) }
        0x1029fa50 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fa54u64) }
        0x1029fa54 => { vm.decode(0x6a09875u32); let d = 13i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa58u64) }
        0x1029fa58 => { vm.decode(0xa39c23b5u32); let d = 17i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa5cu64) }
        0x1029fa5c => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa60u64) }
        0x1029fa60 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa64u64) }
        0x1029fa64 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa68u64) }
        0x1029fa68 => { vm.decode(0x40113835u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa6cu64) }
        0x1029fa6c => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fa70u64) }
        0x1029fa70 => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa74u64) }
        0x1029fa74 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa78u64) }
        0x1029fa78 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa7cu64) }
        0x1029fa7c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa80u64) }
        0x1029fa80 => { vm.decode(0x2a8ce32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa84u64) }
        0x1029fa84 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fa88u64) }
        0x1029fa88 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fa8cu64) }
        0x1029fa8c => { vm.decode(0x428663afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fa90u64) }
        0x1029fa90 => { vm.decode(0x429c43b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fa94u64) }
        0x1029fa94 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fa98u64) }
        0x1029fa98 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fa9cu64) }
        0x1029fa9c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029faa0u64) }
        0x1029faa0 => { vm.decode(0x64112835u32); let d = 17i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029faa4u64) }
        0x1029faa4 => { vm.decode(0x839c43b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029faa8u64) }
        0x1029faa8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029faacu64) }
        0x1029faac => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fab0u64) }
        0x1029fab0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fab4u64) }
        0x1029fab4 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fab8u64) }
        0x1029fab8 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fabcu64) }
        0x1029fabc => { vm.decode(0x806befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fac0u64) }
        0x1029fac0 => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fac4u64) }
        0x1029fac4 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fac8u64) }
        0x1029fac8 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029faccu64) }
        0x1029facc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fad0u64) }
        0x1029fad0 => { vm.decode(0x2a8ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fad4u64) }
        0x1029fad4 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fad8u64) }
        0x1029fad8 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fadcu64) }
        0x1029fadc => { vm.decode(0x40e32bb5u32); let d = 3i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fae0u64) }
        0x1029fae0 => { vm.decode(0xdd4349u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fae4u64) }
        0x1029fae4 => { vm.decode(0x4e809875u32); let d = 31i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fae8u64) }
        0x1029fae8 => { vm.decode(0x30d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029faecu64) }
        0x1029faec => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029faf0u64) }
        0x1029faf0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029faf4u64) }
        0x1029faf4 => { vm.decode(0x2288e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029faf8u64) }
        0x1029faf8 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fafcu64) }
        0x1029fafc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb00u64) }
        0x1029fb00 => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb04u64) }
        0x1029fb04 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fb08u64) }
        0x1029fb08 => { vm.decode(0x4080d875u32); let d = 7i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb0cu64) }
        0x1029fb0c => { vm.decode(0x39c43b5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fb10u64) }
        0x1029fb10 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fb14u64) }
        0x1029fb14 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fb18u64) }
        0x1029fb18 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb1cu64) }
        0x1029fb1c => { vm.decode(0xa8ec36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb20u64) }
        0x1029fb20 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb24u64) }
        0x1029fb24 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fb28u64) }
        0x1029fb28 => { vm.decode(0x439c03b5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029fb2cu64) }
        0x1029fb2c => { vm.decode(0x42901875u32); let d = 14i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb30u64) }
        0x1029fb30 => { vm.decode(0x89281d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fb34u64) }
        0x1029fb34 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fb38u64) }
        0x1029fb38 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fb3cu64) }
        0x1029fb3c => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fb40u64) }
        0x1029fb40 => { vm.decode(0x80e52bb5u32); let d = 22i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fb44u64) }
        0x1029fb44 => { vm.decode(0x60522d35u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fb48u64) }
        0x1029fb48 => { vm.decode(0x42755162u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fb4cu64) }
        0x1029fb4c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fb50u64) }
        0x1029fb50 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fb54u64) }
        0x1029fb54 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fb58u64) }
        0x1029fb58 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb5cu64) }
        0x1029fb5c => { vm.decode(0x2886c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb60u64) }
        0x1029fb60 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb64u64) }
        0x1029fb64 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fb68u64) }
        0x1029fb68 => { vm.decode(0x30bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fb6cu64) }
        0x1029fb6c => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fb70u64) }
        0x1029fb70 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fb74u64) }
        0x1029fb74 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fb78u64) }
        0x1029fb78 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fb7cu64) }
        0x1029fb7c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fb80u64) }
        0x1029fb80 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb84u64) }
        0x1029fb84 => { vm.decode(0x65582835u32); let d = 23i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb88u64) }
        0x1029fb88 => { vm.decode(0x2000402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb8cu64) }
        0x1029fb8c => { vm.decode(0x8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fb90u64) }
        0x1029fb90 => { vm.decode(0x44480835u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fb94u64) }
        0x1029fb94 => { vm.decode(0x80a10bb5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fb98u64) }
        0x1029fb98 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fb9cu64) }
        0x1029fb9c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fba0u64) }
        0x1029fba0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fba4u64) }
        0x1029fba4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fba8u64) }
        0x1029fba8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fbacu64) }
        0x1029fbac => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fbb0u64) }
        0x1029fbb0 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fbb4u64) }
        0x1029fbb4 => { vm.decode(0x81c42bb5u32); let d = 30i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fbb8u64) }
        0x1029fbb8 => { vm.decode(0x60526d35u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fbbcu64) }
        0x1029fbbc => { vm.decode(0x62777162u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fbc0u64) }
        0x1029fbc0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fbc4u64) }
        0x1029fbc4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fbc8u64) }
        0x1029fbc8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fbccu64) }
        0x1029fbcc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fbd0u64) }
        0x1029fbd0 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fbd4u64) }
        0x1029fbd4 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fbd8u64) }
        0x1029fbd8 => { vm.decode(0x40862bb5u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fbdcu64) }
        0x1029fbdc => { vm.decode(0x40922d35u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fbe0u64) }
        0x1029fbe0 => { vm.decode(0x35d1022u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fbe4u64) }
        0x1029fbe4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fbe8u64) }
        0x1029fbe8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fbecu64) }
        0x1029fbec => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fbf0u64) }
        0x1029fbf0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fbf4u64) }
        0x1029fbf4 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fbf8u64) }
        0x1029fbf8 => { vm.decode(0x44482835u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fbfcu64) }
        0x1029fbfc => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc00u64) }
        0x1029fc00 => { vm.decode(0x1820bb5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fc04u64) }
        0x1029fc04 => { vm.decode(0x40924d35u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fc08u64) }
        0x1029fc08 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fc0cu64) }
        0x1029fc0c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fc10u64) }
        0x1029fc10 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fc14u64) }
        0x1029fc14 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc18u64) }
        0x1029fc18 => { vm.decode(0x40032835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fc1cu64) }
        0x1029fc1c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fc20u64) }
        0x1029fc20 => { vm.decode(0x81012bb5u32); let d = 24i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fc24u64) }
        0x1029fc24 => { vm.decode(0x40524d35u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fc28u64) }
        0x1029fc28 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fc2cu64) }
        0x1029fc2c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fc30u64) }
        0x1029fc30 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fc34u64) }
        0x1029fc34 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc38u64) }
        0x1029fc38 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc3cu64) }
        0x1029fc3c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fc40u64) }
        0x1029fc40 => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fc44u64) }
        0x1029fc44 => { vm.decode(0x40220bb5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fc48u64) }
        0x1029fc48 => { vm.decode(0x408a8d35u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fc4cu64) }
        0x1029fc4c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fc50u64) }
        0x1029fc50 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fc54u64) }
        0x1029fc54 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fc58u64) }
        0x1029fc58 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc5cu64) }
        0x1029fc5c => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc60u64) }
        0x1029fc60 => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fc64u64) }
        0x1029fc64 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fc68u64) }
        0x1029fc68 => { vm.decode(0x1002bb5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fc6cu64) }
        0x1029fc6c => { vm.decode(0x401b4d35u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fc70u64) }
        0x1029fc70 => { vm.decode(0x62371062u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fc74u64) }
        0x1029fc74 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fc78u64) }
        0x1029fc78 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fc7cu64) }
        0x1029fc7c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fc80u64) }
        0x1029fc80 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc84u64) }
        0x1029fc84 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc88u64) }
        0x1029fc88 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fc8cu64) }
        0x1029fc8c => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fc90u64) }
        0x1029fc90 => { vm.decode(0x2a8aeb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fc94u64) }
        0x1029fc94 => { vm.decode(0x80a72bb5u32); let d = 21i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fc98u64) }
        0x1029fc98 => { vm.decode(0x40db2d35u32); let d = 11i16; let s = 21i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fc9cu64) }
        0x1029fc9c => { vm.decode(0x43453162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fca0u64) }
        0x1029fca0 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fca4u64) }
        0x1029fca4 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fca8u64) }
        0x1029fca8 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fcacu64) }
        0x1029fcac => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcb0u64) }
        0x1029fcb0 => { vm.decode(0x73140835u32); let d = 12i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fcb4u64) }
        0x1029fcb4 => { vm.decode(0x60c36d35u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fcb8u64) }
        0x1029fcb8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fcbcu64) }
        0x1029fcbc => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fcc0u64) }
        0x1029fcc0 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fcc4u64) }
        0x1029fcc4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcc8u64) }
        0x1029fcc8 => { vm.decode(0xa8ec32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcccu64) }
        0x1029fccc => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fcd0u64) }
        0x1029fcd0 => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fcd4u64) }
        0x1029fcd4 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fcd8u64) }
        0x1029fcd8 => { vm.decode(0x62157062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fcdcu64) }
        0x1029fcdc => { vm.decode(0x60030d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fce0u64) }
        0x1029fce0 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fce4u64) }
        0x1029fce4 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fce8u64) }
        0x1029fce8 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fcecu64) }
        0x1029fcec => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcf0u64) }
        0x1029fcf0 => { vm.decode(0xa8ae32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcf4u64) }
        0x1029fcf4 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fcf8u64) }
        0x1029fcf8 => { vm.decode(0x28c6befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fcfcu64) }
        0x1029fcfc => { vm.decode(0x472bb5u32); let d = 7i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fd00u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
