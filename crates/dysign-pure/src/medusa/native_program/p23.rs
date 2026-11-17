#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102d9850 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9854u64) }
        0x102d9854 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9858u64) }
        0x102d9858 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d985cu64) }
        0x102d985c => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9860u64) }
        0x102d9860 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9864u64) }
        0x102d9864 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9868u64).wrapping_add((imm * 4) as u64) } else { 0x102d9868u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9868u64) }
        0x102d9868 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d986cu64) } }
        0x102d986c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9870u64) }
        0x102d9870 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9874u64) }
        0x102d9874 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9878u64).wrapping_add((imm * 4) as u64) } else { 0x102d9878u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9878u64) }
        0x102d9878 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d987cu64) }
        0x102d987c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9880u64) }
        0x102d9880 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9884u64) }
        0x102d9884 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9888u64) }
        0x102d9888 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d988cu64) }
        0x102d988c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9890u64).wrapping_add((imm * 4) as u64) } else { 0x102d9890u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9890u64) }
        0x102d9890 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9894u64) } }
        0x102d9894 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9898u64) }
        0x102d9898 => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d989cu64) }
        0x102d989c => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d98a0u64) }
        0x102d98a0 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d98a4u64) }
        0x102d98a4 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d98a8u64) }
        0x102d98a8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d98acu64) }
        0x102d98ac => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d98b0u64) }
        0x102d98b0 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d98b4u64) }
        0x102d98b4 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d98b8u64) }
        0x102d98b8 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d98bcu64) }
        0x102d98bc => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d98c0u64) }
        0x102d98c0 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d98c4u64) }
        0x102d98c4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d98c8u64).wrapping_add((imm * 4) as u64) } else { 0x102d98c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d98c8u64) }
        0x102d98c8 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d98ccu64) }
        0x102d98cc => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d98d0u64) }
        0x102d98d0 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d98d4u64) }
        0x102d98d4 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d98d8u64) }
        0x102d98d8 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d98dcu64) }
        0x102d98dc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d98e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d98e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d98e0u64) }
        0x102d98e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d98e4u64) } }
        0x102d98e4 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d98e8u64) }
        0x102d98e8 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d98ecu64) }
        0x102d98ec => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d98f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d98f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d98f0u64) }
        0x102d98f0 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d98f4u64) }
        0x102d98f4 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d98f8u64) }
        0x102d98f8 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d98fcu64) }
        0x102d98fc => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9900u64) }
        0x102d9900 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9904u64) }
        0x102d9904 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9908u64).wrapping_add((imm * 4) as u64) } else { 0x102d9908u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9908u64) }
        0x102d9908 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d990cu64) } }
        0x102d990c => { vm.decode(0x2ca4401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7756i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d9910u64) }
        0x102d9910 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9914u64) } }
        0x102d9914 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d9918u64) }
        0x102d9918 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d991cu64) }
        0x102d991c => { vm.decode(0x88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9920u64) }
        0x102d9920 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9924u64) }
        0x102d9924 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9928u64) }
        0x102d9928 => { vm.decode(0xa04184f5u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d992cu64) }
        0x102d992c => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9930u64) }
        0x102d9930 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9934u64) }
        0x102d9934 => { vm.decode(0x60804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9938u64) }
        0x102d9938 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d993cu64) }
        0x102d993c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9940u64).wrapping_add((imm * 4) as u64) } else { 0x102d9940u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9940u64) }
        0x102d9940 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9944u64) }
        0x102d9944 => { vm.decode(0xa580051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9908i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d9948u64) }
        0x102d994c => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9950u64) }
        0x102d9950 => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9954u64) }
        0x102d9954 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9958u64) }
        0x102d9958 => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d995cu64) }
        0x102d995c => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9960u64) }
        0x102d9960 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9964u64) }
        0x102d9964 => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9968u64) }
        0x102d9968 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d996cu64) }
        0x102d996c => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9970u64) }
        0x102d9970 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9974u64) }
        0x102d9974 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9978u64).wrapping_add((imm * 4) as u64) } else { 0x102d9978u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9978u64) }
        0x102d9978 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d997cu64) } }
        0x102d997c => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d9980u64) }
        0x102d9980 => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9984u64) }
        0x102d9984 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9988u64) }
        0x102d9988 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d998cu64) }
        0x102d998c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9990u64) }
        0x102d9990 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9994u64) }
        0x102d9994 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9998u64).wrapping_add((imm * 4) as u64) } else { 0x102d9998u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9998u64) }
        0x102d9998 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d999cu64) }
        0x102d999c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d99a0u64) }
        0x102d99a0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d99a4u64) }
        0x102d99a4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d99a8u64) }
        0x102d99a8 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d99acu64) }
        0x102d99ac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d99b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d99b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d99b0u64) }
        0x102d99b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d99b4u64) } }
        0x102d99b4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d99b8u64) }
        0x102d99b8 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d99bcu64) }
        0x102d99bc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d99c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d99c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d99c0u64) }
        0x102d99c0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d99c4u64) }
        0x102d99c4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d99c8u64) }
        0x102d99c8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d99ccu64) }
        0x102d99cc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d99d0u64) }
        0x102d99d0 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d99d4u64) }
        0x102d99d4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d99d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d99d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d99d8u64) }
        0x102d99d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d99dcu64) } }
        0x102d99dc => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d99e0u64) }
        0x102d99e0 => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d99e4u64) }
        0x102d99e4 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d99e8u64) }
        0x102d99e8 => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d99ecu64) }
        0x102d99ec => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d99f0u64) }
        0x102d99f0 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d99f4u64) }
        0x102d99f4 => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d99f8u64) }
        0x102d99f8 => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d99fcu64) }
        0x102d99fc => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9a00u64) }
        0x102d9a00 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9a04u64) }
        0x102d9a04 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a08u64) }
        0x102d9a08 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9a0cu64) }
        0x102d9a0c => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9a10u64) }
        0x102d9a10 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9a14u64) }
        0x102d9a14 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a18u64) }
        0x102d9a18 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a1cu64) }
        0x102d9a1c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9a20u64).wrapping_add((imm * 4) as u64) } else { 0x102d9a20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9a20u64) }
        0x102d9a20 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9a24u64) }
        0x102d9a24 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a28u64) }
        0x102d9a28 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9a2cu64) }
        0x102d9a2c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9a30u64) }
        0x102d9a30 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a34u64) }
        0x102d9a34 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9a38u64).wrapping_add((imm * 4) as u64) } else { 0x102d9a38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9a38u64) }
        0x102d9a38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9a3cu64) } }
        0x102d9a3c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a40u64) }
        0x102d9a40 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a44u64) }
        0x102d9a44 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9a48u64).wrapping_add((imm * 4) as u64) } else { 0x102d9a48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9a48u64) }
        0x102d9a48 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9a4cu64) }
        0x102d9a4c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a50u64) }
        0x102d9a50 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9a54u64) }
        0x102d9a54 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9a58u64) }
        0x102d9a58 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a5cu64) }
        0x102d9a5c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9a60u64).wrapping_add((imm * 4) as u64) } else { 0x102d9a60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9a60u64) }
        0x102d9a60 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9a64u64) } }
        0x102d9a64 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9a68u64) }
        0x102d9a68 => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9a6cu64) }
        0x102d9a6c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9a70u64) }
        0x102d9a70 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9a74u64) }
        0x102d9a74 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9a78u64) }
        0x102d9a78 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9a7cu64) }
        0x102d9a7c => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a80u64) }
        0x102d9a80 => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9a84u64) }
        0x102d9a84 => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9a88u64) }
        0x102d9a88 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9a8cu64) }
        0x102d9a8c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9a90u64) }
        0x102d9a90 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9a94u64) }
        0x102d9a94 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a98u64) }
        0x102d9a98 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9a9cu64) }
        0x102d9a9c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9aa0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9aa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9aa0u64) }
        0x102d9aa0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9aa4u64) }
        0x102d9aa4 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9aa8u64) }
        0x102d9aa8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9aacu64) }
        0x102d9aac => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9ab0u64) }
        0x102d9ab0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ab4u64) }
        0x102d9ab4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9ab8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9ab8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9ab8u64) }
        0x102d9ab8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9abcu64) } }
        0x102d9abc => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ac0u64) }
        0x102d9ac0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ac4u64) }
        0x102d9ac4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9ac8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9ac8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9ac8u64) }
        0x102d9ac8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9accu64) }
        0x102d9acc => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ad0u64) }
        0x102d9ad0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9ad4u64) }
        0x102d9ad4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9ad8u64) }
        0x102d9ad8 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9adcu64) }
        0x102d9adc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9ae0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9ae0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9ae0u64) }
        0x102d9ae0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9ae4u64) } }
        0x102d9ae4 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9ae8u64) }
        0x102d9ae8 => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9aecu64) }
        0x102d9aec => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9af0u64) }
        0x102d9af0 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9af4u64) }
        0x102d9af4 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9af8u64) }
        0x102d9af8 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9afcu64) }
        0x102d9afc => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b00u64) }
        0x102d9b00 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b04u64) }
        0x102d9b04 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9b08u64) }
        0x102d9b08 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9b0cu64) }
        0x102d9b0c => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b10u64) }
        0x102d9b10 => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b14u64) }
        0x102d9b14 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9b18u64) }
        0x102d9b18 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9b1cu64) }
        0x102d9b1c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b20u64) }
        0x102d9b20 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b24u64) }
        0x102d9b24 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9b28u64).wrapping_add((imm * 4) as u64) } else { 0x102d9b28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9b28u64) }
        0x102d9b28 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9b2cu64) }
        0x102d9b2c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b30u64) }
        0x102d9b30 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b34u64) }
        0x102d9b34 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9b38u64) }
        0x102d9b38 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b3cu64) }
        0x102d9b3c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9b40u64).wrapping_add((imm * 4) as u64) } else { 0x102d9b40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9b40u64) }
        0x102d9b40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9b44u64) } }
        0x102d9b44 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b48u64) }
        0x102d9b48 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b4cu64) }
        0x102d9b4c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9b50u64).wrapping_add((imm * 4) as u64) } else { 0x102d9b50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9b50u64) }
        0x102d9b50 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9b54u64) }
        0x102d9b54 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b58u64) }
        0x102d9b58 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b5cu64) }
        0x102d9b5c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9b60u64) }
        0x102d9b60 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b64u64) }
        0x102d9b64 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9b68u64).wrapping_add((imm * 4) as u64) } else { 0x102d9b68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9b68u64) }
        0x102d9b68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9b6cu64) } }
        0x102d9b6c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9b70u64) }
        0x102d9b70 => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b74u64) }
        0x102d9b74 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9b78u64) }
        0x102d9b78 => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9b7cu64) }
        0x102d9b7c => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9b80u64) }
        0x102d9b80 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9b84u64) }
        0x102d9b84 => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b88u64) }
        0x102d9b88 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b8cu64) }
        0x102d9b8c => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9b90u64) }
        0x102d9b90 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9b94u64) }
        0x102d9b94 => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9b98u64) }
        0x102d9b98 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9b9cu64) }
        0x102d9b9c => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9ba0u64) }
        0x102d9ba0 => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ba4u64) }
        0x102d9ba4 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ba8u64) }
        0x102d9ba8 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9bacu64) }
        0x102d9bac => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9bb0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9bb0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9bb0u64) }
        0x102d9bb0 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9bb4u64) }
        0x102d9bb4 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9bb8u64) }
        0x102d9bb8 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9bbcu64) }
        0x102d9bbc => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9bc0u64) }
        0x102d9bc0 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9bc4u64) }
        0x102d9bc4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9bc8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9bc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9bc8u64) }
        0x102d9bc8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9bccu64) } }
        0x102d9bcc => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9bd0u64) }
        0x102d9bd0 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9bd4u64) }
        0x102d9bd4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9bd8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9bd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9bd8u64) }
        0x102d9bd8 => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9bdcu64) }
        0x102d9bdc => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9be0u64) }
        0x102d9be0 => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9be4u64) }
        0x102d9be4 => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9be8u64) }
        0x102d9be8 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9becu64) }
        0x102d9bec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9bf0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9bf0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9bf0u64) }
        0x102d9bf0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9bf4u64) } }
        0x102d9bf4 => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9bf8u64) }
        0x102d9bf8 => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9bfcu64) }
        0x102d9bfc => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c00u64) }
        0x102d9c00 => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9c04u64) }
        0x102d9c04 => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c08u64) }
        0x102d9c08 => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c0cu64) }
        0x102d9c0c => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9c10u64) }
        0x102d9c10 => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9c14u64) }
        0x102d9c14 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9c18u64) }
        0x102d9c18 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9c1cu64) }
        0x102d9c1c => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c20u64) }
        0x102d9c20 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c24u64) }
        0x102d9c24 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9c28u64).wrapping_add((imm * 4) as u64) } else { 0x102d9c28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9c28u64) }
        0x102d9c28 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9c2cu64) }
        0x102d9c2c => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c30u64) }
        0x102d9c30 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9c34u64) }
        0x102d9c34 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9c38u64) }
        0x102d9c38 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c3cu64) }
        0x102d9c3c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9c40u64).wrapping_add((imm * 4) as u64) } else { 0x102d9c40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9c40u64) }
        0x102d9c40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9c44u64) } }
        0x102d9c44 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c48u64) }
        0x102d9c48 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c4cu64) }
        0x102d9c4c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9c50u64).wrapping_add((imm * 4) as u64) } else { 0x102d9c50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9c50u64) }
        0x102d9c50 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9c54u64) }
        0x102d9c54 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c58u64) }
        0x102d9c58 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9c5cu64) }
        0x102d9c5c => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9c60u64) }
        0x102d9c60 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c64u64) }
        0x102d9c64 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9c68u64).wrapping_add((imm * 4) as u64) } else { 0x102d9c68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9c68u64) }
        0x102d9c68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9c6cu64) } }
        0x102d9c6c => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c70u64) }
        0x102d9c70 => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9c74u64) }
        0x102d9c74 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c78u64) }
        0x102d9c78 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9c7cu64) }
        0x102d9c7c => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c80u64) }
        0x102d9c80 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9c84u64) }
        0x102d9c84 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9c88u64) }
        0x102d9c88 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9c8cu64) }
        0x102d9c8c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9c90u64) }
        0x102d9c90 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9c94u64) }
        0x102d9c94 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c98u64) }
        0x102d9c98 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9c9cu64) }
        0x102d9c9c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9ca0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9ca0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9ca0u64) }
        0x102d9ca0 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9ca4u64) }
        0x102d9ca4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9ca8u64) }
        0x102d9ca8 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9cacu64) }
        0x102d9cac => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9cb0u64) }
        0x102d9cb0 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9cb4u64) }
        0x102d9cb4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9cb8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9cb8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9cb8u64) }
        0x102d9cb8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9cbcu64) } }
        0x102d9cbc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9cc0u64) }
        0x102d9cc0 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9cc4u64) }
        0x102d9cc4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9cc8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9cc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9cc8u64) }
        0x102d9cc8 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9cccu64) }
        0x102d9ccc => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9cd0u64) }
        0x102d9cd0 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9cd4u64) }
        0x102d9cd4 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9cd8u64) }
        0x102d9cd8 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9cdcu64) }
        0x102d9cdc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9ce0u64).wrapping_add((imm * 4) as u64) } else { 0x102d9ce0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9ce0u64) }
        0x102d9ce0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9ce4u64) } }
        0x102d9ce4 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9ce8u64) }
        0x102d9ce8 => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9cecu64) }
        0x102d9cec => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9cf0u64) }
        0x102d9cf0 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9cf4u64) }
        0x102d9cf4 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9cf8u64) }
        0x102d9cf8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9cfcu64) }
        0x102d9cfc => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9d00u64) }
        0x102d9d00 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9d04u64) }
        0x102d9d04 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9d08u64) }
        0x102d9d08 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9d0cu64) }
        0x102d9d0c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d10u64) }
        0x102d9d10 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d14u64) }
        0x102d9d14 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9d18u64).wrapping_add((imm * 4) as u64) } else { 0x102d9d18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9d18u64) }
        0x102d9d18 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9d1cu64) }
        0x102d9d1c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d20u64) }
        0x102d9d20 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9d24u64) }
        0x102d9d24 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9d28u64) }
        0x102d9d28 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d2cu64) }
        0x102d9d2c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9d30u64).wrapping_add((imm * 4) as u64) } else { 0x102d9d30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9d30u64) }
        0x102d9d30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9d34u64) } }
        0x102d9d34 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d38u64) }
        0x102d9d38 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d3cu64) }
        0x102d9d3c => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9d40u64).wrapping_add((imm * 4) as u64) } else { 0x102d9d40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9d40u64) }
        0x102d9d40 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9d44u64) }
        0x102d9d44 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d48u64) }
        0x102d9d48 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9d4cu64) }
        0x102d9d4c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9d50u64) }
        0x102d9d50 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9d54u64) }
        0x102d9d54 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9d58u64).wrapping_add((imm * 4) as u64) } else { 0x102d9d58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9d58u64) }
        0x102d9d58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9d5cu64) } }
        0x102d9d5c => { vm.decode(0xa880051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8860i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d9d60u64) }
        0x102d9d60 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9d64u64) } }
        0x102d9d64 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d9d68u64) }
        0x102d9d68 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9d6cu64) }
        0x102d9d6c => { vm.decode(0xa04184f5u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9d70u64) }
        0x102d9d70 => { vm.decode(0x90000430u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9d74u64) }
        0x102d9d74 => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9d78u64) }
        0x102d9d78 => { vm.decode(0xd1cc011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d9d7cu64) }
        0x102d9d7c => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9d80u64) }
        0x102d9d80 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9d84u64).wrapping_add((imm * 4) as u64) } else { 0x102d9d84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9d84u64) }
        0x102d9d84 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9d88u64) } }
        0x102d9d88 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9d8cu64) }
        0x102d9d8c => { vm.decode(0x2200212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9d90u64) }
        0x102d9d90 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9d94u64) }
        0x102d9d94 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9d98u64) }
        0x102d9d98 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9d9cu64) }
        0x102d9d9c => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9da0u64) }
        0x102d9da0 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9da4u64) }
        0x102d9da4 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9da8u64) }
        0x102d9da8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d9dacu64 + 4);
        vm.wr(37, 0x102d9dacu64 + 4);
        Ok(0x102d9dacu64) }
        0x102d9dac => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9db0u64) }
        0x102d9db0 => { vm.decode(0x60804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9db4u64) }
        0x102d9db4 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9db8u64) }
        0x102d9db8 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dbcu64) }
        0x102d9dbc => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dc0u64) }
        0x102d9dc0 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dc4u64) }
        0x102d9dc4 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dc8u64) }
        0x102d9dc8 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dccu64) }
        0x102d9dcc => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dd0u64) }
        0x102d9dd0 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9dd4u64) }
        0x102d9dd4 => { vm.decode(0x8098b27u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9dd8u64).wrapping_add((imm * 4) as u64) } else { 0x102d9dd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9dd8u64) }
        0x102d9dd8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9ddcu64) } }
        0x102d9ddc => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d9de0u64) }
        0x102d9de0 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9de4u64) }
        0x102d9de4 => { vm.decode(0xa80184f5u32); let d = 2i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9de8u64) }
        0x102d9de8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9decu64) }
        0x102d9dec => { vm.decode(0x84891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9df0u64) }
        0x102d9df0 => { vm.decode(0xaa00a4f5u32); let d = 2i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9df4u64) }
        0x102d9df4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9df8u64) }
        0x102d9df8 => { vm.decode(0x85091b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9dfcu64) }
        0x102d9dfc => { vm.decode(0xa800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e00u64) }
        0x102d9e00 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e04u64) }
        0x102d9e04 => { vm.decode(0x85891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e08u64) }
        0x102d9e08 => { vm.decode(0x8800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e0cu64) }
        0x102d9e0c => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e10u64) }
        0x102d9e10 => { vm.decode(0x820284f5u32); let d = 4i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e14u64) }
        0x102d9e14 => { vm.decode(0xa04284f5u32); let d = 5i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e18u64) }
        0x102d9e18 => { vm.decode(0x804084f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e1cu64) }
        0x102d9e1c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e20u64) }
        0x102d9e20 => { vm.decode(0x18609030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e24u64) }
        0x102d9e24 => { vm.decode(0x2080012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e28u64) }
        0x102d9e28 => { vm.decode(0x18689030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e2cu64) }
        0x102d9e2c => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e30u64) }
        0x102d9e30 => { vm.decode(0x18709030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e34u64) }
        0x102d9e34 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e38u64) }
        0x102d9e38 => { vm.decode(0x18789030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e3cu64) }
        0x102d9e3c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e40u64) }
        0x102d9e40 => { vm.decode(0x186090b0u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e44u64) }
        0x102d9e44 => { vm.decode(0x4a8043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e48u64) }
        0x102d9e48 => { vm.decode(0x6a8063efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e4cu64) }
        0x102d9e4c => { vm.decode(0x488c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e50u64) }
        0x102d9e50 => { vm.decode(0x688c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e54u64) }
        0x102d9e54 => { vm.decode(0x4a8643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e58u64) }
        0x102d9e58 => { vm.decode(0x6a8663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e5cu64) }
        0x102d9e5c => { vm.decode(0x408ac3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e60u64) }
        0x102d9e60 => { vm.decode(0x608ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e64u64) }
        0x102d9e64 => { vm.decode(0x428acbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e68u64) }
        0x102d9e68 => { vm.decode(0x628aebafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e6cu64) }
        0x102d9e6c => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102d9e70u64) }
        0x102d9e70 => { vm.decode(0x77198059u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9e74u64) }
        0x102d9e74 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e78u64) }
        0x102d9e78 => { vm.decode(0x830204f5u32); let d = 4i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9e7cu64) }
        0x102d9e7c => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e80u64) }
        0x102d9e80 => { vm.decode(0x8b209430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e84u64) }
        0x102d9e84 => { vm.decode(0x1b1894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e88u64) }
        0x102d9e88 => { vm.decode(0x665f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9e8cu64) }
        0x102d9e8c => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e90u64) }
        0x102d9e90 => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e94u64) }
        0x102d9e94 => { vm.decode(0x99201470u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9e98u64) }
        0x102d9e98 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9e9cu64) }
        0x102d9e9c => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ea0u64) }
        0x102d9ea0 => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ea4u64) }
        0x102d9ea4 => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ea8u64) }
        0x102d9ea8 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9eacu64) }
        0x102d9eac => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9eb0u64) }
        0x102d9eb0 => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9eb4u64) }
        0x102d9eb4 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9eb8u64) }
        0x102d9eb8 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ebcu64) }
        0x102d9ebc => { vm.decode(0x89181470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ec0u64) }
        0x102d9ec0 => { vm.decode(0x99105570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9ec4u64) }
        0x102d9ec4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d9ec8u64 + 4);
        vm.wr(37, 0x102d9ec8u64 + 4);
        Ok(0x102d9ec8u64) }
        0x102d9ec8 => { vm.decode(0x9085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9eccu64) }
        0x102d9ecc => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ed0u64) }
        0x102d9ed0 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ed4u64) }
        0x102d9ed4 => { vm.decode(0x42806b2fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ed8u64) }
        0x102d9ed8 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9edcu64) }
        0x102d9edc => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ee0u64) }
        0x102d9ee0 => { vm.decode(0x60804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ee4u64) }
        0x102d9ee4 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ee8u64) }
        0x102d9ee8 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9eecu64) }
        0x102d9eec => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ef0u64) }
        0x102d9ef0 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ef4u64) }
        0x102d9ef4 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9ef8u64) }
        0x102d9ef8 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9efcu64) }
        0x102d9efc => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f00u64) }
        0x102d9f00 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f04u64) }
        0x102d9f04 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f08u64) }
        0x102d9f08 => { vm.decode(0xd98c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d9f0cu64) }
        0x102d9f0c => { vm.decode(0x31c3e27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -2534i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9f10u64).wrapping_add((imm * 4) as u64) } else { 0x102d9f10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9f10u64) }
        0x102d9f10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9f14u64) } }
        0x102d9f14 => { vm.decode(0x4b2514f5u32); let d = 18i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9f18u64) }
        0x102d9f18 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9f1cu64) }
        0x102d9f1c => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f20u64) }
        0x102d9f20 => { vm.decode(0x810724f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9f24u64) }
        0x102d9f24 => { vm.decode(0x2020212fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f28u64) }
        0x102d9f28 => { vm.decode(0x501019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9f2cu64) }
        0x102d9f2c => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f30u64) }
        0x102d9f30 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f34u64) }
        0x102d9f34 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f38u64) }
        0x102d9f38 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f3cu64) }
        0x102d9f3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d9f40u64 + 4);
        vm.wr(37, 0x102d9f40u64 + 4);
        Ok(0x102d9f40u64) }
        0x102d9f40 => { vm.decode(0x661b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9f44u64) }
        0x102d9f44 => { vm.decode(0x6a8c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f48u64) }
        0x102d9f48 => { vm.decode(0x81400530u32); let d = 18i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f4cu64) }
        0x102d9f4c => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9f50u64) }
        0x102d9f50 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f54u64) }
        0x102d9f54 => { vm.decode(0x2204212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f58u64) }
        0x102d9f58 => { vm.decode(0xb109430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f5cu64) }
        0x102d9f5c => { vm.decode(0x60841019u32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9f60u64) }
        0x102d9f60 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f64u64) }
        0x102d9f64 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f68u64) }
        0x102d9f68 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f6cu64) }
        0x102d9f6c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d9f70u64 + 4);
        vm.wr(37, 0x102d9f70u64 + 4);
        Ok(0x102d9f70u64) }
        0x102d9f70 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9f74u64) }
        0x102d9f74 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d9f78u64) }
        0x102d9f78 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9f7cu64) }
        0x102d9f7c => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9f80u64) }
        0x102d9f80 => { vm.decode(0x90000530u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f84u64) }
        0x102d9f84 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f88u64) }
        0x102d9f88 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f8cu64) }
        0x102d9f8c => { vm.decode(0x8b3814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f90u64) }
        0x102d9f90 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9f94u64) }
        0x102d9f94 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f98u64) }
        0x102d9f98 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9f9cu64) }
        0x102d9f9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d9fa0u64 + 4);
        vm.wr(37, 0x102d9fa0u64 + 4);
        Ok(0x102d9fa0u64) }
        0x102d9fa0 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9fa4u64) }
        0x102d9fa4 => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fa8u64) }
        0x102d9fa8 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9facu64) }
        0x102d9fac => { vm.decode(0x42806b2fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fb0u64) }
        0x102d9fb0 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fb4u64) }
        0x102d9fb4 => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fb8u64) }
        0x102d9fb8 => { vm.decode(0x60804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fbcu64) }
        0x102d9fbc => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fc0u64) }
        0x102d9fc0 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fc4u64) }
        0x102d9fc4 => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fc8u64) }
        0x102d9fc8 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fccu64) }
        0x102d9fcc => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fd0u64) }
        0x102d9fd0 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fd4u64) }
        0x102d9fd4 => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fd8u64) }
        0x102d9fd8 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fdcu64) }
        0x102d9fdc => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9fe0u64) }
        0x102d9fe0 => { vm.decode(0x8820001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 200i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d9fe4u64) }
        0x102d9fe4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9fe8u64) } }
        0x102d9fe8 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d9fecu64) }
        0x102de8f0 => { vm.decode(0x7b79cf59u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102de8f4u64) }
        0x102de8f4 => { vm.decode(0x9b08d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de8f8u64) }
        0x102de8f8 => { vm.decode(0x8b00d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de8fcu64) }
        0x102de8fc => { vm.decode(0x9b3855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de900u64) }
        0x102de900 => { vm.decode(0x8b3055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de904u64) }
        0x102de904 => { vm.decode(0x9b2854b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de908u64) }
        0x102de908 => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de90cu64) }
        0x102de90c => { vm.decode(0x9b381530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de910u64) }
        0x102de910 => { vm.decode(0x8b301530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de914u64) }
        0x102de914 => { vm.decode(0x9b281430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de918u64) }
        0x102de918 => { vm.decode(0x8b201430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de91cu64) }
        0x102de91c => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102de920u64) }
        0x102de920 => { vm.decode(0x2280202fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de924u64) }
        0x102de924 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de928u64) }
        0x102de928 => { vm.decode(0x2280002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de92cu64) }
        0x102de92c => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de930u64) }
        0x102de930 => { vm.decode(0x6080202fu32); let d = 17i16; let s = 4i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de934u64) }
        0x102de934 => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de938u64) }
        0x102de938 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de93cu64) }
        0x102de93c => { vm.decode(0x62aa002fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de940u64) }
        0x102de940 => { vm.decode(0x42a8202fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de944u64) }
        0x102de944 => { vm.decode(0x40ae002fu32); let d = 22i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de948u64) }
        0x102de948 => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de94cu64) }
        0x102de94c => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de950u64) }
        0x102de950 => { vm.decode(0x22a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de954u64) }
        0x102de954 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de958u64) }
        0x102de958 => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de95cu64) }
        0x102de95c => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de960u64) }
        0x102de960 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de964u64) }
        0x102de964 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de968u64) }
        0x102de968 => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de96cu64) }
        0x102de96c => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de970u64) }
        0x102de970 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de974u64) }
        0x102de974 => { vm.decode(0x19081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de978u64) }
        0x102de978 => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de97cu64) }
        0x102de97c => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de980u64) }
        0x102de980 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de984u64) }
        0x102de984 => { vm.decode(0x19101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de988u64) }
        0x102de988 => { vm.decode(0x2080212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de98cu64) }
        0x102de98c => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de990u64) }
        0x102de990 => { vm.decode(0x40a2202fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de994u64) }
        0x102de994 => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de998u64) }
        0x102de998 => { vm.decode(0x408a092fu32); let d = 30i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de99cu64) }
        0x102de99c => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9a0u64) }
        0x102de9a0 => { vm.decode(0x92094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9a4u64) }
        0x102de9a4 => { vm.decode(0x1b0df3ceu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -5832704i64;
        vm.st(d, imm as u64);
        Ok(0x102de9a8u64) }
        0x102de9a8 => { vm.decode(0x8a502998u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 44056i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102de9acu64) }
        0x102de9ac => { vm.decode(0x4a01a4f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102de9b0u64) }
        0x102de9b0 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de9b4u64) }
        0x102de9b4 => { vm.decode(0xb1814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9b8u64) }
        0x102de9b8 => { vm.decode(0x725b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102de9bcu64) }
        0x102de9bc => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9c0u64) }
        0x102de9c0 => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9c4u64) }
        0x102de9c4 => { vm.decode(0xad5c2958u32); let d = 19i16; let s = 3i16; let s2 = -1i16; let imm = 43976i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102de9c8u64) }
        0x102de9c8 => { vm.decode(0xe10204f5u32); let d = 4i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102de9ccu64) }
        0x102de9cc => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102de9d0u64) }
        0x102de9d0 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102de9d4u64) }
        0x102de9d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102de9d8u64 + 4);
        vm.wr(37, 0x102de9d8u64 + 4);
        Ok(0x102de9d8u64) }
        0x102de9d8 => { vm.decode(0x93854b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9dcu64) }
        0x102de9dc => { vm.decode(0xc10324f5u32); let d = 4i16; let s = 19i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102de9e0u64) }
        0x102de9e0 => { vm.decode(0xc307a4f5u32); let d = 20i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102de9e4u64) }
        0x102de9e4 => { vm.decode(0x66498019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102de9e8u64) }
        0x102de9e8 => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102de9ecu64) }
        0x102de9ec => { vm.decode(0x92094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9f0u64) }
        0x102de9f0 => { vm.decode(0x88402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102de9f4u64) }
        0x102de9f4 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9f8u64) }
        0x102de9f8 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102de9fcu64) }
        0x102de9fc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dea00u64) }
        0x102dea00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dea04u64 + 4);
        vm.wr(37, 0x102dea04u64 + 4);
        Ok(0x102dea04u64) }
        0x102dea04 => { vm.decode(0x93054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea08u64) }
        0x102dea08 => { vm.decode(0xeb4724f5u32); let d = 23i16; let s = 19i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dea0cu64) }
        0x102dea0c => { vm.decode(0x8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea10u64) }
        0x102dea10 => { vm.decode(0x4000022u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dea14u64) }
        0x102dea14 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dea18u64).wrapping_add((imm * 4) as u64) } else { 0x102dea18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dea18u64) }
        0x102dea18 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea1cu64) }
        0x102dea1c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dea20u64) }
        0x102dea20 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dea24u64) }
        0x102dea24 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dea28u64) }
        0x102dea28 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea2cu64) }
        0x102dea2c => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea30u64) }
        0x102dea30 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea34u64) }
        0x102dea34 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dea38u64) }
        0x102dea38 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dea3cu64) }
        0x102dea3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dea40u64 + 4);
        vm.wr(37, 0x102dea40u64 + 4);
        Ok(0x102dea40u64) }
        0x102dea40 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dea44u64) }
        0x102dea44 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea48u64) }
        0x102dea48 => { vm.decode(0x840a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 14i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dea4cu64).wrapping_add((imm * 4) as u64) } else { 0x102dea4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dea4cu64) }
        0x102dea4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dea50u64) } }
        0x102dea50 => { vm.decode(0xc30304f5u32); let d = 4i16; let s = 19i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dea54u64) }
        0x102dea54 => { vm.decode(0x2080406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea58u64) }
        0x102dea58 => { vm.decode(0x8412fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea5cu64) }
        0x102dea5c => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea60u64) }
        0x102dea60 => { vm.decode(0x604b1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dea64u64) }
        0x102dea64 => { vm.decode(0x9309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea68u64) }
        0x102dea68 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea6cu64) }
        0x102dea6c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dea70u64) }
        0x102dea70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dea74u64 + 4);
        vm.wr(37, 0x102dea74u64 + 4);
        Ok(0x102dea74u64) }
        0x102dea74 => { vm.decode(0x621f8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dea78u64) }
        0x102dea78 => { vm.decode(0x428c636fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dea7cu64) }
        0x102dea7c => { vm.decode(0x180001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 544i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102dea80u64) }
        0x102dea80 => { vm.decode(0x134df3ceu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = -5832704i64;
        vm.st(d, imm as u64);
        Ok(0x102dea84u64) }
        0x102dea84 => { vm.decode(0x9389430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dea88u64) }
        0x102deb10 => { vm.decode(0x9900d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb14u64) }
        0x102deb14 => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb18u64) }
        0x102deb18 => { vm.decode(0x848027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102deb1cu64).wrapping_add((imm * 4) as u64) } else { 0x102deb1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102deb1cu64) }
        0x102deb1c => { vm.decode(0xcb0724f5u32); let d = 22i16; let s = 19i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102deb20u64) }
        0x102deb20 => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb24u64) }
        0x102deb24 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102deb28u64) }
        0x102deb28 => { vm.decode(0x900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102deb2cu64).wrapping_add((imm * 4) as u64) } else { 0x102deb2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102deb2cu64) }
        0x102deb2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102deb30u64) } }
        0x102deb30 => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb34u64) }
        0x102deb48 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb4cu64) }
        0x102deb4c => { vm.decode(0xe94604f5u32); let d = 23i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102deb50u64) }
        0x102deb50 => { vm.decode(0x91894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb54u64) }
        0x102deb54 => { vm.decode(0x91094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb58u64) }
        0x102deb58 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb5cu64) }
        0x102deb5c => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb60u64) }
        0x102deb60 => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb64u64) }
        0x102deb64 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb68u64) }
        0x102deb68 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb6cu64) }
        0x102deb6c => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102deb70u64) }
        0x102deb70 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102deb74u64) }
        0x102deb74 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102deb78u64 + 4);
        vm.wr(37, 0x102deb78u64 + 4);
        Ok(0x102deb78u64) }
        0x102deb78 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102deb7cu64) }
        0x102deb7c => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb80u64) }
        0x102deb80 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb84u64) }
        0x102deb84 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deb88u64) }
        0x102deb88 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102deb8cu64).wrapping_add((imm * 4) as u64) } else { 0x102deb8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102deb8cu64) }
        0x102deb8c => { vm.decode(0x91895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102deb90u64) }
        0x102deb90 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102deb94u64) }
        0x102deb94 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102deb98u64) }
        0x102deb98 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102deb9cu64) }
        0x102deb9c => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deba0u64) }
        0x102deba0 => { vm.decode(0xe98604f5u32); let d = 30i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102deba4u64) }
        0x102deba4 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102deba8u64) }
        0x102deba8 => { vm.decode(0xe14004f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102debacu64) }
        0x102debac => { vm.decode(0x190814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debb0u64) }
        0x102debb0 => { vm.decode(0x991815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debb4u64) }
        0x102debb4 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102debb8u64) }
        0x102debb8 => { vm.decode(0xe94404f5u32); let d = 19i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102debbcu64) }
        0x102debbc => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102debc0u64) }
        0x102debc0 => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debc4u64) }
        0x102debc4 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debc8u64) }
        0x102debc8 => { vm.decode(0x882432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102debccu64) }
        0x102debcc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102debd0u64) }
        0x102debd0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102debd4u64 + 4);
        vm.wr(37, 0x102debd4u64 + 4);
        Ok(0x102debd4u64) }
        0x102debd4 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102debd8u64) }
        0x102debd8 => { vm.decode(0x2280402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102debdcu64) }
        0x102debdc => { vm.decode(0x625f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102debe0u64) }
        0x102debe0 => { vm.decode(0x600b1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102debe4u64) }
        0x102debe4 => { vm.decode(0x605b9019u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102debe8u64) }
        0x102debe8 => { vm.decode(0xa000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102debecu64) }
        0x102debec => { vm.decode(0x53406973u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102debf0u64) }
        0x102debf0 => { vm.decode(0x86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102debf4u64) }
        0x102debf4 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debf8u64) }
        0x102debf8 => { vm.decode(0x19289d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102debfcu64) }
        0x102debfc => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dec00u64) }
        0x102dec00 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dec04u64) }
        0x102dec04 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dec08u64) }
        0x102dec08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dec0cu64 + 4);
        vm.wr(37, 0x102dec0cu64 + 4);
        Ok(0x102dec0cu64) }
        0x102dec0c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dec10u64) }
        0x102dec10 => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dec14u64) }
        0x102dec14 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dec18u64) }
        0x102dec18 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dec1cu64) }
        0x102dec1c => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dec20u64) }
        0x102dec20 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dec24u64) }
        0x102dec24 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dec28u64) }
        0x102dec28 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dec2cu64) }
        0x102dec2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dec30u64 + 4);
        vm.wr(37, 0x102dec30u64 + 4);
        Ok(0x102dec30u64) }
        0x102dec30 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dec34u64) }
        0x102dec34 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dec38u64) }
        0x102dec38 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dec3cu64).wrapping_add((imm * 4) as u64) } else { 0x102dec3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dec3cu64) }
        0x102dec3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dec40u64) } }
        0x102dec40 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dec44u64) }
        0x102dec44 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dec48u64) }
        0x102dec48 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dec4cu64) }
        0x102dec4c => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dec50u64) }
        0x102dec50 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dec54u64) }
        0x102dec54 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102dec58u64).wrapping_add((imm * 4) as u64) } else { 0x102dec58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dec58u64) }
        0x102dec58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dec5cu64) } }
        0x102dec5c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dec60u64).wrapping_add((imm * 4) as u64) } else { 0x102dec60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dec60u64) }
        0x102decb4 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102decb8u64) }
        0x102decb8 => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102decbcu64) }
        0x102decbc => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102decc0u64) }
        0x102decc0 => { vm.decode(0x882432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102decc4u64) }
        0x102decc4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102decc8u64) }
        0x102decc8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102decccu64 + 4);
        vm.wr(37, 0x102decccu64 + 4);
        Ok(0x102decccu64) }
        0x102deccc => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102decd0u64) }
        0x102decd0 => { vm.decode(0x2080606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102decd4u64) }
        0x102decd4 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102decd8u64) }
        0x102decd8 => { vm.decode(0x48408091u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102decdcu64) }
        0x102decdc => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dece0u64) }
        0x102dece0 => { vm.decode(0x62598159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dece4u64) }
        0x102dece4 => { vm.decode(0x9089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dece8u64) }
        0x102dece8 => { vm.decode(0x19009d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dececu64) }
        0x102decec => { vm.decode(0x9381cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102decf0u64) }
        0x102decf0 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102decf4u64) }
        0x102decf4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102decf8u64) }
        0x102decf8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102decfcu64 + 4);
        vm.wr(37, 0x102decfcu64 + 4);
        Ok(0x102decfcu64) }
        0x102decfc => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ded00u64) }
        0x102ded00 => { vm.decode(0x4a8acb2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ded04u64) }
        0x102ded04 => { vm.decode(0x8830927u32); let d = 0i16; let s = 30i16; let s2 = -1i16; let imm = 6i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ded08u64).wrapping_add((imm * 4) as u64) } else { 0x102ded08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ded08u64) }
        0x102ded08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ded0cu64) } }
        0x102ded0c => { vm.decode(0x200820eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 538968064i64;
        vm.st(d, imm as u64);
        Ok(0x102ded10u64) }
        0x102ded10 => { vm.decode(0x30012158u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2340i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ded14u64) }
        0x102ded14 => { vm.decode(0x4150c811u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ded18u64) }
        0x102ded18 => { vm.decode(0x880c827u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ded1cu64).wrapping_add((imm * 4) as u64) } else { 0x102ded1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ded1cu64) }
        0x102ded1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ded20u64) } }
        0x102ded20 => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ded24u64) }
        0x102ded38 => { vm.decode(0x18026271u32); let d = 0i16; let s = 30i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ded3cu64).wrapping_add((imm * 4) as u64) } else { 0x102ded3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ded3cu64) }
        0x102ded3c => { vm.decode(0x675c2958u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 43992i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ded40u64) }
        0x102ded40 => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ded44u64) }
        0x102ded58 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ded5cu64) }
        0x102ded5c => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ded60u64) }
        0x102ded60 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ded64u64) }
        0x102ded64 => { vm.decode(0x9d4c011u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ded68u64) }
        0x102ded68 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ded6cu64) }
        0x102ded6c => { vm.decode(0x322a431u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 307i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ded70u64).wrapping_add((imm * 4) as u64) } else { 0x102ded70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ded70u64) }
        0x102ded70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ded74u64) } }
        0x102ded74 => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ded78u64) }
        0x102df23c => { vm.decode(0x2080402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df240u64) }
        0x102df240 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df244u64) }
        0x102df244 => { vm.decode(0xb181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df248u64) }
        0x102df248 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df24cu64) }
        0x102df24c => { vm.decode(0x76491019u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df250u64) }
        0x102df250 => { vm.decode(0x48b0d875u32); let d = 23i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df254u64) }
        0x102df254 => { vm.decode(0x428e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df258u64) }
        0x102df258 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df25cu64) }
        0x102df25c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df260u64) }
        0x102df260 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df264u64 + 4);
        vm.wr(37, 0x102df264u64 + 4);
        Ok(0x102df264u64) }
        0x102df264 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df268u64) }
        0x102df268 => { vm.decode(0x608443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df26cu64) }
        0x102df26c => { vm.decode(0x91084530u32); let d = 23i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df270u64) }
        0x102df270 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df274u64) }
        0x102df274 => { vm.decode(0x11000430u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df278u64) }
        0x102df278 => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df27cu64) }
        0x102df27c => { vm.decode(0x608e436fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df280u64) }
        0x102df280 => { vm.decode(0x1481430u32); let d = 0i16; let s = 23i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df284u64) }
        0x102df284 => { vm.decode(0x1401430u32); let d = 0i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df288u64) }
        0x102df288 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df28cu64) }
        0x102df28c => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df290u64) }
        0x102df290 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df294u64) }
        0x102df294 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df298u64) }
        0x102df298 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df29cu64) }
        0x102df29c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df2a0u64 + 4);
        vm.wr(37, 0x102df2a0u64 + 4);
        Ok(0x102df2a0u64) }
        0x102df2a0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df2a4u64) }
        0x102df2a4 => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df2a8u64) }
        0x102df2a8 => { vm.decode(0x91401430u32); let d = 17i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2acu64) }
        0x102df2ac => { vm.decode(0x88402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df2b0u64) }
        0x102df2b0 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2b4u64) }
        0x102df2b4 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2b8u64) }
        0x102df2b8 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df2bcu64) }
        0x102df2bc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df2c0u64) }
        0x102df2c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df2c4u64 + 4);
        vm.wr(37, 0x102df2c4u64 + 4);
        Ok(0x102df2c4u64) }
        0x102df2c4 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df2c8u64) }
        0x102df2c8 => { vm.decode(0x8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df2ccu64) }
        0x102df2cc => { vm.decode(0x4000022u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df2d0u64) }
        0x102df2d0 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df2d4u64).wrapping_add((imm * 4) as u64) } else { 0x102df2d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df2d4u64) }
        0x102df2d4 => { vm.decode(0x1481530u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2d8u64) }
        0x102df2d8 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df2dcu64) }
        0x102df2dc => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df2e0u64) }
        0x102df2e0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df2e4u64) }
        0x102df2e4 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df2e8u64) }
        0x102df2e8 => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2ecu64) }
        0x102df2ec => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df2f0u64) }
        0x102df2f0 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df2f4u64) }
        0x102df2f4 => { vm.decode(0x6880432fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df2f8u64) }
        0x102df2f8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df2fcu64) }
        0x102df2fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df300u64 + 4);
        vm.wr(37, 0x102df300u64 + 4);
        Ok(0x102df300u64) }
        0x102df300 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df304u64) }
        0x102df304 => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df308u64) }
        0x102df308 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df30cu64) }
        0x102df30c => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df310u64) }
        0x102df310 => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df314u64) }
        0x102df314 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df318u64) }
        0x102df318 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df31cu64) }
        0x102df31c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df320u64) }
        0x102df320 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df324u64 + 4);
        vm.wr(37, 0x102df324u64 + 4);
        Ok(0x102df324u64) }
        0x102df324 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df328u64) }
        0x102df328 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df32cu64) }
        0x102df32c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df330u64).wrapping_add((imm * 4) as u64) } else { 0x102df330u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df330u64) }
        0x102df330 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df334u64) } }
        0x102df334 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df338u64) }
        0x102df338 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df33cu64) }
        0x102df33c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df340u64) }
        0x102df340 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df344u64) }
        0x102df344 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df348u64) }
        0x102df348 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102df34cu64).wrapping_add((imm * 4) as u64) } else { 0x102df34cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df34cu64) }
        0x102df34c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df350u64) } }
        0x102df350 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df354u64).wrapping_add((imm * 4) as u64) } else { 0x102df354u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df354u64) }
        0x102df354 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df358u64) } }
        0x102df358 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df35cu64) }
        0x102df35c => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df360u64) }
        0x102df360 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df364u64) }
        0x102df364 => { vm.decode(0x621f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df368u64) }
        0x102df368 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df36cu64) }
        0x102df36c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df370u64 + 4);
        vm.wr(37, 0x102df370u64 + 4);
        Ok(0x102df370u64) }
        0x102df370 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df374u64) }
        0x102df374 => { vm.decode(0x9289430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df378u64) }
        0x102df378 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df37cu64) }
        0x102df37c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df380u64).wrapping_add((imm * 4) as u64) } else { 0x102df380u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df380u64) }
        0x102df380 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df384u64) } }
        0x102df384 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df388u64) }
        0x102df388 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df38cu64) }
        0x102df38c => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df390u64) }
        0x102df390 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df394u64) }
        0x102df394 => { vm.decode(0x86632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df398u64) }
        0x102df398 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df39cu64) }
        0x102df39c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df3a0u64 + 4);
        vm.wr(37, 0x102df3a0u64 + 4);
        Ok(0x102df3a0u64) }
        0x102df3a0 => { vm.decode(0x624f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df3a4u64) }
        0x102df3a4 => { vm.decode(0x9209430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3a8u64) }
        0x102df3a8 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df3acu64) }
        0x102df3ac => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3b0u64) }
        0x102df3b0 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3b4u64) }
        0x102df3b4 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df3b8u64) }
        0x102df3b8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df3bcu64) }
        0x102df3bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df3c0u64 + 4);
        vm.wr(37, 0x102df3c0u64 + 4);
        Ok(0x102df3c0u64) }
        0x102df3c0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df3c4u64) }
        0x102df3c4 => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df3c8u64) }
        0x102df3c8 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df3ccu64) }
        0x102df3cc => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df3d0u64) }
        0x102df3d0 => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3d4u64) }
        0x102df3d4 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3d8u64) }
        0x102df3d8 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df3dcu64) }
        0x102df3dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df3e0u64) }
        0x102df3e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df3e4u64 + 4);
        vm.wr(37, 0x102df3e4u64 + 4);
        Ok(0x102df3e4u64) }
        0x102df3e4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df3e8u64) }
        0x102df3e8 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df3ecu64) }
        0x102df3ec => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df3f0u64).wrapping_add((imm * 4) as u64) } else { 0x102df3f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df3f0u64) }
        0x102df3f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df3f4u64) } }
        0x102df3f4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df3f8u64) }
        0x102df3f8 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df3fcu64) }
        0x102df3fc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df400u64) }
        0x102df400 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df404u64) }
        0x102df404 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df408u64) }
        0x102df408 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102df40cu64).wrapping_add((imm * 4) as u64) } else { 0x102df40cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df40cu64) }
        0x102df40c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df410u64) } }
        0x102df410 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df414u64).wrapping_add((imm * 4) as u64) } else { 0x102df414u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df414u64) }
        0x102df414 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df418u64) } }
        0x102df418 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df41cu64) }
        0x102df41c => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df420u64) }
        0x102df420 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df424u64) }
        0x102df424 => { vm.decode(0x621f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df428u64) }
        0x102df428 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df42cu64) }
        0x102df42c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df430u64 + 4);
        vm.wr(37, 0x102df430u64 + 4);
        Ok(0x102df430u64) }
        0x102df430 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df434u64) }
        0x102df434 => { vm.decode(0x9389430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df438u64) }
        0x102df438 => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df43cu64) }
        0x102df43c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df440u64).wrapping_add((imm * 4) as u64) } else { 0x102df440u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df440u64) }
        0x102df440 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df444u64) } }
        0x102df444 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df448u64) }
        0x102df448 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df44cu64) }
        0x102df44c => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df450u64) }
        0x102df450 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df454u64) }
        0x102df454 => { vm.decode(0x86632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df458u64) }
        0x102df458 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df45cu64) }
        0x102df45c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df460u64 + 4);
        vm.wr(37, 0x102df460u64 + 4);
        Ok(0x102df460u64) }
        0x102df460 => { vm.decode(0x624f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df464u64) }
        0x102df464 => { vm.decode(0x9309430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df468u64) }
        0x102df468 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df46cu64) }
        0x102df46c => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df470u64) }
        0x102df470 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df474u64) }
        0x102df474 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df478u64) }
        0x102df478 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df47cu64) }
        0x102df47c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df480u64 + 4);
        vm.wr(37, 0x102df480u64 + 4);
        Ok(0x102df480u64) }
        0x102df480 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df484u64) }
        0x102df484 => { vm.decode(0x661d8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df488u64) }
        0x102df488 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df48cu64) }
        0x102df48c => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df490u64) }
        0x102df490 => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df494u64) }
        0x102df494 => { vm.decode(0x9b1054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df498u64) }
        0x102df498 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df49cu64) }
        0x102df49c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df4a0u64) }
        0x102df4a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df4a4u64 + 4);
        vm.wr(37, 0x102df4a4u64 + 4);
        Ok(0x102df4a4u64) }
        0x102df4a4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df4a8u64) }
        0x102df4a8 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df4acu64) }
        0x102df4ac => { vm.decode(0xc000027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df4b0u64).wrapping_add((imm * 4) as u64) } else { 0x102df4b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df4b0u64) }
        0x102df4b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df4b4u64) } }
        0x102df4b4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df4b8u64) }
        0x102df4b8 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df4bcu64) }
        0x102df4bc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df4c0u64) }
        0x102df4c0 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df4c4u64) }
        0x102df4c4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df4c8u64) }
        0x102df4c8 => { vm.decode(0x10300084u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102df4ccu64).wrapping_add((imm * 4) as u64) } else { 0x102df4ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df4ccu64) }
        0x102df4cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df4d0u64) } }
        0x102df4d0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df4d4u64).wrapping_add((imm * 4) as u64) } else { 0x102df4d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df4d4u64) }
        0x102df4d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df4d8u64) } }
        0x102df4d8 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df4dcu64) }
        0x102df4dc => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df4e0u64) }
        0x102df4e0 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df4e4u64) }
        0x102df4e4 => { vm.decode(0x621f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df4e8u64) }
        0x102df4e8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df4ecu64) }
        0x102df4ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df4f0u64 + 4);
        vm.wr(37, 0x102df4f0u64 + 4);
        Ok(0x102df4f0u64) }
        0x102df4f0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df4f4u64) }
        0x102df4f4 => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df4f8u64) }
        0x102df4f8 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df4fcu64) }
        0x102df4fc => { vm.decode(0x8800227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df500u64).wrapping_add((imm * 4) as u64) } else { 0x102df500u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df500u64) }
        0x102df500 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df504u64) } }
        0x102df504 => { vm.decode(0x2280432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df508u64) }
        0x102df530 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df534u64) }
        0x102df534 => { vm.decode(0x9b085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df538u64) }
        0x102df538 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df53cu64) }
        0x102df53c => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df540u64) }
        0x102df540 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df544u64) }
        0x102df544 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df548u64 + 4);
        vm.wr(37, 0x102df548u64 + 4);
        Ok(0x102df548u64) }
        0x102df548 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df54cu64) }
        0x102df54c => { vm.decode(0x408043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df550u64) }
        0x102df550 => { vm.decode(0x608063efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df554u64) }
        0x102df554 => { vm.decode(0x428843efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df558u64) }
        0x102df558 => { vm.decode(0x628863efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df55cu64) }
        0x102df55c => { vm.decode(0x408643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df560u64) }
        0x102df560 => { vm.decode(0x608663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df564u64) }
        0x102df564 => { vm.decode(0x428e43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df568u64) }
        0x102df568 => { vm.decode(0x628e63efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df56cu64) }
        0x102df56c => { vm.decode(0x488a4bafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df570u64) }
        0x102df570 => { vm.decode(0x688a6bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df574u64) }
        0x102df574 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102df578u64) }
        0x102df578 => { vm.decode(0x774b8019u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df57cu64) }
        0x102df57c => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102df580u64) }
        0x102df580 => { vm.decode(0x7b69cf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -944i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df584u64) }
        0x102df584 => { vm.decode(0x9b08ddf0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df588u64) }
        0x102df588 => { vm.decode(0x8b00ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df58cu64) }
        0x102df58c => { vm.decode(0x9b18dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df590u64) }
        0x102df590 => { vm.decode(0x8b10dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df594u64) }
        0x102df594 => { vm.decode(0x9b08dc30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df598u64) }
        0x102df598 => { vm.decode(0x8b00dc30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df59cu64) }
        0x102df59c => { vm.decode(0x9b381db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5a0u64) }
        0x102df5a0 => { vm.decode(0x8b301db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5a4u64) }
        0x102df5a4 => { vm.decode(0x9b281cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5a8u64) }
        0x102df5a8 => { vm.decode(0x8b201cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5acu64) }
        0x102df5ac => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df5b0u64) }
        0x102df5b0 => { vm.decode(0x2100183eu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102df5b4u64) }
        0x102df5b4 => { vm.decode(0x3053422u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 20i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df5b8u64) }
        0x102df5b8 => { vm.decode(0x2280002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5bcu64) }
        0x102df5bc => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5c0u64) }
        0x102df5c0 => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5c4u64) }
        0x102df5c4 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5c8u64) }
        0x102df5c8 => { vm.decode(0x2080212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5ccu64) }
        0x102df5cc => { vm.decode(0x19109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5d0u64) }
        0x102df5d0 => { vm.decode(0x20a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5d4u64) }
        0x102df5d4 => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5d8u64) }
        0x102df5d8 => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5dcu64) }
        0x102df5dc => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5e0u64) }
        0x102df5e0 => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5e4u64) }
        0x102df5e4 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df5e8u64) }
        0x102df5e8 => { vm.decode(0x60a4002fu32); let d = 17i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5ecu64) }
        0x102df5ec => { vm.decode(0x40a6202fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5f0u64) }
        0x102df5f0 => { vm.decode(0x62ac002fu32); let d = 19i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5f4u64) }
        0x102df5f4 => { vm.decode(0x42ae202fu32); let d = 22i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5f8u64) }
        0x102df5f8 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df5fcu64) }
        0x102df5fc => { vm.decode(0x19081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df600u64) }
        0x102df600 => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df604u64) }
        0x102df604 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df608u64) }
        0x102df608 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df60cu64) }
        0x102df60c => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df610u64) }
        0x102df610 => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df614u64) }
        0x102df614 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df618u64) }
        0x102df618 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df61cu64) }
        0x102df61c => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df620u64) }
        0x102df620 => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df624u64) }
        0x102df624 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df628u64) }
        0x102df628 => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df62cu64) }
        0x102df62c => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df630u64) }
        0x102df630 => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df634u64) }
        0x102df634 => { vm.decode(0x408a282fu32); let d = 30i16; let s = 4i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df638u64) }
        0x102df638 => { vm.decode(0x608a012fu32); let d = 23i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df63cu64) }
        0x102df63c => { vm.decode(0x9009cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df640u64) }
        0x102df640 => { vm.decode(0x9189c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df644u64) }
        0x102df644 => { vm.decode(0x1985338eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = -16121856i64;
        vm.st(d, imm as u64);
        Ok(0x102df648u64) }
        0x102df648 => { vm.decode(0xab480458u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 4568i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102df64cu64) }
        0x102df64c => { vm.decode(0xea4124f5u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df650u64) }
        0x102df650 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df654u64) }
        0x102df654 => { vm.decode(0xa2422fu32); let d = 4i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df658u64) }
        0x102df658 => { vm.decode(0xb385c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df65cu64) }
        0x102df65c => { vm.decode(0x620b0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df660u64) }
        0x102df660 => { vm.decode(0x880c875u32); let d = 21i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df664u64) }
        0x102df664 => { vm.decode(0xb305c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df668u64) }
        0x102df668 => { vm.decode(0x1b281d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df66cu64) }
        0x102df66c => { vm.decode(0x8e480458u32); let d = 18i16; let s = 2i16; let s2 = -1i16; let imm = 4504i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102df670u64) }
        0x102df670 => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df674u64) }
        0x102df674 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df678u64) }
        0x102df678 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df67cu64) }
        0x102df67c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df680u64 + 4);
        vm.wr(37, 0x102df680u64 + 4);
        Ok(0x102df680u64) }
        0x102df680 => { vm.decode(0x900d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df684u64) }
        0x102df684 => { vm.decode(0x434124f5u32); let d = 1i16; let s = 18i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df688u64) }
        0x102df688 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df68cu64) }
        0x102df68c => { vm.decode(0x634104f5u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df690u64) }
        0x102df690 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df694u64) }
        0x102df694 => { vm.decode(0x414124f5u32); let d = 1i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df698u64) }
        0x102df698 => { vm.decode(0x19089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df69cu64) }
        0x102df69c => { vm.decode(0x614104f5u32); let d = 1i16; let s = 18i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6a0u64) }
        0x102df6a0 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6a4u64) }
        0x102df6a4 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6a8u64) }
        0x102df6a8 => { vm.decode(0x690604f5u32); let d = 22i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6acu64) }
        0x102df6ac => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6b0u64) }
        0x102df6b0 => { vm.decode(0x614404f5u32); let d = 17i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6b4u64) }
        0x102df6b4 => { vm.decode(0x2a80632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6b8u64) }
        0x102df6b8 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6bcu64) }
        0x102df6bc => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6c0u64) }
        0x102df6c0 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6c4u64) }
        0x102df6c4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6c8u64) }
        0x102df6c8 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6ccu64) }
        0x102df6cc => { vm.decode(0x6b4724f5u32); let d = 23i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6d0u64) }
        0x102df6d0 => { vm.decode(0x20a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6d4u64) }
        0x102df6d4 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6d8u64) }
        0x102df6d8 => { vm.decode(0xa8622fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6dcu64) }
        0x102df6dc => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102df6e0u64).wrapping_add((imm * 4) as u64) } else { 0x102df6e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df6e0u64) }
        0x102df6e0 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6e4u64) }
        0x102df6e4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df6e8u64) }
        0x102df6e8 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df6ecu64) }
        0x102df6ec => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df6f0u64) }
        0x102df6f0 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df6f4u64) }
        0x102df6f4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df6f8u64) }
        0x102df6f8 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df6fcu64) }
        0x102df6fc => { vm.decode(0x2a84432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df700u64) }
        0x102df700 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df704u64) }
        0x102df704 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df708u64) }
        0x102df708 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df70cu64) }
        0x102df70c => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df710u64) }
        0x102df710 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df714u64) }
        0x102df714 => { vm.decode(0xa82632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df718u64) }
        0x102df718 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df71cu64) }
        0x102df71c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df720u64 + 4);
        vm.wr(37, 0x102df720u64 + 4);
        Ok(0x102df720u64) }
        0x102df720 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df724u64) }
        0x102df724 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df728u64) }
        0x102df728 => { vm.decode(0x771d1059u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df72cu64) }
        0x102df72c => { vm.decode(0x604b1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df730u64) }
        0x102df730 => { vm.decode(0x604bb059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 404i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df734u64) }
        0x102df734 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df738u64) }
        0x102df738 => { vm.decode(0xc000011u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df73cu64) }
        0x102df73c => { vm.decode(0x3253462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 404i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df740u64) }
        0x102df740 => { vm.decode(0x190015b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df744u64) }
        0x102df744 => { vm.decode(0x19089d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df748u64) }
        0x102df748 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df74cu64) }
        0x102df74c => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df750u64) }
        0x102df750 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df754u64) }
        0x102df754 => { vm.decode(0x20a4616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df758u64) }
        0x102df758 => { vm.decode(0x9b105cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df75cu64) }
        0x102df75c => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df760u64) }
        0x102df760 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df764u64) }
        0x102df764 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df768u64) }
        0x102df768 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df76cu64 + 4);
        vm.wr(37, 0x102df76cu64 + 4);
        Ok(0x102df76cu64) }
        0x102df76c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df770u64) }
        0x102df770 => { vm.decode(0x991094b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df774u64) }
        0x102df774 => { vm.decode(0x740d9059u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df778u64) }
        0x102df778 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df77cu64) }
        0x102df77c => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df780u64) }
        0x102df780 => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df784u64) }
        0x102df784 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df788u64) }
        0x102df788 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df78cu64) }
        0x102df78c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df790u64) }
        0x102df790 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df794u64) }
        0x102df794 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df798u64 + 4);
        vm.wr(37, 0x102df798u64 + 4);
        Ok(0x102df798u64) }
        0x102df798 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df79cu64) }
        0x102df79c => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df7a0u64) }
        0x102df7a0 => { vm.decode(0x192054b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7a4u64) }
        0x102df7a4 => { vm.decode(0x890855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7a8u64) }
        0x102df7a8 => { vm.decode(0x721f0059u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df7acu64) }
        0x102df7ac => { vm.decode(0x60499159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df7b0u64) }
        0x102df7b0 => { vm.decode(0x19109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7b4u64) }
        0x102df7b4 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df7b8u64) }
        0x102df7b8 => { vm.decode(0x2880e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df7bcu64) }
        0x102df7bc => { vm.decode(0xc000011u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df7c0u64) }
        0x102df7c0 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df7c4u64) }
        0x102df7c4 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df7c8u64) }
        0x102df7c8 => { vm.decode(0x20d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df7ccu64) }
        0x102df7cc => { vm.decode(0x1b3895b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7d0u64) }
        0x102df7d0 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7d4u64) }
        0x102df7d4 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7d8u64) }
        0x102df7d8 => { vm.decode(0x6884632fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df7dcu64) }
        0x102df7dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df7e0u64) }
        0x102df7e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df7e4u64 + 4);
        vm.wr(37, 0x102df7e4u64 + 4);
        Ok(0x102df7e4u64) }
        0x102df7e4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df7e8u64) }
        0x102df7e8 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df7ecu64) }
        0x102df7ec => { vm.decode(0x8b185c30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7f0u64) }
        0x102df7f0 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7f4u64) }
        0x102df7f4 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df7f8u64) }
        0x102df7f8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df7fcu64) }
        0x102df7fc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df800u64) }
        0x102df800 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df804u64 + 4);
        vm.wr(37, 0x102df804u64 + 4);
        Ok(0x102df804u64) }
        0x102df804 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df808u64) }
        0x102df808 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df80cu64) }
        0x102df80c => { vm.decode(0x8b105cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df810u64) }
        0x102df810 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df814u64) }
        0x102df814 => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df818u64) }
        0x102df818 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df81cu64) }
        0x102df81c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df820u64 + 4);
        vm.wr(37, 0x102df820u64 + 4);
        Ok(0x102df820u64) }
        0x102df820 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df824u64) }
        0x102df824 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df828u64) }
        0x102df828 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df82cu64) }
        0x102df82c => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df830u64) }
        0x102df830 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df834u64) }
        0x102df834 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df838u64) }
        0x102df838 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df83cu64) }
        0x102df83c => { vm.decode(0x6a8c432fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df840u64) }
        0x102df840 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df844u64) }
        0x102df844 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df848u64 + 4);
        vm.wr(37, 0x102df848u64 + 4);
        Ok(0x102df848u64) }
        0x102df848 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df84cu64) }
        0x102df84c => { vm.decode(0x601d8159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df850u64) }
        0x102df850 => { vm.decode(0x2ac616fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df854u64) }
        0x102df854 => { vm.decode(0x5b000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102df858u64) }
        0x102df858 => { vm.decode(0x9b38d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df85cu64) }
        0x102df85c => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df860u64) }
        0x102df860 => { vm.decode(0xb289530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df864u64) }
        0x102df864 => { vm.decode(0x882632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df868u64) }
        0x102df868 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df86cu64) }
        0x102df86c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df870u64 + 4);
        vm.wr(37, 0x102df870u64 + 4);
        Ok(0x102df870u64) }
        0x102df870 => { vm.decode(0x660f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df874u64) }
        0x102df874 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df878u64) }
        0x102df878 => { vm.decode(0x9b105cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df87cu64) }
        0x102df87c => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df880u64) }
        0x102df880 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df884u64) }
        0x102df884 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df888u64) }
        0x102df888 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df88cu64 + 4);
        vm.wr(37, 0x102df88cu64 + 4);
        Ok(0x102df88cu64) }
        0x102df88c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df890u64) }
        0x102df890 => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df894u64) }
        0x102df894 => { vm.decode(0x208012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df898u64) }
        0x102df898 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df89cu64) }
        0x102df89c => { vm.decode(0x20a8412fu32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8a0u64) }
        0x102df8a0 => { vm.decode(0x4a8a432fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8a4u64) }
        0x102df8a4 => { vm.decode(0x8b30d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8a8u64) }
        0x102df8a8 => { vm.decode(0x1b2895b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8acu64) }
        0x102df8ac => { vm.decode(0xb40dfdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102df8b0u64) }
        0x102df8b0 => { vm.decode(0x30d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df8b4u64) }
        0x102df8b4 => { vm.decode(0x80a019u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df8b8u64) }
        0x102df8b8 => { vm.decode(0x1b3894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8bcu64) }
        0x102df8bc => { vm.decode(0x2086436fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8c0u64) }
        0x102df8c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df8c4u64) }
        0x102df8c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df8c8u64 + 4);
        vm.wr(37, 0x102df8c8u64 + 4);
        Ok(0x102df8c8u64) }
        0x102df8c8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df8ccu64) }
        0x102df8cc => { vm.decode(0x6084e36fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8d0u64) }
        0x102df8d0 => { vm.decode(0x20a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8d4u64) }
        0x102df8d4 => { vm.decode(0x8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8d8u64) }
        0x102df8d8 => { vm.decode(0xb209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8dcu64) }
        0x102df8dc => { vm.decode(0x9b18d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8e0u64) }
        0x102df8e0 => { vm.decode(0x1b1094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df8e4u64) }
        0x102df8e4 => { vm.decode(0x286436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df8e8u64) }
        0x102df8e8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df8ecu64) }
        0x102df8ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df8f0u64 + 4);
        vm.wr(37, 0x102df8f0u64 + 4);
        Ok(0x102df8f0u64) }
        0x102df8f0 => { vm.decode(0x665b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df8f4u64) }
        0x102df8f4 => { vm.decode(0x60098159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df8f8u64) }
        0x102df8f8 => { vm.decode(0x22351062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df8fcu64) }
        0x102df8fc => { vm.decode(0x8a8412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df900u64) }
        0x102df900 => { vm.decode(0x8b08d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df904u64) }
        0x102df904 => { vm.decode(0x9b00d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df908u64) }
        0x102df908 => { vm.decode(0x9b189430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df90cu64) }
        0x102df90c => { vm.decode(0x8b10d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df910u64) }
        0x102df910 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df914u64) }
        0x102df914 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df918u64) }
        0x102df918 => { vm.decode(0x286636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df91cu64) }
        0x102df91c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df920u64) }
        0x102df920 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df924u64 + 4);
        vm.wr(37, 0x102df924u64 + 4);
        Ok(0x102df924u64) }
        0x102df924 => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df928u64) }
        0x102df928 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df92cu64) }
        0x102df92c => { vm.decode(0x9b105cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df930u64) }
        0x102df930 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df934u64) }
        0x102df934 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df938u64) }
        0x102df938 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df93cu64) }
        0x102df93c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df940u64 + 4);
        vm.wr(37, 0x102df940u64 + 4);
        Ok(0x102df940u64) }
        0x102df940 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df944u64) }
        0x102df944 => { vm.decode(0x2080c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df948u64) }
        0x102df948 => { vm.decode(0x82000071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102df94cu64).wrapping_add((imm * 4) as u64) } else { 0x102df94cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102df94cu64) }
        0x102df94c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102df950u64) } }
        0x102df950 => { vm.decode(0xd104891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 20i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df954u64) }
        0x102df97c => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df980u64) }
        0x102df980 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df984u64) }
        0x102df984 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df988u64) }
        0x102df988 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df98cu64) }
        0x102df98c => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df990u64) }
        0x102df990 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df994u64) }
        0x102df994 => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df998u64) }
        0x102df998 => { vm.decode(0x614404f5u32); let d = 17i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df99cu64) }
        0x102df99c => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df9a0u64) }
        0x102df9a0 => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df9a4u64) }
        0x102df9a4 => { vm.decode(0x22a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df9a8u64) }
        0x102df9a8 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df9acu64) }
        0x102df9ac => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9b0u64) }
        0x102df9b0 => { vm.decode(0x601d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9b4u64) }
        0x102df9b4 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9b8u64) }
        0x102df9b8 => { vm.decode(0xb301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9bcu64) }
        0x102df9bc => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9c0u64) }
        0x102df9c0 => { vm.decode(0x660d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9c4u64) }
        0x102df9c4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102df9c8u64) }
        0x102df9c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102df9ccu64 + 4);
        vm.wr(37, 0x102df9ccu64 + 4);
        Ok(0x102df9ccu64) }
        0x102df9cc => { vm.decode(0x92854b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9d0u64) }
        0x102df9d0 => { vm.decode(0x2284e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df9d4u64) }
        0x102df9d4 => { vm.decode(0x44000011u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df9d8u64) }
        0x102df9d8 => { vm.decode(0x66198019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9dcu64) }
        0x102df9dc => { vm.decode(0x70191059u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9e0u64) }
        0x102df9e0 => { vm.decode(0xb019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9e4u64) }
        0x102df9e4 => { vm.decode(0x22055462u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102df9e8u64) }
        0x102df9e8 => { vm.decode(0x2a8416fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102df9ecu64) }
        0x102df9ec => { vm.decode(0x1b1815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102df9f0u64) }
        0x102df9f0 => { vm.decode(0x4d146891u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102df9f4u64) }
        0x102df9f4 => { vm.decode(0xe04004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102df9f8u64) }
        0x102df9f8 => { vm.decode(0x4c76ef59u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102df9fcu64) }
        0x102df9fc => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa00u64) }
        0x102dfa00 => { vm.decode(0x8b0815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa04u64) }
        0x102dfa04 => { vm.decode(0xb0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa08u64) }
        0x102dfa08 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa0cu64) }
        0x102dfa0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfa10u64 + 4);
        vm.wr(37, 0x102dfa10u64 + 4);
        Ok(0x102dfa10u64) }
        0x102dfa10 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa14u64) }
        0x102dfa14 => { vm.decode(0x6019a059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 292i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfa18u64) }
        0x102dfa18 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfa1cu64) }
        0x102dfa1c => { vm.decode(0x8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa20u64) }
        0x102dfa20 => { vm.decode(0x9181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa24u64) }
        0x102dfa24 => { vm.decode(0x2888432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa28u64) }
        0x102dfa28 => { vm.decode(0x19101d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa2cu64) }
        0x102dfa2c => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa30u64) }
        0x102dfa30 => { vm.decode(0x9001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa34u64) }
        0x102dfa34 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa38u64) }
        0x102dfa38 => { vm.decode(0x893095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa3cu64) }
        0x102dfa3c => { vm.decode(0x20a4616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa40u64) }
        0x102dfa40 => { vm.decode(0x9b105cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa44u64) }
        0x102dfa44 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa48u64) }
        0x102dfa48 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa4cu64) }
        0x102dfa4c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa50u64) }
        0x102dfa50 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfa54u64 + 4);
        vm.wr(37, 0x102dfa54u64 + 4);
        Ok(0x102dfa54u64) }
        0x102dfa54 => { vm.decode(0x4c004875u32); let d = 18i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa58u64) }
        0x102dfa58 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfa5cu64) }
        0x102dfa5c => { vm.decode(0x1900d430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa60u64) }
        0x102dfa60 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa64u64) }
        0x102dfa64 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfa68u64) }
        0x102dfa68 => { vm.decode(0x20d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfa6cu64) }
        0x102dfa6c => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa70u64) }
        0x102dfa70 => { vm.decode(0xb3895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa74u64) }
        0x102dfa74 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa78u64) }
        0x102dfa78 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa7cu64) }
        0x102dfa7c => { vm.decode(0x488e632fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa80u64) }
        0x102dfa80 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa84u64) }
        0x102dfa84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfa88u64 + 4);
        vm.wr(37, 0x102dfa88u64 + 4);
        Ok(0x102dfa88u64) }
        0x102dfa88 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfa8cu64) }
        0x102dfa8c => { vm.decode(0x740d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfa90u64) }
        0x102dfa90 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfa94u64) }
        0x102dfa94 => { vm.decode(0x8b185c30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa98u64) }
        0x102dfa98 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfa9cu64) }
        0x102dfa9c => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfaa0u64) }
        0x102dfaa0 => { vm.decode(0x408e6b2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfaa4u64) }
        0x102dfaa4 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfaa8u64) }
        0x102dfaa8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfaacu64) }
        0x102dfaac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfab0u64 + 4);
        vm.wr(37, 0x102dfab0u64 + 4);
        Ok(0x102dfab0u64) }
        0x102dfab0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfab4u64) }
        0x102dfab4 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfab8u64) }
        0x102dfab8 => { vm.decode(0x8b105cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfabcu64) }
        0x102dfabc => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfac0u64) }
        0x102dfac0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfac4u64) }
        0x102dfac4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfac8u64) }
        0x102dfac8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfaccu64 + 4);
        vm.wr(37, 0x102dfaccu64 + 4);
        Ok(0x102dfaccu64) }
        0x102dfacc => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfad0u64) }
        0x102dfad0 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfad4u64) }
        0x102dfad4 => { vm.decode(0x288c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfad8u64) }
        0x102dfad8 => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfadcu64) }
        0x102dfadc => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfae0u64) }
        0x102dfae0 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfae4u64) }
        0x102dfae4 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfae8u64) }
        0x102dfae8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfaecu64) }
        0x102dfaec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfaf0u64 + 4);
        vm.wr(37, 0x102dfaf0u64 + 4);
        Ok(0x102dfaf0u64) }
        0x102dfaf0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfaf4u64) }
        0x102dfaf4 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfaf8u64) }
        0x102dfaf8 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfafcu64) }
        0x102dfafc => { vm.decode(0x20d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfb00u64) }
        0x102dfb00 => { vm.decode(0x288e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb04u64) }
        0x102dfb04 => { vm.decode(0xb3895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb08u64) }
        0x102dfb08 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb0cu64) }
        0x102dfb0c => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb10u64) }
        0x102dfb10 => { vm.decode(0x2882432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb14u64) }
        0x102dfb14 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb18u64) }
        0x102dfb18 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfb1cu64 + 4);
        vm.wr(37, 0x102dfb1cu64 + 4);
        Ok(0x102dfb1cu64) }
        0x102dfb1c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb20u64) }
        0x102dfb20 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb24u64) }
        0x102dfb24 => { vm.decode(0x8b185c30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb28u64) }
        0x102dfb28 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb2cu64) }
        0x102dfb2c => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb30u64) }
        0x102dfb30 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb34u64) }
        0x102dfb34 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb38u64) }
        0x102dfb38 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfb3cu64 + 4);
        vm.wr(37, 0x102dfb3cu64 + 4);
        Ok(0x102dfb3cu64) }
        0x102dfb3c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb40u64) }
        0x102dfb40 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb44u64) }
        0x102dfb44 => { vm.decode(0x8b105cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb48u64) }
        0x102dfb48 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb4cu64) }
        0x102dfb4c => { vm.decode(0x4c901875u32); let d = 22i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb50u64) }
        0x102dfb50 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb54u64) }
        0x102dfb54 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb58u64) }
        0x102dfb58 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfb5cu64 + 4);
        vm.wr(37, 0x102dfb5cu64 + 4);
        Ok(0x102dfb5cu64) }
        0x102dfb5c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb60u64) }
        0x102dfb60 => { vm.decode(0x660f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfb64u64) }
        0x102dfb64 => { vm.decode(0x601b8119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfb68u64) }
        0x102dfb68 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb6cu64) }
        0x102dfb6c => { vm.decode(0x5b000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102dfb70u64) }
        0x102dfb70 => { vm.decode(0x9b38d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb74u64) }
        0x102dfb74 => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb78u64) }
        0x102dfb78 => { vm.decode(0xb289530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb7cu64) }
        0x102dfb7c => { vm.decode(0x882632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb80u64) }
        0x102dfb80 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb84u64) }
        0x102dfb84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfb88u64 + 4);
        vm.wr(37, 0x102dfb88u64 + 4);
        Ok(0x102dfb88u64) }
        0x102dfb88 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfb8cu64) }
        0x102dfb8c => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfb90u64) }
        0x102dfb90 => { vm.decode(0x9b105cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb94u64) }
        0x102dfb94 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfb98u64) }
        0x102dfb98 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfb9cu64) }
        0x102dfb9c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfba0u64) }
        0x102dfba0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfba4u64 + 4);
        vm.wr(37, 0x102dfba4u64 + 4);
        Ok(0x102dfba4u64) }
        0x102dfba4 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfba8u64) }
        0x102dfba8 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfbacu64) }
        0x102dfbac => { vm.decode(0x2a8c436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfbb0u64) }
        0x102dfbb0 => { vm.decode(0x4a8a6b2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfbb4u64) }
        0x102dfbb4 => { vm.decode(0x6884432fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfbb8u64) }
        0x102dfbb8 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfbbcu64) }
        0x102dfbbc => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfbc0u64).wrapping_add((imm * 4) as u64) } else { 0x102dfbc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfbc0u64) }
        0x102dfbc0 => { vm.decode(0x48904875u32); let d = 22i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfbc4u64) }
        0x102dfbc4 => { vm.decode(0x624004f5u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dfbc8u64) }
        0x102dfbc8 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102dfbccu64) }
        0x102dfbcc => { vm.decode(0xd26ec12du32); let d = 4i16; let s = 19i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102dfbd0u64) }
        0x102dfbd0 => { vm.decode(0x44092835u32); let d = 19i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfbd4u64) }
        0x102dfbd4 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfbd8u64) }
        0x102dfbd8 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfbdcu64) }
        0x102dfbdc => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102dfbe0u64).wrapping_add((imm * 4) as u64) } else { 0x102dfbe0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfbe0u64) }
        0x102dfbe0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfbe4u64) } }
        0x102dfbe4 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfbe8u64) }
        0x102dfbe8 => { vm.decode(0x2a8c636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfbecu64) }
        0x102dfbec => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfbf0u64) }
        0x102dfbf0 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfbf4u64) }
        0x102dfbf4 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfbf8u64).wrapping_add((imm * 4) as u64) } else { 0x102dfbf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfbf8u64) }
        0x102dfbf8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfbfcu64) } }
        0x102dfbfc => { vm.decode(0x624004f5u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dfc00u64) }
        0x102dfc00 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102dfc04u64) }
        0x102dfc04 => { vm.decode(0xd2eec12du32); let d = 5i16; let s = 19i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102dfc08u64) }
        0x102dfc08 => { vm.decode(0x64092835u32); let d = 19i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfc0cu64) }
        0x102dfc0c => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc10u64) }
        0x102dfc10 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfc14u64) }
        0x102dfc14 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102dfc18u64).wrapping_add((imm * 4) as u64) } else { 0x102dfc18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfc18u64) }
        0x102dfc18 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfc1cu64) } }
        0x102dfc1c => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc20u64) }
        0x102dfc20 => { vm.decode(0x2088c32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfc24u64) }
        0x102dfc24 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfc28u64) }
        0x102dfc28 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfc2cu64).wrapping_add((imm * 4) as u64) } else { 0x102dfc2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfc2cu64) }
        0x102dfc2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfc30u64) } }
        0x102dfc30 => { vm.decode(0x624004f5u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dfc34u64) }
        0x102dfc34 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102dfc38u64) }
        0x102dfc38 => { vm.decode(0xd2eec12du32); let d = 5i16; let s = 19i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102dfc3cu64) }
        0x102dfc3c => { vm.decode(0x64092835u32); let d = 19i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfc40u64) }
        0x102dfc40 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc44u64) }
        0x102dfc44 => { vm.decode(0xa0000040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102dfc48u64) }
        0x102dfc48 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102dfc4cu64).wrapping_add((imm * 4) as u64) } else { 0x102dfc4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfc4cu64) }
        0x102dfc4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfc50u64) } }
        0x102dfc50 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfc54u64) }
        0x102dfc54 => { vm.decode(0x45146891u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfc58u64) }
        0x102dfc58 => { vm.decode(0xb42d4ff6u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = -8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfc5cu64) }
        0x102dfc5c => { vm.decode(0x20d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfc60u64) }
        0x102dfc60 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfc64u64) }
        0x102dfc64 => { vm.decode(0x803019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc68u64) }
        0x102dfc68 => { vm.decode(0xb3895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc6cu64) }
        0x102dfc6c => { vm.decode(0x8b30d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc70u64) }
        0x102dfc70 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc74u64) }
        0x102dfc74 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfc78u64) }
        0x102dfc78 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfc7cu64 + 4);
        vm.wr(37, 0x102dfc7cu64 + 4);
        Ok(0x102dfc7cu64) }
        0x102dfc7c => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc80u64) }
        0x102dfc80 => { vm.decode(0x605f8019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfc84u64) }
        0x102dfc84 => { vm.decode(0xac416fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfc88u64) }
        0x102dfc88 => { vm.decode(0x288c436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfc8cu64) }
        0x102dfc8c => { vm.decode(0x1b181530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc90u64) }
        0x102dfc90 => { vm.decode(0x8b105530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc94u64) }
        0x102dfc94 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc98u64) }
        0x102dfc98 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfc9cu64) }
        0x102dfc9c => { vm.decode(0x86436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfca0u64) }
        0x102dfca0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfca4u64) }
        0x102dfca4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfca8u64 + 4);
        vm.wr(37, 0x102dfca8u64 + 4);
        Ok(0x102dfca8u64) }
        0x102dfca8 => { vm.decode(0x66098019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfcacu64) }
        0x102dfcac => { vm.decode(0x600f9119u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfcb0u64) }
        0x102dfcb0 => { vm.decode(0x22a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfcb4u64) }
        0x102dfcb4 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfcb8u64) }
        0x102dfcb8 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcbcu64) }
        0x102dfcbc => { vm.decode(0x190895b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcc0u64) }
        0x102dfcc0 => { vm.decode(0x1b301530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcc4u64) }
        0x102dfcc4 => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcc8u64) }
        0x102dfcc8 => { vm.decode(0x86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfcccu64) }
        0x102dfccc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfcd0u64) }
        0x102dfcd0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfcd4u64 + 4);
        vm.wr(37, 0x102dfcd4u64 + 4);
        Ok(0x102dfcd4u64) }
        0x102dfcd4 => { vm.decode(0x660d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfcd8u64) }
        0x102dfcd8 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfcdcu64) }
        0x102dfcdc => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfce0u64) }
        0x102dfce0 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfce4u64) }
        0x102dfce4 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfce8u64) }
        0x102dfce8 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcecu64) }
        0x102dfcec => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfcf0u64) }
        0x102dfcf0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfcf4u64) }
        0x102dfcf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfcf8u64 + 4);
        vm.wr(37, 0x102dfcf8u64 + 4);
        Ok(0x102dfcf8u64) }
        0x102dfcf8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfcfcu64) }
        0x102dfcfc => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd00u64) }
        0x102dfd00 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfd04u64).wrapping_add((imm * 4) as u64) } else { 0x102dfd04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfd04u64) }
        0x102dfd04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfd08u64) } }
        0x102dfd08 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfd0cu64) }
        0x102dfd0c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfd10u64) }
        0x102dfd10 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfd14u64) }
        0x102dfd14 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd18u64) }
        0x102dfd18 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfd1cu64) }
        0x102dfd1c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102dfd20u64).wrapping_add((imm * 4) as u64) } else { 0x102dfd20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfd20u64) }
        0x102dfd20 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfd24u64) } }
        0x102dfd24 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfd28u64).wrapping_add((imm * 4) as u64) } else { 0x102dfd28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfd28u64) }
        0x102dfd28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfd2cu64) } }
        0x102dfd2c => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd30u64) }
        0x102dfd30 => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd34u64) }
        0x102dfd34 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd38u64) }
        0x102dfd38 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd3cu64) }
        0x102dfd3c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfd40u64) }
        0x102dfd40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfd44u64 + 4);
        vm.wr(37, 0x102dfd44u64 + 4);
        Ok(0x102dfd44u64) }
        0x102dfd44 => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfd48u64) }
        0x102dfd48 => { vm.decode(0x9209430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd4cu64) }
        0x102dfd4c => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd50u64) }
        0x102dfd50 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfd54u64).wrapping_add((imm * 4) as u64) } else { 0x102dfd54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfd54u64) }
        0x102dfd54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dfd58u64) } }
        0x102dfd58 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd5cu64) }
        0x102dfd5c => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd60u64) }
        0x102dfd60 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd64u64) }
        0x102dfd64 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd68u64) }
        0x102dfd68 => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd6cu64) }
        0x102dfd6c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfd70u64) }
        0x102dfd70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfd74u64 + 4);
        vm.wr(37, 0x102dfd74u64 + 4);
        Ok(0x102dfd74u64) }
        0x102dfd74 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfd78u64) }
        0x102dfd78 => { vm.decode(0x91894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd7cu64) }
        0x102dfd7c => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfd80u64) }
        0x102dfd80 => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd84u64) }
        0x102dfd84 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfd88u64) }
        0x102dfd88 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfd8cu64) }
        0x102dfd8c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfd90u64) }
        0x102dfd90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfd94u64 + 4);
        vm.wr(37, 0x102dfd94u64 + 4);
        Ok(0x102dfd94u64) }
        0x102dfd94 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfd98u64) }
        0x102dfd98 => { vm.decode(0x74198119u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfd9cu64) }
        0x102dfd9c => { vm.decode(0x448019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfda0u64) }
        0x102dfda0 => { vm.decode(0xa8612fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfda4u64) }
        0x102dfda4 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfda8u64) }
        0x102dfda8 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdacu64) }
        0x102dfdac => { vm.decode(0x4282636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdb0u64) }
        0x102dfdb0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfdb4u64) }
        0x102dfdb4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfdb8u64) }
        0x102dfdb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfdbcu64 + 4);
        vm.wr(37, 0x102dfdbcu64 + 4);
        Ok(0x102dfdbcu64) }
        0x102dfdbc => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfdc0u64) }
        0x102dfdc0 => { vm.decode(0x20a4616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdc4u64) }
        0x102dfdc4 => { vm.decode(0x428c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdc8u64) }
        0x102dfdc8 => { vm.decode(0x8b101db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdccu64) }
        0x102dfdcc => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdd0u64) }
        0x102dfdd0 => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdd4u64) }
        0x102dfdd4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfdd8u64) }
        0x102dfdd8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfddcu64 + 4);
        vm.wr(37, 0x102dfddcu64 + 4);
        Ok(0x102dfddcu64) }
        0x102dfddc => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfde0u64) }
        0x102dfde0 => { vm.decode(0x9209430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfde4u64) }
        0x102dfde4 => { vm.decode(0x91894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfde8u64) }
        0x102dfde8 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdecu64) }
        0x102dfdec => { vm.decode(0x8b381d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdf0u64) }
        0x102dfdf0 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdf4u64) }
        0x102dfdf4 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfdf8u64) }
        0x102dfdf8 => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfdfcu64) }
        0x102dfdfc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe00u64) }
        0x102dfe00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfe04u64 + 4);
        vm.wr(37, 0x102dfe04u64 + 4);
        Ok(0x102dfe04u64) }
        0x102dfe04 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfe08u64) }
        0x102dfe08 => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfe0cu64) }
        0x102dfe0c => { vm.decode(0x891895b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe10u64) }
        0x102dfe10 => { vm.decode(0xa8612fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe14u64) }
        0x102dfe14 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe18u64) }
        0x102dfe18 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe1cu64) }
        0x102dfe1c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe20u64) }
        0x102dfe20 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe24u64) }
        0x102dfe24 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfe28u64 + 4);
        vm.wr(37, 0x102dfe28u64 + 4);
        Ok(0x102dfe28u64) }
        0x102dfe28 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe2cu64) }
        0x102dfe2c => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe30u64) }
        0x102dfe30 => { vm.decode(0x4000022u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfe34u64) }
        0x102dfe34 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfe38u64).wrapping_add((imm * 4) as u64) } else { 0x102dfe38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfe38u64) }
        0x102dfe38 => { vm.decode(0x9209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe3cu64) }
        0x102dfe3c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfe40u64) }
        0x102dfe40 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfe44u64) }
        0x102dfe44 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfe48u64) }
        0x102dfe48 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe4cu64) }
        0x102dfe4c => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe50u64) }
        0x102dfe50 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe54u64) }
        0x102dfe54 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfe58u64) }
        0x102dfe58 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe5cu64) }
        0x102dfe5c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfe60u64 + 4);
        vm.wr(37, 0x102dfe60u64 + 4);
        Ok(0x102dfe60u64) }
        0x102dfe60 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe64u64) }
        0x102dfe64 => { vm.decode(0x22a4416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe68u64) }
        0x102dfe68 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfe6cu64) }
        0x102dfe6c => { vm.decode(0x5b004d73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 492i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102dfe70u64) }
        0x102dfe70 => { vm.decode(0x45984891u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 212i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfe74u64) }
        0x102dfe74 => { vm.decode(0x62355062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dfe78u64) }
        0x102dfe78 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe7cu64) }
        0x102dfe7c => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe80u64) }
        0x102dfe80 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfe84u64) }
        0x102dfe84 => { vm.decode(0x286436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfe88u64) }
        0x102dfe88 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfe8cu64) }
        0x102dfe8c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dfe90u64 + 4);
        vm.wr(37, 0x102dfe90u64 + 4);
        Ok(0x102dfe90u64) }
        0x102dfe90 => { vm.decode(0x624f8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfe94u64) }
        0x102dfe94 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfe98u64) }
        0x102dfe98 => { vm.decode(0x4d984891u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 212i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dfe9cu64) }
        0x102dfe9c => { vm.decode(0x488c432fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfea0u64) }
        0x102dfea0 => { vm.decode(0x500e375u32); let d = 1i16; let s = 17i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102dfea4u64) }
        0x102dfea4 => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dfea8u64).wrapping_add((imm * 4) as u64) } else { 0x102dfea8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfea8u64) }
        0x102dfea8 => { vm.decode(0xbe2fddf6u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfeacu64) }
        0x102dfeac => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfeb0u64) }
        0x102dfeb0 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfeb4u64) }
        0x102dfeb4 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dfeb8u64) }
        0x102dfeb8 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102dfebcu64) }
        0x102dfebc => { vm.decode(0xa8a636fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfec0u64) }
        0x102dfec0 => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102dfec4u64) }
        0x102dfec4 => { vm.decode(0x2800123eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102dfec8u64) }
        0x102dfec8 => { vm.decode(0x61053835u32); let d = 5i16; let s = 19i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dfeccu64) }
        0x102dfecc => { vm.decode(0x420833u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102dfed0u64) }
        0x102dfed0 => { vm.decode(0xbe4fcdf6u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfed4u64) }
        0x102dfed4 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dfed8u64) }
        0x102dfed8 => { vm.decode(0x500e375u32); let d = 1i16; let s = 17i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102dfedcu64) }
        0x102dfedc => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102dfee0u64).wrapping_add((imm * 4) as u64) } else { 0x102dfee0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dfee0u64) }
        0x102dfee0 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dfee4u64) }
        0x102dfee4 => { vm.decode(0x6280436fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfee8u64) }
        0x102dfee8 => { vm.decode(0x1080430u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfeecu64) }
        0x102dfeec => { vm.decode(0x1000430u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfef0u64) }
        0x102dfef0 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfef4u64) }
        0x102dfef4 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfef8u64) }
        0x102dfef8 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dfefcu64) }
        0x102dfefc => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff00u64) }
        0x102dff00 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff04u64) }
        0x102dff04 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dff08u64) }
        0x102dff08 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff0cu64) }
        0x102dff0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dff10u64 + 4);
        vm.wr(37, 0x102dff10u64 + 4);
        Ok(0x102dff10u64) }
        0x102dff10 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff14u64) }
        0x102dff14 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff18u64) }
        0x102dff18 => { vm.decode(0x11000430u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff1cu64) }
        0x102dff1c => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff20u64) }
        0x102dff20 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dff24u64).wrapping_add((imm * 4) as u64) } else { 0x102dff24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dff24u64) }
        0x102dff24 => { vm.decode(0x1080530u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff28u64) }
        0x102dff28 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dff2cu64) }
        0x102dff2c => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dff30u64) }
        0x102dff30 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dff34u64) }
        0x102dff34 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff38u64) }
        0x102dff38 => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff3cu64) }
        0x102dff3c => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff40u64) }
        0x102dff40 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dff44u64) }
        0x102dff44 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff48u64) }
        0x102dff48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dff4cu64 + 4);
        vm.wr(37, 0x102dff4cu64 + 4);
        Ok(0x102dff4cu64) }
        0x102dff4c => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff50u64) }
        0x102dff50 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102dff54u64) }
        0x102dff54 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff58u64) }
        0x102dff58 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff5cu64) }
        0x102dff5c => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff60u64) }
        0x102dff60 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff64u64) }
        0x102dff64 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102dff68u64) }
        0x102dff68 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff6cu64) }
        0x102dff6c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102dff70u64 + 4);
        vm.wr(37, 0x102dff70u64 + 4);
        Ok(0x102dff70u64) }
        0x102dff70 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102dff74u64) }
        0x102dff74 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff78u64) }
        0x102dff78 => { vm.decode(0x6a8c432fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff7cu64) }
        0x102dff7c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dff80u64).wrapping_add((imm * 4) as u64) } else { 0x102dff80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dff80u64) }
        0x102dff80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dff84u64) } }
        0x102dff84 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dff88u64) }
        0x102dff88 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102dff8cu64) }
        0x102dff8c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102dff90u64) }
        0x102dff90 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dff94u64) }
        0x102dff94 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102dff98u64) }
        0x102dff98 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102dff9cu64).wrapping_add((imm * 4) as u64) } else { 0x102dff9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dff9cu64) }
        0x102dff9c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102dffa0u64) } }
        0x102dffa0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102dffa4u64).wrapping_add((imm * 4) as u64) } else { 0x102dffa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102dffa4u64) }
        0x102dfff8 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102dfffcu64) }
        0x102dfffc => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0000u64) }
        0x102e0000 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0004u64) }
        0x102e0004 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0008u64) }
        0x102e0008 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e000cu64) }
        0x102e000c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0010u64 + 4);
        vm.wr(37, 0x102e0010u64 + 4);
        Ok(0x102e0010u64) }
        0x102e0010 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0014u64) }
        0x102e0014 => { vm.decode(0x741d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0018u64) }
        0x102e0018 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e001cu64) }
        0x102e001c => { vm.decode(0x88c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0020u64) }
        0x102e0020 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0024u64) }
        0x102e0024 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0028u64) }
        0x102e0028 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e002cu64) }
        0x102e002c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0030u64) }
        0x102e0030 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0034u64 + 4);
        vm.wr(37, 0x102e0034u64 + 4);
        Ok(0x102e0034u64) }
        0x102e0034 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0038u64) }
        0x102e0038 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e003cu64) }
        0x102e003c => { vm.decode(0x8b105db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0040u64) }
        0x102e0040 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0044u64) }
        0x102e0044 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0048u64) }
        0x102e0048 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e004cu64) }
        0x102e004c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0050u64 + 4);
        vm.wr(37, 0x102e0050u64 + 4);
        Ok(0x102e0050u64) }
        0x102e0050 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0054u64) }
        0x102e0054 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0058u64) }
        0x102e0058 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e005cu64) }
        0x102e005c => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0060u64) }
        0x102e0060 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0064u64) }
        0x102e0064 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0068u64) }
        0x102e0068 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e006cu64) }
        0x102e006c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0070u64) }
        0x102e0070 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0074u64 + 4);
        vm.wr(37, 0x102e0074u64 + 4);
        Ok(0x102e0074u64) }
        0x102e0074 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0078u64) }
        0x102e0078 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e007cu64) }
        0x102e007c => { vm.decode(0x6a80432fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0080u64) }
        0x102e0080 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0084u64).wrapping_add((imm * 4) as u64) } else { 0x102e0084u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0084u64) }
        0x102e0084 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0088u64) } }
        0x102e0088 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e008cu64) }
        0x102e008c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e0090u64) }
        0x102e0090 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e0094u64) }
        0x102e0094 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0098u64) }
        0x102e0098 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e009cu64) }
        0x102e009c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e00a0u64).wrapping_add((imm * 4) as u64) } else { 0x102e00a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e00a0u64) }
        0x102e00a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e00a4u64) } }
        0x102e00a4 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e00a8u64).wrapping_add((imm * 4) as u64) } else { 0x102e00a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e00a8u64) }
        0x102e00a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e00acu64) } }
        0x102e00ac => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e00b0u64) }
        0x102e00b0 => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00b4u64) }
        0x102e00b4 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00b8u64) }
        0x102e00b8 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e00bcu64) }
        0x102e00bc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e00c0u64) }
        0x102e00c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e00c4u64 + 4);
        vm.wr(37, 0x102e00c4u64 + 4);
        Ok(0x102e00c4u64) }
        0x102e00c4 => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e00c8u64) }
        0x102e00c8 => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00ccu64) }
        0x102e00cc => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e00d0u64) }
        0x102e00d0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e00d4u64).wrapping_add((imm * 4) as u64) } else { 0x102e00d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e00d4u64) }
        0x102e00d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e00d8u64) } }
        0x102e00d8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e00dcu64) }
        0x102e00dc => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e00e0u64) }
        0x102e00e0 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00e4u64) }
        0x102e00e4 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00e8u64) }
        0x102e00e8 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e00ecu64) }
        0x102e00ec => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e00f0u64) }
        0x102e00f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e00f4u64 + 4);
        vm.wr(37, 0x102e00f4u64 + 4);
        Ok(0x102e00f4u64) }
        0x102e00f4 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e00f8u64) }
        0x102e00f8 => { vm.decode(0x92094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e00fcu64) }
        0x102e00fc => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0100u64) }
        0x102e0100 => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0104u64) }
        0x102e0104 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0108u64) }
        0x102e0108 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e010cu64) }
        0x102e010c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0110u64) }
        0x102e0110 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0114u64 + 4);
        vm.wr(37, 0x102e0114u64 + 4);
        Ok(0x102e0114u64) }
        0x102e0114 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0118u64) }
        0x102e0118 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e011cu64) }
        0x102e011c => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0120u64) }
        0x102e0120 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0124u64) }
        0x102e0124 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0128u64) }
        0x102e0128 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e012cu64) }
        0x102e012c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0130u64) }
        0x102e0130 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0134u64 + 4);
        vm.wr(37, 0x102e0134u64 + 4);
        Ok(0x102e0134u64) }
        0x102e0134 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0138u64) }
        0x102e0138 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e013cu64) }
        0x102e013c => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0140u64) }
        0x102e0140 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0144u64) }
        0x102e0144 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0148u64) }
        0x102e0148 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e014cu64) }
        0x102e014c => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0150u64) }
        0x102e0150 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0154u64) }
        0x102e0154 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0158u64 + 4);
        vm.wr(37, 0x102e0158u64 + 4);
        Ok(0x102e0158u64) }
        0x102e0158 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e015cu64) }
        0x102e015c => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0160u64) }
        0x102e0160 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0164u64).wrapping_add((imm * 4) as u64) } else { 0x102e0164u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0164u64) }
        0x102e0164 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0168u64) } }
        0x102e0168 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e016cu64) }
        0x102e016c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e0170u64) }
        0x102e0170 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e0174u64) }
        0x102e0174 => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0178u64) }
        0x102e0178 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e017cu64) }
        0x102e017c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e0180u64).wrapping_add((imm * 4) as u64) } else { 0x102e0180u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0180u64) }
        0x102e0180 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0184u64) } }
        0x102e0184 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0188u64).wrapping_add((imm * 4) as u64) } else { 0x102e0188u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0188u64) }
        0x102e0188 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e018cu64) } }
        0x102e018c => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0190u64) }
        0x102e0190 => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0194u64) }
        0x102e0194 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0198u64) }
        0x102e0198 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e019cu64) }
        0x102e019c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e01a0u64) }
        0x102e01a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e01a4u64 + 4);
        vm.wr(37, 0x102e01a4u64 + 4);
        Ok(0x102e01a4u64) }
        0x102e01a4 => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e01a8u64) }
        0x102e01a8 => { vm.decode(0x9281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01acu64) }
        0x102e01ac => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e01b0u64) }
        0x102e01b0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e01b4u64).wrapping_add((imm * 4) as u64) } else { 0x102e01b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e01b4u64) }
        0x102e01b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e01b8u64) } }
        0x102e01b8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e01bcu64) }
        0x102e01bc => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e01c0u64) }
        0x102e01c0 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01c4u64) }
        0x102e01c4 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01c8u64) }
        0x102e01c8 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e01ccu64) }
        0x102e01cc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e01d0u64) }
        0x102e01d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e01d4u64 + 4);
        vm.wr(37, 0x102e01d4u64 + 4);
        Ok(0x102e01d4u64) }
        0x102e01d4 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e01d8u64) }
        0x102e01d8 => { vm.decode(0x9201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01dcu64) }
        0x102e01dc => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e01e0u64) }
        0x102e01e0 => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01e4u64) }
        0x102e01e4 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e01e8u64) }
        0x102e01e8 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e01ecu64) }
        0x102e01ec => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e01f0u64) }
        0x102e01f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e01f4u64 + 4);
        vm.wr(37, 0x102e01f4u64 + 4);
        Ok(0x102e01f4u64) }
        0x102e01f4 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e01f8u64) }
        0x102e01f8 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e01fcu64) }
        0x102e01fc => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0200u64) }
        0x102e0200 => { vm.decode(0x9b101cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0204u64) }
        0x102e0204 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0208u64) }
        0x102e0208 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e020cu64) }
        0x102e020c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0210u64 + 4);
        vm.wr(37, 0x102e0210u64 + 4);
        Ok(0x102e0210u64) }
        0x102e0210 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0214u64) }
        0x102e0214 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0218u64) }
        0x102e0218 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e021cu64) }
        0x102e021c => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0220u64) }
        0x102e0220 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0224u64) }
        0x102e0224 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0228u64) }
        0x102e0228 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e022cu64) }
        0x102e022c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0230u64) }
        0x102e0230 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0234u64 + 4);
        vm.wr(37, 0x102e0234u64 + 4);
        Ok(0x102e0234u64) }
        0x102e0234 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0238u64) }
        0x102e0238 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e023cu64) }
        0x102e023c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0240u64).wrapping_add((imm * 4) as u64) } else { 0x102e0240u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0240u64) }
        0x102e0240 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0244u64) } }
        0x102e0244 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e0248u64) }
        0x102e0248 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e024cu64) }
        0x102e024c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e0250u64) }
        0x102e0250 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0254u64) }
        0x102e0254 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e0258u64) }
        0x102e0258 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e025cu64).wrapping_add((imm * 4) as u64) } else { 0x102e025cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e025cu64) }
        0x102e025c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0260u64) } }
        0x102e0260 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0264u64).wrapping_add((imm * 4) as u64) } else { 0x102e0264u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0264u64) }
        0x102e0264 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0268u64) } }
        0x102e0268 => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e026cu64) }
        0x102e026c => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0270u64) }
        0x102e0270 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0274u64) }
        0x102e0274 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0278u64) }
        0x102e0278 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e027cu64) }
        0x102e027c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0280u64 + 4);
        vm.wr(37, 0x102e0280u64 + 4);
        Ok(0x102e0280u64) }
        0x102e0280 => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0284u64) }
        0x102e0284 => { vm.decode(0x9301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0288u64) }
        0x102e0288 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e028cu64) }
        0x102e028c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0290u64).wrapping_add((imm * 4) as u64) } else { 0x102e0290u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0290u64) }
        0x102e0290 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0294u64) } }
        0x102e0294 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0298u64) }
        0x102e0298 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e029cu64) }
        0x102e029c => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02a0u64) }
        0x102e02a0 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02a4u64) }
        0x102e02a4 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e02a8u64) }
        0x102e02a8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02acu64) }
        0x102e02ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e02b0u64 + 4);
        vm.wr(37, 0x102e02b0u64 + 4);
        Ok(0x102e02b0u64) }
        0x102e02b0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02b4u64) }
        0x102e02b4 => { vm.decode(0x9281cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02b8u64) }
        0x102e02b8 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e02bcu64) }
        0x102e02bc => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02c0u64) }
        0x102e02c0 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02c4u64) }
        0x102e02c4 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e02c8u64) }
        0x102e02c8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02ccu64) }
        0x102e02cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e02d0u64 + 4);
        vm.wr(37, 0x102e02d0u64 + 4);
        Ok(0x102e02d0u64) }
        0x102e02d0 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02d4u64) }
        0x102e02d4 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e02d8u64) }
        0x102e02d8 => { vm.decode(0x20a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e02dcu64) }
        0x102e02dc => { vm.decode(0xa88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e02e0u64) }
        0x102e02e0 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02e4u64) }
        0x102e02e4 => { vm.decode(0x9b305c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02e8u64) }
        0x102e02e8 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e02ecu64) }
        0x102e02ec => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02f0u64) }
        0x102e02f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e02f4u64 + 4);
        vm.wr(37, 0x102e02f4u64 + 4);
        Ok(0x102e02f4u64) }
        0x102e02f4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e02f8u64) }
        0x102e02f8 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e02fcu64) }
        0x102e02fc => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0300u64).wrapping_add((imm * 4) as u64) } else { 0x102e0300u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0300u64) }
        0x102e0300 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0304u64) } }
        0x102e0304 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e0308u64) }
        0x102e0308 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e030cu64) }
        0x102e030c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e0310u64) }
        0x102e0310 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0314u64) }
        0x102e0314 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e0318u64) }
        0x102e0318 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e031cu64).wrapping_add((imm * 4) as u64) } else { 0x102e031cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e031cu64) }
        0x102e031c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e0320u64) } }
        0x102e0320 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e0324u64).wrapping_add((imm * 4) as u64) } else { 0x102e0324u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e0324u64) }
        0x102e0378 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e037cu64) }
        0x102e037c => { vm.decode(0x9b205c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0380u64) }
        0x102e0380 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e0384u64) }
        0x102e0384 => { vm.decode(0x665d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e0388u64) }
        0x102e0388 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e038cu64) }
        0x102e038c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e0390u64 + 4);
        vm.wr(37, 0x102e0390u64 + 4);
        Ok(0x102e0390u64) }
        0x102e0390 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e0394u64) }
        0x102e0394 => { vm.decode(0x4084c3efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e0398u64) }
        0x102e0398 => { vm.decode(0x6084e3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e039cu64) }
        0x102e039c => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03a0u64) }
        0x102e03a0 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03a4u64) }
        0x102e03a4 => { vm.decode(0x4882c3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03a8u64) }
        0x102e03a8 => { vm.decode(0x6882e3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03acu64) }
        0x102e03ac => { vm.decode(0x4a8ac3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03b0u64) }
        0x102e03b0 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03b4u64) }
        0x102e03b4 => { vm.decode(0x488ecbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03b8u64) }
        0x102e03b8 => { vm.decode(0x688eebafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03bcu64) }
        0x102e03bc => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102e03c0u64) }
        0x102e03c0 => { vm.decode(0x775b8059u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e03c4u64) }
        0x102e03c4 => { vm.decode(0x6280436fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e03c8u64) }
        0x102e4430 => { vm.decode(0x7b6bce19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -880i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4434u64) }
        0x102e4434 => { vm.decode(0x9b285df0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4438u64) }
        0x102e4438 => { vm.decode(0x8b205df0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e443cu64) }
        0x102e443c => { vm.decode(0x9b385d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4440u64) }
        0x102e4440 => { vm.decode(0x8b305d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4444u64) }
        0x102e4444 => { vm.decode(0x9b285c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4448u64) }
        0x102e4448 => { vm.decode(0x8b205c30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e444cu64) }
        0x102e444c => { vm.decode(0x9b181db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4450u64) }
        0x102e4450 => { vm.decode(0x8b101db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4454u64) }
        0x102e4454 => { vm.decode(0x9b081cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4458u64) }
        0x102e4458 => { vm.decode(0x8b001cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e445cu64) }
        0x102e445c => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4460u64) }
        0x102e4460 => { vm.decode(0x1d4f778eu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = -3997696i64;
        vm.st(d, imm as u64);
        Ok(0x102e4464u64) }
        0x102e4464 => { vm.decode(0x691732d8u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 53088i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e4468u64) }
        0x102e4468 => { vm.decode(0x6088012fu32); let d = 19i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e446cu64) }
        0x102e446c => { vm.decode(0xa24104f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4470u64) }
        0x102e4470 => { vm.decode(0x288012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4474u64) }
        0x102e4474 => { vm.decode(0x90015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4478u64) }
        0x102e4478 => { vm.decode(0x2ac002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e447cu64) }
        0x102e447c => { vm.decode(0x9009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4480u64) }
        0x102e4480 => { vm.decode(0x2ac202fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4484u64) }
        0x102e4484 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4488u64) }
        0x102e4488 => { vm.decode(0xa8006fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e448cu64) }
        0x102e448c => { vm.decode(0x93015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4490u64) }
        0x102e4490 => { vm.decode(0xa8206fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4494u64) }
        0x102e4494 => { vm.decode(0x9381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4498u64) }
        0x102e4498 => { vm.decode(0x2a8006fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e449cu64) }
        0x102e449c => { vm.decode(0x9201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44a0u64) }
        0x102e44a0 => { vm.decode(0x2a8206fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44a4u64) }
        0x102e44a4 => { vm.decode(0x91815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44a8u64) }
        0x102e44a8 => { vm.decode(0xac006fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44acu64) }
        0x102e44ac => { vm.decode(0x9181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44b0u64) }
        0x102e44b0 => { vm.decode(0xac206fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44b4u64) }
        0x102e44b4 => { vm.decode(0x9101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44b8u64) }
        0x102e44b8 => { vm.decode(0x2ac006fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44bcu64) }
        0x102e44bc => { vm.decode(0x90815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44c0u64) }
        0x102e44c0 => { vm.decode(0x2ac206fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44c4u64) }
        0x102e44c4 => { vm.decode(0x9081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44c8u64) }
        0x102e44c8 => { vm.decode(0x8a8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44ccu64) }
        0x102e44cc => { vm.decode(0x91015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44d0u64) }
        0x102e44d0 => { vm.decode(0x8a8202fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44d4u64) }
        0x102e44d4 => { vm.decode(0x9089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44d8u64) }
        0x102e44d8 => { vm.decode(0xac202fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44dcu64) }
        0x102e44dc => { vm.decode(0x90895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44e0u64) }
        0x102e44e0 => { vm.decode(0x88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44e4u64) }
        0x102e44e4 => { vm.decode(0x92815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44e8u64) }
        0x102e44e8 => { vm.decode(0xac002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44ecu64) }
        0x102e44ec => { vm.decode(0x92015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e44f0u64) }
        0x102e44f0 => { vm.decode(0x608a202fu32); let d = 23i16; let s = 4i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44f4u64) }
        0x102e44f4 => { vm.decode(0x62a2202fu32); let d = 21i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44f8u64) }
        0x102e44f8 => { vm.decode(0x4282212fu32); let d = 20i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e44fcu64) }
        0x102e44fc => { vm.decode(0x42aa002fu32); let d = 22i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4500u64) }
        0x102e4500 => { vm.decode(0x60a0202fu32); let d = 17i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4504u64) }
        0x102e4504 => { vm.decode(0xa8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4508u64) }
        0x102e4508 => { vm.decode(0x2088002fu32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e450cu64) }
        0x102e450c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4510u64) }
        0x102e4510 => { vm.decode(0x19189530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4514u64) }
        0x102e4514 => { vm.decode(0x202c012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4518u64) }
        0x102e4518 => { vm.decode(0x1b181d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e451cu64) }
        0x102e451c => { vm.decode(0x604d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4520u64) }
        0x102e4520 => { vm.decode(0x19109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4524u64) }
        0x102e4524 => { vm.decode(0x1b101d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4528u64) }
        0x102e4528 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e452cu64) }
        0x102e452c => { vm.decode(0xce1732d8u32); let d = 18i16; let s = 30i16; let s2 = -1i16; let imm = 53040i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e4530u64) }
        0x102e4530 => { vm.decode(0x430204f5u32); let d = 4i16; let s = 18i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4534u64) }
        0x102e4534 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4538u64) }
        0x102e4538 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e453cu64 + 4);
        vm.wr(37, 0x102e453cu64 + 4);
        Ok(0x102e453cu64) }
        0x102e453c => { vm.decode(0x660d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4540u64) }
        0x102e4540 => { vm.decode(0x6b1732d8u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 53104i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e4544u64) }
        0x102e4544 => { vm.decode(0xa24104f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4548u64) }
        0x102e4548 => { vm.decode(0x610304f5u32); let d = 4i16; let s = 18i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e454cu64) }
        0x102e454c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4550u64) }
        0x102e4550 => { vm.decode(0x600d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4554u64) }
        0x102e4554 => { vm.decode(0xb001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4558u64) }
        0x102e4558 => { vm.decode(0x665f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e455cu64) }
        0x102e455c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4560u64) }
        0x102e4560 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e4564u64 + 4);
        vm.wr(37, 0x102e4564u64 + 4);
        Ok(0x102e4564u64) }
        0x102e4564 => { vm.decode(0x1b3894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4568u64) }
        0x102e4568 => { vm.decode(0x685e32d8u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 53128i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e456cu64) }
        0x102e456c => { vm.decode(0x430324f5u32); let d = 4i16; let s = 18i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4570u64) }
        0x102e4570 => { vm.decode(0x900d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4574u64) }
        0x102e4574 => { vm.decode(0xa24104f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4578u64) }
        0x102e4578 => { vm.decode(0x725b1119u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e457cu64) }
        0x102e457c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4580u64) }
        0x102e4580 => { vm.decode(0x8b30d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4584u64) }
        0x102e4584 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4588u64) }
        0x102e4588 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e458cu64) }
        0x102e458c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e4590u64 + 4);
        vm.wr(37, 0x102e4590u64 + 4);
        Ok(0x102e4590u64) }
        0x102e4590 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4594u64) }
        0x102e4594 => { vm.decode(0x6a5732d8u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 53048i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e4598u64) }
        0x102e4598 => { vm.decode(0xa24104f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e459cu64) }
        0x102e459c => { vm.decode(0x490124f5u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e45a0u64) }
        0x102e45a0 => { vm.decode(0x6020002fu32); let d = 17i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45a4u64) }
        0x102e45a4 => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45a8u64) }
        0x102e45a8 => { vm.decode(0x4222002fu32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45acu64) }
        0x102e45ac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45b0u64) }
        0x102e45b0 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e45b4u64) }
        0x102e45b4 => { vm.decode(0x7b000a73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102e45b8u64) }
        0x102e45b8 => { vm.decode(0x601b1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e45bcu64) }
        0x102e45bc => { vm.decode(0x9301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e45c0u64) }
        0x102e45c0 => { vm.decode(0xb309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e45c4u64) }
        0x102e45c4 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e45c8u64) }
        0x102e45c8 => { vm.decode(0x99285530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e45ccu64) }
        0x102e45cc => { vm.decode(0x9b38d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e45d0u64) }
        0x102e45d0 => { vm.decode(0x4890d875u32); let d = 23i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e45d4u64) }
        0x102e45d4 => { vm.decode(0x1b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e45d8u64) }
        0x102e45d8 => { vm.decode(0x660f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e45dcu64) }
        0x102e45dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e45e0u64) }
        0x102e45e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e45e4u64 + 4);
        vm.wr(37, 0x102e45e4u64 + 4);
        Ok(0x102e45e4u64) }
        0x102e45e4 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e45e8u64) }
        0x102e45e8 => { vm.decode(0x6a1e32d8u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 53136i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102e45ecu64) }
        0x102e45ec => { vm.decode(0xa24104f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e45f0u64) }
        0x102e45f0 => { vm.decode(0x8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45f4u64) }
        0x102e45f4 => { vm.decode(0x430204f5u32); let d = 4i16; let s = 18i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e45f8u64) }
        0x102e45f8 => { vm.decode(0x8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e45fcu64) }
        0x102e45fc => { vm.decode(0x4b0004f5u32); let d = 2i16; let s = 18i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4600u64) }
        0x102e4600 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4604u64) }
        0x102e4604 => { vm.decode(0x288c436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4608u64) }
        0x102e4608 => { vm.decode(0x2228012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e460cu64) }
        0x102e460c => { vm.decode(0x1b209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4610u64) }
        0x102e4610 => { vm.decode(0x664f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4614u64) }
        0x102e4614 => { vm.decode(0x9b1894b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4618u64) }
        0x102e4618 => { vm.decode(0x8b10d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e461cu64) }
        0x102e461c => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4620u64) }
        0x102e4620 => { vm.decode(0x8b00d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4624u64) }
        0x102e4624 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4628u64) }
        0x102e4628 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e462cu64 + 4);
        vm.wr(37, 0x102e462cu64 + 4);
        Ok(0x102e462cu64) }
        0x102e462c => { vm.decode(0x1b189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4630u64) }
        0x102e4630 => { vm.decode(0x6b8704f5u32); let d = 30i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4634u64) }
        0x102e4634 => { vm.decode(0x2884632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4638u64) }
        0x102e4638 => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e463cu64) }
        0x102e463c => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4640u64) }
        0x102e4640 => { vm.decode(0x705d9059u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4644u64) }
        0x102e4644 => { vm.decode(0x20a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4648u64) }
        0x102e4648 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e464cu64) }
        0x102e464c => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4650u64) }
        0x102e4650 => { vm.decode(0x9b089530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4654u64) }
        0x102e4654 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4658u64) }
        0x102e4658 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e465cu64) }
        0x102e465c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e4660u64 + 4);
        vm.wr(37, 0x102e4660u64 + 4);
        Ok(0x102e4660u64) }
        0x102e4660 => { vm.decode(0x908d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4664u64) }
        0x102e4664 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4668u64) }
        0x102e4668 => { vm.decode(0x690604f5u32); let d = 22i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e466cu64) }
        0x102e466c => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4670u64) }
        0x102e4670 => { vm.decode(0x610604f5u32); let d = 20i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e4674u64) }
        0x102e4674 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4678u64) }
        0x102e4678 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e467cu64) }
        0x102e467c => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4680u64) }
        0x102e4680 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4684u64) }
        0x102e4684 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e4688u64).wrapping_add((imm * 4) as u64) } else { 0x102e4688u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e4688u64) }
        0x102e4688 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e468cu64) } }
        0x102e468c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e4690u64) }
        0x102e4690 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e4694u64) }
        0x102e4694 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e4698u64) }
        0x102e4698 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e469cu64) }
        0x102e469c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e46a0u64) }
        0x102e46a0 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e46a4u64).wrapping_add((imm * 4) as u64) } else { 0x102e46a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e46a4u64) }
        0x102e46a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e46a8u64) } }
        0x102e46a8 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e46acu64).wrapping_add((imm * 4) as u64) } else { 0x102e46acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e46acu64) }
        0x102e46ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e46b0u64) } }
        0x102e46b0 => { vm.decode(0x20a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46b4u64) }
        0x102e46b4 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e46b8u64) }
        0x102e46b8 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e46bcu64) }
        0x102e46bc => { vm.decode(0x882432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46c0u64) }
        0x102e46c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e46c4u64) }
        0x102e46c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e46c8u64 + 4);
        vm.wr(37, 0x102e46c8u64 + 4);
        Ok(0x102e46c8u64) }
        0x102e46c8 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e46ccu64) }
        0x102e46cc => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e46d0u64) }
        0x102e46d0 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46d4u64) }
        0x102e46d4 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e46d8u64).wrapping_add((imm * 4) as u64) } else { 0x102e46d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e46d8u64) }
        0x102e46d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e46dcu64) } }
        0x102e46dc => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46e0u64) }
        0x102e46e0 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46e4u64) }
        0x102e46e4 => { vm.decode(0xb3095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e46e8u64) }
        0x102e46e8 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e46ecu64) }
        0x102e46ec => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e46f0u64) }
        0x102e46f0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e46f4u64) }
        0x102e46f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e46f8u64 + 4);
        vm.wr(37, 0x102e46f8u64 + 4);
        Ok(0x102e46f8u64) }
        0x102e46f8 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e46fcu64) }
        0x102e46fc => { vm.decode(0x92094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4700u64) }
        0x102e4700 => { vm.decode(0x28a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4704u64) }
        0x102e4704 => { vm.decode(0x9b2815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4708u64) }
        0x102e4708 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e470cu64) }
        0x102e470c => { vm.decode(0x66198119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4710u64) }
        0x102e4710 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4714u64) }
        0x102e4714 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e4718u64 + 4);
        vm.wr(37, 0x102e4718u64 + 4);
        Ok(0x102e4718u64) }
        0x102e4718 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e471cu64) }
        0x102e471c => { vm.decode(0x660d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e4720u64) }
        0x102e4720 => { vm.decode(0x22a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4724u64) }
        0x102e4724 => { vm.decode(0x9b305530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e4728u64) }
        0x102e4728 => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e472cu64) }
        0x102e472c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4730u64) }
        0x102e4730 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e4734u64 + 4);
        vm.wr(37, 0x102e4734u64 + 4);
        Ok(0x102e4734u64) }
        0x102e4734 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e4738u64) }
        0x102e4738 => { vm.decode(0x21c13a3eu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102e473cu64) }
        0x102e473c => { vm.decode(0x12840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 588i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e4740u64).wrapping_add((imm * 4) as u64) } else { 0x102e4740u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e4740u64) }
        0x102e4740 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e4744u64) } }
        0x102e4744 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e4748u64) }
        0x102e5070 => { vm.decode(0x2a80632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5074u64) }
        0x102e5074 => { vm.decode(0x2200212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5078u64) }
        0x102e5078 => { vm.decode(0x82022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102e507cu64).wrapping_add((imm * 4) as u64) } else { 0x102e507cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e507cu64) }
        0x102e507c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e5080u64) } }
        0x102e5080 => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5084u64) }
        0x102e5084 => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e5088u64) }
        0x102e5088 => { vm.decode(0x22a0426fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e508cu64) }
        0x102e508c => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5090u64) }
        0x102e5090 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e5094u64) }
        0x102e5094 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e5098u64 + 4);
        vm.wr(37, 0x102e5098u64 + 4);
        Ok(0x102e5098u64) }
        0x102e5098 => { vm.decode(0x62198159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e509cu64) }
        0x102e509c => { vm.decode(0x2084e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50a0u64) }
        0x102e50a0 => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50a4u64) }
        0x102e50a4 => { vm.decode(0x10580030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50a8u64) }
        0x102e50a8 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50acu64) }
        0x102e50ac => { vm.decode(0x614604f5u32); let d = 21i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102e50b0u64) }
        0x102e50b0 => { vm.decode(0x2aa0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50b4u64) }
        0x102e50b4 => { vm.decode(0x9b30d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50b8u64) }
        0x102e50b8 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50bcu64) }
        0x102e50bc => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50c0u64) }
        0x102e50c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e50c4u64) }
        0x102e50c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e50c8u64 + 4);
        vm.wr(37, 0x102e50c8u64 + 4);
        Ok(0x102e50c8u64) }
        0x102e50c8 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e50ccu64) }
        0x102e50cc => { vm.decode(0x20a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50d0u64) }
        0x102e50d0 => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50d4u64) }
        0x102e50d4 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50d8u64) }
        0x102e50d8 => { vm.decode(0x9b089530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50dcu64) }
        0x102e50dc => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e50e0u64) }
        0x102e50e0 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50e4u64) }
        0x102e50e4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e50e8u64) }
        0x102e50e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e50ecu64 + 4);
        vm.wr(37, 0x102e50ecu64 + 4);
        Ok(0x102e50ecu64) }
        0x102e50ec => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e50f0u64) }
        0x102e50f0 => { vm.decode(0x288c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e50f4u64) }
        0x102e50f4 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e50f8u64).wrapping_add((imm * 4) as u64) } else { 0x102e50f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e50f8u64) }
        0x102e50f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e50fcu64) } }
        0x102e50fc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e5100u64) }
        0x102e5100 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102e5104u64) }
        0x102e5104 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102e5108u64) }
        0x102e5108 => { vm.decode(0x288c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e510cu64) }
        0x102e510c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102e5110u64) }
        0x102e5110 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102e5114u64).wrapping_add((imm * 4) as u64) } else { 0x102e5114u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e5114u64) }
        0x102e5114 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e5118u64) } }
        0x102e5118 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e511cu64).wrapping_add((imm * 4) as u64) } else { 0x102e511cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e511cu64) }
        0x102e511c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e5120u64) } }
        0x102e5120 => { vm.decode(0x20a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5124u64) }
        0x102e5124 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5128u64) }
        0x102e5128 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e512cu64) }
        0x102e512c => { vm.decode(0x882432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5130u64) }
        0x102e5130 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e5134u64) }
        0x102e5134 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e5138u64 + 4);
        vm.wr(37, 0x102e5138u64 + 4);
        Ok(0x102e5138u64) }
        0x102e5138 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e513cu64) }
        0x102e513c => { vm.decode(0x9101c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5140u64) }
        0x102e5140 => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5144u64) }
        0x102e5144 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102e5148u64).wrapping_add((imm * 4) as u64) } else { 0x102e5148u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102e5148u64) }
        0x102e5148 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102e514cu64) } }
        0x102e514c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5150u64) }
        0x102e5150 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5154u64) }
        0x102e5154 => { vm.decode(0xb3095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5158u64) }
        0x102e5158 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e515cu64) }
        0x102e515c => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5160u64) }
        0x102e5160 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e5164u64) }
        0x102e5164 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e5168u64 + 4);
        vm.wr(37, 0x102e5168u64 + 4);
        Ok(0x102e5168u64) }
        0x102e5168 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e516cu64) }
        0x102e516c => { vm.decode(0x9081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5170u64) }
        0x102e5170 => { vm.decode(0x28a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5174u64) }
        0x102e5174 => { vm.decode(0x9b2815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e5178u64) }
        0x102e5178 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e517cu64) }
        0x102e517c => { vm.decode(0x66198119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e5180u64) }
        0x102e5180 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e5184u64) }
        0x102e5184 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e5188u64 + 4);
        vm.wr(37, 0x102e5188u64 + 4);
        Ok(0x102e5188u64) }
        0x102e5188 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e518cu64) }
        0x102e518c => { vm.decode(0x62498159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e5190u64) }
        0x102e5190 => { vm.decode(0x22a4622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5194u64) }
        0x102e5194 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e5198u64) }
        0x102e5198 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e519cu64) }
        0x102e519c => { vm.decode(0x19301c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102e51a0u64) }
        0x102e51a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e51a4u64) }
        0x102e51a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102e51a8u64 + 4);
        vm.wr(37, 0x102e51a8u64 + 4);
        Ok(0x102e51a8u64) }
        0x102e51a8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102e51acu64) }
        0x102e51ac => { vm.decode(0x4084c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51b0u64) }
        0x102e51b0 => { vm.decode(0x6084e3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51b4u64) }
        0x102e51b4 => { vm.decode(0x428cc3afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51b8u64) }
        0x102e51b8 => { vm.decode(0x628ce3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51bcu64) }
        0x102e51bc => { vm.decode(0x4082c3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51c0u64) }
        0x102e51c0 => { vm.decode(0x6082e3efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51c4u64) }
        0x102e51c4 => { vm.decode(0x428ac3efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51c8u64) }
        0x102e51c8 => { vm.decode(0x628ae3efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51ccu64) }
        0x102e51cc => { vm.decode(0x408ecbefu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51d0u64) }
        0x102e51d0 => { vm.decode(0x608eebefu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102e51d4u64) }
        0x102e51d4 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102e51d8u64) }
        0x102e51d8 => { vm.decode(0x77598159u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102e51dcu64) }
        0x102e51dc => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102e51e0u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
