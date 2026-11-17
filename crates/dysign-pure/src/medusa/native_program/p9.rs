#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102997d4 => { vm.decode(0x131c8835u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997d8u64) }
        0x102997d8 => { vm.decode(0x5459a835u32); let d = 19i16; let s = 13i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997dcu64) }
        0x102997dc => { vm.decode(0x33118835u32); let d = 12i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997e0u64) }
        0x102997e0 => { vm.decode(0x74060835u32); let d = 16i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997e4u64) }
        0x102997e4 => { vm.decode(0x121d0835u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997e8u64) }
        0x102997e8 => { vm.decode(0x22142835u32); let d = 9i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997ecu64) }
        0x102997ec => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997f0u64) }
        0x102997f0 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997f4u64) }
        0x102997f4 => { vm.decode(0xd9ce111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102997f8u64) }
        0x102997f8 => { vm.decode(0x52f84349u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102997fcu64) }
        0x102997fc => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299800u64) }
        0x10299800 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299804u64) }
        0x10299804 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299808u64) }
        0x10299808 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029980cu64) }
        0x1029980c => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299810u64) }
        0x10299810 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299814u64) }
        0x10299814 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299818u64) }
        0x10299818 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029981cu64) }
        0x1029981c => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299820u64) }
        0x10299820 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299824u64) }
        0x10299824 => { vm.decode(0x4a1835u32); let d = 2i16; let s = 10i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299828u64) }
        0x10299828 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029982cu64) }
        0x1029982c => { vm.decode(0x400f1835u32); let d = 2i16; let s = 12i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299830u64) }
        0x10299830 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299834u64) }
        0x10299834 => { vm.decode(0x3473835u32); let d = 13i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299838u64) }
        0x10299838 => { vm.decode(0x4e90d875u32); let d = 31i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029983cu64) }
        0x1029983c => { vm.decode(0x4416b835u32); let d = 17i16; let s = 23i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299840u64) }
        0x10299840 => { vm.decode(0x48a00875u32); let d = 22i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299844u64) }
        0x10299844 => { vm.decode(0x33400835u32); let d = 12i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299848u64) }
        0x10299848 => { vm.decode(0x110d9835u32); let d = 6i16; let s = 4i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029984cu64) }
        0x1029984c => { vm.decode(0x4680d875u32); let d = 15i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299850u64) }
        0x10299850 => { vm.decode(0xca8011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 75336i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10299854u64) }
        0x10299854 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299858u64) }
        0x10299858 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029985cu64) }
        0x1029985c => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299860u64) }
        0x10299860 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299864u64) }
        0x10299864 => { vm.decode(0x404d2835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299868u64) }
        0x10299868 => { vm.decode(0xf0835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029986cu64) }
        0x1029986c => { vm.decode(0x60102835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299870u64) }
        0x10299870 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299874u64) }
        0x10299874 => { vm.decode(0x21038835u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299878u64) }
        0x10299878 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029987cu64) }
        0x1029987c => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299880u64) }
        0x10299880 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299884u64) }
        0x10299884 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299888u64) }
        0x10299888 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029988cu64) }
        0x1029988c => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299890u64) }
        0x10299890 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299894u64) }
        0x10299894 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299898u64) }
        0x10299898 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029989cu64) }
        0x1029989c => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998a0u64) }
        0x102998a0 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998a4u64) }
        0x102998a4 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998a8u64) }
        0x102998a8 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998acu64) }
        0x102998ac => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998b0u64) }
        0x102998b0 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998b4u64) }
        0x102998b4 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998b8u64) }
        0x102998b8 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998bcu64) }
        0x102998bc => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998c0u64) }
        0x102998c0 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998c4u64) }
        0x102998c4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998c8u64) }
        0x102998c8 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998ccu64) }
        0x102998cc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998d0u64) }
        0x102998d0 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998d4u64) }
        0x102998d4 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998d8u64) }
        0x102998d8 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998dcu64) }
        0x102998dc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998e0u64) }
        0x102998e0 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998e4u64) }
        0x102998e4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998e8u64) }
        0x102998e8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102998ecu64) }
        0x102998ec => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102998f0u64) }
        0x102998f0 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998f4u64) }
        0x102998f4 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102998f8u64) }
        0x102998f8 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102998fcu64) }
        0x102998fc => { vm.decode(0x288ce3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299900u64) }
        0x10299900 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299904u64) }
        0x10299904 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299908u64) }
        0x10299908 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029990cu64) }
        0x1029990c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299910u64) }
        0x10299910 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299914u64) }
        0x10299914 => { vm.decode(0x4002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10299918u64) }
        0x10299918 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029991cu64) }
        0x1029991c => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299920u64) }
        0x10299920 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299924u64) }
        0x10299924 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299928u64) }
        0x10299928 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029992cu64) }
        0x1029992c => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299930u64) }
        0x10299930 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299934u64) }
        0x10299934 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299938u64) }
        0x10299938 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029993cu64) }
        0x1029993c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299940u64) }
        0x10299940 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10299944u64 + 4);
        vm.wr(37, 0x10299944u64 + 4);
        Ok(0x10299944u64) }
        0x10299944 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299948u64) }
        0x10299948 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029994cu64) }
        0x1029994c => { vm.decode(0x4828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299950u64) }
        0x10299950 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299954u64) }
        0x10299954 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299958u64) }
        0x10299958 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029995cu64) }
        0x1029995c => { vm.decode(0x282e35u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x10299960u64) }
        0x10299960 => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10299964u64).wrapping_add((imm * 4) as u64) } else { 0x10299964u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10299964u64) }
        0x10299964 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299968u64) }
        0x10299994 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299998u64) }
        0x10299998 => { vm.decode(0x40101019u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029999cu64) }
        0x1029999c => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102999a0u64) }
        0x102999a0 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102999a4u64) }
        0x102999a4 => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102999a8u64).wrapping_add((imm * 4) as u64) } else { 0x102999a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102999a8u64) }
        0x102999a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102999acu64) } }
        0x102999ac => { vm.decode(0x2024002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102999b0u64) }
        0x102999b0 => { vm.decode(0x2060f5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999b4u64) }
        0x102999b4 => { vm.decode(0x22240f5u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999b8u64) }
        0x102999b8 => { vm.decode(0x42040f5u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999bcu64) }
        0x102999bc => { vm.decode(0x92c403b5u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999c0u64) }
        0x102999c0 => { vm.decode(0x820013b5u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999c4u64) }
        0x102999c4 => { vm.decode(0x808413b5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999c8u64) }
        0x102999c8 => { vm.decode(0x802204b3u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999ccu64) }
        0x102999cc => { vm.decode(0x2304b3u32); let d = 9i16; let s = 2i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999d0u64) }
        0x102999d0 => { vm.decode(0x80210433u32); let d = 8i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999d4u64) }
        0x102999d4 => { vm.decode(0x20620433u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999d8u64) }
        0x102999d8 => { vm.decode(0xa06000b3u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999dcu64) }
        0x102999dc => { vm.decode(0x6200b3u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999e0u64) }
        0x102999e0 => { vm.decode(0x80600033u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999e4u64) }
        0x102999e4 => { vm.decode(0x22220f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102999e8u64) }
        0x102999e8 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102999ecu64) }
        0x102999ec => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102999f0u64) }
        0x102999f0 => { vm.decode(0xb0000040u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102999f4u64) }
        0x102999f4 => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102999f8u64).wrapping_add((imm * 4) as u64) } else { 0x102999f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102999f8u64) }
        0x102999f8 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102999fcu64) }
        0x102999fc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10299a00u64) }
        0x10299a00 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a04u64) }
        0x10299a04 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a08u64) }
        0x10299a08 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a0cu64) }
        0x10299a0c => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a10u64) }
        0x10299a10 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a14u64) }
        0x10299a14 => { vm.decode(0x19001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a18u64) }
        0x10299a18 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a1cu64) }
        0x10299a1c => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a20u64) }
        0x10299a20 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a24u64) }
        0x10299a24 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a28u64) }
        0x10299a28 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a2cu64) }
        0x10299a2c => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a30u64) }
        0x10299a30 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a34u64) }
        0x10299a34 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a38u64) }
        0x10299a38 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a3cu64) }
        0x10299a3c => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299a40u64) }
        0x10299a40 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10299a44u64) }
        0x10299a44 => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10299a48u64).wrapping_add((imm * 4) as u64) } else { 0x10299a48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10299a48u64) }
        0x10299a48 => { vm.decode(0x652d1019u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299a4cu64) }
        0x10299a4c => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299a50u64) }
        0x10299a50 => { vm.decode(0x200802beu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299a54u64) }
        0x10299a54 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a58u64) }
        0x10299a58 => { vm.decode(0x90480437u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x10299a5cu64) }
        0x10299a5c => { vm.decode(0x81749935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a60u64) }
        0x10299a60 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299a64u64) }
        0x10299a64 => { vm.decode(0x320a023eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299a68u64) }
        0x10299a68 => { vm.decode(0x81709935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a6cu64) }
        0x10299a6c => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299a70u64) }
        0x10299a70 => { vm.decode(0x320a02beu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299a74u64) }
        0x10299a74 => { vm.decode(0x81309935u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a78u64) }
        0x10299a78 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299a7cu64) }
        0x10299a7c => { vm.decode(0x220a027eu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299a80u64) }
        0x10299a80 => { vm.decode(0x36a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a84u64) }
        0x10299a84 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299a88u64) }
        0x10299a88 => { vm.decode(0x220a02feu32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299a8cu64) }
        0x10299a8c => { vm.decode(0x34a3d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299a90u64) }
        0x10299a90 => { vm.decode(0x5488875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299a94u64) }
        0x10299a94 => { vm.decode(0x41849019u32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299a98u64) }
        0x10299a98 => { vm.decode(0x38c0019u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299a9cu64) }
        0x10299a9c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10299aa0u64) }
        0x10299aa0 => { vm.decode(0x380a02feu32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299aa4u64) }
        0x10299aa4 => { vm.decode(0x1a0a027eu32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10299aa8u64) }
        0x10299aa8 => { vm.decode(0x13289d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299aacu64) }
        0x10299aac => { vm.decode(0x3004a835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299ab0u64) }
        0x10299ab0 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299ab4u64) }
        0x10299ab4 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299ab8u64) }
        0x10299ab8 => { vm.decode(0x42204875u32); let d = 10i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299abcu64) }
        0x10299abc => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10299ac0u64) }
        0x10299ac0 => { vm.decode(0x27d12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -30i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10299ac4u64).wrapping_add((imm * 4) as u64) } else { 0x10299ac4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10299ac4u64) }
        0x10299ac4 => { vm.decode(0x19389570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299ac8u64) }
        0x10299ac8 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299accu64) }
        0x10299acc => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ad0u64) }
        0x10299ad0 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ad4u64) }
        0x10299ad4 => { vm.decode(0x6882c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ad8u64) }
        0x10299ad8 => { vm.decode(0x8100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10299adcu64) }
        0x10299adc => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ae0u64) }
        0x10299ae0 => { vm.decode(0x228453afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ae4u64) }
        0x10299ae4 => { vm.decode(0x8853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ae8u64) }
        0x10299ae8 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299aecu64) }
        0x10299aec => { vm.decode(0x2887bafu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299af0u64) }
        0x10299af0 => { vm.decode(0x4886e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299af4u64) }
        0x10299af4 => { vm.decode(0x208c7bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299af8u64) }
        0x10299af8 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299afcu64) }
        0x10299afc => { vm.decode(0x2825bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b00u64) }
        0x10299b00 => { vm.decode(0x20865bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b04u64) }
        0x10299b04 => { vm.decode(0x28e7bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b08u64) }
        0x10299b08 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b0cu64) }
        0x10299b0c => { vm.decode(0x208a7bafu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b10u64) }
        0x10299b10 => { vm.decode(0x714b0835u32); let d = 6i16; let s = 9i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b14u64) }
        0x10299b14 => { vm.decode(0xb10d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b18u64) }
        0x10299b18 => { vm.decode(0x414b9835u32); let d = 6i16; let s = 14i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b1cu64) }
        0x10299b1c => { vm.decode(0xb3855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b20u64) }
        0x10299b20 => { vm.decode(0x610f1835u32); let d = 6i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b24u64) }
        0x10299b24 => { vm.decode(0xb08d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b28u64) }
        0x10299b28 => { vm.decode(0x10f1835u32); let d = 6i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b2cu64) }
        0x10299b2c => { vm.decode(0xb00d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b30u64) }
        0x10299b30 => { vm.decode(0x274a9835u32); let d = 30i16; let s = 11i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b34u64) }
        0x10299b34 => { vm.decode(0x474aa835u32); let d = 31i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b38u64) }
        0x10299b38 => { vm.decode(0x193095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b3cu64) }
        0x10299b3c => { vm.decode(0x40d29119u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10299b40u64) }
        0x10299b40 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b44u64) }
        0x10299b44 => { vm.decode(0x46449835u32); let d = 24i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b48u64) }
        0x10299b48 => { vm.decode(0x2208a835u32); let d = 11i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b4cu64) }
        0x10299b4c => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10299b50u64) }
        0x10299b50 => { vm.decode(0x420c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 915i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10299b54u64).wrapping_add((imm * 4) as u64) } else { 0x10299b54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10299b54u64) }
        0x10299b54 => { vm.decode(0x2004875u32); let d = 8i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299b58u64) }
        0x10299b58 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b5cu64) }
        0x10299b5c => { vm.decode(0x54a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b60u64) }
        0x10299b60 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b64u64) }
        0x10299b64 => { vm.decode(0x9b1055f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b68u64) }
        0x10299b68 => { vm.decode(0x1b3015f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b6cu64) }
        0x10299b6c => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b70u64) }
        0x10299b70 => { vm.decode(0x5290c0f5u32); let d = 12i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b74u64) }
        0x10299b74 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299b78u64) }
        0x10299b78 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299b7cu64) }
        0x10299b7c => { vm.decode(0xb381470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299b80u64) }
        0x10299b80 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299b84u64) }
        0x10299b84 => { vm.decode(0x2a8e636fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b88u64) }
        0x10299b88 => { vm.decode(0xa2e234f5u32); let d = 13i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299b8cu64) }
        0x10299b8c => { vm.decode(0x82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299b90u64) }
        0x10299b90 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299b94u64) }
        0x10299b94 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299b98u64) }
        0x10299b98 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299b9cu64) }
        0x10299b9c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299ba0u64) }
        0x10299ba0 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299ba4u64) }
        0x10299ba4 => { vm.decode(0x2088e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ba8u64) }
        0x10299ba8 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299bacu64) }
        0x10299bac => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bb0u64) }
        0x10299bb0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299bb4u64) }
        0x10299bb4 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299bb8u64) }
        0x10299bb8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299bbcu64) }
        0x10299bbc => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299bc0u64) }
        0x10299bc0 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299bc4u64) }
        0x10299bc4 => { vm.decode(0x3692c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bc8u64) }
        0x10299bc8 => { vm.decode(0x50d2a0f5u32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bccu64) }
        0x10299bcc => { vm.decode(0x220d1122u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299bd0u64) }
        0x10299bd0 => { vm.decode(0x52e000f5u32); let d = 16i16; let s = 5i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bd4u64) }
        0x10299bd4 => { vm.decode(0x70c3a0f5u32); let d = 17i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bd8u64) }
        0x10299bd8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299bdcu64) }
        0x10299bdc => { vm.decode(0xa34424f5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299be0u64) }
        0x10299be0 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299be4u64) }
        0x10299be4 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299be8u64) }
        0x10299be8 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299becu64) }
        0x10299bec => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299bf0u64) }
        0x10299bf0 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299bf4u64) }
        0x10299bf4 => { vm.decode(0x54e0a0f5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299bf8u64) }
        0x10299bf8 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299bfcu64) }
        0x10299bfc => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c00u64) }
        0x10299c00 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c04u64) }
        0x10299c04 => { vm.decode(0x62e234f5u32); let d = 13i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c08u64) }
        0x10299c08 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299c0cu64) }
        0x10299c0c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c10u64) }
        0x10299c10 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c14u64) }
        0x10299c14 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c18u64) }
        0x10299c18 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c1cu64) }
        0x10299c1c => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c20u64) }
        0x10299c20 => { vm.decode(0x44901875u32); let d = 6i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c24u64) }
        0x10299c24 => { vm.decode(0xab4434f5u32); let d = 19i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c28u64) }
        0x10299c28 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c2cu64) }
        0x10299c2c => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c30u64) }
        0x10299c30 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c34u64) }
        0x10299c34 => { vm.decode(0x32d200f5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299c38u64) }
        0x10299c38 => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299c3cu64) }
        0x10299c3c => { vm.decode(0xc66cb2adu32); let d = 20i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c40u64) }
        0x10299c40 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c44u64) }
        0x10299c44 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299c48u64) }
        0x10299c48 => { vm.decode(0x56a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299c4cu64) }
        0x10299c4c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c50u64) }
        0x10299c50 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c54u64) }
        0x10299c54 => { vm.decode(0x3603a835u32); let d = 25i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c58u64) }
        0x10299c58 => { vm.decode(0x50b1e0f5u32); let d = 12i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299c5cu64) }
        0x10299c5c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c60u64) }
        0x10299c60 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c64u64) }
        0x10299c64 => { vm.decode(0x12624f5u32); let d = 20i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c68u64) }
        0x10299c68 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c6cu64) }
        0x10299c6c => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c70u64) }
        0x10299c70 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c74u64) }
        0x10299c74 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c78u64) }
        0x10299c78 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c7cu64) }
        0x10299c7c => { vm.decode(0x33142835u32); let d = 13i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c80u64) }
        0x10299c80 => { vm.decode(0x3490a0f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299c84u64) }
        0x10299c84 => { vm.decode(0x8b1814f0u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299c88u64) }
        0x10299c88 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299c8cu64) }
        0x10299c8c => { vm.decode(0x6a8034f5u32); let d = 10i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299c90u64) }
        0x10299c90 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299c94u64) }
        0x10299c94 => { vm.decode(0x50462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299c98u64) }
        0x10299c98 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299c9cu64) }
        0x10299c9c => { vm.decode(0x20a80a2fu32); let d = 11i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ca0u64) }
        0x10299ca0 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ca4u64) }
        0x10299ca4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ca8u64) }
        0x10299ca8 => { vm.decode(0x6080402fu32); let d = 17i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299cacu64) }
        0x10299cac => { vm.decode(0x4088022fu32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299cb0u64) }
        0x10299cb0 => { vm.decode(0x24501835u32); let d = 16i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299cb4u64) }
        0x10299cb4 => { vm.decode(0x70b1e0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299cb8u64) }
        0x10299cb8 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299cbcu64) }
        0x10299cbc => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299cc0u64) }
        0x10299cc0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299cc4u64) }
        0x10299cc4 => { vm.decode(0x5007a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299cc8u64) }
        0x10299cc8 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299cccu64) }
        0x10299ccc => { vm.decode(0x688a4bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299cd0u64) }
        0x10299cd0 => { vm.decode(0x76b3c0f5u32); let d = 15i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299cd4u64) }
        0x10299cd4 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299cd8u64) }
        0x10299cd8 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299cdcu64) }
        0x10299cdc => { vm.decode(0x60a8032fu32); let d = 19i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ce0u64) }
        0x10299ce0 => { vm.decode(0x43501835u32); let d = 12i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299ce4u64) }
        0x10299ce4 => { vm.decode(0x54b260f5u32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299ce8u64) }
        0x10299ce8 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299cecu64) }
        0x10299cec => { vm.decode(0xeae234f5u32); let d = 15i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299cf0u64) }
        0x10299cf0 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299cf4u64) }
        0x10299cf4 => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299cf8u64) }
        0x10299cf8 => { vm.decode(0x20472835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299cfcu64) }
        0x10299cfc => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299d00u64) }
        0x10299d00 => { vm.decode(0x534f2835u32); let d = 15i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d04u64) }
        0x10299d04 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d08u64) }
        0x10299d08 => { vm.decode(0x14a020f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d0cu64) }
        0x10299d0c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d10u64) }
        0x10299d10 => { vm.decode(0x6a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d14u64) }
        0x10299d14 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d18u64) }
        0x10299d18 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299d1cu64) }
        0x10299d1c => { vm.decode(0x404c1835u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d20u64) }
        0x10299d20 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299d24u64) }
        0x10299d24 => { vm.decode(0x56a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d28u64) }
        0x10299d28 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d2cu64) }
        0x10299d2c => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d30u64) }
        0x10299d30 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d34u64) }
        0x10299d34 => { vm.decode(0x1b301470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299d38u64) }
        0x10299d38 => { vm.decode(0x34b220f5u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d3cu64) }
        0x10299d3c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d40u64) }
        0x10299d40 => { vm.decode(0xeac034f5u32); let d = 11i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d44u64) }
        0x10299d44 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d48u64) }
        0x10299d48 => { vm.decode(0x25462835u32); let d = 21i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d4cu64) }
        0x10299d4c => { vm.decode(0x569380f5u32); let d = 11i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d50u64) }
        0x10299d50 => { vm.decode(0x42105875u32); let d = 10i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d54u64) }
        0x10299d54 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d58u64) }
        0x10299d58 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d5cu64) }
        0x10299d5c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d60u64) }
        0x10299d60 => { vm.decode(0x549160f5u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d64u64) }
        0x10299d64 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d68u64) }
        0x10299d68 => { vm.decode(0x6aa234f5u32); let d = 14i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d6cu64) }
        0x10299d6c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d70u64) }
        0x10299d70 => { vm.decode(0x524f2835u32); let d = 11i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d74u64) }
        0x10299d74 => { vm.decode(0x16b040f5u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d78u64) }
        0x10299d78 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d7cu64) }
        0x10299d7c => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d80u64) }
        0x10299d80 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d84u64) }
        0x10299d84 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d88u64) }
        0x10299d88 => { vm.decode(0x14e020f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299d8cu64) }
        0x10299d8c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299d90u64) }
        0x10299d90 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299d94u64) }
        0x10299d94 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299d98u64) }
        0x10299d98 => { vm.decode(0x145ca835u32); let d = 19i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299d9cu64) }
        0x10299d9c => { vm.decode(0x7691c0f5u32); let d = 14i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299da0u64) }
        0x10299da0 => { vm.decode(0x40305875u32); let d = 2i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299da4u64) }
        0x10299da4 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299da8u64) }
        0x10299da8 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299dacu64) }
        0x10299dac => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299db0u64) }
        0x10299db0 => { vm.decode(0x74e1a0f5u32); let d = 18i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299db4u64) }
        0x10299db4 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299db8u64) }
        0x10299db8 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299dbcu64) }
        0x10299dbc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299dc0u64) }
        0x10299dc0 => { vm.decode(0x135c8835u32); let d = 14i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299dc4u64) }
        0x10299dc4 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299dc8u64) }
        0x10299dc8 => { vm.decode(0x36c000f5u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299dccu64) }
        0x10299dcc => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299dd0u64) }
        0x10299dd0 => { vm.decode(0x490424f5u32); let d = 18i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299dd4u64) }
        0x10299dd4 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299dd8u64) }
        0x10299dd8 => { vm.decode(0x428e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ddcu64) }
        0x10299ddc => { vm.decode(0x70c1e0f5u32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299de0u64) }
        0x10299de0 => { vm.decode(0xc66c22edu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299de4u64) }
        0x10299de4 => { vm.decode(0x232624f5u32); let d = 20i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299de8u64) }
        0x10299de8 => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299decu64) }
        0x10299dec => { vm.decode(0x9b00d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299df0u64) }
        0x10299df0 => { vm.decode(0x4153835u32); let d = 17i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299df4u64) }
        0x10299df4 => { vm.decode(0x54e000f5u32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299df8u64) }
        0x10299df8 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299dfcu64) }
        0x10299dfc => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e00u64) }
        0x10299e00 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e04u64) }
        0x10299e04 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e08u64) }
        0x10299e08 => { vm.decode(0x99289cf0u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e0cu64) }
        0x10299e0c => { vm.decode(0x131c63b5u32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e10u64) }
        0x10299e10 => { vm.decode(0xb2855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e14u64) }
        0x10299e14 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e18u64) }
        0x10299e18 => { vm.decode(0x1b205430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e1cu64) }
        0x10299e1c => { vm.decode(0x2282c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e20u64) }
        0x10299e20 => { vm.decode(0x690424f5u32); let d = 18i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e24u64) }
        0x10299e24 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e28u64) }
        0x10299e28 => { vm.decode(0x6a84cb6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e2cu64) }
        0x10299e2c => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e30u64) }
        0x10299e30 => { vm.decode(0x16e000f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e34u64) }
        0x10299e34 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e38u64) }
        0x10299e38 => { vm.decode(0x410624f5u32); let d = 20i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e3cu64) }
        0x10299e3c => { vm.decode(0x34c080f5u32); let d = 18i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e40u64) }
        0x10299e40 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e44u64) }
        0x10299e44 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e48u64) }
        0x10299e48 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e4cu64) }
        0x10299e4c => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e50u64) }
        0x10299e50 => { vm.decode(0x250f1835u32); let d = 22i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e54u64) }
        0x10299e54 => { vm.decode(0xb109470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e58u64) }
        0x10299e58 => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e5cu64) }
        0x10299e5c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e60u64) }
        0x10299e60 => { vm.decode(0xaa2434f5u32); let d = 18i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e64u64) }
        0x10299e64 => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e68u64) }
        0x10299e68 => { vm.decode(0x50b3c0f5u32); let d = 13i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e6cu64) }
        0x10299e6c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e70u64) }
        0x10299e70 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e74u64) }
        0x10299e74 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e78u64) }
        0x10299e78 => { vm.decode(0x5010a835u32); let d = 1i16; let s = 13i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e7cu64) }
        0x10299e7c => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299e80u64) }
        0x10299e80 => { vm.decode(0x309240f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e84u64) }
        0x10299e84 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299e88u64) }
        0x10299e88 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299e8cu64) }
        0x10299e8c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e90u64) }
        0x10299e90 => { vm.decode(0x208063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299e94u64) }
        0x10299e94 => { vm.decode(0x72008835u32); let d = 8i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299e98u64) }
        0x10299e98 => { vm.decode(0x30b240f5u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299e9cu64) }
        0x10299e9c => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299ea0u64) }
        0x10299ea0 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299ea4u64) }
        0x10299ea4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ea8u64) }
        0x10299ea8 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299eacu64) }
        0x10299eac => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299eb0u64) }
        0x10299eb0 => { vm.decode(0x7093c0f5u32); let d = 13i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299eb4u64) }
        0x10299eb4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299eb8u64) }
        0x10299eb8 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299ebcu64) }
        0x10299ebc => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ec0u64) }
        0x10299ec0 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ec4u64) }
        0x10299ec4 => { vm.decode(0x7300a835u32); let d = 13i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299ec8u64) }
        0x10299ec8 => { vm.decode(0x309140f5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299eccu64) }
        0x10299ecc => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299ed0u64) }
        0x10299ed0 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299ed4u64) }
        0x10299ed4 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ed8u64) }
        0x10299ed8 => { vm.decode(0x3043a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299edcu64) }
        0x10299edc => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299ee0u64) }
        0x10299ee0 => { vm.decode(0x42806befu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ee4u64) }
        0x10299ee4 => { vm.decode(0x3492c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299ee8u64) }
        0x10299ee8 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299eecu64) }
        0x10299eec => { vm.decode(0xeae214f5u32); let d = 15i16; let s = 3i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299ef0u64) }
        0x10299ef0 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ef4u64) }
        0x10299ef4 => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ef8u64) }
        0x10299ef8 => { vm.decode(0x730ca835u32); let d = 15i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299efcu64) }
        0x10299efc => { vm.decode(0x50c100f5u32); let d = 16i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f00u64) }
        0x10299f00 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f04u64) }
        0x10299f04 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f08u64) }
        0x10299f08 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f0cu64) }
        0x10299f0c => { vm.decode(0x20113835u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f10u64) }
        0x10299f10 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299f14u64) }
        0x10299f14 => { vm.decode(0x2801875u32); let d = 12i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f18u64) }
        0x10299f18 => { vm.decode(0x148100f5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f1cu64) }
        0x10299f1c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f20u64) }
        0x10299f20 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f24u64) }
        0x10299f24 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f28u64) }
        0x10299f28 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f2cu64) }
        0x10299f2c => { vm.decode(0x200c0835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f30u64) }
        0x10299f30 => { vm.decode(0x228e43afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f34u64) }
        0x10299f34 => { vm.decode(0x50e100f5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f38u64) }
        0x10299f38 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f3cu64) }
        0x10299f3c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f40u64) }
        0x10299f40 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f44u64) }
        0x10299f44 => { vm.decode(0x433835u32); let d = 1i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f48u64) }
        0x10299f48 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299f4cu64) }
        0x10299f4c => { vm.decode(0x70e180f5u32); let d = 16i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f50u64) }
        0x10299f50 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f54u64) }
        0x10299f54 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f58u64) }
        0x10299f58 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f5cu64) }
        0x10299f5c => { vm.decode(0x25503835u32); let d = 21i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f60u64) }
        0x10299f60 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f64u64) }
        0x10299f64 => { vm.decode(0x10e000f5u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f68u64) }
        0x10299f68 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f6cu64) }
        0x10299f6c => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f70u64) }
        0x10299f70 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f74u64) }
        0x10299f74 => { vm.decode(0x244a3835u32); let d = 19i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f78u64) }
        0x10299f78 => { vm.decode(0x4a80c875u32); let d = 31i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f7cu64) }
        0x10299f7c => { vm.decode(0x549200f5u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299f80u64) }
        0x10299f80 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299f84u64) }
        0x10299f84 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299f88u64) }
        0x10299f88 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299f8cu64) }
        0x10299f8c => { vm.decode(0x71140835u32); let d = 4i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299f90u64) }
        0x10299f90 => { vm.decode(0x938dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299f94u64) }
        0x10299f94 => { vm.decode(0x52bb5u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299f98u64) }
        0x10299f98 => { vm.decode(0x24d3022u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299f9cu64) }
        0x10299f9c => { vm.decode(0x2209875u32); let d = 9i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299fa0u64) }
        0x10299fa0 => { vm.decode(0xb181570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299fa4u64) }
        0x10299fa4 => { vm.decode(0x401b2d35u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299fa8u64) }
        0x10299fa8 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299facu64) }
        0x10299fac => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299fb0u64) }
        0x10299fb0 => { vm.decode(0x282e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fb4u64) }
        0x10299fb4 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299fb8u64) }
        0x10299fb8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fbcu64) }
        0x10299fbc => { vm.decode(0x74160835u32); let d = 16i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299fc0u64) }
        0x10299fc0 => { vm.decode(0xd39c53b5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299fc4u64) }
        0x10299fc4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299fc8u64) }
        0x10299fc8 => { vm.decode(0xe8c034f5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299fccu64) }
        0x10299fcc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fd0u64) }
        0x10299fd0 => { vm.decode(0x88ee36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fd4u64) }
        0x10299fd4 => { vm.decode(0x554d2835u32); let d = 23i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299fd8u64) }
        0x10299fd8 => { vm.decode(0x6080482fu32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fdcu64) }
        0x10299fdc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299fe0u64) }
        0x10299fe0 => { vm.decode(0x51581835u32); let d = 6i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299fe4u64) }
        0x10299fe4 => { vm.decode(0xb085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299fe8u64) }
        0x10299fe8 => { vm.decode(0x639c73b5u32); let d = 25i16; let s = 30i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299fecu64) }
        0x10299fec => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299ff0u64) }
        0x10299ff0 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299ff4u64) }
        0x10299ff4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299ff8u64) }
        0x10299ff8 => { vm.decode(0x54591835u32); let d = 18i16; let s = 22i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299ffcu64) }
        0x10299ffc => { vm.decode(0x28886bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a000u64) }
        0x1029a000 => { vm.decode(0x30f300f5u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a004u64) }
        0x1029a004 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a008u64) }
        0x1029a008 => { vm.decode(0xab0614f5u32); let d = 22i16; let s = 3i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a00cu64) }
        0x1029a00c => { vm.decode(0xe39c73b5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a010u64) }
        0x1029a010 => { vm.decode(0x9b3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a014u64) }
        0x1029a014 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a018u64) }
        0x1029a018 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a01cu64) }
        0x1029a01c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a020u64) }
        0x1029a020 => { vm.decode(0x55021835u32); let d = 20i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a024u64) }
        0x1029a024 => { vm.decode(0x40b00875u32); let d = 6i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a028u64) }
        0x1029a028 => { vm.decode(0x721c13b5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a02cu64) }
        0x1029a02c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a030u64) }
        0x1029a030 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a034u64) }
        0x1029a034 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a038u64) }
        0x1029a038 => { vm.decode(0x2a8cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a03cu64) }
        0x1029a03c => { vm.decode(0x374c0835u32); let d = 30i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a040u64) }
        0x1029a040 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a044u64) }
        0x1029a044 => { vm.decode(0x229c33b5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a048u64) }
        0x1029a048 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a04cu64) }
        0x1029a04c => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a050u64) }
        0x1029a050 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a054u64) }
        0x1029a054 => { vm.decode(0x700b3835u32); let d = 3i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a058u64) }
        0x1029a058 => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a05cu64) }
        0x1029a05c => { vm.decode(0x886befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a060u64) }
        0x1029a060 => { vm.decode(0x23dc13b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a064u64) }
        0x1029a064 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a068u64) }
        0x1029a068 => { vm.decode(0xa0e234f5u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a06cu64) }
        0x1029a06c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a070u64) }
        0x1029a070 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a074u64) }
        0x1029a074 => { vm.decode(0x7048a835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a078u64) }
        0x1029a078 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a07cu64) }
        0x1029a07c => { vm.decode(0x22824befu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a080u64) }
        0x1029a080 => { vm.decode(0xa29c73b5u32); let d = 25i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a084u64) }
        0x1029a084 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a088u64) }
        0x1029a088 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a08cu64) }
        0x1029a08c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a090u64) }
        0x1029a090 => { vm.decode(0x76433835u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a094u64) }
        0x1029a094 => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a098u64) }
        0x1029a098 => { vm.decode(0x92dc13b5u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a09cu64) }
        0x1029a09c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a0a0u64) }
        0x1029a0a0 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a0a4u64) }
        0x1029a0a4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0a8u64) }
        0x1029a0a8 => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0acu64) }
        0x1029a0ac => { vm.decode(0x7004a835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a0b0u64) }
        0x1029a0b0 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a0b4u64) }
        0x1029a0b4 => { vm.decode(0x93dc53b5u32); let d = 15i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a0b8u64) }
        0x1029a0b8 => { vm.decode(0x46805875u32); let d = 14i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a0bcu64) }
        0x1029a0bc => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a0c0u64) }
        0x1029a0c0 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a0c4u64) }
        0x1029a0c4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0c8u64) }
        0x1029a0c8 => { vm.decode(0x5044a835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a0ccu64) }
        0x1029a0cc => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a0d0u64) }
        0x1029a0d0 => { vm.decode(0xb21c03b5u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a0d4u64) }
        0x1029a0d4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a0d8u64) }
        0x1029a0d8 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a0dcu64) }
        0x1029a0dc => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0e0u64) }
        0x1029a0e0 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0e4u64) }
        0x1029a0e4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a0e8u64) }
        0x1029a0e8 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a0ecu64) }
        0x1029a0ec => { vm.decode(0x608043efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a0f0u64) }
        0x1029a0f0 => { vm.decode(0xd21c03b5u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a0f4u64) }
        0x1029a0f4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a0f8u64) }
        0x1029a0f8 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a0fcu64) }
        0x1029a0fc => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a100u64) }
        0x1029a100 => { vm.decode(0x20542835u32); let d = 1i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a104u64) }
        0x1029a104 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a108u64) }
        0x1029a108 => { vm.decode(0x321c43b5u32); let d = 2i16; let s = 12i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a10cu64) }
        0x1029a10c => { vm.decode(0x4820c875u32); let d = 19i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a110u64) }
        0x1029a110 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a114u64) }
        0x1029a114 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a118u64) }
        0x1029a118 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a11cu64) }
        0x1029a11c => { vm.decode(0x20152835u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a120u64) }
        0x1029a120 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a124u64) }
        0x1029a124 => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a128u64) }
        0x1029a128 => { vm.decode(0x931c43b5u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a12cu64) }
        0x1029a12c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a130u64) }
        0x1029a130 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a134u64) }
        0x1029a134 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a138u64) }
        0x1029a138 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a13cu64) }
        0x1029a13c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a140u64) }
        0x1029a140 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a144u64) }
        0x1029a144 => { vm.decode(0x1830bb5u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a148u64) }
        0x1029a148 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a14cu64) }
        0x1029a14c => { vm.decode(0x30f1022u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a150u64) }
        0x1029a150 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a154u64) }
        0x1029a154 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a158u64) }
        0x1029a158 => { vm.decode(0x208cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a15cu64) }
        0x1029a15c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a160u64) }
        0x1029a160 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a164u64) }
        0x1029a164 => { vm.decode(0x142835u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a168u64) }
        0x1029a168 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a16cu64) }
        0x1029a16c => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a170u64) }
        0x1029a170 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a174u64) }
        0x1029a174 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a178u64) }
        0x1029a178 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a17cu64) }
        0x1029a17c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a180u64) }
        0x1029a180 => { vm.decode(0x2a8ceb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a184u64) }
        0x1029a184 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a188u64) }
        0x1029a188 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a18cu64) }
        0x1029a18c => { vm.decode(0x28884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a190u64) }
        0x1029a190 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029a194u64) }
        0x1029a194 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a198u64) }
        0x1029a198 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a19cu64) }
        0x1029a19c => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a1a0u64) }
        0x1029a1a0 => { vm.decode(0x230bb5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a1a4u64) }
        0x1029a1a4 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a1a8u64) }
        0x1029a1a8 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a1acu64) }
        0x1029a1ac => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a1b0u64) }
        0x1029a1b0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a1b4u64) }
        0x1029a1b4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a1b8u64) }
        0x1029a1b8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a1bcu64) }
        0x1029a1bc => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a1c0u64) }
        0x1029a1c0 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a1c4u64) }
        0x1029a1c4 => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a1c8u64) }
        0x1029a1c8 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a1ccu64) }
        0x1029a1cc => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a1d0u64) }
        0x1029a1d0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a1d4u64) }
        0x1029a1d4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a1d8u64) }
        0x1029a1d8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a1dcu64) }
        0x1029a1dc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a1e0u64) }
        0x1029a1e0 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a1e4u64) }
        0x1029a1e4 => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a1e8u64) }
        0x1029a1e8 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a1ecu64) }
        0x1029a1ec => { vm.decode(0x208263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a1f0u64) }
        0x1029a1f0 => { vm.decode(0x65012835u32); let d = 21i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a1f4u64) }
        0x1029a1f4 => { vm.decode(0x80010bb5u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a1f8u64) }
        0x1029a1f8 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a1fcu64) }
        0x1029a1fc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a200u64) }
        0x1029a200 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a204u64) }
        0x1029a204 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a208u64) }
        0x1029a208 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a20cu64) }
        0x1029a20c => { vm.decode(0x45192835u32); let d = 23i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a210u64) }
        0x1029a210 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a214u64) }
        0x1029a214 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a218u64) }
        0x1029a218 => { vm.decode(0xa001875u32); let d = 24i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a21cu64) }
        0x1029a21c => { vm.decode(0x62357062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a220u64) }
        0x1029a220 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a224u64) }
        0x1029a224 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a228u64) }
        0x1029a228 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a22cu64) }
        0x1029a22c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a230u64) }
        0x1029a230 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a234u64) }
        0x1029a234 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a238u64) }
        0x1029a238 => { vm.decode(0x40e12bb5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a23cu64) }
        0x1029a23c => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a240u64) }
        0x1029a240 => { vm.decode(0x21d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a244u64) }
        0x1029a244 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a248u64) }
        0x1029a248 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a24cu64) }
        0x1029a24c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a250u64) }
        0x1029a250 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a254u64) }
        0x1029a254 => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a258u64) }
        0x1029a258 => { vm.decode(0x64090835u32); let d = 18i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a25cu64) }
        0x1029a25c => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a260u64) }
        0x1029a260 => { vm.decode(0x1840bb5u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a264u64) }
        0x1029a264 => { vm.decode(0x40126d35u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a268u64) }
        0x1029a268 => { vm.decode(0x42471062u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a26cu64) }
        0x1029a26c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a270u64) }
        0x1029a270 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a274u64) }
        0x1029a274 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a278u64) }
        0x1029a278 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a27cu64) }
        0x1029a27c => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a280u64) }
        0x1029a280 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a284u64) }
        0x1029a284 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a288u64) }
        0x1029a288 => { vm.decode(0x1472bb5u32); let d = 15i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a28cu64) }
        0x1029a28c => { vm.decode(0x60926d35u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a290u64) }
        0x1029a290 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a294u64) }
        0x1029a294 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a298u64) }
        0x1029a298 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a29cu64) }
        0x1029a29c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2a0u64) }
        0x1029a2a0 => { vm.decode(0x65120835u32); let d = 20i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a2a4u64) }
        0x1029a2a4 => { vm.decode(0x41800bb5u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a2a8u64) }
        0x1029a2a8 => { vm.decode(0x400b4d35u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a2acu64) }
        0x1029a2ac => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a2b0u64) }
        0x1029a2b0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a2b4u64) }
        0x1029a2b4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a2b8u64) }
        0x1029a2b8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2bcu64) }
        0x1029a2bc => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2c0u64) }
        0x1029a2c0 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a2c4u64) }
        0x1029a2c4 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a2c8u64) }
        0x1029a2c8 => { vm.decode(0xa70bb5u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a2ccu64) }
        0x1029a2cc => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a2d0u64) }
        0x1029a2d0 => { vm.decode(0x4480c875u32); let d = 7i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a2d4u64) }
        0x1029a2d4 => { vm.decode(0x63751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a2d8u64) }
        0x1029a2d8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a2dcu64) }
        0x1029a2dc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a2e0u64) }
        0x1029a2e0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a2e4u64) }
        0x1029a2e4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2e8u64) }
        0x1029a2e8 => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2ecu64) }
        0x1029a2ec => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a2f0u64) }
        0x1029a2f0 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a2f4u64) }
        0x1029a2f4 => { vm.decode(0x2a82cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a2f8u64) }
        0x1029a2f8 => { vm.decode(0xa62bb5u32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a2fcu64) }
        0x1029a2fc => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a300u64) }
        0x1029a300 => { vm.decode(0x43453062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a304u64) }
        0x1029a304 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a308u64) }
        0x1029a308 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a30cu64) }
        0x1029a30c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a310u64) }
        0x1029a310 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a314u64) }
        0x1029a314 => { vm.decode(0x2284e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a318u64) }
        0x1029a318 => { vm.decode(0x36042835u32); let d = 25i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a31cu64) }
        0x1029a31c => { vm.decode(0x408b4d35u32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a320u64) }
        0x1029a320 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a324u64) }
        0x1029a324 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a328u64) }
        0x1029a328 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a32cu64) }
        0x1029a32c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a330u64) }
        0x1029a330 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a334u64) }
        0x1029a334 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a338u64) }
        0x1029a338 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a33cu64) }
        0x1029a33c => { vm.decode(0x232bb5u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a340u64) }
        0x1029a340 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a344u64) }
        0x1029a344 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a348u64) }
        0x1029a348 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a34cu64) }
        0x1029a34c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a350u64) }
        0x1029a350 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a354u64) }
        0x1029a354 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a358u64) }
        0x1029a358 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a35cu64) }
        0x1029a35c => { vm.decode(0x370c2835u32); let d = 31i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a360u64) }
        0x1029a360 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a364u64) }
        0x1029a364 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a368u64) }
        0x1029a368 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a36cu64) }
        0x1029a36c => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029a370u64) }
        0x1029a370 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a374u64) }
        0x1029a374 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a378u64) }
        0x1029a378 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a37cu64) }
        0x1029a37c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a380u64) }
        0x1029a380 => { vm.decode(0x88ac36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a384u64) }
        0x1029a384 => { vm.decode(0x114d0835u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a388u64) }
        0x1029a388 => { vm.decode(0x9305db0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a38cu64) }
        0x1029a38c => { vm.decode(0x524a4349u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a390u64) }
        0x1029a390 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a394u64) }
        0x1029a394 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a398u64) }
        0x1029a398 => { vm.decode(0xc109875u32); let d = 17i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a39cu64) }
        0x1029a39c => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3a0u64) }
        0x1029a3a0 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a3a4u64) }
        0x1029a3a4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3a8u64) }
        0x1029a3a8 => { vm.decode(0x2884cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3acu64) }
        0x1029a3ac => { vm.decode(0x340e2835u32); let d = 19i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a3b0u64) }
        0x1029a3b0 => { vm.decode(0x1c52bb5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a3b4u64) }
        0x1029a3b4 => { vm.decode(0x600b6d35u32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a3b8u64) }
        0x1029a3b8 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a3bcu64) }
        0x1029a3bc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a3c0u64) }
        0x1029a3c0 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a3c4u64) }
        0x1029a3c4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3c8u64) }
        0x1029a3c8 => { vm.decode(0x2880eb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3ccu64) }
        0x1029a3cc => { vm.decode(0x350e0835u32); let d = 22i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a3d0u64) }
        0x1029a3d0 => { vm.decode(0x2884eb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3d4u64) }
        0x1029a3d4 => { vm.decode(0x80b2fu32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3d8u64) }
        0x1029a3d8 => { vm.decode(0x32152835u32); let d = 9i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a3dcu64) }
        0x1029a3dc => { vm.decode(0x19209c70u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a3e0u64) }
        0x1029a3e0 => { vm.decode(0x504d4349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a3e4u64) }
        0x1029a3e4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a3e8u64) }
        0x1029a3e8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a3ecu64) }
        0x1029a3ec => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a3f0u64) }
        0x1029a3f0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3f4u64) }
        0x1029a3f4 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a3f8u64) }
        0x1029a3f8 => { vm.decode(0x15442835u32); let d = 21i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a3fcu64) }
        0x1029a3fc => { vm.decode(0x544e4349u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a400u64) }
        0x1029a400 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a404u64) }
        0x1029a404 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a408u64) }
        0x1029a408 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a40cu64) }
        0x1029a40c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a410u64) }
        0x1029a410 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a414u64) }
        0x1029a414 => { vm.decode(0x16440835u32); let d = 24i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a418u64) }
        0x1029a418 => { vm.decode(0x41c20bb5u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029a41cu64) }
        0x1029a41c => { vm.decode(0x608b4d35u32); let d = 10i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029a420u64) }
        0x1029a420 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a424u64) }
        0x1029a424 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a428u64) }
        0x1029a428 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a42cu64) }
        0x1029a42c => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a430u64) }
        0x1029a430 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a434u64) }
        0x1029a434 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a438u64) }
        0x1029a438 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a43cu64) }
        0x1029a43c => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a440u64) }
        0x1029a440 => { vm.decode(0x105c9835u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a444u64) }
        0x1029a444 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a448u64) }
        0x1029a448 => { vm.decode(0x407f5349u32); let d = 18i16; let s = 15i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a44cu64) }
        0x1029a44c => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a450u64) }
        0x1029a450 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a454u64) }
        0x1029a454 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a458u64) }
        0x1029a458 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a45cu64) }
        0x1029a45c => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a460u64) }
        0x1029a460 => { vm.decode(0x4c1835u32); let d = 2i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a464u64) }
        0x1029a464 => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a468u64) }
        0x1029a468 => { vm.decode(0x406f5349u32); let d = 18i16; let s = 14i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a46cu64) }
        0x1029a46c => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a470u64) }
        0x1029a470 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a474u64) }
        0x1029a474 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a478u64) }
        0x1029a478 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a47cu64) }
        0x1029a47c => { vm.decode(0x205d1835u32); let d = 2i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a480u64) }
        0x1029a480 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a484u64) }
        0x1029a484 => { vm.decode(0x406e5349u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a488u64) }
        0x1029a488 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a48cu64) }
        0x1029a48c => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a490u64) }
        0x1029a490 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a494u64) }
        0x1029a494 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a498u64) }
        0x1029a498 => { vm.decode(0x1d1835u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a49cu64) }
        0x1029a49c => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a4a0u64) }
        0x1029a4a0 => { vm.decode(0xd105a11u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a4a4u64) }
        0x1029a4a4 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a4a8u64) }
        0x1029a4a8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a4acu64) }
        0x1029a4ac => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a4b0u64) }
        0x1029a4b0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a4b4u64) }
        0x1029a4b4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a4b8u64) }
        0x1029a4b8 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a4bcu64) }
        0x1029a4bc => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a4c0u64) }
        0x1029a4c0 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a4c4u64) }
        0x1029a4c4 => { vm.decode(0x5d98e911u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a4c8u64) }
        0x1029a4c8 => { vm.decode(0x447d4349u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a4ccu64) }
        0x1029a4cc => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a4d0u64) }
        0x1029a4d0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a4d4u64) }
        0x1029a4d4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a4d8u64) }
        0x1029a4d8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a4dcu64) }
        0x1029a4dc => { vm.decode(0x201e0835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a4e0u64) }
        0x1029a4e0 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a4e4u64) }
        0x1029a4e4 => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a4e8u64) }
        0x1029a4e8 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a4ecu64) }
        0x1029a4ec => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a4f0u64) }
        0x1029a4f0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a4f4u64) }
        0x1029a4f4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a4f8u64) }
        0x1029a4f8 => { vm.decode(0x2280e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a4fcu64) }
        0x1029a4fc => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a500u64) }
        0x1029a500 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a504u64) }
        0x1029a504 => { vm.decode(0x407c4349u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a508u64) }
        0x1029a508 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a50cu64) }
        0x1029a50c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a510u64) }
        0x1029a510 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a514u64) }
        0x1029a514 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a518u64) }
        0x1029a518 => { vm.decode(0x2a84e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a51cu64) }
        0x1029a51c => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a520u64) }
        0x1029a520 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a524u64) }
        0x1029a524 => { vm.decode(0x407d4349u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a528u64) }
        0x1029a528 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a52cu64) }
        0x1029a52c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a530u64) }
        0x1029a530 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a534u64) }
        0x1029a534 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a538u64) }
        0x1029a538 => { vm.decode(0x20572835u32); let d = 1i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a53cu64) }
        0x1029a53c => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a540u64) }
        0x1029a540 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a544u64) }
        0x1029a544 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a548u64) }
        0x1029a548 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a54cu64) }
        0x1029a54c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a550u64) }
        0x1029a550 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a554u64) }
        0x1029a554 => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a558u64) }
        0x1029a558 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a55cu64) }
        0x1029a55c => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a560u64) }
        0x1029a560 => { vm.decode(0xdf4349u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a564u64) }
        0x1029a564 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a568u64) }
        0x1029a568 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a56cu64) }
        0x1029a56c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a570u64) }
        0x1029a570 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a574u64) }
        0x1029a574 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a578u64) }
        0x1029a578 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a57cu64) }
        0x1029a57c => { vm.decode(0xd2ca916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a580u64) }
        0x1029a580 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a584u64) }
        0x1029a584 => { vm.decode(0x628ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a588u64) }
        0x1029a588 => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a58cu64) }
        0x1029a58c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a590u64) }
        0x1029a590 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a594u64) }
        0x1029a594 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a598u64) }
        0x1029a598 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a59cu64) }
        0x1029a59c => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a5a0u64) }
        0x1029a5a0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a5a4u64) }
        0x1029a5a4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a5a8u64) }
        0x1029a5a8 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a5acu64) }
        0x1029a5ac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a5b0u64) }
        0x1029a5b0 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a5b4u64) }
        0x1029a5b4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a5b8u64) }
        0x1029a5b8 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a5bcu64) }
        0x1029a5bc => { vm.decode(0x2fb4349u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a5c0u64) }
        0x1029a5c0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a5c4u64) }
        0x1029a5c4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a5c8u64) }
        0x1029a5c8 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a5ccu64) }
        0x1029a5cc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a5d0u64) }
        0x1029a5d0 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a5d4u64) }
        0x1029a5d4 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a5d8u64) }
        0x1029a5d8 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a5dcu64) }
        0x1029a5dc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a5e0u64) }
        0x1029a5e0 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a5e4u64) }
        0x1029a5e4 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a5e8u64) }
        0x1029a5e8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a5ecu64) }
        0x1029a5ec => { vm.decode(0x60112835u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a5f0u64) }
        0x1029a5f0 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a5f4u64) }
        0x1029a5f4 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a5f8u64) }
        0x1029a5f8 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a5fcu64) }
        0x1029a5fc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a600u64) }
        0x1029a600 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a604u64) }
        0x1029a604 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a608u64) }
        0x1029a608 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a60cu64) }
        0x1029a60c => { vm.decode(0x2020022fu32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a610u64) }
        0x1029a610 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a614u64) }
        0x1029a614 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a618u64) }
        0x1029a618 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a61cu64) }
        0x1029a61c => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a620u64) }
        0x1029a620 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a624u64) }
        0x1029a624 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a628u64) }
        0x1029a628 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a62cu64) }
        0x1029a62c => { vm.decode(0x208c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a630u64) }
        0x1029a630 => { vm.decode(0xda5ac12du32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a634u64) }
        0x1029a634 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a638u64) }
        0x1029a638 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a63cu64) }
        0x1029a63c => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a640u64) }
        0x1029a640 => { vm.decode(0xdacec12du32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a644u64) }
        0x1029a644 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a648u64) }
        0x1029a648 => { vm.decode(0xdadec12du32); let d = 15i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a64cu64) }
        0x1029a64c => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a650u64) }
        0x1029a650 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a654u64) }
        0x1029a654 => { vm.decode(0x228c63afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a658u64) }
        0x1029a658 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a65cu64) }
        0x1029a65c => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a660u64) }
        0x1029a660 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a664u64) }
        0x1029a664 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a668u64) }
        0x1029a668 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a66cu64) }
        0x1029a66c => { vm.decode(0xb148ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a670u64) }
        0x1029a670 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a674u64) }
        0x1029a674 => { vm.decode(0xe3408d75u32); let d = 21i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a678u64) }
        0x1029a678 => { vm.decode(0xe000cd75u32); let d = 20i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a67cu64) }
        0x1029a67c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a680u64) }
        0x1029a680 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a684u64) }
        0x1029a684 => { vm.decode(0xa1087d75u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a688u64) }
        0x1029a688 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a68cu64) }
        0x1029a68c => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a690u64) }
        0x1029a690 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a694u64) }
        0x1029a694 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a698u64) }
        0x1029a698 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a69cu64) }
        0x1029a69c => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a6a0u64) }
        0x1029a6a0 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a6a4u64) }
        0x1029a6a4 => { vm.decode(0xd25ec1adu32); let d = 22i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a6a8u64) }
        0x1029a6a8 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a6acu64) }
        0x1029a6ac => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a6b0u64) }
        0x1029a6b0 => { vm.decode(0xf3084d75u32); let d = 24i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6b4u64) }
        0x1029a6b4 => { vm.decode(0xb2405d75u32); let d = 3i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6b8u64) }
        0x1029a6b8 => { vm.decode(0xa000cd75u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6bcu64) }
        0x1029a6bc => { vm.decode(0xa140ed75u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6c0u64) }
        0x1029a6c0 => { vm.decode(0xf1009d75u32); let d = 22i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6c4u64) }
        0x1029a6c4 => { vm.decode(0xda6e812du32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029a6c8u64) }
        0x1029a6c8 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a6ccu64) }
        0x1029a6cc => { vm.decode(0x2225b4f5u32); let d = 16i16; let s = 12i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6d0u64) }
        0x1029a6d0 => { vm.decode(0x62c5b4f5u32); let d = 25i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6d4u64) }
        0x1029a6d4 => { vm.decode(0xa283a4f5u32); let d = 12i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6d8u64) }
        0x1029a6d8 => { vm.decode(0x6a81a4f5u32); let d = 10i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6dcu64) }
        0x1029a6dc => { vm.decode(0x2361a4f5u32); let d = 1i16; let s = 20i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6e0u64) }
        0x1029a6e0 => { vm.decode(0x6327a4f5u32); let d = 20i16; let s = 22i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6e4u64) }
        0x1029a6e4 => { vm.decode(0xab21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6e8u64) }
        0x1029a6e8 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a6ecu64) }
        0x1029a6ec => { vm.decode(0xeb01a4f5u32); let d = 2i16; let s = 19i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6f0u64) }
        0x1029a6f0 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a6f4u64) }
        0x1029a6f4 => { vm.decode(0xaa65b4f5u32); let d = 19i16; let s = 13i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6f8u64) }
        0x1029a6f8 => { vm.decode(0xeae3b4f5u32); let d = 15i16; let s = 15i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a6fcu64) }
        0x1029a6fc => { vm.decode(0x6b01a4f5u32); let d = 2i16; let s = 18i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a700u64) }
        0x1029a700 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a704u64) }
        0x1029a704 => { vm.decode(0xa267a4f5u32); let d = 21i16; let s = 5i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a708u64) }
        0x1029a708 => { vm.decode(0x2a25a4f5u32); let d = 18i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a70cu64) }
        0x1029a70c => { vm.decode(0xe2c3a4f5u32); let d = 13i16; let s = 3i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a710u64) }
        0x1029a710 => { vm.decode(0x2b01b4f5u32); let d = 2i16; let s = 24i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a714u64) }
        0x1029a714 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a718u64) }
        0x1029a718 => { vm.decode(0x45906011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a71cu64) }
        0x1029a71c => { vm.decode(0x42494349u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a720u64) }
        0x1029a720 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a724u64) }
        0x1029a724 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a728u64) }
        0x1029a728 => { vm.decode(0x4a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a72cu64) }
        0x1029a72c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a730u64) }
        0x1029a730 => { vm.decode(0x208ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a734u64) }
        0x1029a734 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a738u64) }
        0x1029a738 => { vm.decode(0x42eb4349u32); let d = 3i16; let s = 14i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a73cu64) }
        0x1029a73c => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a740u64) }
        0x1029a740 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a744u64) }
        0x1029a744 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a748u64) }
        0x1029a748 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a74cu64) }
        0x1029a74c => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a750u64) }
        0x1029a750 => { vm.decode(0x430d0835u32); let d = 14i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a754u64) }
        0x1029a754 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a758u64) }
        0x1029a758 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a75cu64) }
        0x1029a75c => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a760u64) }
        0x1029a760 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a764u64) }
        0x1029a764 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a768u64) }
        0x1029a768 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a76cu64) }
        0x1029a76c => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a770u64) }
        0x1029a770 => { vm.decode(0x25bc349u32); let d = 4i16; let s = 11i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a774u64) }
        0x1029a774 => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a778u64) }
        0x1029a778 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a77cu64) }
        0x1029a77c => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a780u64) }
        0x1029a780 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a784u64) }
        0x1029a784 => { vm.decode(0x2886e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a788u64) }
        0x1029a788 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a78cu64) }
        0x1029a78c => { vm.decode(0xd104311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a790u64) }
        0x1029a790 => { vm.decode(0x2f8c349u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a794u64) }
        0x1029a794 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a798u64) }
        0x1029a798 => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a79cu64) }
        0x1029a79c => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a7a0u64) }
        0x1029a7a0 => { vm.decode(0x2082012fu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7a4u64) }
        0x1029a7a4 => { vm.decode(0x2880cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7a8u64) }
        0x1029a7a8 => { vm.decode(0x6102a835u32); let d = 5i16; let s = 5i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a7acu64) }
        0x1029a7ac => { vm.decode(0x52ca4349u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a7b0u64) }
        0x1029a7b0 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a7b4u64) }
        0x1029a7b4 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a7b8u64) }
        0x1029a7b8 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a7bcu64) }
        0x1029a7bc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7c0u64) }
        0x1029a7c0 => { vm.decode(0x284eb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7c4u64) }
        0x1029a7c4 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a7c8u64) }
        0x1029a7c8 => { vm.decode(0x5907811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a7ccu64) }
        0x1029a7cc => { vm.decode(0x124a5349u32); let d = 24i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a7d0u64) }
        0x1029a7d0 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a7d4u64) }
        0x1029a7d4 => { vm.decode(0xe1084d75u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a7d8u64) }
        0x1029a7d8 => { vm.decode(0x38434f5u32); let d = 24i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a7dcu64) }
        0x1029a7dc => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7e0u64) }
        0x1029a7e0 => { vm.decode(0x80eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a7e4u64) }
        0x1029a7e4 => { vm.decode(0x16021835u32); let d = 24i16; let s = 8i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a7e8u64) }
        0x1029a7e8 => { vm.decode(0x518f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a7ecu64) }
        0x1029a7ec => { vm.decode(0x2ca5349u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a7f0u64) }
        0x1029a7f0 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a7f4u64) }
        0x1029a7f4 => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a7f8u64) }
        0x1029a7f8 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a7fcu64) }
        0x1029a7fc => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a800u64) }
        0x1029a800 => { vm.decode(0x280eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a804u64) }
        0x1029a804 => { vm.decode(0x44023835u32); let d = 17i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a808u64) }
        0x1029a808 => { vm.decode(0x46f9d349u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a80cu64) }
        0x1029a80c => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a810u64) }
        0x1029a810 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a814u64) }
        0x1029a814 => { vm.decode(0xcb6624f5u32); let d = 23i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a818u64) }
        0x1029a818 => { vm.decode(0x60aa412fu32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a81cu64) }
        0x1029a81c => { vm.decode(0x884cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a820u64) }
        0x1029a820 => { vm.decode(0x4206b835u32); let d = 9i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a824u64) }
        0x1029a824 => { vm.decode(0x52f9d349u32); let d = 31i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a828u64) }
        0x1029a828 => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a82cu64) }
        0x1029a82c => { vm.decode(0xf348fd75u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a830u64) }
        0x1029a830 => { vm.decode(0xcbe634f5u32); let d = 31i16; let s = 6i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a834u64) }
        0x1029a834 => { vm.decode(0x60aa4b2fu32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a838u64) }
        0x1029a838 => { vm.decode(0x288ae36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a83cu64) }
        0x1029a83c => { vm.decode(0x774db835u32); let d = 31i16; let s = 7i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a840u64) }
        0x1029a840 => { vm.decode(0x4d9ce911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a844u64) }
        0x1029a844 => { vm.decode(0x4279d349u32); let d = 22i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a848u64) }
        0x1029a848 => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a84cu64) }
        0x1029a84c => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a850u64) }
        0x1029a850 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a854u64) }
        0x1029a854 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a858u64) }
        0x1029a858 => { vm.decode(0x288ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a85cu64) }
        0x1029a85c => { vm.decode(0x254d9835u32); let d = 22i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a860u64) }
        0x1029a860 => { vm.decode(0x4d10c311u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a864u64) }
        0x1029a864 => { vm.decode(0x5279d349u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a868u64) }
        0x1029a868 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a86cu64) }
        0x1029a86c => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a870u64) }
        0x1029a870 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a874u64) }
        0x1029a874 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a878u64) }
        0x1029a878 => { vm.decode(0x228ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a87cu64) }
        0x1029a87c => { vm.decode(0x374d9835u32); let d = 30i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a880u64) }
        0x1029a880 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a884u64) }
        0x1029a884 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a888u64) }
        0x1029a888 => { vm.decode(0x208e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a88cu64) }
        0x1029a88c => { vm.decode(0x20a8092fu32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a890u64) }
        0x1029a890 => { vm.decode(0xa80cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a894u64) }
        0x1029a894 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a898u64) }
        0x1029a898 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a89cu64) }
        0x1029a89c => { vm.decode(0x350d2835u32); let d = 23i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8a0u64) }
        0x1029a8a0 => { vm.decode(0x2082022fu32); let d = 5i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8a4u64) }
        0x1029a8a4 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8a8u64) }
        0x1029a8a8 => { vm.decode(0x2002432fu32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8acu64) }
        0x1029a8ac => { vm.decode(0x62408835u32); let d = 8i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8b0u64) }
        0x1029a8b0 => { vm.decode(0x2008402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8b4u64) }
        0x1029a8b4 => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a8b8u64) }
        0x1029a8b8 => { vm.decode(0x404f2835u32); let d = 3i16; let s = 3i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8bcu64) }
        0x1029a8bc => { vm.decode(0x2082402fu32); let d = 5i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8c0u64) }
        0x1029a8c0 => { vm.decode(0x45408835u32); let d = 20i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8c4u64) }
        0x1029a8c4 => { vm.decode(0x42859019u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029a8c8u64) }
        0x1029a8c8 => { vm.decode(0x1b0855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a8ccu64) }
        0x1029a8cc => { vm.decode(0x44a09875u32); let d = 7i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a8d0u64) }
        0x1029a8d0 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029a8d4u64) }
        0x1029a8d4 => { vm.decode(0xa8032fu32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8d8u64) }
        0x1029a8d8 => { vm.decode(0x2082032fu32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8dcu64) }
        0x1029a8dc => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8e0u64) }
        0x1029a8e0 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8e4u64) }
        0x1029a8e4 => { vm.decode(0x8824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8e8u64) }
        0x1029a8e8 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8ecu64) }
        0x1029a8ec => { vm.decode(0x2082492fu32); let d = 13i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8f0u64) }
        0x1029a8f0 => { vm.decode(0x88a6bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8f4u64) }
        0x1029a8f4 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8f8u64) }
        0x1029a8f8 => { vm.decode(0x228e6befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a8fcu64) }
        0x1029a8fc => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a900u64) }
        0x1029a900 => { vm.decode(0x6880eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a904u64) }
        0x1029a904 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a908u64) }
        0x1029a908 => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a90cu64) }
        0x1029a90c => { vm.decode(0x535ca835u32); let d = 15i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a910u64) }
        0x1029a910 => { vm.decode(0x4a88c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a914u64) }
        0x1029a914 => { vm.decode(0x145c8835u32); let d = 18i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a918u64) }
        0x1029a918 => { vm.decode(0x5353a835u32); let d = 13i16; let s = 13i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a91cu64) }
        0x1029a91c => { vm.decode(0x1459a835u32); let d = 19i16; let s = 12i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a920u64) }
        0x1029a920 => { vm.decode(0x725f2835u32); let d = 11i16; let s = 11i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a924u64) }
        0x1029a924 => { vm.decode(0x35062835u32); let d = 21i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a928u64) }
        0x1029a928 => { vm.decode(0x6110a835u32); let d = 5i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a92cu64) }
        0x1029a92c => { vm.decode(0x2162835u32); let d = 9i16; let s = 2i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a930u64) }
        0x1029a930 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a934u64) }
        0x1029a934 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a938u64) }
        0x1029a938 => { vm.decode(0x459cf011u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029a93cu64) }
        0x1029a93c => { vm.decode(0x524b4349u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029a940u64) }
        0x1029a940 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029a944u64) }
        0x1029a944 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029a948u64) }
        0x1029a948 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029a94cu64) }
        0x1029a94c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a950u64) }
        0x1029a950 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a954u64) }
        0x1029a954 => { vm.decode(0xa8a43afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a958u64) }
        0x1029a958 => { vm.decode(0xa8082fu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a95cu64) }
        0x1029a95c => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a960u64) }
        0x1029a960 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a964u64) }
        0x1029a964 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a968u64) }
        0x1029a968 => { vm.decode(0x60099835u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a96cu64) }
        0x1029a96c => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a970u64) }
        0x1029a970 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a974u64) }
        0x1029a974 => { vm.decode(0x604e1835u32); let d = 2i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a978u64) }
        0x1029a978 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a97cu64) }
        0x1029a97c => { vm.decode(0x200f1835u32); let d = 2i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a980u64) }
        0x1029a980 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a984u64) }
        0x1029a984 => { vm.decode(0x274b9835u32); let d = 30i16; let s = 15i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a988u64) }
        0x1029a988 => { vm.decode(0x37482835u32); let d = 31i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a98cu64) }
        0x1029a98c => { vm.decode(0x46059835u32); let d = 24i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a990u64) }
        0x1029a990 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a994u64) }
        0x1029a994 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029a998u64) }
        0x1029a998 => { vm.decode(0xa78329f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -913i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029a99cu64).wrapping_add((imm * 4) as u64) } else { 0x1029a99cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029a99cu64) }
        0x1029a99c => { vm.decode(0x6005875u32); let d = 8i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9a0u64) }
        0x1029a9a0 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9a4u64) }
        0x1029a9a4 => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9a8u64) }
        0x1029a9a8 => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9acu64) }
        0x1029a9ac => { vm.decode(0x401e2835u32); let d = 3i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9b0u64) }
        0x1029a9b0 => { vm.decode(0x205f0835u32); let d = 2i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9b4u64) }
        0x1029a9b4 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9b8u64) }
        0x1029a9b8 => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9bcu64) }
        0x1029a9bc => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9c0u64) }
        0x1029a9c0 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9c4u64) }
        0x1029a9c4 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9c8u64) }
        0x1029a9c8 => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9ccu64) }
        0x1029a9cc => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9d0u64) }
        0x1029a9d0 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9d4u64) }
        0x1029a9d4 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9d8u64) }
        0x1029a9d8 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9dcu64) }
        0x1029a9dc => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9e0u64) }
        0x1029a9e0 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9e4u64) }
        0x1029a9e4 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9e8u64) }
        0x1029a9e8 => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9ecu64) }
        0x1029a9ec => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029a9f0u64) }
        0x1029a9f0 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029a9f4u64) }
        0x1029a9f4 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a9f8u64) }
        0x1029a9f8 => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029a9fcu64) }
        0x1029a9fc => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa00u64) }
        0x1029aa00 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa04u64) }
        0x1029aa04 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa08u64) }
        0x1029aa08 => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa0cu64) }
        0x1029aa0c => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa10u64) }
        0x1029aa10 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa14u64) }
        0x1029aa14 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa18u64) }
        0x1029aa18 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa1cu64) }
        0x1029aa1c => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa20u64) }
        0x1029aa20 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa24u64) }
        0x1029aa24 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa28u64) }
        0x1029aa28 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa2cu64) }
        0x1029aa2c => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa30u64) }
        0x1029aa30 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa34u64) }
        0x1029aa34 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa38u64) }
        0x1029aa38 => { vm.decode(0x949119u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029aa3cu64) }
        0x1029aa3c => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa40u64) }
        0x1029aa40 => { vm.decode(0x82c42099u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029aa44u64) }
        0x1029aa44 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029aa48u64) }
        0x1029aa48 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa4cu64) }
        0x1029aa4c => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa50u64) }
        0x1029aa50 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa54u64) }
        0x1029aa54 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa58u64) }
        0x1029aa58 => { vm.decode(0x2886e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa5cu64) }
        0x1029aa5c => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aa60u64) }
        0x1029aa60 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aa64u64) }
        0x1029aa64 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029aa68u64) }
        0x1029aa68 => { vm.decode(0xc8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029aa6cu64).wrapping_add((imm * 4) as u64) } else { 0x1029aa6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029aa6cu64) }
        0x1029aa6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029aa70u64) } }
        0x1029aa70 => { vm.decode(0x10a080a3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x1029aa74u64) }
        0x1029aa74 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029aa78u64) }
        0x1029aa78 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa7cu64) }
        0x1029aa7c => { vm.decode(0x22200f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029aa80u64) }
        0x1029aa80 => { vm.decode(0xdd9e77b3u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029aa84u64) }
        0x1029aa84 => { vm.decode(0xc77ef59u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029aa88u64) }
        0x1029aa88 => { vm.decode(0x42848019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029aa8cu64) }
        0x1029aa8c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029aa90u64) }
        0x1029aa90 => { vm.decode(0x2a012fu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aa94u64) }
        0x1029aa94 => { vm.decode(0x134443b5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029aa98u64) }
        0x1029aa98 => { vm.decode(0x7ddc77b3u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029aa9cu64) }
        0x1029aa9c => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aaa0u64) }
        0x1029aaa0 => { vm.decode(0x918403b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029aaa4u64) }
        0x1029aaa4 => { vm.decode(0xfd9e7733u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029aaa8u64) }
        0x1029aaa8 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aaacu64) }
        0x1029aaac => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029aab0u64) }
        0x1029aab0 => { vm.decode(0xa7d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029aab4u64).wrapping_add((imm * 4) as u64) } else { 0x1029aab4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029aab4u64) }
        0x1029aab4 => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aab8u64) }
        0x1029aab8 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aabcu64) }
        0x1029aabc => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aac0u64) }
        0x1029aac0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aac4u64) }
        0x1029aac4 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aac8u64) }
        0x1029aac8 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aaccu64) }
        0x1029aacc => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aad0u64) }
        0x1029aad0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aad4u64) }
        0x1029aad4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029aad8u64) }
        0x1029aad8 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029aadcu64) }
        0x1029aadc => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029aae0u64) }
        0x1029aae0 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1029aae4u64) }
        0x1029aae4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029aae8u64) } }
        0x1029abfc => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029ac00u64) }
        0x1029ac00 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ac04u64) }
        0x1029ac04 => { vm.decode(0x40528119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029ac08u64) }
        0x1029ac08 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ac0cu64) }
        0x1029ac0c => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029ac10u64) }
        0x1029ac10 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ac14u64) }
        0x1029ac14 => { vm.decode(0x642d8019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029ac18u64) }
        0x1029ac18 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ac1cu64) }
        0x1029ac1c => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029ac20u64) }
        0x1029ac20 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029ac24u64).wrapping_add((imm * 4) as u64) } else { 0x1029ac24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029ac24u64) }
        0x1029ac24 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029ac28u64) } }
        0x1029ac28 => { vm.decode(0x882cd01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 88520i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1029ac2cu64) }
        0x1029ac2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029ac30u64) } }
        0x1029ac30 => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ac34u64) }
        0x1029bd48 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bd4cu64) }
        0x1029bd4c => { vm.decode(0x2a00212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bd50u64) }
        0x1029bd50 => { vm.decode(0x41159u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 328i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bd54u64) }
        0x1029bd54 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bd58u64) }
        0x1029bd58 => { vm.decode(0x8b21c909u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029bd5cu64) }
        0x1029bd5c => { vm.decode(0x9319db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bd60u64) }
        0x1029bd60 => { vm.decode(0x89299cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bd64u64) }
        0x1029bd64 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bd68u64) }
        0x1029bd68 => { vm.decode(0x9219db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bd6cu64) }
        0x1029bd6c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bd70u64) }
        0x1029bd70 => { vm.decode(0x286432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bd74u64) }
        0x1029bd74 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bd78u64) }
        0x1029bd78 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1029bd7cu64 + 4);
        vm.wr(37, 0x1029bd7cu64 + 4);
        Ok(0x1029bd7cu64) }
        0x1029bd7c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bd80u64) }
        0x1029bd80 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bd84u64) }
        0x1029bd84 => { vm.decode(0x631d1162u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029bd88u64) }
        0x1029bd88 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029bd8cu64) }
        0x1029bd8c => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029bd90u64) }
        0x1029bd90 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bd94u64) }
        0x1029bd94 => { vm.decode(0x10002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029bd98u64) }
        0x1029bd98 => { vm.decode(0x20220033u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029bd9cu64) }
        0x1029bd9c => { vm.decode(0x8001236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029bda0u64) }
        0x1029bda0 => { vm.decode(0xa0006000u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 33i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029bda4u64) }
        0x1029bda4 => { vm.decode(0x3222871u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1079i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029bda8u64).wrapping_add((imm * 4) as u64) } else { 0x1029bda8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029bda8u64) }
        0x1029bda8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029bdacu64) } }
        0x1029bdac => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bdb0u64) }
        0x1029bdb0 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bdb4u64) }
        0x1029bdb4 => { vm.decode(0x2276u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029bdb8u64) }
        0x1029bdb8 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029bdbcu64) }
        0x1029bdbc => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029bdc0u64) }
        0x1029bdc0 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bdc4u64) }
        0x1029bdc4 => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029bdc8u64) }
        0x1029bdc8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029bdccu64) }
        0x1029bdcc => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bdd0u64) }
        0x1029bdd0 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029bdd4u64) }
        0x1029bdd4 => { vm.decode(0x740d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bdd8u64) }
        0x1029bdd8 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bddcu64) }
        0x1029bddc => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bde0u64) }
        0x1029bde0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bde4u64) }
        0x1029bde4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bde8u64) }
        0x1029bde8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1029bdecu64 + 4);
        vm.wr(37, 0x1029bdecu64 + 4);
        Ok(0x1029bdecu64) }
        0x1029bdec => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bdf0u64) }
        0x1029bdf0 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bdf4u64) }
        0x1029bdf4 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bdf8u64) }
        0x1029bdf8 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bdfcu64) }
        0x1029bdfc => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be00u64) }
        0x1029be00 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be04u64) }
        0x1029be04 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be08u64) }
        0x1029be08 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be0cu64) }
        0x1029be0c => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be10u64) }
        0x1029be10 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be14u64) }
        0x1029be14 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be18u64) }
        0x1029be18 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be1cu64) }
        0x1029be1c => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be20u64) }
        0x1029be20 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be24u64) }
        0x1029be24 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be28u64) }
        0x1029be28 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029be2cu64) }
        0x1029be2c => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029be30u64) }
        0x1029be30 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029be34u64) }
        0x1029be34 => { vm.decode(0x8c8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029be38u64).wrapping_add((imm * 4) as u64) } else { 0x1029be38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029be38u64) }
        0x1029be38 => { vm.decode(0x642d1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029be3cu64) }
        0x1029be3c => { vm.decode(0x204002beu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be40u64) }
        0x1029be40 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be44u64) }
        0x1029be44 => { vm.decode(0xa0480037u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x1029be48u64) }
        0x1029be48 => { vm.decode(0x81740935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be4cu64) }
        0x1029be4c => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be50u64) }
        0x1029be50 => { vm.decode(0x1242023eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be54u64) }
        0x1029be54 => { vm.decode(0x81700935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be58u64) }
        0x1029be58 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be5cu64) }
        0x1029be5c => { vm.decode(0x124202beu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be60u64) }
        0x1029be60 => { vm.decode(0x81300935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be64u64) }
        0x1029be64 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be68u64) }
        0x1029be68 => { vm.decode(0x242027eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be6cu64) }
        0x1029be6c => { vm.decode(0x32a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be70u64) }
        0x1029be70 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be74u64) }
        0x1029be74 => { vm.decode(0x24202feu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be78u64) }
        0x1029be78 => { vm.decode(0x30a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be7cu64) }
        0x1029be7c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be80u64) }
        0x1029be80 => { vm.decode(0x61041019u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029be84u64) }
        0x1029be84 => { vm.decode(0x1849019u32); let d = 11i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029be88u64) }
        0x1029be88 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029be8cu64) }
        0x1029be8c => { vm.decode(0x184202feu32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be90u64) }
        0x1029be90 => { vm.decode(0x3842027eu32); let d = 13i16; let s = 17i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029be94u64) }
        0x1029be94 => { vm.decode(0x1168ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029be98u64) }
        0x1029be98 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029be9cu64) }
        0x1029be9c => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bea0u64) }
        0x1029bea0 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bea4u64) }
        0x1029bea4 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bea8u64) }
        0x1029bea8 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029beacu64) }
        0x1029beac => { vm.decode(0x27d32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -29i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1029beb0u64).wrapping_add((imm * 4) as u64) } else { 0x1029beb0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029beb0u64) }
        0x1029beb0 => { vm.decode(0x8209875u32); let d = 17i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029beb4u64) }
        0x1029beb4 => { vm.decode(0x4a8aebafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029beb8u64) }
        0x1029beb8 => { vm.decode(0x886c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bebcu64) }
        0x1029bebc => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bec0u64) }
        0x1029bec0 => { vm.decode(0x4880c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bec4u64) }
        0x1029bec4 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1029bec8u64) }
        0x1029bec8 => { vm.decode(0x631d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029beccu64) }
        0x1029becc => { vm.decode(0x2a82c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bed0u64) }
        0x1029bed0 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bed4u64) }
        0x1029bed4 => { vm.decode(0x8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bed8u64) }
        0x1029bed8 => { vm.decode(0x650d1059u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bedcu64) }
        0x1029bedc => { vm.decode(0x228c7bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bee0u64) }
        0x1029bee0 => { vm.decode(0x88ee3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bee4u64) }
        0x1029bee4 => { vm.decode(0x865bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bee8u64) }
        0x1029bee8 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029beecu64) }
        0x1029beec => { vm.decode(0x20867bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bef0u64) }
        0x1029bef0 => { vm.decode(0x28e5bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bef4u64) }
        0x1029bef4 => { vm.decode(0x42805bafu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bef8u64) }
        0x1029bef8 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029befcu64) }
        0x1029befc => { vm.decode(0x62807bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bf00u64) }
        0x1029bf00 => { vm.decode(0x701a2835u32); let d = 3i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf04u64) }
        0x1029bf04 => { vm.decode(0x1b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf08u64) }
        0x1029bf08 => { vm.decode(0x1a3835u32); let d = 3i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf0cu64) }
        0x1029bf0c => { vm.decode(0x1b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf10u64) }
        0x1029bf10 => { vm.decode(0x404f3835u32); let d = 3i16; let s = 14i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf14u64) }
        0x1029bf14 => { vm.decode(0x1b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf18u64) }
        0x1029bf18 => { vm.decode(0x23073835u32); let d = 13i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf1cu64) }
        0x1029bf1c => { vm.decode(0x4a80d875u32); let d = 31i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf20u64) }
        0x1029bf20 => { vm.decode(0x407a835u32); let d = 17i16; let s = 6i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf24u64) }
        0x1029bf24 => { vm.decode(0x48804875u32); let d = 22i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf28u64) }
        0x1029bf28 => { vm.decode(0x23428835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf2cu64) }
        0x1029bf2c => { vm.decode(0xda9119u32); let d = 3i16; let s = 10i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029bf30u64) }
        0x1029bf30 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf34u64) }
        0x1029bf34 => { vm.decode(0x114c9835u32); let d = 6i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf38u64) }
        0x1029bf38 => { vm.decode(0x4680c875u32); let d = 15i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf3cu64) }
        0x1029bf3c => { vm.decode(0x6208a835u32); let d = 11i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf40u64) }
        0x1029bf40 => { vm.decode(0xd94c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029bf44u64) }
        0x1029bf44 => { vm.decode(0x30102200u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029bf48u64) }
        0x1029bf48 => { vm.decode(0x460480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 929i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1029bf4cu64).wrapping_add((imm * 4) as u64) } else { 0x1029bf4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1029bf4cu64) }
        0x1029bf4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029bf50u64) } }
        0x1029bf50 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf54u64) }
        0x1029bf54 => { vm.decode(0x148080f5u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf58u64) }
        0x1029bf58 => { vm.decode(0x769000f5u32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf5cu64) }
        0x1029bf5c => { vm.decode(0x1b3815f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf60u64) }
        0x1029bf60 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf64u64) }
        0x1029bf64 => { vm.decode(0xa0c875u32); let d = 5i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf68u64) }
        0x1029bf68 => { vm.decode(0xb1054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf6cu64) }
        0x1029bf6c => { vm.decode(0x72b1c0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf70u64) }
        0x1029bf70 => { vm.decode(0x9b385570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf74u64) }
        0x1029bf74 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bf78u64) }
        0x1029bf78 => { vm.decode(0x6204875u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bf7cu64) }
        0x1029bf7c => { vm.decode(0x70b360f5u32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bf80u64) }
        0x1029bf80 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bf84u64) }
        0x1029bf84 => { vm.decode(0x1b3014f0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bf88u64) }
        0x1029bf88 => { vm.decode(0x2a844b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bf8cu64) }
        0x1029bf8c => { vm.decode(0xa0e294f5u32); let d = 13i16; let s = 9i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bf90u64) }
        0x1029bf90 => { vm.decode(0xa86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bf94u64) }
        0x1029bf94 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bf98u64) }
        0x1029bf98 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bf9cu64) }
        0x1029bf9c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bfa0u64) }
        0x1029bfa0 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bfa4u64) }
        0x1029bfa4 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bfa8u64) }
        0x1029bfa8 => { vm.decode(0x2088c32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bfacu64) }
        0x1029bfac => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bfb0u64) }
        0x1029bfb0 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bfb4u64) }
        0x1029bfb4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bfb8u64) }
        0x1029bfb8 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bfbcu64) }
        0x1029bfbc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bfc0u64) }
        0x1029bfc0 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bfc4u64) }
        0x1029bfc4 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bfc8u64) }
        0x1029bfc8 => { vm.decode(0x52d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bfccu64) }
        0x1029bfcc => { vm.decode(0x70f3a0f5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bfd0u64) }
        0x1029bfd0 => { vm.decode(0x40908875u32); let d = 7i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bfd4u64) }
        0x1029bfd4 => { vm.decode(0x52c180f5u32); let d = 16i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bfd8u64) }
        0x1029bfd8 => { vm.decode(0x9b201430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029bfdcu64) }
        0x1029bfdc => { vm.decode(0x10e2a0f5u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029bfe0u64) }
        0x1029bfe0 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bfe4u64) }
        0x1029bfe4 => { vm.decode(0xa14484f5u32); let d = 17i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029bfe8u64) }
        0x1029bfe8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bfecu64) }
        0x1029bfec => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029bff0u64) }
        0x1029bff0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bff4u64) }
        0x1029bff4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029bff8u64) }
        0x1029bff8 => { vm.decode(0x4a909875u32); let d = 31i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029bffcu64) }
        0x1029bffc => { vm.decode(0x54e120f5u32); let d = 18i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c000u64) }
        0x1029c000 => { vm.decode(0x1b305530u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c004u64) }
        0x1029c004 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c008u64) }
        0x1029c008 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c00cu64) }
        0x1029c00c => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c010u64) }
        0x1029c010 => { vm.decode(0x60e294f5u32); let d = 13i16; let s = 9i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c014u64) }
        0x1029c014 => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c018u64) }
        0x1029c018 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c01cu64) }
        0x1029c01c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c020u64) }
        0x1029c020 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c024u64) }
        0x1029c024 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c028u64) }
        0x1029c028 => { vm.decode(0x18434f5u32); let d = 24i16; let s = 4i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c02cu64) }
        0x1029c02c => { vm.decode(0x4c909875u32); let d = 23i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c030u64) }
        0x1029c030 => { vm.decode(0xa94494f5u32); let d = 19i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c034u64) }
        0x1029c034 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c038u64) }
        0x1029c038 => { vm.decode(0xc64c72edu32); let d = 20i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c03cu64) }
        0x1029c03c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c040u64) }
        0x1029c040 => { vm.decode(0x76b140f5u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c044u64) }
        0x1029c044 => { vm.decode(0xc6cc72edu32); let d = 21i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c048u64) }
        0x1029c048 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c04cu64) }
        0x1029c04c => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c050u64) }
        0x1029c050 => { vm.decode(0x728380f5u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c054u64) }
        0x1029c054 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c058u64) }
        0x1029c058 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c05cu64) }
        0x1029c05c => { vm.decode(0x330b8835u32); let d = 14i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c060u64) }
        0x1029c060 => { vm.decode(0x7481a0f5u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c064u64) }
        0x1029c064 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c068u64) }
        0x1029c068 => { vm.decode(0x22492fu32); let d = 12i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c06cu64) }
        0x1029c06c => { vm.decode(0x81e224f5u32); let d = 13i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c070u64) }
        0x1029c070 => { vm.decode(0x690484f5u32); let d = 18i16; let s = 9i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c074u64) }
        0x1029c074 => { vm.decode(0x96424f5u32); let d = 19i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c078u64) }
        0x1029c078 => { vm.decode(0x4002422fu32); let d = 20i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c07cu64) }
        0x1029c07c => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c080u64) }
        0x1029c080 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c084u64) }
        0x1029c084 => { vm.decode(0x321c0835u32); let d = 10i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c088u64) }
        0x1029c088 => { vm.decode(0x9309d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c08cu64) }
        0x1029c08c => { vm.decode(0x549120f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c090u64) }
        0x1029c090 => { vm.decode(0xb285530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c094u64) }
        0x1029c094 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c098u64) }
        0x1029c098 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c09cu64) }
        0x1029c09c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0a0u64) }
        0x1029c0a0 => { vm.decode(0x524e0835u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0a4u64) }
        0x1029c0a4 => { vm.decode(0xb081570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c0a8u64) }
        0x1029c0a8 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0acu64) }
        0x1029c0ac => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0b0u64) }
        0x1029c0b0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0b4u64) }
        0x1029c0b4 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0b8u64) }
        0x1029c0b8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0bcu64) }
        0x1029c0bc => { vm.decode(0x4031835u32); let d = 16i16; let s = 12i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0c0u64) }
        0x1029c0c0 => { vm.decode(0x50b060f5u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c0c4u64) }
        0x1029c0c4 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c0c8u64) }
        0x1029c0c8 => { vm.decode(0x20a294f5u32); let d = 12i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c0ccu64) }
        0x1029c0cc => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0d0u64) }
        0x1029c0d0 => { vm.decode(0x103b835u32); let d = 5i16; let s = 12i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0d4u64) }
        0x1029c0d4 => { vm.decode(0x1938dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c0d8u64) }
        0x1029c0d8 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0dcu64) }
        0x1029c0dc => { vm.decode(0x32c200f5u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c0e0u64) }
        0x1029c0e0 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c0e4u64) }
        0x1029c0e4 => { vm.decode(0x814424f5u32); let d = 17i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c0e8u64) }
        0x1029c0e8 => { vm.decode(0x4008412fu32); let d = 18i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c0ecu64) }
        0x1029c0ec => { vm.decode(0x53408835u32); let d = 12i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0f0u64) }
        0x1029c0f0 => { vm.decode(0xa105875u32); let d = 24i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c0f4u64) }
        0x1029c0f4 => { vm.decode(0x8b1855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c0f8u64) }
        0x1029c0f8 => { vm.decode(0x5481a0f5u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c0fcu64) }
        0x1029c0fc => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c100u64) }
        0x1029c100 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c104u64) }
        0x1029c104 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c108u64) }
        0x1029c108 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c10cu64) }
        0x1029c10c => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c110u64) }
        0x1029c110 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c114u64) }
        0x1029c114 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c118u64) }
        0x1029c118 => { vm.decode(0x28826bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c11cu64) }
        0x1029c11c => { vm.decode(0x70a220f5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c120u64) }
        0x1029c120 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c124u64) }
        0x1029c124 => { vm.decode(0xa04084f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c128u64) }
        0x1029c128 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c12cu64) }
        0x1029c12c => { vm.decode(0x20043835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c130u64) }
        0x1029c130 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c134u64) }
        0x1029c134 => { vm.decode(0x56b100f5u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c138u64) }
        0x1029c138 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c13cu64) }
        0x1029c13c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c140u64) }
        0x1029c140 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c144u64) }
        0x1029c144 => { vm.decode(0x2844befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c148u64) }
        0x1029c148 => { vm.decode(0x349220f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c14cu64) }
        0x1029c14c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c150u64) }
        0x1029c150 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c154u64) }
        0x1029c154 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c158u64) }
        0x1029c158 => { vm.decode(0x35462835u32); let d = 21i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c15cu64) }
        0x1029c15c => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c160u64) }
        0x1029c160 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c164u64) }
        0x1029c164 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c168u64) }
        0x1029c168 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c16cu64) }
        0x1029c16c => { vm.decode(0x54b3a0f5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c170u64) }
        0x1029c170 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c174u64) }
        0x1029c174 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c178u64) }
        0x1029c178 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c17cu64) }
        0x1029c17c => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c180u64) }
        0x1029c180 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c184u64) }
        0x1029c184 => { vm.decode(0x76b000f5u32); let d = 10i16; let s = 13i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c188u64) }
        0x1029c188 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c18cu64) }
        0x1029c18c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c190u64) }
        0x1029c190 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c194u64) }
        0x1029c194 => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c198u64) }
        0x1029c198 => { vm.decode(0x14b3a0f5u32); let d = 11i16; let s = 19i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c19cu64) }
        0x1029c19c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c1a0u64) }
        0x1029c1a0 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c1a4u64) }
        0x1029c1a4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1a8u64) }
        0x1029c1a8 => { vm.decode(0x33462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c1acu64) }
        0x1029c1ac => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1b0u64) }
        0x1029c1b0 => { vm.decode(0x16b000f5u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c1b4u64) }
        0x1029c1b4 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c1b8u64) }
        0x1029c1b8 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c1bcu64) }
        0x1029c1bc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1c0u64) }
        0x1029c1c0 => { vm.decode(0x54b220f5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c1c4u64) }
        0x1029c1c4 => { vm.decode(0x4c004875u32); let d = 18i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c1c8u64) }
        0x1029c1c8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c1ccu64) }
        0x1029c1cc => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c1d0u64) }
        0x1029c1d0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1d4u64) }
        0x1029c1d4 => { vm.decode(0x354e0835u32); let d = 22i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c1d8u64) }
        0x1029c1d8 => { vm.decode(0x4a8243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1dcu64) }
        0x1029c1dc => { vm.decode(0x569080f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c1e0u64) }
        0x1029c1e0 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c1e4u64) }
        0x1029c1e4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c1e8u64) }
        0x1029c1e8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1ecu64) }
        0x1029c1ec => { vm.decode(0x228e63efu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c1f0u64) }
        0x1029c1f0 => { vm.decode(0x54b320f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c1f4u64) }
        0x1029c1f4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c1f8u64) }
        0x1029c1f8 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c1fcu64) }
        0x1029c1fc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c200u64) }
        0x1029c200 => { vm.decode(0x36462835u32); let d = 25i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c204u64) }
        0x1029c204 => { vm.decode(0x349080f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c208u64) }
        0x1029c208 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c20cu64) }
        0x1029c20c => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c210u64) }
        0x1029c210 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c214u64) }
        0x1029c214 => { vm.decode(0x11472835u32); let d = 5i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c218u64) }
        0x1029c218 => { vm.decode(0x1928dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c21cu64) }
        0x1029c21c => { vm.decode(0xd31c53b5u32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c220u64) }
        0x1029c220 => { vm.decode(0x9b2055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c224u64) }
        0x1029c224 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c228u64) }
        0x1029c228 => { vm.decode(0x2082e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c22cu64) }
        0x1029c22c => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c230u64) }
        0x1029c230 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c234u64) }
        0x1029c234 => { vm.decode(0x4210c875u32); let d = 11i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c238u64) }
        0x1029c238 => { vm.decode(0x88eeb6fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c23cu64) }
        0x1029c23c => { vm.decode(0x134f0835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c240u64) }
        0x1029c240 => { vm.decode(0x16b180f5u32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c244u64) }
        0x1029c244 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c248u64) }
        0x1029c248 => { vm.decode(0x408434f5u32); let d = 24i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c24cu64) }
        0x1029c24c => { vm.decode(0x549100f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c250u64) }
        0x1029c250 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c254u64) }
        0x1029c254 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c258u64) }
        0x1029c258 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c25cu64) }
        0x1029c25c => { vm.decode(0xa82c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c260u64) }
        0x1029c260 => { vm.decode(0x14052835u32); let d = 17i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c264u64) }
        0x1029c264 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c268u64) }
        0x1029c268 => { vm.decode(0x8b10d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c26cu64) }
        0x1029c26c => { vm.decode(0x5490a0f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c270u64) }
        0x1029c270 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c274u64) }
        0x1029c274 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c278u64) }
        0x1029c278 => { vm.decode(0x230d875u32); let d = 9i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c27cu64) }
        0x1029c27c => { vm.decode(0x8938dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c280u64) }
        0x1029c280 => { vm.decode(0x749380f5u32); let d = 11i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c284u64) }
        0x1029c284 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c288u64) }
        0x1029c288 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c28cu64) }
        0x1029c28c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c290u64) }
        0x1029c290 => { vm.decode(0x511c0835u32); let d = 6i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c294u64) }
        0x1029c294 => { vm.decode(0x928ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c298u64) }
        0x1029c298 => { vm.decode(0x549280f5u32); let d = 11i16; let s = 20i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c29cu64) }
        0x1029c29c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c2a0u64) }
        0x1029c2a0 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c2a4u64) }
        0x1029c2a4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2a8u64) }
        0x1029c2a8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2acu64) }
        0x1029c2ac => { vm.decode(0x570d0835u32); let d = 30i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c2b0u64) }
        0x1029c2b0 => { vm.decode(0x349200f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c2b4u64) }
        0x1029c2b4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c2b8u64) }
        0x1029c2b8 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c2bcu64) }
        0x1029c2bc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2c0u64) }
        0x1029c2c0 => { vm.decode(0x51070835u32); let d = 4i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c2c4u64) }
        0x1029c2c4 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c2c8u64) }
        0x1029c2c8 => { vm.decode(0x44805875u32); let d = 6i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c2ccu64) }
        0x1029c2cc => { vm.decode(0x54b300f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c2d0u64) }
        0x1029c2d0 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c2d4u64) }
        0x1029c2d4 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c2d8u64) }
        0x1029c2d8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2dcu64) }
        0x1029c2dc => { vm.decode(0xa86e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2e0u64) }
        0x1029c2e0 => { vm.decode(0x53050835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c2e4u64) }
        0x1029c2e4 => { vm.decode(0x6305875u32); let d = 8i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c2e8u64) }
        0x1029c2e8 => { vm.decode(0x74b380f5u32); let d = 11i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c2ecu64) }
        0x1029c2ec => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c2f0u64) }
        0x1029c2f0 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c2f4u64) }
        0x1029c2f4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2f8u64) }
        0x1029c2f8 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c2fcu64) }
        0x1029c2fc => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c300u64) }
        0x1029c300 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c304u64) }
        0x1029c304 => { vm.decode(0x74a100f5u32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c308u64) }
        0x1029c308 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c30cu64) }
        0x1029c30c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c310u64) }
        0x1029c310 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c314u64) }
        0x1029c314 => { vm.decode(0x628a6befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c318u64) }
        0x1029c318 => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c31cu64) }
        0x1029c31c => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c320u64) }
        0x1029c320 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c324u64) }
        0x1029c324 => { vm.decode(0x54a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c328u64) }
        0x1029c328 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c32cu64) }
        0x1029c32c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c330u64) }
        0x1029c330 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c334u64) }
        0x1029c334 => { vm.decode(0x201d0835u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c338u64) }
        0x1029c338 => { vm.decode(0x9189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c33cu64) }
        0x1029c33c => { vm.decode(0x44808875u32); let d = 7i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c340u64) }
        0x1029c340 => { vm.decode(0x14a000f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c344u64) }
        0x1029c344 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c348u64) }
        0x1029c348 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c34cu64) }
        0x1029c34c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c350u64) }
        0x1029c350 => { vm.decode(0x228063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c354u64) }
        0x1029c354 => { vm.decode(0x25040835u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c358u64) }
        0x1029c358 => { vm.decode(0x228c4bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c35cu64) }
        0x1029c35c => { vm.decode(0x34a100f5u32); let d = 2i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c360u64) }
        0x1029c360 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c364u64) }
        0x1029c364 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c368u64) }
        0x1029c368 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c36cu64) }
        0x1029c36c => { vm.decode(0x552835u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c370u64) }
        0x1029c370 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c374u64) }
        0x1029c374 => { vm.decode(0x148180f5u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c378u64) }
        0x1029c378 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c37cu64) }
        0x1029c37c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c380u64) }
        0x1029c380 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c384u64) }
        0x1029c384 => { vm.decode(0x250f0835u32); let d = 22i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c388u64) }
        0x1029c388 => { vm.decode(0x74a180f5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c38cu64) }
        0x1029c38c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c390u64) }
        0x1029c390 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c394u64) }
        0x1029c394 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c398u64) }
        0x1029c398 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c39cu64) }
        0x1029c39c => { vm.decode(0x25042835u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c3a0u64) }
        0x1029c3a0 => { vm.decode(0x6909875u32); let d = 13i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c3a4u64) }
        0x1029c3a4 => { vm.decode(0x10e180f5u32); let d = 16i16; let s = 19i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c3a8u64) }
        0x1029c3a8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c3acu64) }
        0x1029c3ac => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c3b0u64) }
        0x1029c3b0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c3b4u64) }
        0x1029c3b4 => { vm.decode(0x20123835u32); let d = 1i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c3b8u64) }
        0x1029c3b8 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c3bcu64) }
        0x1029c3bc => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c3c0u64) }
        0x1029c3c0 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c3c4u64) }
        0x1029c3c4 => { vm.decode(0xb0014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c3c8u64) }
        0x1029c3c8 => { vm.decode(0x44820d35u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c3ccu64) }
        0x1029c3cc => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c3d0u64) }
        0x1029c3d0 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c3d4u64) }
        0x1029c3d4 => { vm.decode(0x282c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c3d8u64) }
        0x1029c3d8 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c3dcu64) }
        0x1029c3dc => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c3e0u64) }
        0x1029c3e0 => { vm.decode(0x54b3835u32); let d = 23i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c3e4u64) }
        0x1029c3e4 => { vm.decode(0xa21c23b5u32); let d = 16i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c3e8u64) }
        0x1029c3e8 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c3ecu64) }
        0x1029c3ec => { vm.decode(0x210424f5u32); let d = 16i16; let s = 5i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c3f0u64) }
        0x1029c3f0 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c3f4u64) }
        0x1029c3f4 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c3f8u64) }
        0x1029c3f8 => { vm.decode(0x20003835u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c3fcu64) }
        0x1029c3fc => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c400u64) }
        0x1029c400 => { vm.decode(0x60004a2fu32); let d = 25i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c404u64) }
        0x1029c404 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c408u64) }
        0x1029c408 => { vm.decode(0x50423835u32); let d = 1i16; let s = 10i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c40cu64) }
        0x1029c40c => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c410u64) }
        0x1029c410 => { vm.decode(0x39c73b5u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c414u64) }
        0x1029c414 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c418u64) }
        0x1029c418 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c41cu64) }
        0x1029c41c => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c420u64) }
        0x1029c420 => { vm.decode(0x74183835u32); let d = 19i16; let s = 17i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c424u64) }
        0x1029c424 => { vm.decode(0x88a63afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c428u64) }
        0x1029c428 => { vm.decode(0x50d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c42cu64) }
        0x1029c42c => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c430u64) }
        0x1029c430 => { vm.decode(0x230414f5u32); let d = 16i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c434u64) }
        0x1029c434 => { vm.decode(0xa21c73b5u32); let d = 24i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c438u64) }
        0x1029c438 => { vm.decode(0xe20c875u32); let d = 25i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c43cu64) }
        0x1029c43c => { vm.decode(0x1b00d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c440u64) }
        0x1029c440 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c444u64) }
        0x1029c444 => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c448u64) }
        0x1029c448 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c44cu64) }
        0x1029c44c => { vm.decode(0x175b1835u32); let d = 30i16; let s = 24i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c450u64) }
        0x1029c450 => { vm.decode(0xa31c73b5u32); let d = 24i16; let s = 15i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c454u64) }
        0x1029c454 => { vm.decode(0x1b105570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c458u64) }
        0x1029c458 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c45cu64) }
        0x1029c45c => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c460u64) }
        0x1029c460 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c464u64) }
        0x1029c464 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c468u64) }
        0x1029c468 => { vm.decode(0x36001835u32); let d = 24i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c46cu64) }
        0x1029c46c => { vm.decode(0xa8843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c470u64) }
        0x1029c470 => { vm.decode(0x135c13b5u32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c474u64) }
        0x1029c474 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c478u64) }
        0x1029c478 => { vm.decode(0x68a234f5u32); let d = 14i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c47cu64) }
        0x1029c47c => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c480u64) }
        0x1029c480 => { vm.decode(0x1007a835u32); let d = 1i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c484u64) }
        0x1029c484 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c488u64) }
        0x1029c488 => { vm.decode(0x2884befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c48cu64) }
        0x1029c48c => { vm.decode(0x235c13b5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c490u64) }
        0x1029c490 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c494u64) }
        0x1029c494 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c498u64) }
        0x1029c498 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c49cu64) }
        0x1029c49c => { vm.decode(0x2a80c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4a0u64) }
        0x1029c4a0 => { vm.decode(0x33008835u32); let d = 12i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c4a4u64) }
        0x1029c4a4 => { vm.decode(0x22844befu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4a8u64) }
        0x1029c4a8 => { vm.decode(0xa29c23b5u32); let d = 17i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c4acu64) }
        0x1029c4ac => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c4b0u64) }
        0x1029c4b0 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c4b4u64) }
        0x1029c4b4 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4b8u64) }
        0x1029c4b8 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4bcu64) }
        0x1029c4bc => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c4c0u64) }
        0x1029c4c0 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c4c4u64) }
        0x1029c4c4 => { vm.decode(0xe39c63b5u32); let d = 17i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c4c8u64) }
        0x1029c4c8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c4ccu64) }
        0x1029c4cc => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c4d0u64) }
        0x1029c4d0 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4d4u64) }
        0x1029c4d4 => { vm.decode(0x2880c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4d8u64) }
        0x1029c4d8 => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c4dcu64) }
        0x1029c4dc => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c4e0u64) }
        0x1029c4e0 => { vm.decode(0x6a09875u32); let d = 13i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c4e4u64) }
        0x1029c4e4 => { vm.decode(0xa39c23b5u32); let d = 17i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c4e8u64) }
        0x1029c4e8 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c4ecu64) }
        0x1029c4ec => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c4f0u64) }
        0x1029c4f0 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c4f4u64) }
        0x1029c4f4 => { vm.decode(0x40113835u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c4f8u64) }
        0x1029c4f8 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c4fcu64) }
        0x1029c4fc => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c500u64) }
        0x1029c500 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c504u64) }
        0x1029c504 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c508u64) }
        0x1029c508 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c50cu64) }
        0x1029c50c => { vm.decode(0x2a8ce32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c510u64) }
        0x1029c510 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c514u64) }
        0x1029c514 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c518u64) }
        0x1029c518 => { vm.decode(0x428663afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c51cu64) }
        0x1029c51c => { vm.decode(0x429c43b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c520u64) }
        0x1029c520 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c524u64) }
        0x1029c524 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c528u64) }
        0x1029c528 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c52cu64) }
        0x1029c52c => { vm.decode(0x64112835u32); let d = 17i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c530u64) }
        0x1029c530 => { vm.decode(0x839c43b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c534u64) }
        0x1029c534 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c538u64) }
        0x1029c538 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c53cu64) }
        0x1029c53c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c540u64) }
        0x1029c540 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c544u64) }
        0x1029c544 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c548u64) }
        0x1029c548 => { vm.decode(0x806befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c54cu64) }
        0x1029c54c => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c550u64) }
        0x1029c550 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c554u64) }
        0x1029c554 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c558u64) }
        0x1029c558 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c55cu64) }
        0x1029c55c => { vm.decode(0x2a8ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c560u64) }
        0x1029c560 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c564u64) }
        0x1029c564 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c568u64) }
        0x1029c568 => { vm.decode(0x40e32bb5u32); let d = 3i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c56cu64) }
        0x1029c56c => { vm.decode(0xdd4349u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c570u64) }
        0x1029c570 => { vm.decode(0x4e809875u32); let d = 31i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c574u64) }
        0x1029c574 => { vm.decode(0x30d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c578u64) }
        0x1029c578 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c57cu64) }
        0x1029c57c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c580u64) }
        0x1029c580 => { vm.decode(0x2288e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c584u64) }
        0x1029c584 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c588u64) }
        0x1029c588 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c58cu64) }
        0x1029c58c => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c590u64) }
        0x1029c590 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c594u64) }
        0x1029c594 => { vm.decode(0x4080d875u32); let d = 7i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c598u64) }
        0x1029c598 => { vm.decode(0x39c43b5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c59cu64) }
        0x1029c59c => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c5a0u64) }
        0x1029c5a0 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c5a4u64) }
        0x1029c5a4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c5a8u64) }
        0x1029c5a8 => { vm.decode(0xa8ec36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c5acu64) }
        0x1029c5ac => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c5b0u64) }
        0x1029c5b0 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c5b4u64) }
        0x1029c5b4 => { vm.decode(0x439c03b5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029c5b8u64) }
        0x1029c5b8 => { vm.decode(0x42901875u32); let d = 14i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c5bcu64) }
        0x1029c5bc => { vm.decode(0x89281d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c5c0u64) }
        0x1029c5c0 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c5c4u64) }
        0x1029c5c4 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c5c8u64) }
        0x1029c5c8 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c5ccu64) }
        0x1029c5cc => { vm.decode(0x80e52bb5u32); let d = 22i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c5d0u64) }
        0x1029c5d0 => { vm.decode(0x60522d35u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c5d4u64) }
        0x1029c5d4 => { vm.decode(0x42755162u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c5d8u64) }
        0x1029c5d8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c5dcu64) }
        0x1029c5dc => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c5e0u64) }
        0x1029c5e0 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c5e4u64) }
        0x1029c5e4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c5e8u64) }
        0x1029c5e8 => { vm.decode(0x2886c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c5ecu64) }
        0x1029c5ec => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c5f0u64) }
        0x1029c5f0 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c5f4u64) }
        0x1029c5f4 => { vm.decode(0x30bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c5f8u64) }
        0x1029c5f8 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c5fcu64) }
        0x1029c5fc => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c600u64) }
        0x1029c600 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c604u64) }
        0x1029c604 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c608u64) }
        0x1029c608 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c60cu64) }
        0x1029c60c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c610u64) }
        0x1029c610 => { vm.decode(0x65582835u32); let d = 23i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c614u64) }
        0x1029c614 => { vm.decode(0x2000402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c618u64) }
        0x1029c618 => { vm.decode(0x8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c61cu64) }
        0x1029c61c => { vm.decode(0x44480835u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c620u64) }
        0x1029c620 => { vm.decode(0x80a10bb5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c624u64) }
        0x1029c624 => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c628u64) }
        0x1029c628 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c62cu64) }
        0x1029c62c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c630u64) }
        0x1029c630 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c634u64) }
        0x1029c634 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c638u64) }
        0x1029c638 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c63cu64) }
        0x1029c63c => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c640u64) }
        0x1029c640 => { vm.decode(0x81c42bb5u32); let d = 30i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c644u64) }
        0x1029c644 => { vm.decode(0x60526d35u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c648u64) }
        0x1029c648 => { vm.decode(0x62777162u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c64cu64) }
        0x1029c64c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c650u64) }
        0x1029c650 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c654u64) }
        0x1029c654 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c658u64) }
        0x1029c658 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c65cu64) }
        0x1029c65c => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c660u64) }
        0x1029c660 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c664u64) }
        0x1029c664 => { vm.decode(0x40862bb5u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c668u64) }
        0x1029c668 => { vm.decode(0x40922d35u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c66cu64) }
        0x1029c66c => { vm.decode(0x35d1022u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c670u64) }
        0x1029c670 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c674u64) }
        0x1029c674 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c678u64) }
        0x1029c678 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c67cu64) }
        0x1029c67c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c680u64) }
        0x1029c680 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c684u64) }
        0x1029c684 => { vm.decode(0x44482835u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c688u64) }
        0x1029c688 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c68cu64) }
        0x1029c68c => { vm.decode(0x1820bb5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c690u64) }
        0x1029c690 => { vm.decode(0x40924d35u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c694u64) }
        0x1029c694 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c698u64) }
        0x1029c698 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c69cu64) }
        0x1029c69c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c6a0u64) }
        0x1029c6a0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c6a4u64) }
        0x1029c6a4 => { vm.decode(0x40032835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c6a8u64) }
        0x1029c6a8 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c6acu64) }
        0x1029c6ac => { vm.decode(0x81012bb5u32); let d = 24i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c6b0u64) }
        0x1029c6b0 => { vm.decode(0x40524d35u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c6b4u64) }
        0x1029c6b4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c6b8u64) }
        0x1029c6b8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c6bcu64) }
        0x1029c6bc => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c6c0u64) }
        0x1029c6c0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c6c4u64) }
        0x1029c6c4 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c6c8u64) }
        0x1029c6c8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c6ccu64) }
        0x1029c6cc => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c6d0u64) }
        0x1029c6d0 => { vm.decode(0x40220bb5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c6d4u64) }
        0x1029c6d4 => { vm.decode(0x408a8d35u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c6d8u64) }
        0x1029c6d8 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c6dcu64) }
        0x1029c6dc => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c6e0u64) }
        0x1029c6e0 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c6e4u64) }
        0x1029c6e4 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c6e8u64) }
        0x1029c6e8 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c6ecu64) }
        0x1029c6ec => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c6f0u64) }
        0x1029c6f0 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c6f4u64) }
        0x1029c6f4 => { vm.decode(0x1002bb5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c6f8u64) }
        0x1029c6f8 => { vm.decode(0x401b4d35u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c6fcu64) }
        0x1029c6fc => { vm.decode(0x62371062u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c700u64) }
        0x1029c700 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c704u64) }
        0x1029c704 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c708u64) }
        0x1029c708 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c70cu64) }
        0x1029c70c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c710u64) }
        0x1029c710 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c714u64) }
        0x1029c714 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c718u64) }
        0x1029c718 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c71cu64) }
        0x1029c71c => { vm.decode(0x2a8aeb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c720u64) }
        0x1029c720 => { vm.decode(0x80a72bb5u32); let d = 21i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c724u64) }
        0x1029c724 => { vm.decode(0x40db2d35u32); let d = 11i16; let s = 21i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c728u64) }
        0x1029c728 => { vm.decode(0x43453162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c72cu64) }
        0x1029c72c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c730u64) }
        0x1029c730 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c734u64) }
        0x1029c734 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c738u64) }
        0x1029c738 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c73cu64) }
        0x1029c73c => { vm.decode(0x73140835u32); let d = 12i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c740u64) }
        0x1029c740 => { vm.decode(0x60c36d35u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c744u64) }
        0x1029c744 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c748u64) }
        0x1029c748 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c74cu64) }
        0x1029c74c => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c750u64) }
        0x1029c750 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c754u64) }
        0x1029c754 => { vm.decode(0xa8ec32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c758u64) }
        0x1029c758 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c75cu64) }
        0x1029c75c => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c760u64) }
        0x1029c760 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c764u64) }
        0x1029c764 => { vm.decode(0x62157062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c768u64) }
        0x1029c768 => { vm.decode(0x60030d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c76cu64) }
        0x1029c76c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c770u64) }
        0x1029c770 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c774u64) }
        0x1029c774 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c778u64) }
        0x1029c778 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c77cu64) }
        0x1029c77c => { vm.decode(0xa8ae32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c780u64) }
        0x1029c780 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c784u64) }
        0x1029c784 => { vm.decode(0x28c6befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c788u64) }
        0x1029c788 => { vm.decode(0x472bb5u32); let d = 7i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c78cu64) }
        0x1029c78c => { vm.decode(0x609b2d35u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c790u64) }
        0x1029c790 => { vm.decode(0x3757062u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029c794u64) }
        0x1029c794 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c798u64) }
        0x1029c798 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c79cu64) }
        0x1029c79c => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c7a0u64) }
        0x1029c7a0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7a4u64) }
        0x1029c7a4 => { vm.decode(0xa8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7a8u64) }
        0x1029c7a8 => { vm.decode(0x514d0835u32); let d = 6i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c7acu64) }
        0x1029c7ac => { vm.decode(0x918ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c7b0u64) }
        0x1029c7b0 => { vm.decode(0x52d84349u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c7b4u64) }
        0x1029c7b4 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c7b8u64) }
        0x1029c7b8 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c7bcu64) }
        0x1029c7bc => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7c0u64) }
        0x1029c7c0 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c7c4u64) }
        0x1029c7c4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7c8u64) }
        0x1029c7c8 => { vm.decode(0xa88cb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7ccu64) }
        0x1029c7cc => { vm.decode(0x53462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c7d0u64) }
        0x1029c7d0 => { vm.decode(0x80232bb5u32); let d = 17i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c7d4u64) }
        0x1029c7d4 => { vm.decode(0x40db0d35u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c7d8u64) }
        0x1029c7d8 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c7dcu64) }
        0x1029c7dc => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c7e0u64) }
        0x1029c7e0 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c7e4u64) }
        0x1029c7e4 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7e8u64) }
        0x1029c7e8 => { vm.decode(0x888cb6fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7ecu64) }
        0x1029c7ec => { vm.decode(0x524e2835u32); let d = 11i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c7f0u64) }
        0x1029c7f0 => { vm.decode(0x88ceb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7f4u64) }
        0x1029c7f4 => { vm.decode(0x60200a2fu32); let d = 25i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c7f8u64) }
        0x1029c7f8 => { vm.decode(0x52581835u32); let d = 10i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c7fcu64) }
        0x1029c7fc => { vm.decode(0x9209d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c800u64) }
        0x1029c800 => { vm.decode(0x507c4349u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c804u64) }
        0x1029c804 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c808u64) }
        0x1029c808 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c80cu64) }
        0x1029c80c => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c810u64) }
        0x1029c810 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c814u64) }
        0x1029c814 => { vm.decode(0x36552835u32); let d = 25i16; let s = 10i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c818u64) }
        0x1029c818 => { vm.decode(0x546f4349u32); let d = 10i16; let s = 30i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c81cu64) }
        0x1029c81c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c820u64) }
        0x1029c820 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c824u64) }
        0x1029c824 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c828u64) }
        0x1029c828 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c82cu64) }
        0x1029c82c => { vm.decode(0x2a82c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c830u64) }
        0x1029c830 => { vm.decode(0x330d0835u32); let d = 14i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c834u64) }
        0x1029c834 => { vm.decode(0x608a4befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c838u64) }
        0x1029c838 => { vm.decode(0x41e12bb5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029c83cu64) }
        0x1029c83c => { vm.decode(0x6413cd35u32); let d = 23i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029c840u64) }
        0x1029c840 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c844u64) }
        0x1029c844 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c848u64) }
        0x1029c848 => { vm.decode(0xc96624f5u32); let d = 23i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c84cu64) }
        0x1029c84c => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c850u64) }
        0x1029c850 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c854u64) }
        0x1029c854 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c858u64) }
        0x1029c858 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c85cu64) }
        0x1029c85c => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c860u64) }
        0x1029c860 => { vm.decode(0x31549835u32); let d = 4i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c864u64) }
        0x1029c864 => { vm.decode(0x910dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c868u64) }
        0x1029c868 => { vm.decode(0x44ce5349u32); let d = 19i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c86cu64) }
        0x1029c86c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c870u64) }
        0x1029c870 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c874u64) }
        0x1029c874 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c878u64) }
        0x1029c878 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c87cu64) }
        0x1029c87c => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c880u64) }
        0x1029c880 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c884u64) }
        0x1029c884 => { vm.decode(0x9285cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c888u64) }
        0x1029c888 => { vm.decode(0x44dc5349u32); let d = 19i16; let s = 17i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c88cu64) }
        0x1029c88c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c890u64) }
        0x1029c890 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c894u64) }
        0x1029c894 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c898u64) }
        0x1029c898 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c89cu64) }
        0x1029c89c => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c8a0u64) }
        0x1029c8a0 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c8a4u64) }
        0x1029c8a4 => { vm.decode(0x908dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c8a8u64) }
        0x1029c8a8 => { vm.decode(0x40de5349u32); let d = 19i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c8acu64) }
        0x1029c8ac => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c8b0u64) }
        0x1029c8b0 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c8b4u64) }
        0x1029c8b4 => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c8b8u64) }
        0x1029c8b8 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c8bcu64) }
        0x1029c8bc => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c8c0u64) }
        0x1029c8c0 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c8c4u64) }
        0x1029c8c4 => { vm.decode(0x9205cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c8c8u64) }
        0x1029c8c8 => { vm.decode(0x1510cb11u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029c8ccu64) }
        0x1029c8cc => { vm.decode(0x446c5349u32); let d = 18i16; let s = 20i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c8d0u64) }
        0x1029c8d0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c8d4u64) }
        0x1029c8d4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c8d8u64) }
        0x1029c8d8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c8dcu64) }
        0x1029c8dc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c8e0u64) }
        0x1029c8e0 => { vm.decode(0x882e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c8e4u64) }
        0x1029c8e4 => { vm.decode(0x1051835u32); let d = 4i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c8e8u64) }
        0x1029c8e8 => { vm.decode(0x900dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c8ecu64) }
        0x1029c8ec => { vm.decode(0x446d5349u32); let d = 18i16; let s = 22i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c8f0u64) }
        0x1029c8f0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c8f4u64) }
        0x1029c8f4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c8f8u64) }
        0x1029c8f8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c8fcu64) }
        0x1029c8fc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c900u64) }
        0x1029c900 => { vm.decode(0x1071835u32); let d = 4i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c904u64) }
        0x1029c904 => { vm.decode(0x920dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c908u64) }
        0x1029c908 => { vm.decode(0x107dc349u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c90cu64) }
        0x1029c90c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c910u64) }
        0x1029c910 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c914u64) }
        0x1029c914 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c918u64) }
        0x1029c918 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c91cu64) }
        0x1029c91c => { vm.decode(0x282e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c920u64) }
        0x1029c920 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c924u64) }
        0x1029c924 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c928u64) }
        0x1029c928 => { vm.decode(0x147cc349u32); let d = 12i16; let s = 21i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c92cu64) }
        0x1029c92c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c930u64) }
        0x1029c930 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c934u64) }
        0x1029c934 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c938u64) }
        0x1029c938 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c93cu64) }
        0x1029c93c => { vm.decode(0x886c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c940u64) }
        0x1029c940 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c944u64) }
        0x1029c944 => { vm.decode(0x900dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c948u64) }
        0x1029c948 => { vm.decode(0x104ec349u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c94cu64) }
        0x1029c94c => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c950u64) }
        0x1029c950 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c954u64) }
        0x1029c954 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c958u64) }
        0x1029c958 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c95cu64) }
        0x1029c95c => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c960u64) }
        0x1029c960 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c964u64) }
        0x1029c964 => { vm.decode(0xdc4349u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c968u64) }
        0x1029c968 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c96cu64) }
        0x1029c96c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c970u64) }
        0x1029c970 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c974u64) }
        0x1029c974 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c978u64) }
        0x1029c978 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c97cu64) }
        0x1029c97c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c980u64) }
        0x1029c980 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c984u64) }
        0x1029c984 => { vm.decode(0xcf4349u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c988u64) }
        0x1029c988 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c98cu64) }
        0x1029c98c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c990u64) }
        0x1029c990 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c994u64) }
        0x1029c994 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c998u64) }
        0x1029c998 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c99cu64) }
        0x1029c99c => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c9a0u64) }
        0x1029c9a0 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029c9a4u64) }
        0x1029c9a4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c9a8u64) }
        0x1029c9a8 => { vm.decode(0x2286c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c9acu64) }
        0x1029c9ac => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c9b0u64) }
        0x1029c9b0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c9b4u64) }
        0x1029c9b4 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c9b8u64) }
        0x1029c9b8 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c9bcu64) }
        0x1029c9bc => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029c9c0u64) }
        0x1029c9c0 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c9c4u64) }
        0x1029c9c4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c9c8u64) }
        0x1029c9c8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c9ccu64) }
        0x1029c9cc => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c9d0u64) }
        0x1029c9d0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c9d4u64) }
        0x1029c9d4 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c9d8u64) }
        0x1029c9d8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c9dcu64) }
        0x1029c9dc => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c9e0u64) }
        0x1029c9e0 => { vm.decode(0x6ca4349u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029c9e4u64) }
        0x1029c9e4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029c9e8u64) }
        0x1029c9e8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029c9ecu64) }
        0x1029c9ec => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029c9f0u64) }
        0x1029c9f0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029c9f4u64) }
        0x1029c9f4 => { vm.decode(0x40432835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029c9f8u64) }
        0x1029c9f8 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029c9fcu64) }
        0x1029c9fc => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ca00u64) }
        0x1029ca00 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ca04u64) }
        0x1029ca04 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca08u64) }
        0x1029ca08 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ca0cu64) }
        0x1029ca0c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca10u64) }
        0x1029ca10 => { vm.decode(0x60122835u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ca14u64) }
        0x1029ca14 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ca18u64) }
        0x1029ca18 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029ca1cu64) }
        0x1029ca1c => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029ca20u64) }
        0x1029ca20 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029ca24u64) }
        0x1029ca24 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029ca28u64) }
        0x1029ca28 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029ca2cu64) }
        0x1029ca2c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ca30u64) }
        0x1029ca30 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca34u64) }
        0x1029ca34 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca38u64) }
        0x1029ca38 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029ca3cu64) }
        0x1029ca3c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029ca40u64) }
        0x1029ca40 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca44u64) }
        0x1029ca44 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca48u64) }
        0x1029ca48 => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca4cu64) }
        0x1029ca4c => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca50u64) }
        0x1029ca50 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca54u64) }
        0x1029ca54 => { vm.decode(0xdadac12du32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca58u64) }
        0x1029ca58 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca5cu64) }
        0x1029ca5c => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca60u64) }
        0x1029ca60 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca64u64) }
        0x1029ca64 => { vm.decode(0xda5ec12du32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca68u64) }
        0x1029ca68 => { vm.decode(0x228843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029ca6cu64) }
        0x1029ca6c => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029ca70u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
