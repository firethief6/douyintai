#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x1029fd00 => { vm.decode(0x609b2d35u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fd04u64) }
        0x1029fd04 => { vm.decode(0x3757062u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029fd08u64) }
        0x1029fd08 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fd0cu64) }
        0x1029fd0c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fd10u64) }
        0x1029fd10 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fd14u64) }
        0x1029fd14 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd18u64) }
        0x1029fd18 => { vm.decode(0xa8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd1cu64) }
        0x1029fd1c => { vm.decode(0x514d0835u32); let d = 6i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fd20u64) }
        0x1029fd20 => { vm.decode(0x918ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fd24u64) }
        0x1029fd24 => { vm.decode(0x52d84349u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fd28u64) }
        0x1029fd28 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fd2cu64) }
        0x1029fd2c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fd30u64) }
        0x1029fd30 => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd34u64) }
        0x1029fd34 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fd38u64) }
        0x1029fd38 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd3cu64) }
        0x1029fd3c => { vm.decode(0xa88cb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd40u64) }
        0x1029fd40 => { vm.decode(0x53462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fd44u64) }
        0x1029fd44 => { vm.decode(0x80232bb5u32); let d = 17i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fd48u64) }
        0x1029fd48 => { vm.decode(0x40db0d35u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fd4cu64) }
        0x1029fd4c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fd50u64) }
        0x1029fd50 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fd54u64) }
        0x1029fd54 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fd58u64) }
        0x1029fd58 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd5cu64) }
        0x1029fd5c => { vm.decode(0x888cb6fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd60u64) }
        0x1029fd60 => { vm.decode(0x524e2835u32); let d = 11i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fd64u64) }
        0x1029fd64 => { vm.decode(0x88ceb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd68u64) }
        0x1029fd68 => { vm.decode(0x60200a2fu32); let d = 25i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd6cu64) }
        0x1029fd6c => { vm.decode(0x52581835u32); let d = 10i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fd70u64) }
        0x1029fd70 => { vm.decode(0x9209d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fd74u64) }
        0x1029fd74 => { vm.decode(0x507c4349u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fd78u64) }
        0x1029fd78 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fd7cu64) }
        0x1029fd7c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fd80u64) }
        0x1029fd80 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fd84u64) }
        0x1029fd84 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fd88u64) }
        0x1029fd88 => { vm.decode(0x36552835u32); let d = 25i16; let s = 10i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fd8cu64) }
        0x1029fd8c => { vm.decode(0x546f4349u32); let d = 10i16; let s = 30i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fd90u64) }
        0x1029fd90 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fd94u64) }
        0x1029fd94 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fd98u64) }
        0x1029fd98 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fd9cu64) }
        0x1029fd9c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fda0u64) }
        0x1029fda0 => { vm.decode(0x2a82c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fda4u64) }
        0x1029fda4 => { vm.decode(0x330d0835u32); let d = 14i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fda8u64) }
        0x1029fda8 => { vm.decode(0x608a4befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fdacu64) }
        0x1029fdac => { vm.decode(0x41e12bb5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029fdb0u64) }
        0x1029fdb0 => { vm.decode(0x6413cd35u32); let d = 23i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029fdb4u64) }
        0x1029fdb4 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fdb8u64) }
        0x1029fdb8 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fdbcu64) }
        0x1029fdbc => { vm.decode(0xc96624f5u32); let d = 23i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fdc0u64) }
        0x1029fdc0 => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fdc4u64) }
        0x1029fdc4 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fdc8u64) }
        0x1029fdc8 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fdccu64) }
        0x1029fdcc => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fdd0u64) }
        0x1029fdd0 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fdd4u64) }
        0x1029fdd4 => { vm.decode(0x31549835u32); let d = 4i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fdd8u64) }
        0x1029fdd8 => { vm.decode(0x910dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fddcu64) }
        0x1029fddc => { vm.decode(0x44ce5349u32); let d = 19i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fde0u64) }
        0x1029fde0 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fde4u64) }
        0x1029fde4 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fde8u64) }
        0x1029fde8 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fdecu64) }
        0x1029fdec => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fdf0u64) }
        0x1029fdf0 => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fdf4u64) }
        0x1029fdf4 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fdf8u64) }
        0x1029fdf8 => { vm.decode(0x9285cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fdfcu64) }
        0x1029fdfc => { vm.decode(0x44dc5349u32); let d = 19i16; let s = 17i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fe00u64) }
        0x1029fe00 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fe04u64) }
        0x1029fe04 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fe08u64) }
        0x1029fe08 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fe0cu64) }
        0x1029fe0c => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe10u64) }
        0x1029fe10 => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe14u64) }
        0x1029fe14 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fe18u64) }
        0x1029fe18 => { vm.decode(0x908dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fe1cu64) }
        0x1029fe1c => { vm.decode(0x40de5349u32); let d = 19i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fe20u64) }
        0x1029fe20 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fe24u64) }
        0x1029fe24 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fe28u64) }
        0x1029fe28 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fe2cu64) }
        0x1029fe2c => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe30u64) }
        0x1029fe30 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe34u64) }
        0x1029fe34 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fe38u64) }
        0x1029fe38 => { vm.decode(0x9205cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fe3cu64) }
        0x1029fe3c => { vm.decode(0x1510cb11u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029fe40u64) }
        0x1029fe40 => { vm.decode(0x446c5349u32); let d = 18i16; let s = 20i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fe44u64) }
        0x1029fe44 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fe48u64) }
        0x1029fe48 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fe4cu64) }
        0x1029fe4c => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fe50u64) }
        0x1029fe50 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe54u64) }
        0x1029fe54 => { vm.decode(0x882e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe58u64) }
        0x1029fe58 => { vm.decode(0x1051835u32); let d = 4i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fe5cu64) }
        0x1029fe5c => { vm.decode(0x900dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fe60u64) }
        0x1029fe60 => { vm.decode(0x446d5349u32); let d = 18i16; let s = 22i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fe64u64) }
        0x1029fe64 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fe68u64) }
        0x1029fe68 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fe6cu64) }
        0x1029fe6c => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fe70u64) }
        0x1029fe70 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe74u64) }
        0x1029fe74 => { vm.decode(0x1071835u32); let d = 4i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fe78u64) }
        0x1029fe78 => { vm.decode(0x920dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fe7cu64) }
        0x1029fe7c => { vm.decode(0x107dc349u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fe80u64) }
        0x1029fe80 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fe84u64) }
        0x1029fe84 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fe88u64) }
        0x1029fe88 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fe8cu64) }
        0x1029fe8c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe90u64) }
        0x1029fe90 => { vm.decode(0x282e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fe94u64) }
        0x1029fe94 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fe98u64) }
        0x1029fe98 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fe9cu64) }
        0x1029fe9c => { vm.decode(0x147cc349u32); let d = 12i16; let s = 21i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fea0u64) }
        0x1029fea0 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fea4u64) }
        0x1029fea4 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fea8u64) }
        0x1029fea8 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029feacu64) }
        0x1029feac => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029feb0u64) }
        0x1029feb0 => { vm.decode(0x886c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029feb4u64) }
        0x1029feb4 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029feb8u64) }
        0x1029feb8 => { vm.decode(0x900dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029febcu64) }
        0x1029febc => { vm.decode(0x104ec349u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fec0u64) }
        0x1029fec0 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fec4u64) }
        0x1029fec4 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fec8u64) }
        0x1029fec8 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029feccu64) }
        0x1029fecc => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fed0u64) }
        0x1029fed0 => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fed4u64) }
        0x1029fed4 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fed8u64) }
        0x1029fed8 => { vm.decode(0xdc4349u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fedcu64) }
        0x1029fedc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029fee0u64) }
        0x1029fee0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029fee4u64) }
        0x1029fee4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029fee8u64) }
        0x1029fee8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029feecu64) }
        0x1029feec => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fef0u64) }
        0x1029fef0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029fef4u64) }
        0x1029fef4 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029fef8u64) }
        0x1029fef8 => { vm.decode(0xcf4349u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029fefcu64) }
        0x1029fefc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ff00u64) }
        0x1029ff00 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff04u64) }
        0x1029ff04 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ff08u64) }
        0x1029ff08 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff0cu64) }
        0x1029ff0c => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ff10u64) }
        0x1029ff10 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ff14u64) }
        0x1029ff14 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ff18u64) }
        0x1029ff18 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff1cu64) }
        0x1029ff1c => { vm.decode(0x2286c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff20u64) }
        0x1029ff20 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ff24u64) }
        0x1029ff24 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff28u64) }
        0x1029ff28 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ff2cu64) }
        0x1029ff2c => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ff30u64) }
        0x1029ff30 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ff34u64) }
        0x1029ff34 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ff38u64) }
        0x1029ff38 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ff3cu64) }
        0x1029ff3c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff40u64) }
        0x1029ff40 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ff44u64) }
        0x1029ff44 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff48u64) }
        0x1029ff48 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff4cu64) }
        0x1029ff4c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ff50u64) }
        0x1029ff50 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ff54u64) }
        0x1029ff54 => { vm.decode(0x6ca4349u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ff58u64) }
        0x1029ff58 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ff5cu64) }
        0x1029ff5c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff60u64) }
        0x1029ff60 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ff64u64) }
        0x1029ff64 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff68u64) }
        0x1029ff68 => { vm.decode(0x40432835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ff6cu64) }
        0x1029ff6c => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ff70u64) }
        0x1029ff70 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ff74u64) }
        0x1029ff74 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ff78u64) }
        0x1029ff78 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff7cu64) }
        0x1029ff7c => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ff80u64) }
        0x1029ff80 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ff84u64) }
        0x1029ff84 => { vm.decode(0x60122835u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ff88u64) }
        0x1029ff88 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ff8cu64) }
        0x1029ff8c => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ff90u64) }
        0x1029ff90 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ff94u64) }
        0x1029ff94 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ff98u64) }
        0x1029ff98 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ff9cu64) }
        0x1029ff9c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ffa0u64) }
        0x1029ffa0 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ffa4u64) }
        0x1029ffa4 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffa8u64) }
        0x1029ffa8 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffacu64) }
        0x1029ffac => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ffb0u64) }
        0x1029ffb0 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ffb4u64) }
        0x1029ffb4 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffb8u64) }
        0x1029ffb8 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffbcu64) }
        0x1029ffbc => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffc0u64) }
        0x1029ffc0 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffc4u64) }
        0x1029ffc4 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffc8u64) }
        0x1029ffc8 => { vm.decode(0xdadac12du32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffccu64) }
        0x1029ffcc => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffd0u64) }
        0x1029ffd0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffd4u64) }
        0x1029ffd4 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffd8u64) }
        0x1029ffd8 => { vm.decode(0xda5ec12du32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffdcu64) }
        0x1029ffdc => { vm.decode(0x228843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffe0u64) }
        0x1029ffe0 => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffe4u64) }
        0x1029ffe4 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ffe8u64) }
        0x1029ffe8 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ffecu64) }
        0x1029ffec => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fff0u64) }
        0x1029fff0 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fff4u64) }
        0x1029fff4 => { vm.decode(0x2088e36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029fff8u64) }
        0x1029fff8 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029fffcu64) }
        0x1029fffc => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0000u64) }
        0x102a0000 => { vm.decode(0xe100dd75u32); let d = 22i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0004u64) }
        0x102a0004 => { vm.decode(0xb3089d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0008u64) }
        0x102a0008 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a000cu64) }
        0x102a000c => { vm.decode(0xe340ad75u32); let d = 21i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0010u64) }
        0x102a0010 => { vm.decode(0xe040dd75u32); let d = 23i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0014u64) }
        0x102a0014 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0018u64) }
        0x102a0018 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a001cu64) }
        0x102a001c => { vm.decode(0xa1487d75u32); let d = 11i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0020u64) }
        0x102a0020 => { vm.decode(0x208c43afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0024u64) }
        0x102a0024 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0028u64) }
        0x102a0028 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a002cu64) }
        0x102a002c => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0030u64) }
        0x102a0030 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0034u64) }
        0x102a0034 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0038u64) }
        0x102a0038 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a003cu64) }
        0x102a003c => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0040u64) }
        0x102a0040 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0044u64) }
        0x102a0044 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0048u64) }
        0x102a0048 => { vm.decode(0xb348dd75u32); let d = 15i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a004cu64) }
        0x102a004c => { vm.decode(0xa0405d75u32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0050u64) }
        0x102a0050 => { vm.decode(0xe1486d75u32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0054u64) }
        0x102a0054 => { vm.decode(0xa100cd75u32); let d = 4i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0058u64) }
        0x102a0058 => { vm.decode(0xf1089d75u32); let d = 30i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a005cu64) }
        0x102a005c => { vm.decode(0xda6ed16du32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0060u64) }
        0x102a0060 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0064u64) }
        0x102a0064 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0068u64) }
        0x102a0068 => { vm.decode(0xb3054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a006cu64) }
        0x102a006c => { vm.decode(0xe08434f5u32); let d = 24i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0070u64) }
        0x102a0070 => { vm.decode(0xa08224f5u32); let d = 12i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0074u64) }
        0x102a0074 => { vm.decode(0x68c024f5u32); let d = 11i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0078u64) }
        0x102a0078 => { vm.decode(0xe16024f5u32); let d = 1i16; let s = 5i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a007cu64) }
        0x102a007c => { vm.decode(0x696634f5u32); let d = 23i16; let s = 5i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0080u64) }
        0x102a0080 => { vm.decode(0xa92024f5u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0084u64) }
        0x102a0084 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0088u64) }
        0x102a0088 => { vm.decode(0xe90024f5u32); let d = 2i16; let s = 5i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a008cu64) }
        0x102a008c => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0090u64) }
        0x102a0090 => { vm.decode(0x682034f5u32); let d = 2i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0094u64) }
        0x102a0094 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0098u64) }
        0x102a0098 => { vm.decode(0x696424f5u32); let d = 19i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a009cu64) }
        0x102a009c => { vm.decode(0x690024f5u32); let d = 2i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00a0u64) }
        0x102a00a0 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a00a4u64) }
        0x102a00a4 => { vm.decode(0x282424f5u32); let d = 18i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00a8u64) }
        0x102a00a8 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00acu64) }
        0x102a00ac => { vm.decode(0xe88224f5u32); let d = 14i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00b0u64) }
        0x102a00b0 => { vm.decode(0xe82034f5u32); let d = 2i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00b4u64) }
        0x102a00b4 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a00b8u64) }
        0x102a00b8 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a00bcu64) }
        0x102a00bc => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a00c0u64) }
        0x102a00c0 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a00c4u64) }
        0x102a00c4 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00c8u64) }
        0x102a00c8 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a00ccu64) }
        0x102a00cc => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a00d0u64) }
        0x102a00d0 => { vm.decode(0x474c2835u32); let d = 31i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a00d4u64) }
        0x102a00d4 => { vm.decode(0x46d84349u32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a00d8u64) }
        0x102a00d8 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a00dcu64) }
        0x102a00dc => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a00e0u64) }
        0x102a00e0 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a00e4u64) }
        0x102a00e4 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a00e8u64) }
        0x102a00e8 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a00ecu64) }
        0x102a00ec => { vm.decode(0x43442835u32); let d = 13i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a00f0u64) }
        0x102a00f0 => { vm.decode(0x46e84349u32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a00f4u64) }
        0x102a00f4 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a00f8u64) }
        0x102a00f8 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a00fcu64) }
        0x102a00fc => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0100u64) }
        0x102a0100 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0104u64) }
        0x102a0104 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0108u64) }
        0x102a0108 => { vm.decode(0x404c2835u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a010cu64) }
        0x102a010c => { vm.decode(0x24bc349u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0110u64) }
        0x102a0110 => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0114u64) }
        0x102a0114 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0118u64) }
        0x102a0118 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a011cu64) }
        0x102a011c => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0120u64) }
        0x102a0120 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0124u64) }
        0x102a0124 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0128u64) }
        0x102a0128 => { vm.decode(0x45104211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a012cu64) }
        0x102a012c => { vm.decode(0x52494349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0130u64) }
        0x102a0130 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0134u64) }
        0x102a0134 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0138u64) }
        0x102a0138 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a013cu64) }
        0x102a013c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0140u64) }
        0x102a0140 => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0144u64) }
        0x102a0144 => { vm.decode(0x124c0835u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0148u64) }
        0x102a0148 => { vm.decode(0x52cac349u32); let d = 15i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a014cu64) }
        0x102a014c => { vm.decode(0xfa2fc909u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0150u64) }
        0x102a0150 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0154u64) }
        0x102a0154 => { vm.decode(0xcae234f5u32); let d = 15i16; let s = 6i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0158u64) }
        0x102a0158 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a015cu64) }
        0x102a015c => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0160u64) }
        0x102a0160 => { vm.decode(0x534ca835u32); let d = 15i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0164u64) }
        0x102a0164 => { vm.decode(0x45906811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0168u64) }
        0x102a0168 => { vm.decode(0x2c95349u32); let d = 17i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a016cu64) }
        0x102a016c => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0170u64) }
        0x102a0170 => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0174u64) }
        0x102a0174 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0178u64) }
        0x102a0178 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a017cu64) }
        0x102a017c => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0180u64) }
        0x102a0180 => { vm.decode(0x44403835u32); let d = 17i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0184u64) }
        0x102a0184 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0188u64) }
        0x102a0188 => { vm.decode(0x249d349u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a018cu64) }
        0x102a018c => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0190u64) }
        0x102a0190 => { vm.decode(0xf000cd75u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0194u64) }
        0x102a0194 => { vm.decode(0x32624f5u32); let d = 20i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0198u64) }
        0x102a0198 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a019cu64) }
        0x102a019c => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01a0u64) }
        0x102a01a0 => { vm.decode(0x5409835u32); let d = 20i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a01a4u64) }
        0x102a01a4 => { vm.decode(0x4598e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a01a8u64) }
        0x102a01a8 => { vm.decode(0x5249d349u32); let d = 30i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a01acu64) }
        0x102a01ac => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a01b0u64) }
        0x102a01b0 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a01b4u64) }
        0x102a01b4 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a01b8u64) }
        0x102a01b8 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01bcu64) }
        0x102a01bc => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01c0u64) }
        0x102a01c0 => { vm.decode(0x1244b835u32); let d = 9i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a01c4u64) }
        0x102a01c4 => { vm.decode(0x2f9d349u32); let d = 21i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a01c8u64) }
        0x102a01c8 => { vm.decode(0xc967c909u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a01ccu64) }
        0x102a01cc => { vm.decode(0xf040ed75u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a01d0u64) }
        0x102a01d0 => { vm.decode(0x836624f5u32); let d = 21i16; let s = 6i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a01d4u64) }
        0x102a01d4 => { vm.decode(0x6082412fu32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01d8u64) }
        0x102a01d8 => { vm.decode(0xa88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01dcu64) }
        0x102a01dc => { vm.decode(0x4540b835u32); let d = 21i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a01e0u64) }
        0x102a01e0 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a01e4u64) }
        0x102a01e4 => { vm.decode(0x4249d349u32); let d = 22i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a01e8u64) }
        0x102a01e8 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a01ecu64) }
        0x102a01ec => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a01f0u64) }
        0x102a01f0 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a01f4u64) }
        0x102a01f4 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01f8u64) }
        0x102a01f8 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a01fcu64) }
        0x102a01fc => { vm.decode(0x54c9835u32); let d = 22i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0200u64) }
        0x102a0200 => { vm.decode(0x4510c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0204u64) }
        0x102a0204 => { vm.decode(0x2495349u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0208u64) }
        0x102a0208 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a020cu64) }
        0x102a020c => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0210u64) }
        0x102a0210 => { vm.decode(0x30424f5u32); let d = 16i16; let s = 6i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0214u64) }
        0x102a0214 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0218u64) }
        0x102a0218 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a021cu64) }
        0x102a021c => { vm.decode(0x4401835u32); let d = 16i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0220u64) }
        0x102a0220 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0224u64) }
        0x102a0224 => { vm.decode(0x60432835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0228u64) }
        0x102a0228 => { vm.decode(0x208663afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a022cu64) }
        0x102a022c => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0230u64) }
        0x102a0230 => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0234u64) }
        0x102a0234 => { vm.decode(0x514ca835u32); let d = 7i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0238u64) }
        0x102a0238 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a023cu64) }
        0x102a023c => { vm.decode(0x574e0835u32); let d = 30i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0240u64) }
        0x102a0240 => { vm.decode(0x880a2fu32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0244u64) }
        0x102a0244 => { vm.decode(0x11050835u32); let d = 4i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0248u64) }
        0x102a0248 => { vm.decode(0x84a2fu32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a024cu64) }
        0x102a024c => { vm.decode(0x32440835u32); let d = 8i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0250u64) }
        0x102a0250 => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0254u64) }
        0x102a0254 => { vm.decode(0x2028002fu32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0258u64) }
        0x102a0258 => { vm.decode(0x600f2835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a025cu64) }
        0x102a025c => { vm.decode(0xa8492fu32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0260u64) }
        0x102a0260 => { vm.decode(0x355f2835u32); let d = 23i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0264u64) }
        0x102a0264 => { vm.decode(0x42058019u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a0268u64) }
        0x102a0268 => { vm.decode(0x1b0854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a026cu64) }
        0x102a026c => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0270u64) }
        0x102a0270 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a0274u64) }
        0x102a0274 => { vm.decode(0x621d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0278u64) }
        0x102a0278 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a027cu64) }
        0x102a027c => { vm.decode(0xa80a2fu32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0280u64) }
        0x102a0280 => { vm.decode(0x208643efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0284u64) }
        0x102a0284 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0288u64) }
        0x102a0288 => { vm.decode(0x24b2fu32); let d = 12i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a028cu64) }
        0x102a028c => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0290u64) }
        0x102a0290 => { vm.decode(0x2082092fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0294u64) }
        0x102a0294 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0298u64) }
        0x102a0298 => { vm.decode(0x288263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a029cu64) }
        0x102a029c => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02a0u64) }
        0x102a02a0 => { vm.decode(0x228663efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02a4u64) }
        0x102a02a4 => { vm.decode(0x4080092fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02a8u64) }
        0x102a02a8 => { vm.decode(0x2882e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02acu64) }
        0x102a02ac => { vm.decode(0x6080092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02b0u64) }
        0x102a02b0 => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02b4u64) }
        0x102a02b4 => { vm.decode(0x36011835u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02b8u64) }
        0x102a02b8 => { vm.decode(0x2a82e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02bcu64) }
        0x102a02bc => { vm.decode(0x740d8835u32); let d = 18i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02c0u64) }
        0x102a02c0 => { vm.decode(0x131c8835u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02c4u64) }
        0x102a02c4 => { vm.decode(0x5459a835u32); let d = 19i16; let s = 13i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02c8u64) }
        0x102a02c8 => { vm.decode(0x33118835u32); let d = 12i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02ccu64) }
        0x102a02cc => { vm.decode(0x6a82636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02d0u64) }
        0x102a02d0 => { vm.decode(0x74060835u32); let d = 16i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02d4u64) }
        0x102a02d4 => { vm.decode(0x121d0835u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02d8u64) }
        0x102a02d8 => { vm.decode(0x22142835u32); let d = 9i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02dcu64) }
        0x102a02dc => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02e0u64) }
        0x102a02e0 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a02e4u64) }
        0x102a02e4 => { vm.decode(0xd9ce111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a02e8u64) }
        0x102a02e8 => { vm.decode(0x52f84349u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a02ecu64) }
        0x102a02ec => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a02f0u64) }
        0x102a02f0 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a02f4u64) }
        0x102a02f4 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a02f8u64) }
        0x102a02f8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a02fcu64) }
        0x102a02fc => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0300u64) }
        0x102a0300 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0304u64) }
        0x102a0304 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0308u64) }
        0x102a0308 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a030cu64) }
        0x102a030c => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0310u64) }
        0x102a0310 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0314u64) }
        0x102a0314 => { vm.decode(0x4a1835u32); let d = 2i16; let s = 10i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0318u64) }
        0x102a0318 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a031cu64) }
        0x102a031c => { vm.decode(0x400f1835u32); let d = 2i16; let s = 12i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0320u64) }
        0x102a0320 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0324u64) }
        0x102a0324 => { vm.decode(0x3473835u32); let d = 13i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0328u64) }
        0x102a0328 => { vm.decode(0x4e90d875u32); let d = 31i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a032cu64) }
        0x102a032c => { vm.decode(0x4416b835u32); let d = 17i16; let s = 23i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0330u64) }
        0x102a0330 => { vm.decode(0x48a00875u32); let d = 22i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0334u64) }
        0x102a0334 => { vm.decode(0x33400835u32); let d = 12i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0338u64) }
        0x102a0338 => { vm.decode(0x110d9835u32); let d = 6i16; let s = 4i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a033cu64) }
        0x102a033c => { vm.decode(0x4680d875u32); let d = 15i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0340u64) }
        0x102a0340 => { vm.decode(0x214c141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 102708i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a0344u64) }
        0x102a0344 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0348u64) }
        0x102a0348 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a034cu64) }
        0x102a034c => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0350u64) }
        0x102a0350 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0354u64) }
        0x102a0354 => { vm.decode(0x404d2835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0358u64) }
        0x102a0358 => { vm.decode(0xf0835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a035cu64) }
        0x102a035c => { vm.decode(0x60102835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0360u64) }
        0x102a0360 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0364u64) }
        0x102a0364 => { vm.decode(0x21038835u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0368u64) }
        0x102a0368 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a036cu64) }
        0x102a036c => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0370u64) }
        0x102a0370 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0374u64) }
        0x102a0374 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0378u64) }
        0x102a0378 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a037cu64) }
        0x102a037c => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0380u64) }
        0x102a0380 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0384u64) }
        0x102a0384 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0388u64) }
        0x102a0388 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a038cu64) }
        0x102a038c => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0390u64) }
        0x102a0390 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0394u64) }
        0x102a0394 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0398u64) }
        0x102a0398 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a039cu64) }
        0x102a039c => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03a0u64) }
        0x102a03a0 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03a4u64) }
        0x102a03a4 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03a8u64) }
        0x102a03a8 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03acu64) }
        0x102a03ac => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03b0u64) }
        0x102a03b0 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03b4u64) }
        0x102a03b4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03b8u64) }
        0x102a03b8 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03bcu64) }
        0x102a03bc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03c0u64) }
        0x102a03c0 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03c4u64) }
        0x102a03c4 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03c8u64) }
        0x102a03c8 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03ccu64) }
        0x102a03cc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03d0u64) }
        0x102a03d0 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03d4u64) }
        0x102a03d4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03d8u64) }
        0x102a03d8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a03dcu64) }
        0x102a03dc => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03e0u64) }
        0x102a03e0 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03e4u64) }
        0x102a03e4 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03e8u64) }
        0x102a03e8 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03ecu64) }
        0x102a03ec => { vm.decode(0x288ce3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03f0u64) }
        0x102a03f0 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a03f4u64) }
        0x102a03f4 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a03f8u64) }
        0x102a03f8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a03fcu64) }
        0x102a03fc => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0400u64) }
        0x102a0400 => { vm.decode(0x4002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a0404u64) }
        0x102a0404 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a0408u64) }
        0x102a0408 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a040cu64) }
        0x102a040c => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0410u64) }
        0x102a0410 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0414u64) }
        0x102a0414 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0418u64) }
        0x102a0418 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a041cu64) }
        0x102a041c => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0420u64) }
        0x102a0420 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0424u64) }
        0x102a0424 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0428u64) }
        0x102a0428 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a042cu64) }
        0x102a042c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0430u64) }
        0x102a0430 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a0434u64 + 4);
        vm.wr(37, 0x102a0434u64 + 4);
        Ok(0x102a0434u64) }
        0x102a0434 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a0438u64) }
        0x102a0438 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a043cu64) }
        0x102a043c => { vm.decode(0x4828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0440u64) }
        0x102a0440 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0444u64) }
        0x102a0444 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0448u64) }
        0x102a0448 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a044cu64) }
        0x102a044c => { vm.decode(0x282e35u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102a0450u64) }
        0x102a0450 => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a0454u64).wrapping_add((imm * 4) as u64) } else { 0x102a0454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a0454u64) }
        0x102a0454 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0458u64) }
        0x102a0458 => { vm.decode(0x2280e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a045cu64) }
        0x102a0484 => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0488u64) }
        0x102a0488 => { vm.decode(0x40101019u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a048cu64) }
        0x102a048c => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a0490u64) }
        0x102a0490 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a0494u64) }
        0x102a0494 => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a0498u64).wrapping_add((imm * 4) as u64) } else { 0x102a0498u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a0498u64) }
        0x102a0498 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a049cu64) } }
        0x102a049c => { vm.decode(0x2024002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a04a0u64) }
        0x102a04a0 => { vm.decode(0x2060f5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04a4u64) }
        0x102a04a4 => { vm.decode(0x22240f5u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04a8u64) }
        0x102a04a8 => { vm.decode(0x42040f5u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04acu64) }
        0x102a04ac => { vm.decode(0x92c403b5u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04b0u64) }
        0x102a04b0 => { vm.decode(0x820013b5u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04b4u64) }
        0x102a04b4 => { vm.decode(0x808413b5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04b8u64) }
        0x102a04b8 => { vm.decode(0x802204b3u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04bcu64) }
        0x102a04bc => { vm.decode(0x2304b3u32); let d = 9i16; let s = 2i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04c0u64) }
        0x102a04c0 => { vm.decode(0x80210433u32); let d = 8i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04c4u64) }
        0x102a04c4 => { vm.decode(0x20620433u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04c8u64) }
        0x102a04c8 => { vm.decode(0xa06000b3u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04ccu64) }
        0x102a04cc => { vm.decode(0x6200b3u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04d0u64) }
        0x102a04d0 => { vm.decode(0x80600033u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04d4u64) }
        0x102a04d4 => { vm.decode(0x22220f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a04d8u64) }
        0x102a04d8 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a04dcu64) }
        0x102a04dc => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a04e0u64) }
        0x102a04e0 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a04e4u64) }
        0x102a04e4 => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a04e8u64).wrapping_add((imm * 4) as u64) } else { 0x102a04e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a04e8u64) }
        0x102a04e8 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a04ecu64) }
        0x102a04ec => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a04f0u64) }
        0x102a04f0 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a04f4u64) }
        0x102a04f4 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a04f8u64) }
        0x102a04f8 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a04fcu64) }
        0x102a04fc => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0500u64) }
        0x102a0500 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0504u64) }
        0x102a0504 => { vm.decode(0x19001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0508u64) }
        0x102a0508 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a050cu64) }
        0x102a050c => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0510u64) }
        0x102a0510 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0514u64) }
        0x102a0514 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0518u64) }
        0x102a0518 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a051cu64) }
        0x102a051c => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0520u64) }
        0x102a0520 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0524u64) }
        0x102a0524 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0528u64) }
        0x102a0528 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a052cu64) }
        0x102a052c => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0530u64) }
        0x102a0530 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a0534u64) }
        0x102a0534 => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a0538u64).wrapping_add((imm * 4) as u64) } else { 0x102a0538u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a0538u64) }
        0x102a0538 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a053cu64) } }
        0x102a053c => { vm.decode(0x2a88436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0540u64) }
        0x102a0540 => { vm.decode(0x200802beu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0544u64) }
        0x102a0544 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0548u64) }
        0x102a0548 => { vm.decode(0x80480437u32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102a054cu64) }
        0x102a054c => { vm.decode(0x81740935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0550u64) }
        0x102a0550 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0554u64) }
        0x102a0554 => { vm.decode(0x120a023eu32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0558u64) }
        0x102a0558 => { vm.decode(0x81700935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a055cu64) }
        0x102a055c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0560u64) }
        0x102a0560 => { vm.decode(0x120a02beu32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0564u64) }
        0x102a0564 => { vm.decode(0x81300935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0568u64) }
        0x102a0568 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a056cu64) }
        0x102a056c => { vm.decode(0x20a027eu32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0570u64) }
        0x102a0570 => { vm.decode(0x32a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0574u64) }
        0x102a0574 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0578u64) }
        0x102a0578 => { vm.decode(0x20a02feu32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a057cu64) }
        0x102a057c => { vm.decode(0x30a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0580u64) }
        0x102a0580 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0584u64) }
        0x102a0584 => { vm.decode(0x41841019u32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a0588u64) }
        0x102a0588 => { vm.decode(0x61059019u32); let d = 11i16; let s = 21i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a058cu64) }
        0x102a058c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a0590u64) }
        0x102a0590 => { vm.decode(0x180a02feu32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0594u64) }
        0x102a0594 => { vm.decode(0x380a027eu32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a0598u64) }
        0x102a0598 => { vm.decode(0x1168ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a059cu64) }
        0x102a059c => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a05a0u64) }
        0x102a05a0 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a05a4u64) }
        0x102a05a4 => { vm.decode(0x11001430u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a05a8u64) }
        0x102a05a8 => { vm.decode(0xca09875u32); let d = 21i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a05acu64) }
        0x102a05ac => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a05b0u64) }
        0x102a05b0 => { vm.decode(0x27d12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -30i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a05b4u64).wrapping_add((imm * 4) as u64) } else { 0x102a05b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a05b4u64) }
        0x102a05b4 => { vm.decode(0x9309570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a05b8u64) }
        0x102a05b8 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05bcu64) }
        0x102a05bc => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05c0u64) }
        0x102a05c0 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05c4u64) }
        0x102a05c4 => { vm.decode(0x6882c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05c8u64) }
        0x102a05c8 => { vm.decode(0x8100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a05ccu64) }
        0x102a05cc => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05d0u64) }
        0x102a05d0 => { vm.decode(0x228073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05d4u64) }
        0x102a05d4 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05d8u64) }
        0x102a05d8 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a05dcu64) }
        0x102a05dc => { vm.decode(0x28c7bafu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05e0u64) }
        0x102a05e0 => { vm.decode(0x4886e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05e4u64) }
        0x102a05e4 => { vm.decode(0x20885befu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05e8u64) }
        0x102a05e8 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05ecu64) }
        0x102a05ec => { vm.decode(0x865bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05f0u64) }
        0x102a05f0 => { vm.decode(0x20867bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05f4u64) }
        0x102a05f4 => { vm.decode(0x8a7bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05f8u64) }
        0x102a05f8 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a05fcu64) }
        0x102a05fc => { vm.decode(0x228a5bafu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0600u64) }
        0x102a0600 => { vm.decode(0x714b0835u32); let d = 6i16; let s = 9i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0604u64) }
        0x102a0604 => { vm.decode(0xb10d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0608u64) }
        0x102a0608 => { vm.decode(0x414b9835u32); let d = 6i16; let s = 14i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a060cu64) }
        0x102a060c => { vm.decode(0xb3855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0610u64) }
        0x102a0610 => { vm.decode(0x610f1835u32); let d = 6i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0614u64) }
        0x102a0614 => { vm.decode(0xb08d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0618u64) }
        0x102a0618 => { vm.decode(0x10f1835u32); let d = 6i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a061cu64) }
        0x102a061c => { vm.decode(0xb00d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0620u64) }
        0x102a0620 => { vm.decode(0x274a9835u32); let d = 30i16; let s = 11i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0624u64) }
        0x102a0624 => { vm.decode(0x474aa835u32); let d = 31i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0628u64) }
        0x102a0628 => { vm.decode(0x193095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a062cu64) }
        0x102a062c => { vm.decode(0x40d29119u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a0630u64) }
        0x102a0630 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0634u64) }
        0x102a0634 => { vm.decode(0x46449835u32); let d = 24i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0638u64) }
        0x102a0638 => { vm.decode(0x2208a835u32); let d = 11i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a063cu64) }
        0x102a063c => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a0640u64) }
        0x102a0640 => { vm.decode(0x420c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 915i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a0644u64).wrapping_add((imm * 4) as u64) } else { 0x102a0644u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a0644u64) }
        0x102a0644 => { vm.decode(0x2004875u32); let d = 8i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0648u64) }
        0x102a0648 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a064cu64) }
        0x102a064c => { vm.decode(0x54a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0650u64) }
        0x102a0650 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0654u64) }
        0x102a0654 => { vm.decode(0x9b1055f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0658u64) }
        0x102a0658 => { vm.decode(0x1b3015f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a065cu64) }
        0x102a065c => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0660u64) }
        0x102a0660 => { vm.decode(0x5290c0f5u32); let d = 12i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0664u64) }
        0x102a0664 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0668u64) }
        0x102a0668 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a066cu64) }
        0x102a066c => { vm.decode(0xb381470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0670u64) }
        0x102a0670 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0674u64) }
        0x102a0674 => { vm.decode(0x2a8e636fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0678u64) }
        0x102a0678 => { vm.decode(0xa2e234f5u32); let d = 13i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a067cu64) }
        0x102a067c => { vm.decode(0x82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0680u64) }
        0x102a0680 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0684u64) }
        0x102a0684 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0688u64) }
        0x102a0688 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a068cu64) }
        0x102a068c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0690u64) }
        0x102a0690 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0694u64) }
        0x102a0694 => { vm.decode(0x2088e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0698u64) }
        0x102a0698 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a069cu64) }
        0x102a069c => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06a0u64) }
        0x102a06a0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a06a4u64) }
        0x102a06a4 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a06a8u64) }
        0x102a06a8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a06acu64) }
        0x102a06ac => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a06b0u64) }
        0x102a06b0 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a06b4u64) }
        0x102a06b4 => { vm.decode(0x3692c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06b8u64) }
        0x102a06b8 => { vm.decode(0x50d2a0f5u32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06bcu64) }
        0x102a06bc => { vm.decode(0x220d1122u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a06c0u64) }
        0x102a06c0 => { vm.decode(0x52e000f5u32); let d = 16i16; let s = 5i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06c4u64) }
        0x102a06c4 => { vm.decode(0x70c3a0f5u32); let d = 17i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06c8u64) }
        0x102a06c8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a06ccu64) }
        0x102a06cc => { vm.decode(0xa34424f5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a06d0u64) }
        0x102a06d0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a06d4u64) }
        0x102a06d4 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a06d8u64) }
        0x102a06d8 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a06dcu64) }
        0x102a06dc => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a06e0u64) }
        0x102a06e0 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a06e4u64) }
        0x102a06e4 => { vm.decode(0x54e0a0f5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06e8u64) }
        0x102a06e8 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a06ecu64) }
        0x102a06ec => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a06f0u64) }
        0x102a06f0 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a06f4u64) }
        0x102a06f4 => { vm.decode(0x62e234f5u32); let d = 13i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a06f8u64) }
        0x102a06f8 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a06fcu64) }
        0x102a06fc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0700u64) }
        0x102a0700 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0704u64) }
        0x102a0704 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0708u64) }
        0x102a0708 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a070cu64) }
        0x102a070c => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0710u64) }
        0x102a0710 => { vm.decode(0x44901875u32); let d = 6i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0714u64) }
        0x102a0714 => { vm.decode(0xab4434f5u32); let d = 19i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0718u64) }
        0x102a0718 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a071cu64) }
        0x102a071c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0720u64) }
        0x102a0720 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0724u64) }
        0x102a0724 => { vm.decode(0x32d200f5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0728u64) }
        0x102a0728 => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a072cu64) }
        0x102a072c => { vm.decode(0xc66cb2adu32); let d = 20i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0730u64) }
        0x102a0730 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0734u64) }
        0x102a0734 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0738u64) }
        0x102a0738 => { vm.decode(0x56a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a073cu64) }
        0x102a073c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0740u64) }
        0x102a0740 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0744u64) }
        0x102a0744 => { vm.decode(0x3603a835u32); let d = 25i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0748u64) }
        0x102a0748 => { vm.decode(0x50b1e0f5u32); let d = 12i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a074cu64) }
        0x102a074c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0750u64) }
        0x102a0750 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0754u64) }
        0x102a0754 => { vm.decode(0x12624f5u32); let d = 20i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0758u64) }
        0x102a0758 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a075cu64) }
        0x102a075c => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0760u64) }
        0x102a0760 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0764u64) }
        0x102a0764 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0768u64) }
        0x102a0768 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a076cu64) }
        0x102a076c => { vm.decode(0x33142835u32); let d = 13i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0770u64) }
        0x102a0770 => { vm.decode(0x3490a0f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0774u64) }
        0x102a0774 => { vm.decode(0x8b1814f0u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0778u64) }
        0x102a0778 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a077cu64) }
        0x102a077c => { vm.decode(0x6a8034f5u32); let d = 10i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0780u64) }
        0x102a0780 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0784u64) }
        0x102a0784 => { vm.decode(0x50462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0788u64) }
        0x102a0788 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a078cu64) }
        0x102a078c => { vm.decode(0x20a80a2fu32); let d = 11i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0790u64) }
        0x102a0790 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0794u64) }
        0x102a0794 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0798u64) }
        0x102a0798 => { vm.decode(0x6080402fu32); let d = 17i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a079cu64) }
        0x102a079c => { vm.decode(0x4088022fu32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07a0u64) }
        0x102a07a0 => { vm.decode(0x24501835u32); let d = 16i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a07a4u64) }
        0x102a07a4 => { vm.decode(0x70b1e0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a07a8u64) }
        0x102a07a8 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a07acu64) }
        0x102a07ac => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a07b0u64) }
        0x102a07b0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07b4u64) }
        0x102a07b4 => { vm.decode(0x5007a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a07b8u64) }
        0x102a07b8 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a07bcu64) }
        0x102a07bc => { vm.decode(0x688a4bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07c0u64) }
        0x102a07c0 => { vm.decode(0x76b3c0f5u32); let d = 15i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a07c4u64) }
        0x102a07c4 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a07c8u64) }
        0x102a07c8 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a07ccu64) }
        0x102a07cc => { vm.decode(0x60a8032fu32); let d = 19i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07d0u64) }
        0x102a07d0 => { vm.decode(0x43501835u32); let d = 12i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a07d4u64) }
        0x102a07d4 => { vm.decode(0x54b260f5u32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a07d8u64) }
        0x102a07d8 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a07dcu64) }
        0x102a07dc => { vm.decode(0xeae234f5u32); let d = 15i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a07e0u64) }
        0x102a07e0 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07e4u64) }
        0x102a07e4 => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a07e8u64) }
        0x102a07e8 => { vm.decode(0x20472835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a07ecu64) }
        0x102a07ec => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a07f0u64) }
        0x102a07f0 => { vm.decode(0x534f2835u32); let d = 15i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a07f4u64) }
        0x102a07f4 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a07f8u64) }
        0x102a07f8 => { vm.decode(0x14a020f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a07fcu64) }
        0x102a07fc => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0800u64) }
        0x102a0800 => { vm.decode(0x6a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0804u64) }
        0x102a0804 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0808u64) }
        0x102a0808 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a080cu64) }
        0x102a080c => { vm.decode(0x404c1835u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0810u64) }
        0x102a0810 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0814u64) }
        0x102a0814 => { vm.decode(0x56a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0818u64) }
        0x102a0818 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a081cu64) }
        0x102a081c => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0820u64) }
        0x102a0820 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0824u64) }
        0x102a0824 => { vm.decode(0x1b301470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0828u64) }
        0x102a0828 => { vm.decode(0x34b220f5u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a082cu64) }
        0x102a082c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0830u64) }
        0x102a0830 => { vm.decode(0xeac034f5u32); let d = 11i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0834u64) }
        0x102a0834 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0838u64) }
        0x102a0838 => { vm.decode(0x25462835u32); let d = 21i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a083cu64) }
        0x102a083c => { vm.decode(0x569380f5u32); let d = 11i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0840u64) }
        0x102a0840 => { vm.decode(0x42105875u32); let d = 10i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0844u64) }
        0x102a0844 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0848u64) }
        0x102a0848 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a084cu64) }
        0x102a084c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0850u64) }
        0x102a0850 => { vm.decode(0x549160f5u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0854u64) }
        0x102a0854 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0858u64) }
        0x102a0858 => { vm.decode(0x6aa234f5u32); let d = 14i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a085cu64) }
        0x102a085c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0860u64) }
        0x102a0860 => { vm.decode(0x524f2835u32); let d = 11i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0864u64) }
        0x102a0864 => { vm.decode(0x16b040f5u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0868u64) }
        0x102a0868 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a086cu64) }
        0x102a086c => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0870u64) }
        0x102a0870 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0874u64) }
        0x102a0874 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0878u64) }
        0x102a0878 => { vm.decode(0x14e020f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a087cu64) }
        0x102a087c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0880u64) }
        0x102a0880 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0884u64) }
        0x102a0884 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0888u64) }
        0x102a0888 => { vm.decode(0x145ca835u32); let d = 19i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a088cu64) }
        0x102a088c => { vm.decode(0x7691c0f5u32); let d = 14i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0890u64) }
        0x102a0890 => { vm.decode(0x40305875u32); let d = 2i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0894u64) }
        0x102a0894 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0898u64) }
        0x102a0898 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a089cu64) }
        0x102a089c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08a0u64) }
        0x102a08a0 => { vm.decode(0x74e1a0f5u32); let d = 18i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a08a4u64) }
        0x102a08a4 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a08a8u64) }
        0x102a08a8 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a08acu64) }
        0x102a08ac => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08b0u64) }
        0x102a08b0 => { vm.decode(0x135c8835u32); let d = 14i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a08b4u64) }
        0x102a08b4 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08b8u64) }
        0x102a08b8 => { vm.decode(0x36c000f5u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a08bcu64) }
        0x102a08bc => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a08c0u64) }
        0x102a08c0 => { vm.decode(0x490424f5u32); let d = 18i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a08c4u64) }
        0x102a08c4 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08c8u64) }
        0x102a08c8 => { vm.decode(0x428e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08ccu64) }
        0x102a08cc => { vm.decode(0x70c1e0f5u32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a08d0u64) }
        0x102a08d0 => { vm.decode(0xc66c22edu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a08d4u64) }
        0x102a08d4 => { vm.decode(0x232624f5u32); let d = 20i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a08d8u64) }
        0x102a08d8 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08dcu64) }
        0x102a08dc => { vm.decode(0x9b00d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a08e0u64) }
        0x102a08e0 => { vm.decode(0x4153835u32); let d = 17i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a08e4u64) }
        0x102a08e4 => { vm.decode(0x54e000f5u32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a08e8u64) }
        0x102a08e8 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a08ecu64) }
        0x102a08ec => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a08f0u64) }
        0x102a08f0 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a08f4u64) }
        0x102a08f4 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a08f8u64) }
        0x102a08f8 => { vm.decode(0x99289cf0u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a08fcu64) }
        0x102a08fc => { vm.decode(0x131c63b5u32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0900u64) }
        0x102a0900 => { vm.decode(0xb2855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0904u64) }
        0x102a0904 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0908u64) }
        0x102a0908 => { vm.decode(0x1b205430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a090cu64) }
        0x102a090c => { vm.decode(0x2282c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0910u64) }
        0x102a0910 => { vm.decode(0x690424f5u32); let d = 18i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0914u64) }
        0x102a0914 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0918u64) }
        0x102a0918 => { vm.decode(0x6a84cb6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a091cu64) }
        0x102a091c => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0920u64) }
        0x102a0920 => { vm.decode(0x16e000f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0924u64) }
        0x102a0924 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0928u64) }
        0x102a0928 => { vm.decode(0x410624f5u32); let d = 20i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a092cu64) }
        0x102a092c => { vm.decode(0x34c080f5u32); let d = 18i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0930u64) }
        0x102a0930 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0934u64) }
        0x102a0934 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0938u64) }
        0x102a0938 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a093cu64) }
        0x102a093c => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0940u64) }
        0x102a0940 => { vm.decode(0x250f1835u32); let d = 22i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0944u64) }
        0x102a0944 => { vm.decode(0xb109470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0948u64) }
        0x102a0948 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a094cu64) }
        0x102a094c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0950u64) }
        0x102a0950 => { vm.decode(0xaa2434f5u32); let d = 18i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0954u64) }
        0x102a0954 => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0958u64) }
        0x102a0958 => { vm.decode(0x50b3c0f5u32); let d = 13i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a095cu64) }
        0x102a095c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0960u64) }
        0x102a0960 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0964u64) }
        0x102a0964 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0968u64) }
        0x102a0968 => { vm.decode(0x5010a835u32); let d = 1i16; let s = 13i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a096cu64) }
        0x102a096c => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0970u64) }
        0x102a0970 => { vm.decode(0x309240f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0974u64) }
        0x102a0974 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0978u64) }
        0x102a0978 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a097cu64) }
        0x102a097c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0980u64) }
        0x102a0980 => { vm.decode(0x208063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0984u64) }
        0x102a0984 => { vm.decode(0x72008835u32); let d = 8i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0988u64) }
        0x102a0988 => { vm.decode(0x30b240f5u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a098cu64) }
        0x102a098c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0990u64) }
        0x102a0990 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0994u64) }
        0x102a0994 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0998u64) }
        0x102a0998 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a099cu64) }
        0x102a099c => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a09a0u64) }
        0x102a09a0 => { vm.decode(0x7093c0f5u32); let d = 13i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a09a4u64) }
        0x102a09a4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a09a8u64) }
        0x102a09a8 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a09acu64) }
        0x102a09ac => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09b0u64) }
        0x102a09b0 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09b4u64) }
        0x102a09b4 => { vm.decode(0x7300a835u32); let d = 13i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a09b8u64) }
        0x102a09b8 => { vm.decode(0x309140f5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a09bcu64) }
        0x102a09bc => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a09c0u64) }
        0x102a09c0 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a09c4u64) }
        0x102a09c4 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09c8u64) }
        0x102a09c8 => { vm.decode(0x3043a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a09ccu64) }
        0x102a09cc => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a09d0u64) }
        0x102a09d0 => { vm.decode(0x42806befu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09d4u64) }
        0x102a09d4 => { vm.decode(0x3492c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a09d8u64) }
        0x102a09d8 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a09dcu64) }
        0x102a09dc => { vm.decode(0xeae214f5u32); let d = 15i16; let s = 3i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a09e0u64) }
        0x102a09e0 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09e4u64) }
        0x102a09e4 => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09e8u64) }
        0x102a09e8 => { vm.decode(0x730ca835u32); let d = 15i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a09ecu64) }
        0x102a09ec => { vm.decode(0x50c100f5u32); let d = 16i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a09f0u64) }
        0x102a09f0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a09f4u64) }
        0x102a09f4 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a09f8u64) }
        0x102a09f8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a09fcu64) }
        0x102a09fc => { vm.decode(0x20113835u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a00u64) }
        0x102a0a00 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0a04u64) }
        0x102a0a04 => { vm.decode(0x2801875u32); let d = 12i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a08u64) }
        0x102a0a08 => { vm.decode(0x148100f5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0a0cu64) }
        0x102a0a0c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0a10u64) }
        0x102a0a10 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0a14u64) }
        0x102a0a14 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a18u64) }
        0x102a0a18 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a1cu64) }
        0x102a0a1c => { vm.decode(0x200c0835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a20u64) }
        0x102a0a20 => { vm.decode(0x228e43afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a24u64) }
        0x102a0a24 => { vm.decode(0x50e100f5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0a28u64) }
        0x102a0a28 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0a2cu64) }
        0x102a0a2c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0a30u64) }
        0x102a0a30 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a34u64) }
        0x102a0a34 => { vm.decode(0x433835u32); let d = 1i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a38u64) }
        0x102a0a38 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0a3cu64) }
        0x102a0a3c => { vm.decode(0x70e180f5u32); let d = 16i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0a40u64) }
        0x102a0a40 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0a44u64) }
        0x102a0a44 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0a48u64) }
        0x102a0a48 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a4cu64) }
        0x102a0a4c => { vm.decode(0x25503835u32); let d = 21i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a50u64) }
        0x102a0a50 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a54u64) }
        0x102a0a54 => { vm.decode(0x10e000f5u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0a58u64) }
        0x102a0a58 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0a5cu64) }
        0x102a0a5c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0a60u64) }
        0x102a0a60 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a64u64) }
        0x102a0a64 => { vm.decode(0x244a3835u32); let d = 19i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a68u64) }
        0x102a0a68 => { vm.decode(0x4a80c875u32); let d = 31i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a6cu64) }
        0x102a0a6c => { vm.decode(0x549200f5u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0a70u64) }
        0x102a0a70 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0a74u64) }
        0x102a0a74 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0a78u64) }
        0x102a0a78 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0a7cu64) }
        0x102a0a7c => { vm.decode(0x71140835u32); let d = 4i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a80u64) }
        0x102a0a80 => { vm.decode(0x938dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0a84u64) }
        0x102a0a84 => { vm.decode(0x52bb5u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0a88u64) }
        0x102a0a88 => { vm.decode(0x24d3022u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0a8cu64) }
        0x102a0a8c => { vm.decode(0x2209875u32); let d = 9i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0a90u64) }
        0x102a0a90 => { vm.decode(0xb181570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0a94u64) }
        0x102a0a94 => { vm.decode(0x401b2d35u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0a98u64) }
        0x102a0a98 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0a9cu64) }
        0x102a0a9c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0aa0u64) }
        0x102a0aa0 => { vm.decode(0x282e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0aa4u64) }
        0x102a0aa4 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0aa8u64) }
        0x102a0aa8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0aacu64) }
        0x102a0aac => { vm.decode(0x74160835u32); let d = 16i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ab0u64) }
        0x102a0ab0 => { vm.decode(0xd39c53b5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0ab4u64) }
        0x102a0ab4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0ab8u64) }
        0x102a0ab8 => { vm.decode(0xe8c034f5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0abcu64) }
        0x102a0abc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ac0u64) }
        0x102a0ac0 => { vm.decode(0x88ee36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ac4u64) }
        0x102a0ac4 => { vm.decode(0x554d2835u32); let d = 23i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ac8u64) }
        0x102a0ac8 => { vm.decode(0x6080482fu32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0accu64) }
        0x102a0acc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ad0u64) }
        0x102a0ad0 => { vm.decode(0x51581835u32); let d = 6i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ad4u64) }
        0x102a0ad4 => { vm.decode(0xb085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0ad8u64) }
        0x102a0ad8 => { vm.decode(0x639c73b5u32); let d = 25i16; let s = 30i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0adcu64) }
        0x102a0adc => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0ae0u64) }
        0x102a0ae0 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0ae4u64) }
        0x102a0ae4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ae8u64) }
        0x102a0ae8 => { vm.decode(0x54591835u32); let d = 18i16; let s = 22i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0aecu64) }
        0x102a0aec => { vm.decode(0x28886bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0af0u64) }
        0x102a0af0 => { vm.decode(0x30f300f5u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0af4u64) }
        0x102a0af4 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0af8u64) }
        0x102a0af8 => { vm.decode(0xab0614f5u32); let d = 22i16; let s = 3i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0afcu64) }
        0x102a0afc => { vm.decode(0xe39c73b5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b00u64) }
        0x102a0b00 => { vm.decode(0x9b3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0b04u64) }
        0x102a0b04 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b08u64) }
        0x102a0b08 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b0cu64) }
        0x102a0b0c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b10u64) }
        0x102a0b10 => { vm.decode(0x55021835u32); let d = 20i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b14u64) }
        0x102a0b14 => { vm.decode(0x40b00875u32); let d = 6i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b18u64) }
        0x102a0b18 => { vm.decode(0x721c13b5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b1cu64) }
        0x102a0b1c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b20u64) }
        0x102a0b20 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b24u64) }
        0x102a0b24 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b28u64) }
        0x102a0b28 => { vm.decode(0x2a8cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b2cu64) }
        0x102a0b2c => { vm.decode(0x374c0835u32); let d = 30i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b30u64) }
        0x102a0b30 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b34u64) }
        0x102a0b34 => { vm.decode(0x229c33b5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b38u64) }
        0x102a0b38 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b3cu64) }
        0x102a0b3c => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b40u64) }
        0x102a0b40 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b44u64) }
        0x102a0b44 => { vm.decode(0x700b3835u32); let d = 3i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b48u64) }
        0x102a0b48 => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0b4cu64) }
        0x102a0b4c => { vm.decode(0x886befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b50u64) }
        0x102a0b50 => { vm.decode(0x23dc13b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b54u64) }
        0x102a0b54 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b58u64) }
        0x102a0b58 => { vm.decode(0xa0e234f5u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b5cu64) }
        0x102a0b5c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b60u64) }
        0x102a0b60 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b64u64) }
        0x102a0b64 => { vm.decode(0x7048a835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b68u64) }
        0x102a0b68 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0b6cu64) }
        0x102a0b6c => { vm.decode(0x22824befu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b70u64) }
        0x102a0b70 => { vm.decode(0xa29c73b5u32); let d = 25i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b74u64) }
        0x102a0b74 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b78u64) }
        0x102a0b78 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b7cu64) }
        0x102a0b7c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b80u64) }
        0x102a0b80 => { vm.decode(0x76433835u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b84u64) }
        0x102a0b84 => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0b88u64) }
        0x102a0b88 => { vm.decode(0x92dc13b5u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0b8cu64) }
        0x102a0b8c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0b90u64) }
        0x102a0b90 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0b94u64) }
        0x102a0b94 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b98u64) }
        0x102a0b98 => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0b9cu64) }
        0x102a0b9c => { vm.decode(0x7004a835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ba0u64) }
        0x102a0ba0 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0ba4u64) }
        0x102a0ba4 => { vm.decode(0x93dc53b5u32); let d = 15i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0ba8u64) }
        0x102a0ba8 => { vm.decode(0x46805875u32); let d = 14i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0bacu64) }
        0x102a0bac => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0bb0u64) }
        0x102a0bb0 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0bb4u64) }
        0x102a0bb4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0bb8u64) }
        0x102a0bb8 => { vm.decode(0x5044a835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0bbcu64) }
        0x102a0bbc => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0bc0u64) }
        0x102a0bc0 => { vm.decode(0xb21c03b5u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0bc4u64) }
        0x102a0bc4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0bc8u64) }
        0x102a0bc8 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0bccu64) }
        0x102a0bcc => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0bd0u64) }
        0x102a0bd0 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0bd4u64) }
        0x102a0bd4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0bd8u64) }
        0x102a0bd8 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0bdcu64) }
        0x102a0bdc => { vm.decode(0x608043efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0be0u64) }
        0x102a0be0 => { vm.decode(0xd21c03b5u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0be4u64) }
        0x102a0be4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0be8u64) }
        0x102a0be8 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0becu64) }
        0x102a0bec => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0bf0u64) }
        0x102a0bf0 => { vm.decode(0x20542835u32); let d = 1i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0bf4u64) }
        0x102a0bf4 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0bf8u64) }
        0x102a0bf8 => { vm.decode(0x321c43b5u32); let d = 2i16; let s = 12i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0bfcu64) }
        0x102a0bfc => { vm.decode(0x4820c875u32); let d = 19i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c00u64) }
        0x102a0c00 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0c04u64) }
        0x102a0c04 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0c08u64) }
        0x102a0c08 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c0cu64) }
        0x102a0c0c => { vm.decode(0x20152835u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c10u64) }
        0x102a0c10 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0c14u64) }
        0x102a0c14 => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c18u64) }
        0x102a0c18 => { vm.decode(0x931c43b5u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0c1cu64) }
        0x102a0c1c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0c20u64) }
        0x102a0c20 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0c24u64) }
        0x102a0c24 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c28u64) }
        0x102a0c28 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c2cu64) }
        0x102a0c2c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c30u64) }
        0x102a0c30 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0c34u64) }
        0x102a0c34 => { vm.decode(0x1830bb5u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0c38u64) }
        0x102a0c38 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0c3cu64) }
        0x102a0c3c => { vm.decode(0x30f1022u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0c40u64) }
        0x102a0c40 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0c44u64) }
        0x102a0c44 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0c48u64) }
        0x102a0c48 => { vm.decode(0x208cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c4cu64) }
        0x102a0c4c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0c50u64) }
        0x102a0c50 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c54u64) }
        0x102a0c54 => { vm.decode(0x142835u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c58u64) }
        0x102a0c58 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0c5cu64) }
        0x102a0c5c => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c60u64) }
        0x102a0c60 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0c64u64) }
        0x102a0c64 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0c68u64) }
        0x102a0c68 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0c6cu64) }
        0x102a0c6c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c70u64) }
        0x102a0c70 => { vm.decode(0x2a8ceb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c74u64) }
        0x102a0c74 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0c78u64) }
        0x102a0c78 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0c7cu64) }
        0x102a0c7c => { vm.decode(0x28884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0c80u64) }
        0x102a0c80 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a0c84u64) }
        0x102a0c84 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a0c88u64) }
        0x102a0c88 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0c8cu64) }
        0x102a0c8c => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0c90u64) }
        0x102a0c90 => { vm.decode(0x230bb5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0c94u64) }
        0x102a0c94 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0c98u64) }
        0x102a0c98 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0c9cu64) }
        0x102a0c9c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0ca0u64) }
        0x102a0ca0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0ca4u64) }
        0x102a0ca4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0ca8u64) }
        0x102a0ca8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0cacu64) }
        0x102a0cac => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0cb0u64) }
        0x102a0cb0 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0cb4u64) }
        0x102a0cb4 => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0cb8u64) }
        0x102a0cb8 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0cbcu64) }
        0x102a0cbc => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0cc0u64) }
        0x102a0cc0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0cc4u64) }
        0x102a0cc4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0cc8u64) }
        0x102a0cc8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0cccu64) }
        0x102a0ccc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0cd0u64) }
        0x102a0cd0 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0cd4u64) }
        0x102a0cd4 => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0cd8u64) }
        0x102a0cd8 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0cdcu64) }
        0x102a0cdc => { vm.decode(0x208263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ce0u64) }
        0x102a0ce0 => { vm.decode(0x65012835u32); let d = 21i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ce4u64) }
        0x102a0ce4 => { vm.decode(0x80010bb5u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0ce8u64) }
        0x102a0ce8 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0cecu64) }
        0x102a0cec => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0cf0u64) }
        0x102a0cf0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0cf4u64) }
        0x102a0cf4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0cf8u64) }
        0x102a0cf8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0cfcu64) }
        0x102a0cfc => { vm.decode(0x45192835u32); let d = 23i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d00u64) }
        0x102a0d00 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0d04u64) }
        0x102a0d04 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0d08u64) }
        0x102a0d08 => { vm.decode(0xa001875u32); let d = 24i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d0cu64) }
        0x102a0d0c => { vm.decode(0x62357062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0d10u64) }
        0x102a0d10 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0d14u64) }
        0x102a0d14 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0d18u64) }
        0x102a0d18 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0d1cu64) }
        0x102a0d1c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d20u64) }
        0x102a0d20 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d24u64) }
        0x102a0d24 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0d28u64) }
        0x102a0d28 => { vm.decode(0x40e12bb5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0d2cu64) }
        0x102a0d2c => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0d30u64) }
        0x102a0d30 => { vm.decode(0x21d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0d34u64) }
        0x102a0d34 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0d38u64) }
        0x102a0d38 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0d3cu64) }
        0x102a0d3c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0d40u64) }
        0x102a0d40 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d44u64) }
        0x102a0d44 => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d48u64) }
        0x102a0d48 => { vm.decode(0x64090835u32); let d = 18i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d4cu64) }
        0x102a0d4c => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d50u64) }
        0x102a0d50 => { vm.decode(0x1840bb5u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0d54u64) }
        0x102a0d54 => { vm.decode(0x40126d35u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0d58u64) }
        0x102a0d58 => { vm.decode(0x42471062u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0d5cu64) }
        0x102a0d5c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0d60u64) }
        0x102a0d60 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0d64u64) }
        0x102a0d64 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0d68u64) }
        0x102a0d68 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d6cu64) }
        0x102a0d6c => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d70u64) }
        0x102a0d70 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d74u64) }
        0x102a0d74 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0d78u64) }
        0x102a0d78 => { vm.decode(0x1472bb5u32); let d = 15i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0d7cu64) }
        0x102a0d7c => { vm.decode(0x60926d35u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0d80u64) }
        0x102a0d80 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0d84u64) }
        0x102a0d84 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0d88u64) }
        0x102a0d88 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0d8cu64) }
        0x102a0d8c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0d90u64) }
        0x102a0d90 => { vm.decode(0x65120835u32); let d = 20i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0d94u64) }
        0x102a0d94 => { vm.decode(0x41800bb5u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0d98u64) }
        0x102a0d98 => { vm.decode(0x400b4d35u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0d9cu64) }
        0x102a0d9c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0da0u64) }
        0x102a0da0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0da4u64) }
        0x102a0da4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0da8u64) }
        0x102a0da8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0dacu64) }
        0x102a0dac => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0db0u64) }
        0x102a0db0 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0db4u64) }
        0x102a0db4 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0db8u64) }
        0x102a0db8 => { vm.decode(0xa70bb5u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0dbcu64) }
        0x102a0dbc => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0dc0u64) }
        0x102a0dc0 => { vm.decode(0x4480c875u32); let d = 7i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0dc4u64) }
        0x102a0dc4 => { vm.decode(0x63751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0dc8u64) }
        0x102a0dc8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0dccu64) }
        0x102a0dcc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0dd0u64) }
        0x102a0dd0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0dd4u64) }
        0x102a0dd4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0dd8u64) }
        0x102a0dd8 => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ddcu64) }
        0x102a0ddc => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0de0u64) }
        0x102a0de0 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0de4u64) }
        0x102a0de4 => { vm.decode(0x2a82cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0de8u64) }
        0x102a0de8 => { vm.decode(0xa62bb5u32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0decu64) }
        0x102a0dec => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0df0u64) }
        0x102a0df0 => { vm.decode(0x43453062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0df4u64) }
        0x102a0df4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0df8u64) }
        0x102a0df8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0dfcu64) }
        0x102a0dfc => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0e00u64) }
        0x102a0e00 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e04u64) }
        0x102a0e04 => { vm.decode(0x2284e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e08u64) }
        0x102a0e08 => { vm.decode(0x36042835u32); let d = 25i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0e0cu64) }
        0x102a0e0c => { vm.decode(0x408b4d35u32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0e10u64) }
        0x102a0e10 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0e14u64) }
        0x102a0e14 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0e18u64) }
        0x102a0e18 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0e1cu64) }
        0x102a0e1c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e20u64) }
        0x102a0e20 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e24u64) }
        0x102a0e24 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0e28u64) }
        0x102a0e28 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0e2cu64) }
        0x102a0e2c => { vm.decode(0x232bb5u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0e30u64) }
        0x102a0e30 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0e34u64) }
        0x102a0e34 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0e38u64) }
        0x102a0e38 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0e3cu64) }
        0x102a0e3c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0e40u64) }
        0x102a0e40 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0e44u64) }
        0x102a0e44 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e48u64) }
        0x102a0e48 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e4cu64) }
        0x102a0e4c => { vm.decode(0x370c2835u32); let d = 31i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0e50u64) }
        0x102a0e50 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e54u64) }
        0x102a0e54 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0e58u64) }
        0x102a0e58 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0e5cu64) }
        0x102a0e5c => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a0e60u64) }
        0x102a0e60 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0e64u64) }
        0x102a0e64 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0e68u64) }
        0x102a0e68 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0e6cu64) }
        0x102a0e6c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e70u64) }
        0x102a0e70 => { vm.decode(0x88ac36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e74u64) }
        0x102a0e74 => { vm.decode(0x114d0835u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0e78u64) }
        0x102a0e78 => { vm.decode(0x9305db0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0e7cu64) }
        0x102a0e7c => { vm.decode(0x524a4349u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0e80u64) }
        0x102a0e80 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0e84u64) }
        0x102a0e84 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0e88u64) }
        0x102a0e88 => { vm.decode(0xc109875u32); let d = 17i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0e8cu64) }
        0x102a0e8c => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e90u64) }
        0x102a0e90 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0e94u64) }
        0x102a0e94 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e98u64) }
        0x102a0e98 => { vm.decode(0x2884cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0e9cu64) }
        0x102a0e9c => { vm.decode(0x340e2835u32); let d = 19i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ea0u64) }
        0x102a0ea0 => { vm.decode(0x1c52bb5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0ea4u64) }
        0x102a0ea4 => { vm.decode(0x600b6d35u32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0ea8u64) }
        0x102a0ea8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0eacu64) }
        0x102a0eac => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0eb0u64) }
        0x102a0eb0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0eb4u64) }
        0x102a0eb4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0eb8u64) }
        0x102a0eb8 => { vm.decode(0x2880eb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ebcu64) }
        0x102a0ebc => { vm.decode(0x350e0835u32); let d = 22i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ec0u64) }
        0x102a0ec0 => { vm.decode(0x2884eb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ec4u64) }
        0x102a0ec4 => { vm.decode(0x80b2fu32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ec8u64) }
        0x102a0ec8 => { vm.decode(0x32152835u32); let d = 9i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0eccu64) }
        0x102a0ecc => { vm.decode(0x19209c70u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0ed0u64) }
        0x102a0ed0 => { vm.decode(0x504d4349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0ed4u64) }
        0x102a0ed4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0ed8u64) }
        0x102a0ed8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0edcu64) }
        0x102a0edc => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0ee0u64) }
        0x102a0ee0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ee4u64) }
        0x102a0ee4 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0ee8u64) }
        0x102a0ee8 => { vm.decode(0x15442835u32); let d = 21i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0eecu64) }
        0x102a0eec => { vm.decode(0x544e4349u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0ef0u64) }
        0x102a0ef0 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0ef4u64) }
        0x102a0ef4 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0ef8u64) }
        0x102a0ef8 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0efcu64) }
        0x102a0efc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f00u64) }
        0x102a0f00 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f04u64) }
        0x102a0f04 => { vm.decode(0x16440835u32); let d = 24i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0f08u64) }
        0x102a0f08 => { vm.decode(0x41c20bb5u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a0f0cu64) }
        0x102a0f0c => { vm.decode(0x608b4d35u32); let d = 10i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a0f10u64) }
        0x102a0f10 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f14u64) }
        0x102a0f14 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0f18u64) }
        0x102a0f18 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0f1cu64) }
        0x102a0f1c => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0f20u64) }
        0x102a0f20 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f24u64) }
        0x102a0f24 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0f28u64) }
        0x102a0f28 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0f2cu64) }
        0x102a0f2c => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f30u64) }
        0x102a0f30 => { vm.decode(0x105c9835u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0f34u64) }
        0x102a0f34 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0f38u64) }
        0x102a0f38 => { vm.decode(0x407f5349u32); let d = 18i16; let s = 15i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0f3cu64) }
        0x102a0f3c => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f40u64) }
        0x102a0f40 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0f44u64) }
        0x102a0f44 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0f48u64) }
        0x102a0f48 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f4cu64) }
        0x102a0f4c => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f50u64) }
        0x102a0f50 => { vm.decode(0x4c1835u32); let d = 2i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0f54u64) }
        0x102a0f54 => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0f58u64) }
        0x102a0f58 => { vm.decode(0x406f5349u32); let d = 18i16; let s = 14i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0f5cu64) }
        0x102a0f5c => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f60u64) }
        0x102a0f60 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0f64u64) }
        0x102a0f64 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0f68u64) }
        0x102a0f68 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f6cu64) }
        0x102a0f6c => { vm.decode(0x205d1835u32); let d = 2i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0f70u64) }
        0x102a0f70 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0f74u64) }
        0x102a0f74 => { vm.decode(0x406e5349u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0f78u64) }
        0x102a0f78 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f7cu64) }
        0x102a0f7c => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0f80u64) }
        0x102a0f80 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0f84u64) }
        0x102a0f84 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0f88u64) }
        0x102a0f88 => { vm.decode(0x1d1835u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0f8cu64) }
        0x102a0f8c => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0f90u64) }
        0x102a0f90 => { vm.decode(0xd105a11u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0f94u64) }
        0x102a0f94 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0f98u64) }
        0x102a0f98 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0f9cu64) }
        0x102a0f9c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0fa0u64) }
        0x102a0fa0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0fa4u64) }
        0x102a0fa4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0fa8u64) }
        0x102a0fa8 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0facu64) }
        0x102a0fac => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0fb0u64) }
        0x102a0fb0 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0fb4u64) }
        0x102a0fb4 => { vm.decode(0x5d98e911u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a0fb8u64) }
        0x102a0fb8 => { vm.decode(0x447d4349u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0fbcu64) }
        0x102a0fbc => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0fc0u64) }
        0x102a0fc0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0fc4u64) }
        0x102a0fc4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0fc8u64) }
        0x102a0fc8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0fccu64) }
        0x102a0fcc => { vm.decode(0x201e0835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0fd0u64) }
        0x102a0fd0 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0fd4u64) }
        0x102a0fd4 => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0fd8u64) }
        0x102a0fd8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0fdcu64) }
        0x102a0fdc => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a0fe0u64) }
        0x102a0fe0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a0fe4u64) }
        0x102a0fe4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0fe8u64) }
        0x102a0fe8 => { vm.decode(0x2280e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a0fecu64) }
        0x102a0fec => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a0ff0u64) }
        0x102a0ff0 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a0ff4u64) }
        0x102a0ff4 => { vm.decode(0x407c4349u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a0ff8u64) }
        0x102a0ff8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a0ffcu64) }
        0x102a0ffc => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1000u64) }
        0x102a1000 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1004u64) }
        0x102a1004 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1008u64) }
        0x102a1008 => { vm.decode(0x2a84e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a100cu64) }
        0x102a100c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1010u64) }
        0x102a1010 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1014u64) }
        0x102a1014 => { vm.decode(0x407d4349u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1018u64) }
        0x102a1018 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a101cu64) }
        0x102a101c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1020u64) }
        0x102a1020 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1024u64) }
        0x102a1024 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1028u64) }
        0x102a1028 => { vm.decode(0x20572835u32); let d = 1i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a102cu64) }
        0x102a102c => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1030u64) }
        0x102a1030 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1034u64) }
        0x102a1034 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1038u64) }
        0x102a1038 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a103cu64) }
        0x102a103c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1040u64) }
        0x102a1040 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1044u64) }
        0x102a1044 => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1048u64) }
        0x102a1048 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a104cu64) }
        0x102a104c => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1050u64) }
        0x102a1050 => { vm.decode(0xdf4349u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1054u64) }
        0x102a1054 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1058u64) }
        0x102a1058 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a105cu64) }
        0x102a105c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1060u64) }
        0x102a1060 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1064u64) }
        0x102a1064 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1068u64) }
        0x102a1068 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a106cu64) }
        0x102a106c => { vm.decode(0xd2ca916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1070u64) }
        0x102a1070 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1074u64) }
        0x102a1074 => { vm.decode(0x628ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1078u64) }
        0x102a1078 => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a107cu64) }
        0x102a107c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1080u64) }
        0x102a1080 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1084u64) }
        0x102a1084 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1088u64) }
        0x102a1088 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a108cu64) }
        0x102a108c => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1090u64) }
        0x102a1090 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1094u64) }
        0x102a1094 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1098u64) }
        0x102a1098 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a109cu64) }
        0x102a109c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a10a0u64) }
        0x102a10a0 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a10a4u64) }
        0x102a10a4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a10a8u64) }
        0x102a10a8 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a10acu64) }
        0x102a10ac => { vm.decode(0x2fb4349u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a10b0u64) }
        0x102a10b0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a10b4u64) }
        0x102a10b4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a10b8u64) }
        0x102a10b8 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a10bcu64) }
        0x102a10bc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a10c0u64) }
        0x102a10c0 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a10c4u64) }
        0x102a10c4 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a10c8u64) }
        0x102a10c8 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a10ccu64) }
        0x102a10cc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a10d0u64) }
        0x102a10d0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a10d4u64) }
        0x102a10d4 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a10d8u64) }
        0x102a10d8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a10dcu64) }
        0x102a10dc => { vm.decode(0x60112835u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a10e0u64) }
        0x102a10e0 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a10e4u64) }
        0x102a10e4 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a10e8u64) }
        0x102a10e8 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a10ecu64) }
        0x102a10ec => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a10f0u64) }
        0x102a10f0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a10f4u64) }
        0x102a10f4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a10f8u64) }
        0x102a10f8 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a10fcu64) }
        0x102a10fc => { vm.decode(0x2020022fu32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1100u64) }
        0x102a1100 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1104u64) }
        0x102a1104 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1108u64) }
        0x102a1108 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a110cu64) }
        0x102a110c => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1110u64) }
        0x102a1110 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1114u64) }
        0x102a1114 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1118u64) }
        0x102a1118 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a111cu64) }
        0x102a111c => { vm.decode(0x208c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1120u64) }
        0x102a1120 => { vm.decode(0xda5ac12du32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1124u64) }
        0x102a1124 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1128u64) }
        0x102a1128 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a112cu64) }
        0x102a112c => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1130u64) }
        0x102a1130 => { vm.decode(0xdacec12du32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1134u64) }
        0x102a1134 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1138u64) }
        0x102a1138 => { vm.decode(0xdadec12du32); let d = 15i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a113cu64) }
        0x102a113c => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1140u64) }
        0x102a1140 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1144u64) }
        0x102a1144 => { vm.decode(0x228c63afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1148u64) }
        0x102a1148 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a114cu64) }
        0x102a114c => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1150u64) }
        0x102a1150 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1154u64) }
        0x102a1154 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1158u64) }
        0x102a1158 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a115cu64) }
        0x102a115c => { vm.decode(0xb148ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1160u64) }
        0x102a1160 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1164u64) }
        0x102a1164 => { vm.decode(0xe3408d75u32); let d = 21i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1168u64) }
        0x102a1168 => { vm.decode(0xe000cd75u32); let d = 20i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a116cu64) }
        0x102a116c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1170u64) }
        0x102a1170 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1174u64) }
        0x102a1174 => { vm.decode(0xa1087d75u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1178u64) }
        0x102a1178 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a117cu64) }
        0x102a117c => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1180u64) }
        0x102a1180 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1184u64) }
        0x102a1184 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1188u64) }
        0x102a1188 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a118cu64) }
        0x102a118c => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1190u64) }
        0x102a1190 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1194u64) }
        0x102a1194 => { vm.decode(0xd25ec1adu32); let d = 22i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a1198u64) }
        0x102a1198 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a119cu64) }
        0x102a119c => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a11a0u64) }
        0x102a11a0 => { vm.decode(0xf3084d75u32); let d = 24i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11a4u64) }
        0x102a11a4 => { vm.decode(0xb2405d75u32); let d = 3i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11a8u64) }
        0x102a11a8 => { vm.decode(0xa000cd75u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11acu64) }
        0x102a11ac => { vm.decode(0xa140ed75u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11b0u64) }
        0x102a11b0 => { vm.decode(0xf1009d75u32); let d = 22i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11b4u64) }
        0x102a11b4 => { vm.decode(0xda6e812du32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a11b8u64) }
        0x102a11b8 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a11bcu64) }
        0x102a11bc => { vm.decode(0x2225b4f5u32); let d = 16i16; let s = 12i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11c0u64) }
        0x102a11c0 => { vm.decode(0x62c5b4f5u32); let d = 25i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11c4u64) }
        0x102a11c4 => { vm.decode(0xa283a4f5u32); let d = 12i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11c8u64) }
        0x102a11c8 => { vm.decode(0x6a81a4f5u32); let d = 10i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11ccu64) }
        0x102a11cc => { vm.decode(0x2361a4f5u32); let d = 1i16; let s = 20i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11d0u64) }
        0x102a11d0 => { vm.decode(0x6327a4f5u32); let d = 20i16; let s = 22i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11d4u64) }
        0x102a11d4 => { vm.decode(0xab21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11d8u64) }
        0x102a11d8 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a11dcu64) }
        0x102a11dc => { vm.decode(0xeb01a4f5u32); let d = 2i16; let s = 19i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11e0u64) }
        0x102a11e0 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a11e4u64) }
        0x102a11e4 => { vm.decode(0xaa65b4f5u32); let d = 19i16; let s = 13i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11e8u64) }
        0x102a11e8 => { vm.decode(0xeae3b4f5u32); let d = 15i16; let s = 15i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11ecu64) }
        0x102a11ec => { vm.decode(0x6b01a4f5u32); let d = 2i16; let s = 18i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11f0u64) }
        0x102a11f0 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a11f4u64) }
        0x102a11f4 => { vm.decode(0xa267a4f5u32); let d = 21i16; let s = 5i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11f8u64) }
        0x102a11f8 => { vm.decode(0x2a25a4f5u32); let d = 18i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a11fcu64) }
        0x102a11fc => { vm.decode(0xe2c3a4f5u32); let d = 13i16; let s = 3i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1200u64) }
        0x102a1200 => { vm.decode(0x2b01b4f5u32); let d = 2i16; let s = 24i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1204u64) }
        0x102a1204 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1208u64) }
        0x102a1208 => { vm.decode(0x45906011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a120cu64) }
        0x102a120c => { vm.decode(0x42494349u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1210u64) }
        0x102a1210 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1214u64) }
        0x102a1214 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1218u64) }
        0x102a1218 => { vm.decode(0x4a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a121cu64) }
        0x102a121c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1220u64) }
        0x102a1220 => { vm.decode(0x208ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1224u64) }
        0x102a1224 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1228u64) }
        0x102a1228 => { vm.decode(0x42eb4349u32); let d = 3i16; let s = 14i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a122cu64) }
        0x102a122c => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1230u64) }
        0x102a1230 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1234u64) }
        0x102a1234 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1238u64) }
        0x102a1238 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a123cu64) }
        0x102a123c => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1240u64) }
        0x102a1240 => { vm.decode(0x430d0835u32); let d = 14i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1244u64) }
        0x102a1244 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1248u64) }
        0x102a1248 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a124cu64) }
        0x102a124c => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1250u64) }
        0x102a1250 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1254u64) }
        0x102a1254 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1258u64) }
        0x102a1258 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a125cu64) }
        0x102a125c => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1260u64) }
        0x102a1260 => { vm.decode(0x25bc349u32); let d = 4i16; let s = 11i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1264u64) }
        0x102a1264 => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1268u64) }
        0x102a1268 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a126cu64) }
        0x102a126c => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1270u64) }
        0x102a1270 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1274u64) }
        0x102a1274 => { vm.decode(0x2886e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1278u64) }
        0x102a1278 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a127cu64) }
        0x102a127c => { vm.decode(0xd104311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a1280u64) }
        0x102a1280 => { vm.decode(0x2f8c349u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1284u64) }
        0x102a1284 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1288u64) }
        0x102a1288 => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a128cu64) }
        0x102a128c => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1290u64) }
        0x102a1290 => { vm.decode(0x2082012fu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1294u64) }
        0x102a1294 => { vm.decode(0x2880cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1298u64) }
        0x102a1298 => { vm.decode(0x6102a835u32); let d = 5i16; let s = 5i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a129cu64) }
        0x102a129c => { vm.decode(0x52ca4349u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a12a0u64) }
        0x102a12a0 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a12a4u64) }
        0x102a12a4 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a12a8u64) }
        0x102a12a8 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a12acu64) }
        0x102a12ac => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12b0u64) }
        0x102a12b0 => { vm.decode(0x284eb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12b4u64) }
        0x102a12b4 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a12b8u64) }
        0x102a12b8 => { vm.decode(0x5907811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a12bcu64) }
        0x102a12bc => { vm.decode(0x124a5349u32); let d = 24i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a12c0u64) }
        0x102a12c0 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a12c4u64) }
        0x102a12c4 => { vm.decode(0xe1084d75u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a12c8u64) }
        0x102a12c8 => { vm.decode(0x38434f5u32); let d = 24i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a12ccu64) }
        0x102a12cc => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12d0u64) }
        0x102a12d0 => { vm.decode(0x80eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12d4u64) }
        0x102a12d4 => { vm.decode(0x16021835u32); let d = 24i16; let s = 8i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a12d8u64) }
        0x102a12d8 => { vm.decode(0x518f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a12dcu64) }
        0x102a12dc => { vm.decode(0x2ca5349u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a12e0u64) }
        0x102a12e0 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a12e4u64) }
        0x102a12e4 => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a12e8u64) }
        0x102a12e8 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a12ecu64) }
        0x102a12ec => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12f0u64) }
        0x102a12f0 => { vm.decode(0x280eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a12f4u64) }
        0x102a12f4 => { vm.decode(0x44023835u32); let d = 17i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a12f8u64) }
        0x102a12f8 => { vm.decode(0x46f9d349u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a12fcu64) }
        0x102a12fc => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1300u64) }
        0x102a1300 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1304u64) }
        0x102a1304 => { vm.decode(0xcb6624f5u32); let d = 23i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1308u64) }
        0x102a1308 => { vm.decode(0x60aa412fu32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a130cu64) }
        0x102a130c => { vm.decode(0x884cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1310u64) }
        0x102a1310 => { vm.decode(0x4206b835u32); let d = 9i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1314u64) }
        0x102a1314 => { vm.decode(0x52f9d349u32); let d = 31i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1318u64) }
        0x102a1318 => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a131cu64) }
        0x102a131c => { vm.decode(0xf348fd75u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1320u64) }
        0x102a1320 => { vm.decode(0xcbe634f5u32); let d = 31i16; let s = 6i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1324u64) }
        0x102a1324 => { vm.decode(0x60aa4b2fu32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1328u64) }
        0x102a1328 => { vm.decode(0x288ae36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a132cu64) }
        0x102a132c => { vm.decode(0x774db835u32); let d = 31i16; let s = 7i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1330u64) }
        0x102a1330 => { vm.decode(0x4d9ce911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a1334u64) }
        0x102a1334 => { vm.decode(0x4279d349u32); let d = 22i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1338u64) }
        0x102a1338 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a133cu64) }
        0x102a133c => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1340u64) }
        0x102a1340 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1344u64) }
        0x102a1344 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1348u64) }
        0x102a1348 => { vm.decode(0x288ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a134cu64) }
        0x102a134c => { vm.decode(0x254d9835u32); let d = 22i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1350u64) }
        0x102a1350 => { vm.decode(0x4d10c311u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a1354u64) }
        0x102a1354 => { vm.decode(0x5279d349u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1358u64) }
        0x102a1358 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a135cu64) }
        0x102a135c => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1360u64) }
        0x102a1360 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a1364u64) }
        0x102a1364 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1368u64) }
        0x102a1368 => { vm.decode(0x228ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a136cu64) }
        0x102a136c => { vm.decode(0x374d9835u32); let d = 30i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1370u64) }
        0x102a1370 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1374u64) }
        0x102a1374 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1378u64) }
        0x102a1378 => { vm.decode(0x208e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a137cu64) }
        0x102a137c => { vm.decode(0x20a8092fu32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1380u64) }
        0x102a1380 => { vm.decode(0xa80cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1384u64) }
        0x102a1384 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1388u64) }
        0x102a1388 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a138cu64) }
        0x102a138c => { vm.decode(0x350d2835u32); let d = 23i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1390u64) }
        0x102a1390 => { vm.decode(0x2082022fu32); let d = 5i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1394u64) }
        0x102a1394 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1398u64) }
        0x102a1398 => { vm.decode(0x2002432fu32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a139cu64) }
        0x102a139c => { vm.decode(0x62408835u32); let d = 8i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a13a0u64) }
        0x102a13a0 => { vm.decode(0x2008402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13a4u64) }
        0x102a13a4 => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a13a8u64) }
        0x102a13a8 => { vm.decode(0x404f2835u32); let d = 3i16; let s = 3i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a13acu64) }
        0x102a13ac => { vm.decode(0x2082402fu32); let d = 5i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13b0u64) }
        0x102a13b0 => { vm.decode(0x45408835u32); let d = 20i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a13b4u64) }
        0x102a13b4 => { vm.decode(0x42859019u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a13b8u64) }
        0x102a13b8 => { vm.decode(0x1b0855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a13bcu64) }
        0x102a13bc => { vm.decode(0x44a09875u32); let d = 7i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a13c0u64) }
        0x102a13c0 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a13c4u64) }
        0x102a13c4 => { vm.decode(0xa8032fu32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13c8u64) }
        0x102a13c8 => { vm.decode(0x2082032fu32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13ccu64) }
        0x102a13cc => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13d0u64) }
        0x102a13d0 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13d4u64) }
        0x102a13d4 => { vm.decode(0x8824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13d8u64) }
        0x102a13d8 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13dcu64) }
        0x102a13dc => { vm.decode(0x2082492fu32); let d = 13i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13e0u64) }
        0x102a13e0 => { vm.decode(0x88a6bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13e4u64) }
        0x102a13e4 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13e8u64) }
        0x102a13e8 => { vm.decode(0x228e6befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13ecu64) }
        0x102a13ec => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13f0u64) }
        0x102a13f0 => { vm.decode(0x6880eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13f4u64) }
        0x102a13f4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13f8u64) }
        0x102a13f8 => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a13fcu64) }
        0x102a13fc => { vm.decode(0x535ca835u32); let d = 15i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1400u64) }
        0x102a1400 => { vm.decode(0x4a88c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1404u64) }
        0x102a1404 => { vm.decode(0x145c8835u32); let d = 18i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1408u64) }
        0x102a1408 => { vm.decode(0x5353a835u32); let d = 13i16; let s = 13i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a140cu64) }
        0x102a140c => { vm.decode(0x1459a835u32); let d = 19i16; let s = 12i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1410u64) }
        0x102a1410 => { vm.decode(0x725f2835u32); let d = 11i16; let s = 11i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1414u64) }
        0x102a1414 => { vm.decode(0x35062835u32); let d = 21i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1418u64) }
        0x102a1418 => { vm.decode(0x6110a835u32); let d = 5i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a141cu64) }
        0x102a141c => { vm.decode(0x2162835u32); let d = 9i16; let s = 2i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1420u64) }
        0x102a1420 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1424u64) }
        0x102a1424 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1428u64) }
        0x102a1428 => { vm.decode(0x459cf011u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a142cu64) }
        0x102a142c => { vm.decode(0x524b4349u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102a1430u64) }
        0x102a1430 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a1434u64) }
        0x102a1434 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a1438u64) }
        0x102a1438 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a143cu64) }
        0x102a143c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1440u64) }
        0x102a1440 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1444u64) }
        0x102a1444 => { vm.decode(0xa8a43afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1448u64) }
        0x102a1448 => { vm.decode(0xa8082fu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a144cu64) }
        0x102a144c => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1450u64) }
        0x102a1450 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1454u64) }
        0x102a1454 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1458u64) }
        0x102a1458 => { vm.decode(0x60099835u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a145cu64) }
        0x102a145c => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1460u64) }
        0x102a1460 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1464u64) }
        0x102a1464 => { vm.decode(0x604e1835u32); let d = 2i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1468u64) }
        0x102a1468 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a146cu64) }
        0x102a146c => { vm.decode(0x200f1835u32); let d = 2i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1470u64) }
        0x102a1470 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1474u64) }
        0x102a1474 => { vm.decode(0x274b9835u32); let d = 30i16; let s = 15i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1478u64) }
        0x102a1478 => { vm.decode(0x37482835u32); let d = 31i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a147cu64) }
        0x102a147c => { vm.decode(0x46059835u32); let d = 24i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1480u64) }
        0x102a1480 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1484u64) }
        0x102a1484 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a1488u64) }
        0x102a1488 => { vm.decode(0xa78329f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -913i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a148cu64).wrapping_add((imm * 4) as u64) } else { 0x102a148cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a148cu64) }
        0x102a148c => { vm.decode(0x6005875u32); let d = 8i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1490u64) }
        0x102a1490 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1494u64) }
        0x102a1494 => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1498u64) }
        0x102a1498 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a149cu64) }
        0x102a149c => { vm.decode(0x401e2835u32); let d = 3i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14a0u64) }
        0x102a14a0 => { vm.decode(0x205f0835u32); let d = 2i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14a4u64) }
        0x102a14a4 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14a8u64) }
        0x102a14a8 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14acu64) }
        0x102a14ac => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14b0u64) }
        0x102a14b0 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14b4u64) }
        0x102a14b4 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14b8u64) }
        0x102a14b8 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14bcu64) }
        0x102a14bc => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14c0u64) }
        0x102a14c0 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14c4u64) }
        0x102a14c4 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14c8u64) }
        0x102a14c8 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14ccu64) }
        0x102a14cc => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14d0u64) }
        0x102a14d0 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14d4u64) }
        0x102a14d4 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14d8u64) }
        0x102a14d8 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14dcu64) }
        0x102a14dc => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14e0u64) }
        0x102a14e0 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a14e4u64) }
        0x102a14e4 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a14e8u64) }
        0x102a14e8 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a14ecu64) }
        0x102a14ec => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a14f0u64) }
        0x102a14f0 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a14f4u64) }
        0x102a14f4 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a14f8u64) }
        0x102a14f8 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a14fcu64) }
        0x102a14fc => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1500u64) }
        0x102a1500 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1504u64) }
        0x102a1504 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1508u64) }
        0x102a1508 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a150cu64) }
        0x102a150c => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1510u64) }
        0x102a1510 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1514u64) }
        0x102a1514 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1518u64) }
        0x102a1518 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a151cu64) }
        0x102a151c => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1520u64) }
        0x102a1520 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1524u64) }
        0x102a1524 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1528u64) }
        0x102a1528 => { vm.decode(0x949119u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a152cu64) }
        0x102a152c => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1530u64) }
        0x102a1530 => { vm.decode(0x82c42099u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a1534u64) }
        0x102a1534 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a1538u64) }
        0x102a1538 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a153cu64) }
        0x102a153c => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1540u64) }
        0x102a1540 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1544u64) }
        0x102a1544 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1548u64) }
        0x102a1548 => { vm.decode(0x2886e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a154cu64) }
        0x102a154c => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a1550u64) }
        0x102a1550 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1554u64) }
        0x102a1554 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a1558u64) }
        0x102a1558 => { vm.decode(0xc8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a155cu64).wrapping_add((imm * 4) as u64) } else { 0x102a155cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a155cu64) }
        0x102a155c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a1560u64) } }
        0x102a1560 => { vm.decode(0x10a080a3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102a1564u64) }
        0x102a1564 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a1568u64) }
        0x102a1568 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a156cu64) }
        0x102a156c => { vm.decode(0x22200f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a1570u64) }
        0x102a1570 => { vm.decode(0xdd9e77b3u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a1574u64) }
        0x102a1574 => { vm.decode(0xc77ef59u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a1578u64) }
        0x102a1578 => { vm.decode(0x42848019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a157cu64) }
        0x102a157c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a1580u64) }
        0x102a1580 => { vm.decode(0x2a012fu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1584u64) }
        0x102a1584 => { vm.decode(0x134443b5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a1588u64) }
        0x102a1588 => { vm.decode(0x7ddc77b3u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a158cu64) }
        0x102a158c => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a1590u64) }
        0x102a1590 => { vm.decode(0x918403b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a1594u64) }
        0x102a1594 => { vm.decode(0xfd9e7733u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a1598u64) }
        0x102a1598 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a159cu64) }
        0x102a159c => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a15a0u64) }
        0x102a15a0 => { vm.decode(0xa7d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a15a4u64).wrapping_add((imm * 4) as u64) } else { 0x102a15a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a15a4u64) }
        0x102a15a4 => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a15a8u64) }
        0x102a15a8 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15acu64) }
        0x102a15ac => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15b0u64) }
        0x102a15b0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a15b4u64) }
        0x102a15b4 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15b8u64) }
        0x102a15b8 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a15bcu64) }
        0x102a15bc => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15c0u64) }
        0x102a15c0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15c4u64) }
        0x102a15c4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a15c8u64) }
        0x102a15c8 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a15ccu64) }
        0x102a15cc => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15d0u64) }
        0x102a15d0 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a15d4u64) }
        0x102a15d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a15d8u64) } }
        0x102a15d8 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a15dcu64) }
        0x102a16ec => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a16f0u64) }
        0x102a16f0 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a16f4u64) }
        0x102a16f4 => { vm.decode(0x40528119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a16f8u64) }
        0x102a16f8 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a16fcu64) }
        0x102a16fc => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a1700u64) }
        0x102a1700 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a1704u64) }
        0x102a1704 => { vm.decode(0x642d8019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a1708u64) }
        0x102a1708 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a170cu64) }
        0x102a170c => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a1710u64) }
        0x102a1710 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a1714u64).wrapping_add((imm * 4) as u64) } else { 0x102a1714u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a1714u64) }
        0x102a1714 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a1718u64) } }
        0x102a1718 => { vm.decode(0x8d48841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 115896i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a171cu64) }
        0x102a171c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a1720u64) } }
        0x102a1720 => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a1724u64) }
        0x102a2838 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a283cu64) }
        0x102a283c => { vm.decode(0x2a00212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2840u64) }
        0x102a2840 => { vm.decode(0x41159u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 328i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2844u64) }
        0x102a2844 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2848u64) }
        0x102a2848 => { vm.decode(0x8b21c909u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a284cu64) }
        0x102a284c => { vm.decode(0x9319db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2850u64) }
        0x102a2850 => { vm.decode(0x89299cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2854u64) }
        0x102a2854 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2858u64) }
        0x102a2858 => { vm.decode(0x9219db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a285cu64) }
        0x102a285c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2860u64) }
        0x102a2860 => { vm.decode(0x286432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2864u64) }
        0x102a2864 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2868u64) }
        0x102a2868 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a286cu64 + 4);
        vm.wr(37, 0x102a286cu64 + 4);
        Ok(0x102a286cu64) }
        0x102a286c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2870u64) }
        0x102a2870 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2874u64) }
        0x102a2874 => { vm.decode(0x631d1162u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a2878u64) }
        0x102a2878 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a287cu64) }
        0x102a287c => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a2880u64) }
        0x102a2880 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2884u64) }
        0x102a2884 => { vm.decode(0x10002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a2888u64) }
        0x102a2888 => { vm.decode(0x20220033u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a288cu64) }
        0x102a288c => { vm.decode(0x8001236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a2890u64) }
        0x102a2890 => { vm.decode(0xa0006000u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 33i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a2894u64) }
        0x102a2894 => { vm.decode(0x3222871u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1079i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a2898u64).wrapping_add((imm * 4) as u64) } else { 0x102a2898u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a2898u64) }
        0x102a2898 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a289cu64) } }
        0x102a289c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28a0u64) }
        0x102a28a0 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28a4u64) }
        0x102a28a4 => { vm.decode(0x2276u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a28a8u64) }
        0x102a28a8 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a28acu64) }
        0x102a28ac => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a28b0u64) }
        0x102a28b0 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28b4u64) }
        0x102a28b4 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a28b8u64) }
        0x102a28b8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a28bcu64) }
        0x102a28bc => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28c0u64) }
        0x102a28c0 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a28c4u64) }
        0x102a28c4 => { vm.decode(0x740d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a28c8u64) }
        0x102a28c8 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a28ccu64) }
        0x102a28cc => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28d0u64) }
        0x102a28d0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28d4u64) }
        0x102a28d4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28d8u64) }
        0x102a28d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a28dcu64 + 4);
        vm.wr(37, 0x102a28dcu64 + 4);
        Ok(0x102a28dcu64) }
        0x102a28dc => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a28e0u64) }
        0x102a28e0 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28e4u64) }
        0x102a28e4 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28e8u64) }
        0x102a28e8 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28ecu64) }
        0x102a28ec => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28f0u64) }
        0x102a28f0 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28f4u64) }
        0x102a28f4 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a28f8u64) }
        0x102a28f8 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a28fcu64) }
        0x102a28fc => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2900u64) }
        0x102a2900 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2904u64) }
        0x102a2904 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2908u64) }
        0x102a2908 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a290cu64) }
        0x102a290c => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2910u64) }
        0x102a2910 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2914u64) }
        0x102a2914 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2918u64) }
        0x102a2918 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a291cu64) }
        0x102a291c => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2920u64) }
        0x102a2920 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a2924u64) }
        0x102a2924 => { vm.decode(0x8c8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a2928u64).wrapping_add((imm * 4) as u64) } else { 0x102a2928u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a2928u64) }
        0x102a2928 => { vm.decode(0x642d1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a292cu64) }
        0x102a292c => { vm.decode(0x204002beu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2930u64) }
        0x102a2930 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2934u64) }
        0x102a2934 => { vm.decode(0xa0480037u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102a2938u64) }
        0x102a2938 => { vm.decode(0x81740935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a293cu64) }
        0x102a293c => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2940u64) }
        0x102a2940 => { vm.decode(0x1242023eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2944u64) }
        0x102a2944 => { vm.decode(0x81700935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2948u64) }
        0x102a2948 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a294cu64) }
        0x102a294c => { vm.decode(0x124202beu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2950u64) }
        0x102a2950 => { vm.decode(0x81300935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2954u64) }
        0x102a2954 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2958u64) }
        0x102a2958 => { vm.decode(0x242027eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a295cu64) }
        0x102a295c => { vm.decode(0x32a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2960u64) }
        0x102a2960 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2964u64) }
        0x102a2964 => { vm.decode(0x24202feu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2968u64) }
        0x102a2968 => { vm.decode(0x30a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a296cu64) }
        0x102a296c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2970u64) }
        0x102a2970 => { vm.decode(0x61041019u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2974u64) }
        0x102a2974 => { vm.decode(0x1849019u32); let d = 11i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2978u64) }
        0x102a2978 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a297cu64) }
        0x102a297c => { vm.decode(0x184202feu32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2980u64) }
        0x102a2980 => { vm.decode(0x3842027eu32); let d = 13i16; let s = 17i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a2984u64) }
        0x102a2984 => { vm.decode(0x1168ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2988u64) }
        0x102a2988 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a298cu64) }
        0x102a298c => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2990u64) }
        0x102a2990 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2994u64) }
        0x102a2994 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2998u64) }
        0x102a2998 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a299cu64) }
        0x102a299c => { vm.decode(0x27d32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -29i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a29a0u64).wrapping_add((imm * 4) as u64) } else { 0x102a29a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a29a0u64) }
        0x102a29a0 => { vm.decode(0x8209875u32); let d = 17i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a29a4u64) }
        0x102a29a4 => { vm.decode(0x4a8aebafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29a8u64) }
        0x102a29a8 => { vm.decode(0x886c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29acu64) }
        0x102a29ac => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29b0u64) }
        0x102a29b0 => { vm.decode(0x4880c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29b4u64) }
        0x102a29b4 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a29b8u64) }
        0x102a29b8 => { vm.decode(0x631d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a29bcu64) }
        0x102a29bc => { vm.decode(0x2a82c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29c0u64) }
        0x102a29c0 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29c4u64) }
        0x102a29c4 => { vm.decode(0x8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29c8u64) }
        0x102a29c8 => { vm.decode(0x650d1059u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a29ccu64) }
        0x102a29cc => { vm.decode(0x228c5bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29d0u64) }
        0x102a29d0 => { vm.decode(0x88ee3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29d4u64) }
        0x102a29d4 => { vm.decode(0x865bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29d8u64) }
        0x102a29d8 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29dcu64) }
        0x102a29dc => { vm.decode(0x22827bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29e0u64) }
        0x102a29e0 => { vm.decode(0x28e7bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29e4u64) }
        0x102a29e4 => { vm.decode(0x42805bafu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29e8u64) }
        0x102a29e8 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29ecu64) }
        0x102a29ec => { vm.decode(0x60805befu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a29f0u64) }
        0x102a29f0 => { vm.decode(0x701a2835u32); let d = 3i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a29f4u64) }
        0x102a29f4 => { vm.decode(0x1b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a29f8u64) }
        0x102a29f8 => { vm.decode(0x1a3835u32); let d = 3i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a29fcu64) }
        0x102a29fc => { vm.decode(0x1b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a00u64) }
        0x102a2a00 => { vm.decode(0x404f3835u32); let d = 3i16; let s = 14i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a04u64) }
        0x102a2a04 => { vm.decode(0x1b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a08u64) }
        0x102a2a08 => { vm.decode(0x23073835u32); let d = 13i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a0cu64) }
        0x102a2a0c => { vm.decode(0x4a80d875u32); let d = 31i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a10u64) }
        0x102a2a10 => { vm.decode(0x407a835u32); let d = 17i16; let s = 6i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a14u64) }
        0x102a2a14 => { vm.decode(0x48804875u32); let d = 22i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a18u64) }
        0x102a2a18 => { vm.decode(0x23428835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a1cu64) }
        0x102a2a1c => { vm.decode(0xda9119u32); let d = 3i16; let s = 10i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a2a20u64) }
        0x102a2a20 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a24u64) }
        0x102a2a24 => { vm.decode(0x114c9835u32); let d = 6i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a28u64) }
        0x102a2a28 => { vm.decode(0x4680c875u32); let d = 15i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a2cu64) }
        0x102a2a2c => { vm.decode(0x6208a835u32); let d = 11i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a30u64) }
        0x102a2a30 => { vm.decode(0xd94c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102a2a34u64) }
        0x102a2a34 => { vm.decode(0x30102200u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a2a38u64) }
        0x102a2a38 => { vm.decode(0x460480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 929i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a2a3cu64).wrapping_add((imm * 4) as u64) } else { 0x102a2a3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a2a3cu64) }
        0x102a2a3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a2a40u64) } }
        0x102a2a40 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a44u64) }
        0x102a2a44 => { vm.decode(0x148080f5u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a48u64) }
        0x102a2a48 => { vm.decode(0x769000f5u32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a4cu64) }
        0x102a2a4c => { vm.decode(0x1b3815f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a50u64) }
        0x102a2a50 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a54u64) }
        0x102a2a54 => { vm.decode(0xa0c875u32); let d = 5i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a58u64) }
        0x102a2a58 => { vm.decode(0xb1054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a5cu64) }
        0x102a2a5c => { vm.decode(0x72b1c0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a60u64) }
        0x102a2a60 => { vm.decode(0x9b385570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a64u64) }
        0x102a2a64 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a68u64) }
        0x102a2a68 => { vm.decode(0x6204875u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2a6cu64) }
        0x102a2a6c => { vm.decode(0x70b360f5u32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2a70u64) }
        0x102a2a70 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a74u64) }
        0x102a2a74 => { vm.decode(0x1b3014f0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2a78u64) }
        0x102a2a78 => { vm.decode(0x2a844b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2a7cu64) }
        0x102a2a7c => { vm.decode(0xa0e294f5u32); let d = 13i16; let s = 9i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2a80u64) }
        0x102a2a80 => { vm.decode(0xa86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2a84u64) }
        0x102a2a84 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2a88u64) }
        0x102a2a88 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a8cu64) }
        0x102a2a8c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a90u64) }
        0x102a2a90 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a94u64) }
        0x102a2a94 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2a98u64) }
        0x102a2a98 => { vm.decode(0x2088c32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2a9cu64) }
        0x102a2a9c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2aa0u64) }
        0x102a2aa0 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2aa4u64) }
        0x102a2aa4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2aa8u64) }
        0x102a2aa8 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2aacu64) }
        0x102a2aac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ab0u64) }
        0x102a2ab0 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ab4u64) }
        0x102a2ab4 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2ab8u64) }
        0x102a2ab8 => { vm.decode(0x52d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2abcu64) }
        0x102a2abc => { vm.decode(0x70f3a0f5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ac0u64) }
        0x102a2ac0 => { vm.decode(0x40908875u32); let d = 7i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2ac4u64) }
        0x102a2ac4 => { vm.decode(0x52c180f5u32); let d = 16i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ac8u64) }
        0x102a2ac8 => { vm.decode(0x9b201430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2accu64) }
        0x102a2acc => { vm.decode(0x10e2a0f5u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ad0u64) }
        0x102a2ad0 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2ad4u64) }
        0x102a2ad4 => { vm.decode(0xa14484f5u32); let d = 17i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2ad8u64) }
        0x102a2ad8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2adcu64) }
        0x102a2adc => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2ae0u64) }
        0x102a2ae0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ae4u64) }
        0x102a2ae4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ae8u64) }
        0x102a2ae8 => { vm.decode(0x4a909875u32); let d = 31i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2aecu64) }
        0x102a2aec => { vm.decode(0x54e120f5u32); let d = 18i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2af0u64) }
        0x102a2af0 => { vm.decode(0x1b305530u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2af4u64) }
        0x102a2af4 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2af8u64) }
        0x102a2af8 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2afcu64) }
        0x102a2afc => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b00u64) }
        0x102a2b00 => { vm.decode(0x60e294f5u32); let d = 13i16; let s = 9i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b04u64) }
        0x102a2b04 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2b08u64) }
        0x102a2b08 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b0cu64) }
        0x102a2b0c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b10u64) }
        0x102a2b10 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b14u64) }
        0x102a2b14 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b18u64) }
        0x102a2b18 => { vm.decode(0x18434f5u32); let d = 24i16; let s = 4i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b1cu64) }
        0x102a2b1c => { vm.decode(0x4c909875u32); let d = 23i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b20u64) }
        0x102a2b20 => { vm.decode(0xa94494f5u32); let d = 19i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b24u64) }
        0x102a2b24 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b28u64) }
        0x102a2b28 => { vm.decode(0xc64c72edu32); let d = 20i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b2cu64) }
        0x102a2b2c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b30u64) }
        0x102a2b30 => { vm.decode(0x76b140f5u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2b34u64) }
        0x102a2b34 => { vm.decode(0xc6cc72edu32); let d = 21i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b38u64) }
        0x102a2b38 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b3cu64) }
        0x102a2b3c => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2b40u64) }
        0x102a2b40 => { vm.decode(0x728380f5u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2b44u64) }
        0x102a2b44 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b48u64) }
        0x102a2b48 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b4cu64) }
        0x102a2b4c => { vm.decode(0x330b8835u32); let d = 14i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b50u64) }
        0x102a2b50 => { vm.decode(0x7481a0f5u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2b54u64) }
        0x102a2b54 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b58u64) }
        0x102a2b58 => { vm.decode(0x22492fu32); let d = 12i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b5cu64) }
        0x102a2b5c => { vm.decode(0x81e224f5u32); let d = 13i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b60u64) }
        0x102a2b60 => { vm.decode(0x690484f5u32); let d = 18i16; let s = 9i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b64u64) }
        0x102a2b64 => { vm.decode(0x96424f5u32); let d = 19i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b68u64) }
        0x102a2b68 => { vm.decode(0x4002422fu32); let d = 20i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b6cu64) }
        0x102a2b6c => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b70u64) }
        0x102a2b70 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b74u64) }
        0x102a2b74 => { vm.decode(0x321c0835u32); let d = 10i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b78u64) }
        0x102a2b78 => { vm.decode(0x9309d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2b7cu64) }
        0x102a2b7c => { vm.decode(0x549120f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2b80u64) }
        0x102a2b80 => { vm.decode(0xb285530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2b84u64) }
        0x102a2b84 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2b88u64) }
        0x102a2b88 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2b8cu64) }
        0x102a2b8c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b90u64) }
        0x102a2b90 => { vm.decode(0x524e0835u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2b94u64) }
        0x102a2b94 => { vm.decode(0xb081570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2b98u64) }
        0x102a2b98 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2b9cu64) }
        0x102a2b9c => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ba0u64) }
        0x102a2ba0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ba4u64) }
        0x102a2ba4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ba8u64) }
        0x102a2ba8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2bacu64) }
        0x102a2bac => { vm.decode(0x4031835u32); let d = 16i16; let s = 12i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2bb0u64) }
        0x102a2bb0 => { vm.decode(0x50b060f5u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2bb4u64) }
        0x102a2bb4 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2bb8u64) }
        0x102a2bb8 => { vm.decode(0x20a294f5u32); let d = 12i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2bbcu64) }
        0x102a2bbc => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2bc0u64) }
        0x102a2bc0 => { vm.decode(0x103b835u32); let d = 5i16; let s = 12i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2bc4u64) }
        0x102a2bc4 => { vm.decode(0x1938dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2bc8u64) }
        0x102a2bc8 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2bccu64) }
        0x102a2bcc => { vm.decode(0x32c200f5u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2bd0u64) }
        0x102a2bd0 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2bd4u64) }
        0x102a2bd4 => { vm.decode(0x814424f5u32); let d = 17i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2bd8u64) }
        0x102a2bd8 => { vm.decode(0x4008412fu32); let d = 18i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2bdcu64) }
        0x102a2bdc => { vm.decode(0x53408835u32); let d = 12i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2be0u64) }
        0x102a2be0 => { vm.decode(0xa105875u32); let d = 24i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2be4u64) }
        0x102a2be4 => { vm.decode(0x8b1855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2be8u64) }
        0x102a2be8 => { vm.decode(0x5481a0f5u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2becu64) }
        0x102a2bec => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2bf0u64) }
        0x102a2bf0 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2bf4u64) }
        0x102a2bf4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2bf8u64) }
        0x102a2bf8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2bfcu64) }
        0x102a2bfc => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2c00u64) }
        0x102a2c00 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2c04u64) }
        0x102a2c04 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2c08u64) }
        0x102a2c08 => { vm.decode(0x28826bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c0cu64) }
        0x102a2c0c => { vm.decode(0x70a220f5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c10u64) }
        0x102a2c10 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c14u64) }
        0x102a2c14 => { vm.decode(0xa04084f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c18u64) }
        0x102a2c18 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c1cu64) }
        0x102a2c1c => { vm.decode(0x20043835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2c20u64) }
        0x102a2c20 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2c24u64) }
        0x102a2c24 => { vm.decode(0x56b100f5u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c28u64) }
        0x102a2c28 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c2cu64) }
        0x102a2c2c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c30u64) }
        0x102a2c30 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c34u64) }
        0x102a2c34 => { vm.decode(0x2844befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c38u64) }
        0x102a2c38 => { vm.decode(0x349220f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c3cu64) }
        0x102a2c3c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c40u64) }
        0x102a2c40 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c44u64) }
        0x102a2c44 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c48u64) }
        0x102a2c48 => { vm.decode(0x35462835u32); let d = 21i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2c4cu64) }
        0x102a2c4c => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c50u64) }
        0x102a2c50 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c54u64) }
        0x102a2c54 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c58u64) }
        0x102a2c58 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c5cu64) }
        0x102a2c5c => { vm.decode(0x54b3a0f5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c60u64) }
        0x102a2c60 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c64u64) }
        0x102a2c64 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c68u64) }
        0x102a2c68 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c6cu64) }
        0x102a2c6c => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2c70u64) }
        0x102a2c70 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2c74u64) }
        0x102a2c74 => { vm.decode(0x76b000f5u32); let d = 10i16; let s = 13i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c78u64) }
        0x102a2c78 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c7cu64) }
        0x102a2c7c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c80u64) }
        0x102a2c80 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c84u64) }
        0x102a2c84 => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c88u64) }
        0x102a2c88 => { vm.decode(0x14b3a0f5u32); let d = 11i16; let s = 19i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2c8cu64) }
        0x102a2c8c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2c90u64) }
        0x102a2c90 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2c94u64) }
        0x102a2c94 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2c98u64) }
        0x102a2c98 => { vm.decode(0x33462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2c9cu64) }
        0x102a2c9c => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ca0u64) }
        0x102a2ca0 => { vm.decode(0x16b000f5u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ca4u64) }
        0x102a2ca4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2ca8u64) }
        0x102a2ca8 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2cacu64) }
        0x102a2cac => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2cb0u64) }
        0x102a2cb0 => { vm.decode(0x54b220f5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2cb4u64) }
        0x102a2cb4 => { vm.decode(0x4c004875u32); let d = 18i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2cb8u64) }
        0x102a2cb8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2cbcu64) }
        0x102a2cbc => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2cc0u64) }
        0x102a2cc0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2cc4u64) }
        0x102a2cc4 => { vm.decode(0x354e0835u32); let d = 22i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2cc8u64) }
        0x102a2cc8 => { vm.decode(0x4a8243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2cccu64) }
        0x102a2ccc => { vm.decode(0x569080f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2cd0u64) }
        0x102a2cd0 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2cd4u64) }
        0x102a2cd4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2cd8u64) }
        0x102a2cd8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2cdcu64) }
        0x102a2cdc => { vm.decode(0x228e63efu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ce0u64) }
        0x102a2ce0 => { vm.decode(0x54b320f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ce4u64) }
        0x102a2ce4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2ce8u64) }
        0x102a2ce8 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2cecu64) }
        0x102a2cec => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2cf0u64) }
        0x102a2cf0 => { vm.decode(0x36462835u32); let d = 25i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2cf4u64) }
        0x102a2cf4 => { vm.decode(0x349080f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2cf8u64) }
        0x102a2cf8 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2cfcu64) }
        0x102a2cfc => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d00u64) }
        0x102a2d00 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d04u64) }
        0x102a2d04 => { vm.decode(0x11472835u32); let d = 5i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d08u64) }
        0x102a2d08 => { vm.decode(0x1928dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2d0cu64) }
        0x102a2d0c => { vm.decode(0xd31c53b5u32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d10u64) }
        0x102a2d10 => { vm.decode(0x9b2055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2d14u64) }
        0x102a2d14 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d18u64) }
        0x102a2d18 => { vm.decode(0x2082e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d1cu64) }
        0x102a2d1c => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d20u64) }
        0x102a2d20 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d24u64) }
        0x102a2d24 => { vm.decode(0x4210c875u32); let d = 11i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d28u64) }
        0x102a2d28 => { vm.decode(0x88eeb6fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d2cu64) }
        0x102a2d2c => { vm.decode(0x134f0835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d30u64) }
        0x102a2d30 => { vm.decode(0x16b180f5u32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d34u64) }
        0x102a2d34 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d38u64) }
        0x102a2d38 => { vm.decode(0x408434f5u32); let d = 24i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d3cu64) }
        0x102a2d3c => { vm.decode(0x549100f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d40u64) }
        0x102a2d40 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d44u64) }
        0x102a2d44 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d48u64) }
        0x102a2d48 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d4cu64) }
        0x102a2d4c => { vm.decode(0xa82c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d50u64) }
        0x102a2d50 => { vm.decode(0x14052835u32); let d = 17i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d54u64) }
        0x102a2d54 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d58u64) }
        0x102a2d58 => { vm.decode(0x8b10d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2d5cu64) }
        0x102a2d5c => { vm.decode(0x5490a0f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d60u64) }
        0x102a2d60 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d64u64) }
        0x102a2d64 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d68u64) }
        0x102a2d68 => { vm.decode(0x230d875u32); let d = 9i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d6cu64) }
        0x102a2d6c => { vm.decode(0x8938dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2d70u64) }
        0x102a2d70 => { vm.decode(0x749380f5u32); let d = 11i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d74u64) }
        0x102a2d74 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d78u64) }
        0x102a2d78 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d7cu64) }
        0x102a2d7c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d80u64) }
        0x102a2d80 => { vm.decode(0x511c0835u32); let d = 6i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2d84u64) }
        0x102a2d84 => { vm.decode(0x928ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2d88u64) }
        0x102a2d88 => { vm.decode(0x549280f5u32); let d = 11i16; let s = 20i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2d8cu64) }
        0x102a2d8c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2d90u64) }
        0x102a2d90 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2d94u64) }
        0x102a2d94 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d98u64) }
        0x102a2d98 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2d9cu64) }
        0x102a2d9c => { vm.decode(0x570d0835u32); let d = 30i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2da0u64) }
        0x102a2da0 => { vm.decode(0x349200f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2da4u64) }
        0x102a2da4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2da8u64) }
        0x102a2da8 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2dacu64) }
        0x102a2dac => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2db0u64) }
        0x102a2db0 => { vm.decode(0x51070835u32); let d = 4i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2db4u64) }
        0x102a2db4 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2db8u64) }
        0x102a2db8 => { vm.decode(0x44805875u32); let d = 6i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2dbcu64) }
        0x102a2dbc => { vm.decode(0x54b300f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2dc0u64) }
        0x102a2dc0 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2dc4u64) }
        0x102a2dc4 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2dc8u64) }
        0x102a2dc8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2dccu64) }
        0x102a2dcc => { vm.decode(0xa86e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2dd0u64) }
        0x102a2dd0 => { vm.decode(0x53050835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2dd4u64) }
        0x102a2dd4 => { vm.decode(0x6305875u32); let d = 8i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2dd8u64) }
        0x102a2dd8 => { vm.decode(0x74b380f5u32); let d = 11i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ddcu64) }
        0x102a2ddc => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2de0u64) }
        0x102a2de0 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2de4u64) }
        0x102a2de4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2de8u64) }
        0x102a2de8 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2decu64) }
        0x102a2dec => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2df0u64) }
        0x102a2df0 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2df4u64) }
        0x102a2df4 => { vm.decode(0x74a100f5u32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2df8u64) }
        0x102a2df8 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2dfcu64) }
        0x102a2dfc => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e00u64) }
        0x102a2e00 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e04u64) }
        0x102a2e04 => { vm.decode(0x628a6befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e08u64) }
        0x102a2e08 => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e0cu64) }
        0x102a2e0c => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e10u64) }
        0x102a2e10 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2e14u64) }
        0x102a2e14 => { vm.decode(0x54a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e18u64) }
        0x102a2e18 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e1cu64) }
        0x102a2e1c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e20u64) }
        0x102a2e20 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e24u64) }
        0x102a2e24 => { vm.decode(0x201d0835u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e28u64) }
        0x102a2e28 => { vm.decode(0x9189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2e2cu64) }
        0x102a2e2c => { vm.decode(0x44808875u32); let d = 7i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e30u64) }
        0x102a2e30 => { vm.decode(0x14a000f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e34u64) }
        0x102a2e34 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e38u64) }
        0x102a2e38 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e3cu64) }
        0x102a2e3c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e40u64) }
        0x102a2e40 => { vm.decode(0x228063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e44u64) }
        0x102a2e44 => { vm.decode(0x25040835u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e48u64) }
        0x102a2e48 => { vm.decode(0x228c4bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e4cu64) }
        0x102a2e4c => { vm.decode(0x34a100f5u32); let d = 2i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e50u64) }
        0x102a2e50 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e54u64) }
        0x102a2e54 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e58u64) }
        0x102a2e58 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e5cu64) }
        0x102a2e5c => { vm.decode(0x552835u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e60u64) }
        0x102a2e60 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2e64u64) }
        0x102a2e64 => { vm.decode(0x148180f5u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e68u64) }
        0x102a2e68 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e6cu64) }
        0x102a2e6c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e70u64) }
        0x102a2e70 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e74u64) }
        0x102a2e74 => { vm.decode(0x250f0835u32); let d = 22i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e78u64) }
        0x102a2e78 => { vm.decode(0x74a180f5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e7cu64) }
        0x102a2e7c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e80u64) }
        0x102a2e80 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2e84u64) }
        0x102a2e84 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e88u64) }
        0x102a2e88 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2e8cu64) }
        0x102a2e8c => { vm.decode(0x25042835u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e90u64) }
        0x102a2e90 => { vm.decode(0x6909875u32); let d = 13i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2e94u64) }
        0x102a2e94 => { vm.decode(0x10e180f5u32); let d = 16i16; let s = 19i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2e98u64) }
        0x102a2e98 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2e9cu64) }
        0x102a2e9c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2ea0u64) }
        0x102a2ea0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ea4u64) }
        0x102a2ea4 => { vm.decode(0x20123835u32); let d = 1i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2ea8u64) }
        0x102a2ea8 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2eacu64) }
        0x102a2eac => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102a2eb0u64) }
        0x102a2eb0 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102a2eb4u64) }
        0x102a2eb4 => { vm.decode(0xb0014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2eb8u64) }
        0x102a2eb8 => { vm.decode(0x44820d35u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102a2ebcu64) }
        0x102a2ebc => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102a2ec0u64) }
        0x102a2ec0 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ec4u64) }
        0x102a2ec4 => { vm.decode(0x282c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ec8u64) }
        0x102a2ec8 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2eccu64) }
        0x102a2ecc => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ed0u64) }
        0x102a2ed0 => { vm.decode(0x54b3835u32); let d = 23i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2ed4u64) }
        0x102a2ed4 => { vm.decode(0xa21c23b5u32); let d = 16i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2ed8u64) }
        0x102a2ed8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2edcu64) }
        0x102a2edc => { vm.decode(0x210424f5u32); let d = 16i16; let s = 5i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2ee0u64) }
        0x102a2ee0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ee4u64) }
        0x102a2ee4 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ee8u64) }
        0x102a2ee8 => { vm.decode(0x20003835u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2eecu64) }
        0x102a2eec => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2ef0u64) }
        0x102a2ef0 => { vm.decode(0x60004a2fu32); let d = 25i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ef4u64) }
        0x102a2ef4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2ef8u64) }
        0x102a2ef8 => { vm.decode(0x50423835u32); let d = 1i16; let s = 10i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2efcu64) }
        0x102a2efc => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2f00u64) }
        0x102a2f00 => { vm.decode(0x39c73b5u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f04u64) }
        0x102a2f04 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f08u64) }
        0x102a2f08 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f0cu64) }
        0x102a2f0c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f10u64) }
        0x102a2f10 => { vm.decode(0x74183835u32); let d = 19i16; let s = 17i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f14u64) }
        0x102a2f14 => { vm.decode(0x88a63afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f18u64) }
        0x102a2f18 => { vm.decode(0x50d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f1cu64) }
        0x102a2f1c => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f20u64) }
        0x102a2f20 => { vm.decode(0x230414f5u32); let d = 16i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f24u64) }
        0x102a2f24 => { vm.decode(0xa21c73b5u32); let d = 24i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f28u64) }
        0x102a2f28 => { vm.decode(0xe20c875u32); let d = 25i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f2cu64) }
        0x102a2f2c => { vm.decode(0x1b00d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2f30u64) }
        0x102a2f30 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f34u64) }
        0x102a2f34 => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f38u64) }
        0x102a2f38 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f3cu64) }
        0x102a2f3c => { vm.decode(0x175b1835u32); let d = 30i16; let s = 24i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f40u64) }
        0x102a2f40 => { vm.decode(0xa31c73b5u32); let d = 24i16; let s = 15i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f44u64) }
        0x102a2f44 => { vm.decode(0x1b105570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2f48u64) }
        0x102a2f48 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f4cu64) }
        0x102a2f4c => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f50u64) }
        0x102a2f50 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f54u64) }
        0x102a2f54 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f58u64) }
        0x102a2f58 => { vm.decode(0x36001835u32); let d = 24i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f5cu64) }
        0x102a2f5c => { vm.decode(0xa8843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f60u64) }
        0x102a2f60 => { vm.decode(0x135c13b5u32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f64u64) }
        0x102a2f64 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f68u64) }
        0x102a2f68 => { vm.decode(0x68a234f5u32); let d = 14i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f6cu64) }
        0x102a2f6c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f70u64) }
        0x102a2f70 => { vm.decode(0x1007a835u32); let d = 1i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f74u64) }
        0x102a2f74 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a2f78u64) }
        0x102a2f78 => { vm.decode(0x2884befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f7cu64) }
        0x102a2f7c => { vm.decode(0x235c13b5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a2f80u64) }
        0x102a2f80 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102a2f84u64) }
        0x102a2f84 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a2f88u64) }
        0x102a2f88 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f8cu64) }
        0x102a2f8c => { vm.decode(0x2a80c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a2f90u64) }
        0x102a2f90 => { vm.decode(0x33008835u32); let d = 12i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a2f94u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
