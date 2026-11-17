#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102d3714 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3718u64) }
        0x102d3718 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d371cu64) }
        0x102d371c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3720u64) }
        0x102d3720 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3724u64) }
        0x102d3724 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3728u64).wrapping_add((imm * 4) as u64) } else { 0x102d3728u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3728u64) }
        0x102d3728 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d372cu64) } }
        0x102d372c => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3730u64) }
        0x102d3730 => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3734u64) }
        0x102d3734 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3738u64) }
        0x102d3738 => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d373cu64) }
        0x102d373c => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3740u64) }
        0x102d3740 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3744u64) }
        0x102d3744 => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3748u64) }
        0x102d3748 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d374cu64) }
        0x102d374c => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3750u64) }
        0x102d3750 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3754u64) }
        0x102d3754 => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3758u64) }
        0x102d3758 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d375cu64) }
        0x102d375c => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3760u64) }
        0x102d3760 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3764u64) }
        0x102d3764 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3768u64) }
        0x102d3768 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d376cu64) }
        0x102d376c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3770u64).wrapping_add((imm * 4) as u64) } else { 0x102d3770u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3770u64) }
        0x102d3770 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3774u64) }
        0x102d3774 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3778u64) }
        0x102d3778 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d377cu64) }
        0x102d377c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3780u64) }
        0x102d3780 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3784u64) }
        0x102d3784 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3788u64).wrapping_add((imm * 4) as u64) } else { 0x102d3788u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3788u64) }
        0x102d3788 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d378cu64) } }
        0x102d378c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3790u64) }
        0x102d3790 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3794u64) }
        0x102d3794 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3798u64).wrapping_add((imm * 4) as u64) } else { 0x102d3798u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3798u64) }
        0x102d3798 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d379cu64) }
        0x102d379c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d37a0u64) }
        0x102d37a0 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d37a4u64) }
        0x102d37a4 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d37a8u64) }
        0x102d37a8 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d37acu64) }
        0x102d37ac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d37b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d37b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d37b0u64) }
        0x102d37b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d37b4u64) } }
        0x102d37b4 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d37b8u64) }
        0x102d37b8 => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d37bcu64) }
        0x102d37bc => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d37c0u64) }
        0x102d37c0 => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d37c4u64) }
        0x102d37c4 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d37c8u64) }
        0x102d37c8 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d37ccu64) }
        0x102d37cc => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d37d0u64) }
        0x102d37d0 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d37d4u64) }
        0x102d37d4 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d37d8u64) }
        0x102d37d8 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d37dcu64) }
        0x102d37dc => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d37e0u64) }
        0x102d37e0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d37e4u64) }
        0x102d37e4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d37e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d37e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d37e8u64) }
        0x102d37e8 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d37ecu64) }
        0x102d37ec => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d37f0u64) }
        0x102d37f0 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d37f4u64) }
        0x102d37f4 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d37f8u64) }
        0x102d37f8 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d37fcu64) }
        0x102d37fc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3800u64).wrapping_add((imm * 4) as u64) } else { 0x102d3800u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3800u64) }
        0x102d3800 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3804u64) } }
        0x102d3804 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3808u64) }
        0x102d3808 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d380cu64) }
        0x102d380c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3810u64).wrapping_add((imm * 4) as u64) } else { 0x102d3810u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3810u64) }
        0x102d3810 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3814u64) }
        0x102d3814 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3818u64) }
        0x102d3818 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d381cu64) }
        0x102d381c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3820u64) }
        0x102d3820 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3824u64) }
        0x102d3824 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3828u64).wrapping_add((imm * 4) as u64) } else { 0x102d3828u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3828u64) }
        0x102d3828 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d382cu64) } }
        0x102d382c => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3830u64) }
        0x102d3830 => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3834u64) }
        0x102d3834 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3838u64) }
        0x102d3838 => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d383cu64) }
        0x102d383c => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3840u64) }
        0x102d3840 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3844u64) }
        0x102d3844 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3848u64) }
        0x102d3848 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d384cu64) }
        0x102d384c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3850u64) }
        0x102d3850 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3854u64) }
        0x102d3854 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3858u64) }
        0x102d3858 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d385cu64) }
        0x102d385c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3860u64).wrapping_add((imm * 4) as u64) } else { 0x102d3860u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3860u64) }
        0x102d3860 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3864u64) }
        0x102d3864 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3868u64) }
        0x102d3868 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d386cu64) }
        0x102d386c => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3870u64) }
        0x102d3870 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3874u64) }
        0x102d3874 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3878u64).wrapping_add((imm * 4) as u64) } else { 0x102d3878u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3878u64) }
        0x102d3878 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d387cu64) } }
        0x102d387c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3880u64) }
        0x102d3880 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3884u64) }
        0x102d3884 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3888u64).wrapping_add((imm * 4) as u64) } else { 0x102d3888u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3888u64) }
        0x102d3888 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d388cu64) }
        0x102d388c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3890u64) }
        0x102d3890 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3894u64) }
        0x102d3894 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3898u64) }
        0x102d3898 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d389cu64) }
        0x102d389c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d38a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d38a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d38a0u64) }
        0x102d38a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d38a4u64) } }
        0x102d38a4 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d38a8u64) }
        0x102d38a8 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d38acu64) }
        0x102d38ac => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d38b0u64) }
        0x102d38b0 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d38b4u64) }
        0x102d38b4 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d38b8u64) }
        0x102d38b8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d38bcu64) }
        0x102d38bc => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d38c0u64) }
        0x102d38c0 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d38c4u64) }
        0x102d38c4 => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d38c8u64) }
        0x102d38c8 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d38ccu64) }
        0x102d38cc => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d38d0u64) }
        0x102d38d0 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d38d4u64) }
        0x102d38d4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d38d8u64) }
        0x102d38d8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d38dcu64) }
        0x102d38dc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d38e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d38e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d38e0u64) }
        0x102d38e0 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d38e4u64) }
        0x102d38e4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d38e8u64) }
        0x102d38e8 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d38ecu64) }
        0x102d38ec => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d38f0u64) }
        0x102d38f0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d38f4u64) }
        0x102d38f4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d38f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d38f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d38f8u64) }
        0x102d38f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d38fcu64) } }
        0x102d38fc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3900u64) }
        0x102d3900 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3904u64) }
        0x102d3904 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3908u64).wrapping_add((imm * 4) as u64) } else { 0x102d3908u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3908u64) }
        0x102d3908 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d390cu64) }
        0x102d390c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3910u64) }
        0x102d3910 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3914u64) }
        0x102d3914 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3918u64) }
        0x102d3918 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d391cu64) }
        0x102d391c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3920u64).wrapping_add((imm * 4) as u64) } else { 0x102d3920u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3920u64) }
        0x102d3920 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3924u64) } }
        0x102d3924 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3928u64) }
        0x102d3928 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d392cu64) }
        0x102d392c => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3930u64) }
        0x102d3930 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3934u64) }
        0x102d3934 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3938u64) }
        0x102d3938 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d393cu64) }
        0x102d393c => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3940u64) }
        0x102d3940 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3944u64) }
        0x102d3944 => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3948u64) }
        0x102d3948 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d394cu64) }
        0x102d394c => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3950u64) }
        0x102d3950 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3954u64) }
        0x102d3954 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3958u64) }
        0x102d3958 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d395cu64) }
        0x102d395c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3960u64).wrapping_add((imm * 4) as u64) } else { 0x102d3960u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3960u64) }
        0x102d3960 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3964u64) }
        0x102d3964 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3968u64) }
        0x102d3968 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d396cu64) }
        0x102d396c => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3970u64) }
        0x102d3970 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3974u64) }
        0x102d3974 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3978u64).wrapping_add((imm * 4) as u64) } else { 0x102d3978u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3978u64) }
        0x102d3978 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d397cu64) } }
        0x102d397c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3980u64) }
        0x102d3980 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3984u64) }
        0x102d3984 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3988u64).wrapping_add((imm * 4) as u64) } else { 0x102d3988u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3988u64) }
        0x102d3988 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d398cu64) }
        0x102d398c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3990u64) }
        0x102d3990 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3994u64) }
        0x102d3994 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3998u64) }
        0x102d3998 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d399cu64) }
        0x102d399c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d39a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d39a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d39a0u64) }
        0x102d39a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d39a4u64) } }
        0x102d39a4 => { vm.decode(0x2124101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4452i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d39a8u64) }
        0x102d39a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d39acu64) } }
        0x102d39ac => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d39b0u64) }
        0x102d39b0 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d39b4u64) }
        0x102d39b4 => { vm.decode(0xaa01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d39b8u64) }
        0x102d39b8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d39bcu64) }
        0x102d39bc => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d39c0u64) }
        0x102d39c0 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d39c4u64) }
        0x102d39c4 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d39c8u64) }
        0x102d39c8 => { vm.decode(0x408a632fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d39ccu64) }
        0x102d39cc => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d39d0u64) }
        0x102d39d0 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d39d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d39d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d39d4u64) }
        0x102d39d4 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d39d8u64) }
        0x102d39d8 => { vm.decode(0x8824501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6600i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d39dcu64) }
        0x102d39e0 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d39e4u64) }
        0x102d39e4 => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d39e8u64) }
        0x102d39e8 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d39ecu64) }
        0x102d39ec => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d39f0u64) }
        0x102d39f0 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d39f4u64) }
        0x102d39f4 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d39f8u64) }
        0x102d39f8 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d39fcu64) }
        0x102d39fc => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3a00u64) }
        0x102d3a00 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a04u64) }
        0x102d3a04 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a08u64) }
        0x102d3a08 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3a0cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3a0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3a0cu64) }
        0x102d3a0c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3a10u64) } }
        0x102d3a10 => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d3a14u64) }
        0x102d3a14 => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3a18u64) }
        0x102d3a18 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3a1cu64) }
        0x102d3a1c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3a20u64) }
        0x102d3a20 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a24u64) }
        0x102d3a24 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a28u64) }
        0x102d3a28 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3a2cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3a2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3a2cu64) }
        0x102d3a2c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3a30u64) }
        0x102d3a30 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a34u64) }
        0x102d3a34 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3a38u64) }
        0x102d3a38 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3a3cu64) }
        0x102d3a3c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a40u64) }
        0x102d3a40 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3a44u64).wrapping_add((imm * 4) as u64) } else { 0x102d3a44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3a44u64) }
        0x102d3a44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3a48u64) } }
        0x102d3a48 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a4cu64) }
        0x102d3a4c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a50u64) }
        0x102d3a50 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3a54u64).wrapping_add((imm * 4) as u64) } else { 0x102d3a54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3a54u64) }
        0x102d3a54 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3a58u64) }
        0x102d3a58 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a5cu64) }
        0x102d3a5c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3a60u64) }
        0x102d3a60 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3a64u64) }
        0x102d3a64 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a68u64) }
        0x102d3a68 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3a6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3a6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3a6cu64) }
        0x102d3a6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3a70u64) } }
        0x102d3a70 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3a74u64) }
        0x102d3a74 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3a78u64) }
        0x102d3a78 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3a7cu64) }
        0x102d3a7c => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3a80u64) }
        0x102d3a80 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3a84u64) }
        0x102d3a84 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3a88u64) }
        0x102d3a88 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3a8cu64) }
        0x102d3a8c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3a90u64) }
        0x102d3a90 => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3a94u64) }
        0x102d3a94 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3a98u64) }
        0x102d3a98 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3a9cu64) }
        0x102d3a9c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3aa0u64) }
        0x102d3aa0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3aa4u64) }
        0x102d3aa4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3aa8u64) }
        0x102d3aa8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3aacu64).wrapping_add((imm * 4) as u64) } else { 0x102d3aacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3aacu64) }
        0x102d3aac => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3ab0u64) }
        0x102d3ab0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ab4u64) }
        0x102d3ab4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ab8u64) }
        0x102d3ab8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3abcu64) }
        0x102d3abc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ac0u64) }
        0x102d3ac0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3ac4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3ac4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3ac4u64) }
        0x102d3ac4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3ac8u64) } }
        0x102d3ac8 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3accu64) }
        0x102d3acc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ad0u64) }
        0x102d3ad0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3ad4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3ad4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3ad4u64) }
        0x102d3ad4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3ad8u64) }
        0x102d3ad8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3adcu64) }
        0x102d3adc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ae0u64) }
        0x102d3ae0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3ae4u64) }
        0x102d3ae4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ae8u64) }
        0x102d3ae8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3aecu64).wrapping_add((imm * 4) as u64) } else { 0x102d3aecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3aecu64) }
        0x102d3aec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3af0u64) } }
        0x102d3af0 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3af4u64) }
        0x102d3af4 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3af8u64) }
        0x102d3af8 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3afcu64) }
        0x102d3afc => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3b00u64) }
        0x102d3b00 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b04u64) }
        0x102d3b04 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b08u64) }
        0x102d3b08 => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b0cu64) }
        0x102d3b0c => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b10u64) }
        0x102d3b10 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3b14u64) }
        0x102d3b14 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3b18u64) }
        0x102d3b18 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b1cu64) }
        0x102d3b1c => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b20u64) }
        0x102d3b20 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3b24u64) }
        0x102d3b24 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3b28u64) }
        0x102d3b28 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b2cu64) }
        0x102d3b2c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b30u64) }
        0x102d3b30 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3b34u64).wrapping_add((imm * 4) as u64) } else { 0x102d3b34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3b34u64) }
        0x102d3b34 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3b38u64) }
        0x102d3b38 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b3cu64) }
        0x102d3b3c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b40u64) }
        0x102d3b40 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3b44u64) }
        0x102d3b44 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b48u64) }
        0x102d3b48 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3b4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3b4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3b4cu64) }
        0x102d3b4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3b50u64) } }
        0x102d3b50 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b54u64) }
        0x102d3b54 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b58u64) }
        0x102d3b58 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3b5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3b5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3b5cu64) }
        0x102d3b5c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3b60u64) }
        0x102d3b60 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b64u64) }
        0x102d3b64 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b68u64) }
        0x102d3b68 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3b6cu64) }
        0x102d3b6c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b70u64) }
        0x102d3b70 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3b74u64).wrapping_add((imm * 4) as u64) } else { 0x102d3b74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3b74u64) }
        0x102d3b74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3b78u64) } }
        0x102d3b78 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b7cu64) }
        0x102d3b7c => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b80u64) }
        0x102d3b80 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b84u64) }
        0x102d3b84 => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3b88u64) }
        0x102d3b88 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b8cu64) }
        0x102d3b8c => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3b90u64) }
        0x102d3b90 => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3b94u64) }
        0x102d3b94 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3b98u64) }
        0x102d3b98 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3b9cu64) }
        0x102d3b9c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3ba0u64) }
        0x102d3ba0 => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ba4u64) }
        0x102d3ba4 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ba8u64) }
        0x102d3ba8 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3bacu64) }
        0x102d3bac => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3bb0u64) }
        0x102d3bb0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bb4u64) }
        0x102d3bb4 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bb8u64) }
        0x102d3bb8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3bbcu64).wrapping_add((imm * 4) as u64) } else { 0x102d3bbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3bbcu64) }
        0x102d3bbc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3bc0u64) }
        0x102d3bc0 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bc4u64) }
        0x102d3bc4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3bc8u64) }
        0x102d3bc8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3bccu64) }
        0x102d3bcc => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bd0u64) }
        0x102d3bd0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3bd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3bd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3bd4u64) }
        0x102d3bd4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3bd8u64) } }
        0x102d3bd8 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bdcu64) }
        0x102d3bdc => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3be0u64) }
        0x102d3be0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3be4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3be4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3be4u64) }
        0x102d3be4 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3be8u64) }
        0x102d3be8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3becu64) }
        0x102d3bec => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3bf0u64) }
        0x102d3bf0 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3bf4u64) }
        0x102d3bf4 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3bf8u64) }
        0x102d3bf8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3bfcu64).wrapping_add((imm * 4) as u64) } else { 0x102d3bfcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3bfcu64) }
        0x102d3bfc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3c00u64) } }
        0x102d3c00 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c04u64) }
        0x102d3c04 => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3c08u64) }
        0x102d3c08 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c0cu64) }
        0x102d3c0c => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3c10u64) }
        0x102d3c10 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c14u64) }
        0x102d3c14 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c18u64) }
        0x102d3c18 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3c1cu64) }
        0x102d3c1c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3c20u64) }
        0x102d3c20 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3c24u64) }
        0x102d3c24 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3c28u64) }
        0x102d3c28 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c2cu64) }
        0x102d3c2c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c30u64) }
        0x102d3c30 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3c34u64).wrapping_add((imm * 4) as u64) } else { 0x102d3c34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3c34u64) }
        0x102d3c34 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3c38u64) }
        0x102d3c38 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c3cu64) }
        0x102d3c3c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3c40u64) }
        0x102d3c40 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3c44u64) }
        0x102d3c44 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c48u64) }
        0x102d3c48 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3c4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3c4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3c4cu64) }
        0x102d3c4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3c50u64) } }
        0x102d3c50 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c54u64) }
        0x102d3c54 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c58u64) }
        0x102d3c58 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3c5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3c5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3c5cu64) }
        0x102d3c5c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3c60u64) }
        0x102d3c60 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c64u64) }
        0x102d3c64 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3c68u64) }
        0x102d3c68 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3c6cu64) }
        0x102d3c6c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3c70u64) }
        0x102d3c70 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3c74u64).wrapping_add((imm * 4) as u64) } else { 0x102d3c74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3c74u64) }
        0x102d3c74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3c78u64) } }
        0x102d3c78 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c7cu64) }
        0x102d3c7c => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3c80u64) }
        0x102d3c80 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c84u64) }
        0x102d3c84 => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3c88u64) }
        0x102d3c88 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c8cu64) }
        0x102d3c8c => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3c90u64) }
        0x102d3c90 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3c94u64) }
        0x102d3c94 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3c98u64) }
        0x102d3c98 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3c9cu64) }
        0x102d3c9c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3ca0u64) }
        0x102d3ca0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ca4u64) }
        0x102d3ca4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ca8u64) }
        0x102d3ca8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3cacu64).wrapping_add((imm * 4) as u64) } else { 0x102d3cacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3cacu64) }
        0x102d3cac => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3cb0u64) }
        0x102d3cb0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3cb4u64) }
        0x102d3cb4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3cb8u64) }
        0x102d3cb8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3cbcu64) }
        0x102d3cbc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3cc0u64) }
        0x102d3cc0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3cc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3cc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3cc4u64) }
        0x102d3cc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3cc8u64) } }
        0x102d3cc8 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3cccu64) }
        0x102d3ccc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3cd0u64) }
        0x102d3cd0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3cd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3cd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3cd4u64) }
        0x102d3cd4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3cd8u64) }
        0x102d3cd8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3cdcu64) }
        0x102d3cdc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ce0u64) }
        0x102d3ce0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3ce4u64) }
        0x102d3ce4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ce8u64) }
        0x102d3ce8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3cecu64).wrapping_add((imm * 4) as u64) } else { 0x102d3cecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3cecu64) }
        0x102d3cec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3cf0u64) } }
        0x102d3cf0 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3cf4u64) }
        0x102d3cf4 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3cf8u64) }
        0x102d3cf8 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3cfcu64) }
        0x102d3cfc => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3d00u64) }
        0x102d3d00 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d04u64) }
        0x102d3d04 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d08u64) }
        0x102d3d08 => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d0cu64) }
        0x102d3d0c => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3d10u64) }
        0x102d3d10 => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3d14u64) }
        0x102d3d14 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3d18u64) }
        0x102d3d18 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3d1cu64) }
        0x102d3d1c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3d20u64) }
        0x102d3d20 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d24u64) }
        0x102d3d24 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d28u64) }
        0x102d3d28 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3d2cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3d2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3d2cu64) }
        0x102d3d2c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3d30u64) }
        0x102d3d30 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d34u64) }
        0x102d3d34 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3d38u64) }
        0x102d3d38 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3d3cu64) }
        0x102d3d3c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d40u64) }
        0x102d3d40 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3d44u64).wrapping_add((imm * 4) as u64) } else { 0x102d3d44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3d44u64) }
        0x102d3d44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3d48u64) } }
        0x102d3d48 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d4cu64) }
        0x102d3d4c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d50u64) }
        0x102d3d50 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3d54u64).wrapping_add((imm * 4) as u64) } else { 0x102d3d54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3d54u64) }
        0x102d3d54 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3d58u64) }
        0x102d3d58 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d5cu64) }
        0x102d3d5c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3d60u64) }
        0x102d3d60 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3d64u64) }
        0x102d3d64 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d68u64) }
        0x102d3d68 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3d6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3d6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3d6cu64) }
        0x102d3d6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3d70u64) } }
        0x102d3d70 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d74u64) }
        0x102d3d74 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3d78u64) }
        0x102d3d78 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d7cu64) }
        0x102d3d7c => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3d80u64) }
        0x102d3d80 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d84u64) }
        0x102d3d84 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3d88u64) }
        0x102d3d88 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3d8cu64) }
        0x102d3d8c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3d90u64) }
        0x102d3d90 => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3d94u64) }
        0x102d3d94 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3d98u64) }
        0x102d3d98 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3d9cu64) }
        0x102d3d9c => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3da0u64) }
        0x102d3da0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3da4u64) }
        0x102d3da4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3da8u64) }
        0x102d3da8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3dacu64).wrapping_add((imm * 4) as u64) } else { 0x102d3dacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3dacu64) }
        0x102d3dac => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3db0u64) }
        0x102d3db0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3db4u64) }
        0x102d3db4 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3db8u64) }
        0x102d3db8 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3dbcu64) }
        0x102d3dbc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3dc0u64) }
        0x102d3dc0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3dc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3dc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3dc4u64) }
        0x102d3dc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3dc8u64) } }
        0x102d3dc8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3dccu64) }
        0x102d3dcc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3dd0u64) }
        0x102d3dd0 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3dd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3dd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3dd4u64) }
        0x102d3dd4 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3dd8u64) }
        0x102d3dd8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ddcu64) }
        0x102d3ddc => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3de0u64) }
        0x102d3de0 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3de4u64) }
        0x102d3de4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3de8u64) }
        0x102d3de8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3decu64).wrapping_add((imm * 4) as u64) } else { 0x102d3decu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3decu64) }
        0x102d3dec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3df0u64) } }
        0x102d3df0 => { vm.decode(0x8504141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5552i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d3df4u64) }
        0x102d3df4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3df8u64) } }
        0x102d3df8 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d3dfcu64) }
        0x102d3dfc => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3e00u64) }
        0x102d3e00 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3e04u64) }
        0x102d3e04 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3e08u64) }
        0x102d3e08 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3e0cu64) }
        0x102d3e0c => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3e10u64) }
        0x102d3e10 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3e14u64) }
        0x102d3e14 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e18u64) }
        0x102d3e18 => { vm.decode(0x42804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3e1cu64) }
        0x102d3e1c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e20u64) }
        0x102d3e20 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3e24u64).wrapping_add((imm * 4) as u64) } else { 0x102d3e24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3e24u64) }
        0x102d3e24 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3e28u64) }
        0x102d3e28 => { vm.decode(0xc84441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7704i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d3e2cu64) }
        0x102d3e30 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3e34u64) }
        0x102d3e34 => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3e38u64) }
        0x102d3e38 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3e3cu64) }
        0x102d3e3c => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3e40u64) }
        0x102d3e40 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3e44u64) }
        0x102d3e44 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3e48u64) }
        0x102d3e48 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3e4cu64) }
        0x102d3e4c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3e50u64) }
        0x102d3e50 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e54u64) }
        0x102d3e54 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e58u64) }
        0x102d3e58 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3e5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3e5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3e5cu64) }
        0x102d3e5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3e60u64) } }
        0x102d3e60 => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d3e64u64) }
        0x102d3e64 => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3e68u64) }
        0x102d3e68 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3e6cu64) }
        0x102d3e6c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3e70u64) }
        0x102d3e70 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e74u64) }
        0x102d3e74 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e78u64) }
        0x102d3e78 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3e7cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3e7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3e7cu64) }
        0x102d3e7c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3e80u64) }
        0x102d3e80 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e84u64) }
        0x102d3e84 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3e88u64) }
        0x102d3e88 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3e8cu64) }
        0x102d3e8c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e90u64) }
        0x102d3e90 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3e94u64).wrapping_add((imm * 4) as u64) } else { 0x102d3e94u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3e94u64) }
        0x102d3e94 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3e98u64) } }
        0x102d3e98 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3e9cu64) }
        0x102d3e9c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ea0u64) }
        0x102d3ea0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3ea4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3ea4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3ea4u64) }
        0x102d3ea4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3ea8u64) }
        0x102d3ea8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3eacu64) }
        0x102d3eac => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3eb0u64) }
        0x102d3eb0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3eb4u64) }
        0x102d3eb4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3eb8u64) }
        0x102d3eb8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3ebcu64).wrapping_add((imm * 4) as u64) } else { 0x102d3ebcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3ebcu64) }
        0x102d3ebc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3ec0u64) } }
        0x102d3ec0 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3ec4u64) }
        0x102d3ec4 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ec8u64) }
        0x102d3ec8 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3eccu64) }
        0x102d3ecc => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3ed0u64) }
        0x102d3ed0 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3ed4u64) }
        0x102d3ed4 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3ed8u64) }
        0x102d3ed8 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3edcu64) }
        0x102d3edc => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3ee0u64) }
        0x102d3ee0 => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ee4u64) }
        0x102d3ee4 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ee8u64) }
        0x102d3ee8 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3eecu64) }
        0x102d3eec => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3ef0u64) }
        0x102d3ef0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ef4u64) }
        0x102d3ef4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ef8u64) }
        0x102d3ef8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3efcu64).wrapping_add((imm * 4) as u64) } else { 0x102d3efcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3efcu64) }
        0x102d3efc => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3f00u64) }
        0x102d3f00 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f04u64) }
        0x102d3f04 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f08u64) }
        0x102d3f08 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3f0cu64) }
        0x102d3f0c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f10u64) }
        0x102d3f10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3f14u64).wrapping_add((imm * 4) as u64) } else { 0x102d3f14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3f14u64) }
        0x102d3f14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3f18u64) } }
        0x102d3f18 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f1cu64) }
        0x102d3f1c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f20u64) }
        0x102d3f20 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3f24u64).wrapping_add((imm * 4) as u64) } else { 0x102d3f24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3f24u64) }
        0x102d3f24 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3f28u64) }
        0x102d3f28 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f2cu64) }
        0x102d3f2c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f30u64) }
        0x102d3f30 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3f34u64) }
        0x102d3f34 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f38u64) }
        0x102d3f38 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3f3cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3f3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3f3cu64) }
        0x102d3f3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3f40u64) } }
        0x102d3f40 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3f44u64) }
        0x102d3f44 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f48u64) }
        0x102d3f48 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3f4cu64) }
        0x102d3f4c => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3f50u64) }
        0x102d3f50 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3f54u64) }
        0x102d3f54 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3f58u64) }
        0x102d3f58 => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f5cu64) }
        0x102d3f5c => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f60u64) }
        0x102d3f60 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3f64u64) }
        0x102d3f64 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3f68u64) }
        0x102d3f68 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f6cu64) }
        0x102d3f6c => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f70u64) }
        0x102d3f70 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3f74u64) }
        0x102d3f74 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3f78u64) }
        0x102d3f78 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f7cu64) }
        0x102d3f7c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f80u64) }
        0x102d3f80 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3f84u64).wrapping_add((imm * 4) as u64) } else { 0x102d3f84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3f84u64) }
        0x102d3f84 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3f88u64) }
        0x102d3f88 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f8cu64) }
        0x102d3f8c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3f90u64) }
        0x102d3f90 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3f94u64) }
        0x102d3f94 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3f98u64) }
        0x102d3f98 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3f9cu64).wrapping_add((imm * 4) as u64) } else { 0x102d3f9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3f9cu64) }
        0x102d3f9c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3fa0u64) } }
        0x102d3fa0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3fa4u64) }
        0x102d3fa4 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3fa8u64) }
        0x102d3fa8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3facu64).wrapping_add((imm * 4) as u64) } else { 0x102d3facu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3facu64) }
        0x102d3fac => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3fb0u64) }
        0x102d3fb0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3fb4u64) }
        0x102d3fb4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3fb8u64) }
        0x102d3fb8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3fbcu64) }
        0x102d3fbc => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3fc0u64) }
        0x102d3fc0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3fc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d3fc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3fc4u64) }
        0x102d3fc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3fc8u64) } }
        0x102d3fc8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3fccu64) }
        0x102d3fcc => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3fd0u64) }
        0x102d3fd0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3fd4u64) }
        0x102d3fd4 => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3fd8u64) }
        0x102d3fd8 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3fdcu64) }
        0x102d3fdc => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3fe0u64) }
        0x102d3fe0 => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3fe4u64) }
        0x102d3fe4 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3fe8u64) }
        0x102d3fe8 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3fecu64) }
        0x102d3fec => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3ff0u64) }
        0x102d3ff0 => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3ff4u64) }
        0x102d3ff4 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3ff8u64) }
        0x102d3ff8 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3ffcu64) }
        0x102d3ffc => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4000u64) }
        0x102d4000 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4004u64) }
        0x102d4004 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4008u64) }
        0x102d4008 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d400cu64).wrapping_add((imm * 4) as u64) } else { 0x102d400cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d400cu64) }
        0x102d400c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4010u64) }
        0x102d4010 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4014u64) }
        0x102d4014 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4018u64) }
        0x102d4018 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d401cu64) }
        0x102d401c => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4020u64) }
        0x102d4020 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4024u64).wrapping_add((imm * 4) as u64) } else { 0x102d4024u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4024u64) }
        0x102d4024 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4028u64) } }
        0x102d4028 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d402cu64) }
        0x102d402c => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4030u64) }
        0x102d4030 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4034u64).wrapping_add((imm * 4) as u64) } else { 0x102d4034u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4034u64) }
        0x102d4034 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4038u64) }
        0x102d4038 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d403cu64) }
        0x102d403c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4040u64) }
        0x102d4040 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4044u64) }
        0x102d4044 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4048u64) }
        0x102d4048 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d404cu64).wrapping_add((imm * 4) as u64) } else { 0x102d404cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d404cu64) }
        0x102d404c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4050u64) } }
        0x102d4050 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4054u64) }
        0x102d4054 => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4058u64) }
        0x102d4058 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d405cu64) }
        0x102d405c => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4060u64) }
        0x102d4060 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4064u64) }
        0x102d4064 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4068u64) }
        0x102d4068 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d406cu64) }
        0x102d406c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4070u64) }
        0x102d4070 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4074u64) }
        0x102d4074 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4078u64) }
        0x102d4078 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d407cu64) }
        0x102d407c => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4080u64) }
        0x102d4080 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4084u64).wrapping_add((imm * 4) as u64) } else { 0x102d4084u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4084u64) }
        0x102d4084 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4088u64) }
        0x102d4088 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d408cu64) }
        0x102d408c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4090u64) }
        0x102d4090 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4094u64) }
        0x102d4094 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4098u64) }
        0x102d4098 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d409cu64).wrapping_add((imm * 4) as u64) } else { 0x102d409cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d409cu64) }
        0x102d409c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d40a0u64) } }
        0x102d40a0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d40a4u64) }
        0x102d40a4 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d40a8u64) }
        0x102d40a8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d40acu64).wrapping_add((imm * 4) as u64) } else { 0x102d40acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d40acu64) }
        0x102d40ac => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d40b0u64) }
        0x102d40b0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d40b4u64) }
        0x102d40b4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d40b8u64) }
        0x102d40b8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d40bcu64) }
        0x102d40bc => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d40c0u64) }
        0x102d40c0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d40c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d40c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d40c4u64) }
        0x102d40c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d40c8u64) } }
        0x102d40c8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d40ccu64) }
        0x102d40cc => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d40d0u64) }
        0x102d40d0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d40d4u64) }
        0x102d40d4 => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d40d8u64) }
        0x102d40d8 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d40dcu64) }
        0x102d40dc => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d40e0u64) }
        0x102d40e0 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d40e4u64) }
        0x102d40e4 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d40e8u64) }
        0x102d40e8 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d40ecu64) }
        0x102d40ec => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d40f0u64) }
        0x102d40f0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d40f4u64) }
        0x102d40f4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d40f8u64) }
        0x102d40f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d40fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d40fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d40fcu64) }
        0x102d40fc => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4100u64) }
        0x102d4100 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4104u64) }
        0x102d4104 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4108u64) }
        0x102d4108 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d410cu64) }
        0x102d410c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4110u64) }
        0x102d4110 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4114u64).wrapping_add((imm * 4) as u64) } else { 0x102d4114u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4114u64) }
        0x102d4114 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4118u64) } }
        0x102d4118 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d411cu64) }
        0x102d411c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4120u64) }
        0x102d4120 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4124u64).wrapping_add((imm * 4) as u64) } else { 0x102d4124u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4124u64) }
        0x102d4124 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4128u64) }
        0x102d4128 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d412cu64) }
        0x102d412c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4130u64) }
        0x102d4130 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4134u64) }
        0x102d4134 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4138u64) }
        0x102d4138 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d413cu64).wrapping_add((imm * 4) as u64) } else { 0x102d413cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d413cu64) }
        0x102d413c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4140u64) } }
        0x102d4140 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4144u64) }
        0x102d4144 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4148u64) }
        0x102d4148 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d414cu64) }
        0x102d414c => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4150u64) }
        0x102d4150 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4154u64) }
        0x102d4154 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4158u64) }
        0x102d4158 => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d415cu64) }
        0x102d415c => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4160u64) }
        0x102d4160 => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4164u64) }
        0x102d4164 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4168u64) }
        0x102d4168 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d416cu64) }
        0x102d416c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4170u64) }
        0x102d4170 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4174u64) }
        0x102d4174 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4178u64) }
        0x102d4178 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d417cu64).wrapping_add((imm * 4) as u64) } else { 0x102d417cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d417cu64) }
        0x102d417c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4180u64) }
        0x102d4180 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4184u64) }
        0x102d4184 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4188u64) }
        0x102d4188 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d418cu64) }
        0x102d418c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4190u64) }
        0x102d4190 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4194u64).wrapping_add((imm * 4) as u64) } else { 0x102d4194u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4194u64) }
        0x102d4194 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4198u64) } }
        0x102d4198 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d419cu64) }
        0x102d419c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d41a0u64) }
        0x102d41a0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d41a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d41a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d41a4u64) }
        0x102d41a4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d41a8u64) }
        0x102d41a8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d41acu64) }
        0x102d41ac => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d41b0u64) }
        0x102d41b0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d41b4u64) }
        0x102d41b4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d41b8u64) }
        0x102d41b8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d41bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d41bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d41bcu64) }
        0x102d41bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d41c0u64) } }
        0x102d41c0 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d41c4u64) }
        0x102d41c4 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d41c8u64) }
        0x102d41c8 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d41ccu64) }
        0x102d41cc => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d41d0u64) }
        0x102d41d0 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d41d4u64) }
        0x102d41d4 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d41d8u64) }
        0x102d41d8 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d41dcu64) }
        0x102d41dc => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d41e0u64) }
        0x102d41e0 => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d41e4u64) }
        0x102d41e4 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d41e8u64) }
        0x102d41e8 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d41ecu64) }
        0x102d41ec => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d41f0u64) }
        0x102d41f0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d41f4u64) }
        0x102d41f4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d41f8u64) }
        0x102d41f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d41fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d41fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d41fcu64) }
        0x102d41fc => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4200u64) }
        0x102d4200 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4204u64) }
        0x102d4204 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4208u64) }
        0x102d4208 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d420cu64) }
        0x102d420c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4210u64) }
        0x102d4210 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4214u64).wrapping_add((imm * 4) as u64) } else { 0x102d4214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4214u64) }
        0x102d4214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4218u64) } }
        0x102d4218 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d421cu64) }
        0x102d421c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4220u64) }
        0x102d4220 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4224u64).wrapping_add((imm * 4) as u64) } else { 0x102d4224u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4224u64) }
        0x102d4224 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4228u64) }
        0x102d4228 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d422cu64) }
        0x102d422c => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4230u64) }
        0x102d4230 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4234u64) }
        0x102d4234 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4238u64) }
        0x102d4238 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d423cu64).wrapping_add((imm * 4) as u64) } else { 0x102d423cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d423cu64) }
        0x102d423c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4240u64) } }
        0x102d4240 => { vm.decode(0x84401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6656i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d4244u64) }
        0x102d4244 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4248u64) } }
        0x102d4248 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d424cu64) }
        0x102d424c => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4250u64) }
        0x102d4250 => { vm.decode(0x880184f5u32); let d = 2i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4254u64) }
        0x102d4254 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4258u64) }
        0x102d4258 => { vm.decode(0x2a88432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d425cu64) }
        0x102d425c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4260u64) }
        0x102d4260 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4264u64) }
        0x102d4264 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4268u64) }
        0x102d4268 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d426cu64) }
        0x102d426c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4270u64).wrapping_add((imm * 4) as u64) } else { 0x102d4270u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4270u64) }
        0x102d4270 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4274u64) }
        0x102d4274 => { vm.decode(0x21a0011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8804i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d4278u64) }
        0x102d427c => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4280u64) }
        0x102d4280 => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4284u64) }
        0x102d4284 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4288u64) }
        0x102d4288 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d428cu64) }
        0x102d428c => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4290u64) }
        0x102d4290 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4294u64) }
        0x102d4294 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4298u64) }
        0x102d4298 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d429cu64) }
        0x102d429c => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d42a0u64) }
        0x102d42a0 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d42a4u64) }
        0x102d42a4 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d42a8u64).wrapping_add((imm * 4) as u64) } else { 0x102d42a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d42a8u64) }
        0x102d42a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d42acu64) } }
        0x102d42ac => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d42b0u64) }
        0x102d42b0 => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d42b4u64) }
        0x102d42b4 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d42b8u64) }
        0x102d42b8 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d42bcu64) }
        0x102d42bc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d42c0u64) }
        0x102d42c0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d42c4u64) }
        0x102d42c4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d42c8u64).wrapping_add((imm * 4) as u64) } else { 0x102d42c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d42c8u64) }
        0x102d42c8 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d42ccu64) }
        0x102d42cc => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d42d0u64) }
        0x102d42d0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d42d4u64) }
        0x102d42d4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d42d8u64) }
        0x102d42d8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d42dcu64) }
        0x102d42dc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d42e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d42e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d42e0u64) }
        0x102d42e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d42e4u64) } }
        0x102d42e4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d42e8u64) }
        0x102d42e8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d42ecu64) }
        0x102d42ec => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d42f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d42f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d42f0u64) }
        0x102d42f0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d42f4u64) }
        0x102d42f4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d42f8u64) }
        0x102d42f8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d42fcu64) }
        0x102d42fc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4300u64) }
        0x102d4300 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4304u64) }
        0x102d4304 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4308u64).wrapping_add((imm * 4) as u64) } else { 0x102d4308u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4308u64) }
        0x102d4308 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d430cu64) } }
        0x102d430c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4310u64) }
        0x102d4310 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4314u64) }
        0x102d4314 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4318u64) }
        0x102d4318 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d431cu64) }
        0x102d431c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4320u64) }
        0x102d4320 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4324u64) }
        0x102d4324 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4328u64) }
        0x102d4328 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d432cu64) }
        0x102d432c => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4330u64) }
        0x102d4330 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4334u64) }
        0x102d4334 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4338u64) }
        0x102d4338 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d433cu64) }
        0x102d433c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4340u64) }
        0x102d4340 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4344u64) }
        0x102d4344 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4348u64).wrapping_add((imm * 4) as u64) } else { 0x102d4348u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4348u64) }
        0x102d4348 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d434cu64) }
        0x102d434c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4350u64) }
        0x102d4350 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4354u64) }
        0x102d4354 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4358u64) }
        0x102d4358 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d435cu64) }
        0x102d435c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4360u64).wrapping_add((imm * 4) as u64) } else { 0x102d4360u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4360u64) }
        0x102d4360 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4364u64) } }
        0x102d4364 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4368u64) }
        0x102d4368 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d436cu64) }
        0x102d436c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4370u64).wrapping_add((imm * 4) as u64) } else { 0x102d4370u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4370u64) }
        0x102d4370 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4374u64) }
        0x102d4374 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4378u64) }
        0x102d4378 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d437cu64) }
        0x102d437c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4380u64) }
        0x102d4380 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4384u64) }
        0x102d4384 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4388u64).wrapping_add((imm * 4) as u64) } else { 0x102d4388u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4388u64) }
        0x102d4388 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d438cu64) } }
        0x102d438c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4390u64) }
        0x102d4390 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4394u64) }
        0x102d4394 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4398u64) }
        0x102d4398 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d439cu64) }
        0x102d439c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d43a0u64) }
        0x102d43a0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d43a4u64) }
        0x102d43a4 => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d43a8u64) }
        0x102d43a8 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d43acu64) }
        0x102d43ac => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d43b0u64) }
        0x102d43b0 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d43b4u64) }
        0x102d43b4 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d43b8u64) }
        0x102d43b8 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d43bcu64) }
        0x102d43bc => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d43c0u64) }
        0x102d43c0 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d43c4u64) }
        0x102d43c4 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d43c8u64) }
        0x102d43c8 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d43ccu64) }
        0x102d43cc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d43d0u64).wrapping_add((imm * 4) as u64) } else { 0x102d43d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d43d0u64) }
        0x102d43d0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d43d4u64) }
        0x102d43d4 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d43d8u64) }
        0x102d43d8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d43dcu64) }
        0x102d43dc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d43e0u64) }
        0x102d43e0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d43e4u64) }
        0x102d43e4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d43e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d43e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d43e8u64) }
        0x102d43e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d43ecu64) } }
        0x102d43ec => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d43f0u64) }
        0x102d43f0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d43f4u64) }
        0x102d43f4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d43f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d43f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d43f8u64) }
        0x102d43f8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d43fcu64) }
        0x102d43fc => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4400u64) }
        0x102d4400 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4404u64) }
        0x102d4404 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4408u64) }
        0x102d4408 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d440cu64) }
        0x102d440c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4410u64).wrapping_add((imm * 4) as u64) } else { 0x102d4410u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4410u64) }
        0x102d4410 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4414u64) } }
        0x102d4414 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4418u64) }
        0x102d4418 => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d441cu64) }
        0x102d441c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4420u64) }
        0x102d4420 => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4424u64) }
        0x102d4424 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4428u64) }
        0x102d4428 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d442cu64) }
        0x102d442c => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4430u64) }
        0x102d4430 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4434u64) }
        0x102d4434 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4438u64) }
        0x102d4438 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d443cu64) }
        0x102d443c => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4440u64) }
        0x102d4440 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4444u64) }
        0x102d4444 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4448u64) }
        0x102d4448 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d444cu64) }
        0x102d444c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4450u64) }
        0x102d4450 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4454u64) }
        0x102d4454 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4458u64).wrapping_add((imm * 4) as u64) } else { 0x102d4458u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4458u64) }
        0x102d4458 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d445cu64) }
        0x102d445c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4460u64) }
        0x102d4460 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4464u64) }
        0x102d4464 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4468u64) }
        0x102d4468 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d446cu64) }
        0x102d446c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4470u64).wrapping_add((imm * 4) as u64) } else { 0x102d4470u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4470u64) }
        0x102d4470 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4474u64) } }
        0x102d4474 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4478u64) }
        0x102d4478 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d447cu64) }
        0x102d447c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4480u64).wrapping_add((imm * 4) as u64) } else { 0x102d4480u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4480u64) }
        0x102d4480 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4484u64) }
        0x102d4484 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4488u64) }
        0x102d4488 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d448cu64) }
        0x102d448c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4490u64) }
        0x102d4490 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4494u64) }
        0x102d4494 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4498u64).wrapping_add((imm * 4) as u64) } else { 0x102d4498u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4498u64) }
        0x102d4498 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d449cu64) } }
        0x102d449c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d44a0u64) }
        0x102d44a0 => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d44a4u64) }
        0x102d44a4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d44a8u64) }
        0x102d44a8 => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d44acu64) }
        0x102d44ac => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d44b0u64) }
        0x102d44b0 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d44b4u64) }
        0x102d44b4 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d44b8u64) }
        0x102d44b8 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d44bcu64) }
        0x102d44bc => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d44c0u64) }
        0x102d44c0 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d44c4u64) }
        0x102d44c4 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d44c8u64) }
        0x102d44c8 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d44ccu64) }
        0x102d44cc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d44d0u64).wrapping_add((imm * 4) as u64) } else { 0x102d44d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d44d0u64) }
        0x102d44d0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d44d4u64) }
        0x102d44d4 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d44d8u64) }
        0x102d44d8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d44dcu64) }
        0x102d44dc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d44e0u64) }
        0x102d44e0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d44e4u64) }
        0x102d44e4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d44e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d44e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d44e8u64) }
        0x102d44e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d44ecu64) } }
        0x102d44ec => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d44f0u64) }
        0x102d44f0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d44f4u64) }
        0x102d44f4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d44f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d44f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d44f8u64) }
        0x102d44f8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d44fcu64) }
        0x102d44fc => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4500u64) }
        0x102d4500 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4504u64) }
        0x102d4504 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4508u64) }
        0x102d4508 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d450cu64) }
        0x102d450c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4510u64).wrapping_add((imm * 4) as u64) } else { 0x102d4510u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4510u64) }
        0x102d4510 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4514u64) } }
        0x102d4514 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4518u64) }
        0x102d4518 => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d451cu64) }
        0x102d451c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4520u64) }
        0x102d4520 => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4524u64) }
        0x102d4524 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4528u64) }
        0x102d4528 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d452cu64) }
        0x102d452c => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4530u64) }
        0x102d4530 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4534u64) }
        0x102d4534 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4538u64) }
        0x102d4538 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d453cu64) }
        0x102d453c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4540u64) }
        0x102d4540 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4544u64) }
        0x102d4544 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4548u64).wrapping_add((imm * 4) as u64) } else { 0x102d4548u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4548u64) }
        0x102d4548 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d454cu64) }
        0x102d454c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4550u64) }
        0x102d4550 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4554u64) }
        0x102d4554 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4558u64) }
        0x102d4558 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d455cu64) }
        0x102d455c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4560u64).wrapping_add((imm * 4) as u64) } else { 0x102d4560u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4560u64) }
        0x102d4560 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4564u64) } }
        0x102d4564 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4568u64) }
        0x102d4568 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d456cu64) }
        0x102d456c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4570u64).wrapping_add((imm * 4) as u64) } else { 0x102d4570u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4570u64) }
        0x102d4570 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4574u64) }
        0x102d4574 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4578u64) }
        0x102d4578 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d457cu64) }
        0x102d457c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4580u64) }
        0x102d4580 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4584u64) }
        0x102d4584 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4588u64).wrapping_add((imm * 4) as u64) } else { 0x102d4588u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4588u64) }
        0x102d4588 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d458cu64) } }
        0x102d458c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4590u64) }
        0x102d4590 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4594u64) }
        0x102d4594 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4598u64) }
        0x102d4598 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d459cu64) }
        0x102d459c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d45a0u64) }
        0x102d45a0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d45a4u64) }
        0x102d45a4 => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d45a8u64) }
        0x102d45a8 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d45acu64) }
        0x102d45ac => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d45b0u64) }
        0x102d45b0 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d45b4u64) }
        0x102d45b4 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d45b8u64) }
        0x102d45b8 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d45bcu64) }
        0x102d45bc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d45c0u64) }
        0x102d45c0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d45c4u64) }
        0x102d45c4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d45c8u64).wrapping_add((imm * 4) as u64) } else { 0x102d45c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d45c8u64) }
        0x102d45c8 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d45ccu64) }
        0x102d45cc => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d45d0u64) }
        0x102d45d0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d45d4u64) }
        0x102d45d4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d45d8u64) }
        0x102d45d8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d45dcu64) }
        0x102d45dc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d45e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d45e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d45e0u64) }
        0x102d45e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d45e4u64) } }
        0x102d45e4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d45e8u64) }
        0x102d45e8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d45ecu64) }
        0x102d45ec => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d45f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d45f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d45f0u64) }
        0x102d45f0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d45f4u64) }
        0x102d45f4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d45f8u64) }
        0x102d45f8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d45fcu64) }
        0x102d45fc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4600u64) }
        0x102d4600 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4604u64) }
        0x102d4604 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4608u64).wrapping_add((imm * 4) as u64) } else { 0x102d4608u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4608u64) }
        0x102d4608 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d460cu64) } }
        0x102d460c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4610u64) }
        0x102d4610 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4614u64) }
        0x102d4614 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4618u64) }
        0x102d4618 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d461cu64) }
        0x102d461c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4620u64) }
        0x102d4620 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4624u64) }
        0x102d4624 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4628u64) }
        0x102d4628 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d462cu64) }
        0x102d462c => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4630u64) }
        0x102d4630 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4634u64) }
        0x102d4634 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4638u64) }
        0x102d4638 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d463cu64) }
        0x102d463c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4640u64) }
        0x102d4640 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4644u64) }
        0x102d4644 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4648u64).wrapping_add((imm * 4) as u64) } else { 0x102d4648u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4648u64) }
        0x102d4648 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d464cu64) }
        0x102d464c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4650u64) }
        0x102d4650 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4654u64) }
        0x102d4654 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4658u64) }
        0x102d4658 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d465cu64) }
        0x102d465c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4660u64).wrapping_add((imm * 4) as u64) } else { 0x102d4660u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4660u64) }
        0x102d4660 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4664u64) } }
        0x102d4664 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4668u64) }
        0x102d4668 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d466cu64) }
        0x102d466c => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4670u64).wrapping_add((imm * 4) as u64) } else { 0x102d4670u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4670u64) }
        0x102d4670 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4674u64) }
        0x102d4674 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4678u64) }
        0x102d4678 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d467cu64) }
        0x102d467c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4680u64) }
        0x102d4680 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4684u64) }
        0x102d4684 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4688u64).wrapping_add((imm * 4) as u64) } else { 0x102d4688u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4688u64) }
        0x102d4688 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d468cu64) } }
        0x102d468c => { vm.decode(0x2ca4401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7756i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d4690u64) }
        0x102d4690 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4694u64) } }
        0x102d4694 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d4698u64) }
        0x102d4698 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d469cu64) }
        0x102d469c => { vm.decode(0x88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d46a0u64) }
        0x102d46a0 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d46a4u64) }
        0x102d46a4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d46a8u64) }
        0x102d46a8 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d46acu64) }
        0x102d46ac => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d46b0u64) }
        0x102d46b0 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d46b4u64) }
        0x102d46b4 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d46b8u64) }
        0x102d46b8 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d46bcu64) }
        0x102d46bc => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d46c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d46c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d46c0u64) }
        0x102d46c0 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d46c4u64) }
        0x102d46c4 => { vm.decode(0xa580051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9908i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d46c8u64) }
        0x102d46cc => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d46d0u64) }
        0x102d46d0 => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d46d4u64) }
        0x102d46d4 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d46d8u64) }
        0x102d46d8 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d46dcu64) }
        0x102d46dc => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d46e0u64) }
        0x102d46e0 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d46e4u64) }
        0x102d46e4 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d46e8u64) }
        0x102d46e8 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d46ecu64) }
        0x102d46ec => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d46f0u64) }
        0x102d46f0 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d46f4u64) }
        0x102d46f4 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d46f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d46f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d46f8u64) }
        0x102d46f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d46fcu64) } }
        0x102d46fc => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d4700u64) }
        0x102d4700 => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4704u64) }
        0x102d4704 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4708u64) }
        0x102d4708 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d470cu64) }
        0x102d470c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4710u64) }
        0x102d4710 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4714u64) }
        0x102d4714 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4718u64).wrapping_add((imm * 4) as u64) } else { 0x102d4718u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4718u64) }
        0x102d4718 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d471cu64) }
        0x102d471c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4720u64) }
        0x102d4720 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4724u64) }
        0x102d4724 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4728u64) }
        0x102d4728 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d472cu64) }
        0x102d472c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4730u64).wrapping_add((imm * 4) as u64) } else { 0x102d4730u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4730u64) }
        0x102d4730 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4734u64) } }
        0x102d4734 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4738u64) }
        0x102d4738 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d473cu64) }
        0x102d473c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4740u64).wrapping_add((imm * 4) as u64) } else { 0x102d4740u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4740u64) }
        0x102d4740 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4744u64) }
        0x102d4744 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4748u64) }
        0x102d4748 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d474cu64) }
        0x102d474c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4750u64) }
        0x102d4750 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4754u64) }
        0x102d4754 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4758u64).wrapping_add((imm * 4) as u64) } else { 0x102d4758u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4758u64) }
        0x102d4758 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d475cu64) } }
        0x102d475c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4760u64) }
        0x102d4760 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4764u64) }
        0x102d4764 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4768u64) }
        0x102d4768 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d476cu64) }
        0x102d476c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4770u64) }
        0x102d4770 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4774u64) }
        0x102d4774 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4778u64) }
        0x102d4778 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d477cu64) }
        0x102d477c => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4780u64) }
        0x102d4780 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4784u64) }
        0x102d4784 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4788u64) }
        0x102d4788 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d478cu64) }
        0x102d478c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4790u64) }
        0x102d4790 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4794u64) }
        0x102d4794 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4798u64).wrapping_add((imm * 4) as u64) } else { 0x102d4798u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4798u64) }
        0x102d4798 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d479cu64) }
        0x102d479c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d47a0u64) }
        0x102d47a0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d47a4u64) }
        0x102d47a4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d47a8u64) }
        0x102d47a8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d47acu64) }
        0x102d47ac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d47b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d47b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d47b0u64) }
        0x102d47b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d47b4u64) } }
        0x102d47b4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d47b8u64) }
        0x102d47b8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d47bcu64) }
        0x102d47bc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d47c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d47c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d47c0u64) }
        0x102d47c0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d47c4u64) }
        0x102d47c4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d47c8u64) }
        0x102d47c8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d47ccu64) }
        0x102d47cc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d47d0u64) }
        0x102d47d0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d47d4u64) }
        0x102d47d4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d47d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d47d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d47d8u64) }
        0x102d47d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d47dcu64) } }
        0x102d47dc => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d47e0u64) }
        0x102d47e0 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d47e4u64) }
        0x102d47e4 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d47e8u64) }
        0x102d47e8 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d47ecu64) }
        0x102d47ec => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d47f0u64) }
        0x102d47f0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d47f4u64) }
        0x102d47f4 => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d47f8u64) }
        0x102d47f8 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d47fcu64) }
        0x102d47fc => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4800u64) }
        0x102d4800 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4804u64) }
        0x102d4804 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4808u64) }
        0x102d4808 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d480cu64) }
        0x102d480c => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4810u64) }
        0x102d4810 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4814u64) }
        0x102d4814 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4818u64) }
        0x102d4818 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d481cu64) }
        0x102d481c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4820u64).wrapping_add((imm * 4) as u64) } else { 0x102d4820u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4820u64) }
        0x102d4820 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4824u64) }
        0x102d4824 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4828u64) }
        0x102d4828 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d482cu64) }
        0x102d482c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4830u64) }
        0x102d4830 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4834u64) }
        0x102d4834 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4838u64).wrapping_add((imm * 4) as u64) } else { 0x102d4838u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4838u64) }
        0x102d4838 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d483cu64) } }
        0x102d483c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4840u64) }
        0x102d4840 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4844u64) }
        0x102d4844 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4848u64).wrapping_add((imm * 4) as u64) } else { 0x102d4848u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4848u64) }
        0x102d4848 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d484cu64) }
        0x102d484c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4850u64) }
        0x102d4850 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4854u64) }
        0x102d4854 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4858u64) }
        0x102d4858 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d485cu64) }
        0x102d485c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4860u64).wrapping_add((imm * 4) as u64) } else { 0x102d4860u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4860u64) }
        0x102d4860 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4864u64) } }
        0x102d4864 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4868u64) }
        0x102d4868 => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d486cu64) }
        0x102d486c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4870u64) }
        0x102d4870 => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4874u64) }
        0x102d4874 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4878u64) }
        0x102d4878 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d487cu64) }
        0x102d487c => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4880u64) }
        0x102d4880 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4884u64) }
        0x102d4884 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4888u64) }
        0x102d4888 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d488cu64) }
        0x102d488c => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4890u64) }
        0x102d4890 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4894u64) }
        0x102d4894 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4898u64) }
        0x102d4898 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d489cu64) }
        0x102d489c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d48a0u64) }
        0x102d48a0 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d48a4u64) }
        0x102d48a4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d48a8u64).wrapping_add((imm * 4) as u64) } else { 0x102d48a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d48a8u64) }
        0x102d48a8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d48acu64) }
        0x102d48ac => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d48b0u64) }
        0x102d48b0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d48b4u64) }
        0x102d48b4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d48b8u64) }
        0x102d48b8 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d48bcu64) }
        0x102d48bc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d48c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d48c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d48c0u64) }
        0x102d48c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d48c4u64) } }
        0x102d48c4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d48c8u64) }
        0x102d48c8 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d48ccu64) }
        0x102d48cc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d48d0u64).wrapping_add((imm * 4) as u64) } else { 0x102d48d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d48d0u64) }
        0x102d48d0 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d48d4u64) }
        0x102d48d4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d48d8u64) }
        0x102d48d8 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d48dcu64) }
        0x102d48dc => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d48e0u64) }
        0x102d48e0 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d48e4u64) }
        0x102d48e4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d48e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d48e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d48e8u64) }
        0x102d48e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d48ecu64) } }
        0x102d48ec => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d48f0u64) }
        0x102d48f0 => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d48f4u64) }
        0x102d48f4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d48f8u64) }
        0x102d48f8 => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d48fcu64) }
        0x102d48fc => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4900u64) }
        0x102d4900 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4904u64) }
        0x102d4904 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4908u64) }
        0x102d4908 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d490cu64) }
        0x102d490c => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4910u64) }
        0x102d4910 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4914u64) }
        0x102d4914 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4918u64) }
        0x102d4918 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d491cu64) }
        0x102d491c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4920u64).wrapping_add((imm * 4) as u64) } else { 0x102d4920u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4920u64) }
        0x102d4920 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4924u64) }
        0x102d4924 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4928u64) }
        0x102d4928 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d492cu64) }
        0x102d492c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4930u64) }
        0x102d4930 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4934u64) }
        0x102d4934 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4938u64).wrapping_add((imm * 4) as u64) } else { 0x102d4938u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4938u64) }
        0x102d4938 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d493cu64) } }
        0x102d493c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4940u64) }
        0x102d4940 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4944u64) }
        0x102d4944 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4948u64).wrapping_add((imm * 4) as u64) } else { 0x102d4948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4948u64) }
        0x102d4948 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d494cu64) }
        0x102d494c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4950u64) }
        0x102d4950 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4954u64) }
        0x102d4954 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4958u64) }
        0x102d4958 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d495cu64) }
        0x102d495c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4960u64).wrapping_add((imm * 4) as u64) } else { 0x102d4960u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4960u64) }
        0x102d4960 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4964u64) } }
        0x102d4964 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4968u64) }
        0x102d4968 => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d496cu64) }
        0x102d496c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4970u64) }
        0x102d4970 => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4974u64) }
        0x102d4974 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4978u64) }
        0x102d4978 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d497cu64) }
        0x102d497c => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4980u64) }
        0x102d4980 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4984u64) }
        0x102d4984 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4988u64) }
        0x102d4988 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d498cu64) }
        0x102d498c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4990u64) }
        0x102d4990 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4994u64) }
        0x102d4994 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4998u64).wrapping_add((imm * 4) as u64) } else { 0x102d4998u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4998u64) }
        0x102d4998 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d499cu64) }
        0x102d499c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d49a0u64) }
        0x102d49a0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d49a4u64) }
        0x102d49a4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d49a8u64) }
        0x102d49a8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d49acu64) }
        0x102d49ac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d49b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d49b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d49b0u64) }
        0x102d49b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d49b4u64) } }
        0x102d49b4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d49b8u64) }
        0x102d49b8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d49bcu64) }
        0x102d49bc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d49c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d49c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d49c0u64) }
        0x102d49c0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d49c4u64) }
        0x102d49c4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d49c8u64) }
        0x102d49c8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d49ccu64) }
        0x102d49cc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d49d0u64) }
        0x102d49d0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d49d4u64) }
        0x102d49d4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d49d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d49d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d49d8u64) }
        0x102d49d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d49dcu64) } }
        0x102d49dc => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d49e0u64) }
        0x102d49e0 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d49e4u64) }
        0x102d49e4 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d49e8u64) }
        0x102d49e8 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d49ecu64) }
        0x102d49ec => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d49f0u64) }
        0x102d49f0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d49f4u64) }
        0x102d49f4 => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d49f8u64) }
        0x102d49f8 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d49fcu64) }
        0x102d49fc => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4a00u64) }
        0x102d4a00 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4a04u64) }
        0x102d4a04 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4a08u64) }
        0x102d4a08 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4a0cu64) }
        0x102d4a0c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a10u64) }
        0x102d4a10 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a14u64) }
        0x102d4a14 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4a18u64).wrapping_add((imm * 4) as u64) } else { 0x102d4a18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4a18u64) }
        0x102d4a18 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4a1cu64) }
        0x102d4a1c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a20u64) }
        0x102d4a20 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4a24u64) }
        0x102d4a24 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4a28u64) }
        0x102d4a28 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a2cu64) }
        0x102d4a2c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4a30u64).wrapping_add((imm * 4) as u64) } else { 0x102d4a30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4a30u64) }
        0x102d4a30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4a34u64) } }
        0x102d4a34 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a38u64) }
        0x102d4a38 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a3cu64) }
        0x102d4a3c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4a40u64).wrapping_add((imm * 4) as u64) } else { 0x102d4a40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4a40u64) }
        0x102d4a40 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4a44u64) }
        0x102d4a44 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a48u64) }
        0x102d4a48 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4a4cu64) }
        0x102d4a4c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4a50u64) }
        0x102d4a50 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a54u64) }
        0x102d4a54 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4a58u64).wrapping_add((imm * 4) as u64) } else { 0x102d4a58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4a58u64) }
        0x102d4a58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4a5cu64) } }
        0x102d4a5c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4a60u64) }
        0x102d4a60 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4a64u64) }
        0x102d4a64 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4a68u64) }
        0x102d4a68 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d4a6cu64) }
        0x102d4a6c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4a70u64) }
        0x102d4a70 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4a74u64) }
        0x102d4a74 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4a78u64) }
        0x102d4a78 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4a7cu64) }
        0x102d4a7c => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a80u64) }
        0x102d4a80 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4a84u64) }
        0x102d4a84 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4a88u64) }
        0x102d4a88 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4a8cu64) }
        0x102d4a8c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a90u64) }
        0x102d4a90 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4a94u64) }
        0x102d4a94 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4a98u64).wrapping_add((imm * 4) as u64) } else { 0x102d4a98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4a98u64) }
        0x102d4a98 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4a9cu64) }
        0x102d4a9c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4aa0u64) }
        0x102d4aa0 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4aa4u64) }
        0x102d4aa4 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4aa8u64) }
        0x102d4aa8 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4aacu64) }
        0x102d4aac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4ab0u64).wrapping_add((imm * 4) as u64) } else { 0x102d4ab0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4ab0u64) }
        0x102d4ab0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4ab4u64) } }
        0x102d4ab4 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4ab8u64) }
        0x102d4ab8 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4abcu64) }
        0x102d4abc => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4ac0u64).wrapping_add((imm * 4) as u64) } else { 0x102d4ac0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4ac0u64) }
        0x102d4ac0 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4ac4u64) }
        0x102d4ac4 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4ac8u64) }
        0x102d4ac8 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d4accu64) }
        0x102d4acc => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4ad0u64) }
        0x102d4ad0 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d4ad4u64) }
        0x102d4ad4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4ad8u64).wrapping_add((imm * 4) as u64) } else { 0x102d4ad8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4ad8u64) }
        0x102d4ad8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4adcu64) } }
        0x102d4adc => { vm.decode(0xa880051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8860i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d4ae0u64) }
        0x102d4ae0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4ae4u64) } }
        0x102d4ae4 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d4ae8u64) }
        0x102d4ae8 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4aecu64) }
        0x102d4aec => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4af0u64) }
        0x102d4af0 => { vm.decode(0x90000430u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4af4u64) }
        0x102d4af4 => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4af8u64) }
        0x102d4af8 => { vm.decode(0xd1cc011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d4afcu64) }
        0x102d4afc => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b00u64) }
        0x102d4b00 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4b04u64).wrapping_add((imm * 4) as u64) } else { 0x102d4b04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4b04u64) }
        0x102d4b04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4b08u64) } }
        0x102d4b08 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b0cu64) }
        0x102d4b0c => { vm.decode(0x2200212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b10u64) }
        0x102d4b10 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b14u64) }
        0x102d4b14 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4b18u64) }
        0x102d4b18 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4b1cu64) }
        0x102d4b1c => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4b20u64) }
        0x102d4b20 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4b24u64) }
        0x102d4b24 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b28u64) }
        0x102d4b28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4b2cu64 + 4);
        vm.wr(37, 0x102d4b2cu64 + 4);
        Ok(0x102d4b2cu64) }
        0x102d4b2c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4b30u64) }
        0x102d4b30 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b34u64) }
        0x102d4b34 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b38u64) }
        0x102d4b38 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b3cu64) }
        0x102d4b3c => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b40u64) }
        0x102d4b40 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b44u64) }
        0x102d4b44 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b48u64) }
        0x102d4b48 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b4cu64) }
        0x102d4b4c => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b50u64) }
        0x102d4b50 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b54u64) }
        0x102d4b54 => { vm.decode(0x8098b27u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4b58u64).wrapping_add((imm * 4) as u64) } else { 0x102d4b58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4b58u64) }
        0x102d4b58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4b5cu64) } }
        0x102d4b5c => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d4b60u64) }
        0x102d4b60 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4b64u64) }
        0x102d4b64 => { vm.decode(0x880184f5u32); let d = 2i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b68u64) }
        0x102d4b68 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b6cu64) }
        0x102d4b6c => { vm.decode(0x84891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4b70u64) }
        0x102d4b70 => { vm.decode(0xaa00a4f5u32); let d = 2i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b74u64) }
        0x102d4b74 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b78u64) }
        0x102d4b78 => { vm.decode(0x85091b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4b7cu64) }
        0x102d4b7c => { vm.decode(0xa800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b80u64) }
        0x102d4b80 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4b84u64) }
        0x102d4b84 => { vm.decode(0x85891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4b88u64) }
        0x102d4b88 => { vm.decode(0x8800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b8cu64) }
        0x102d4b8c => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b90u64) }
        0x102d4b90 => { vm.decode(0x820284f5u32); let d = 4i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b94u64) }
        0x102d4b94 => { vm.decode(0xa04284f5u32); let d = 5i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b98u64) }
        0x102d4b98 => { vm.decode(0x804084f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4b9cu64) }
        0x102d4b9c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ba0u64) }
        0x102d4ba0 => { vm.decode(0x18609030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ba4u64) }
        0x102d4ba4 => { vm.decode(0x2080012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ba8u64) }
        0x102d4ba8 => { vm.decode(0x18689030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4bacu64) }
        0x102d4bac => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bb0u64) }
        0x102d4bb0 => { vm.decode(0x18709030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4bb4u64) }
        0x102d4bb4 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bb8u64) }
        0x102d4bb8 => { vm.decode(0x18789030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4bbcu64) }
        0x102d4bbc => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bc0u64) }
        0x102d4bc0 => { vm.decode(0x186090b0u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4bc4u64) }
        0x102d4bc4 => { vm.decode(0x4a8043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bc8u64) }
        0x102d4bc8 => { vm.decode(0x6a8063efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bccu64) }
        0x102d4bcc => { vm.decode(0x488c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bd0u64) }
        0x102d4bd0 => { vm.decode(0x688c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bd4u64) }
        0x102d4bd4 => { vm.decode(0x4a8643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bd8u64) }
        0x102d4bd8 => { vm.decode(0x6a8663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4bdcu64) }
        0x102d4bdc => { vm.decode(0x408ac3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4be0u64) }
        0x102d4be0 => { vm.decode(0x608ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4be4u64) }
        0x102d4be4 => { vm.decode(0x428acbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4be8u64) }
        0x102d4be8 => { vm.decode(0x628aebafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4becu64) }
        0x102d4bec => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102d4bf0u64) }
        0x102d4bf0 => { vm.decode(0x77198059u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4bf4u64) }
        0x102d4bf4 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4bf8u64) }
        0x102d4bf8 => { vm.decode(0x830204f5u32); let d = 4i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4bfcu64) }
        0x102d4bfc => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c00u64) }
        0x102d4c00 => { vm.decode(0x8b209430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c04u64) }
        0x102d4c04 => { vm.decode(0x1b1894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c08u64) }
        0x102d4c08 => { vm.decode(0x665f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4c0cu64) }
        0x102d4c0c => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c10u64) }
        0x102d4c10 => { vm.decode(0x99181470u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c14u64) }
        0x102d4c14 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c18u64) }
        0x102d4c18 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c1cu64) }
        0x102d4c1c => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c20u64) }
        0x102d4c20 => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c24u64) }
        0x102d4c24 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c28u64) }
        0x102d4c28 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c2cu64) }
        0x102d4c2c => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c30u64) }
        0x102d4c30 => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c34u64) }
        0x102d4c34 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c38u64) }
        0x102d4c38 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c3cu64) }
        0x102d4c3c => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c40u64) }
        0x102d4c40 => { vm.decode(0x99105570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c44u64) }
        0x102d4c44 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4c48u64 + 4);
        vm.wr(37, 0x102d4c48u64 + 4);
        Ok(0x102d4c48u64) }
        0x102d4c48 => { vm.decode(0x9085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4c4cu64) }
        0x102d4c4c => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c50u64) }
        0x102d4c50 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c54u64) }
        0x102d4c54 => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c58u64) }
        0x102d4c58 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c5cu64) }
        0x102d4c5c => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c60u64) }
        0x102d4c60 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c64u64) }
        0x102d4c64 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c68u64) }
        0x102d4c68 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c6cu64) }
        0x102d4c6c => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c70u64) }
        0x102d4c70 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c74u64) }
        0x102d4c74 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c78u64) }
        0x102d4c78 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c7cu64) }
        0x102d4c7c => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c80u64) }
        0x102d4c80 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c84u64) }
        0x102d4c84 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4c88u64) }
        0x102d4c88 => { vm.decode(0xd98c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d4c8cu64) }
        0x102d4c8c => { vm.decode(0x31c3e27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -2534i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4c90u64).wrapping_add((imm * 4) as u64) } else { 0x102d4c90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4c90u64) }
        0x102d4c90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4c94u64) } }
        0x102d4c94 => { vm.decode(0x4b2514f5u32); let d = 18i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4c98u64) }
        0x102d4c98 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4c9cu64) }
        0x102d4c9c => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ca0u64) }
        0x102d4ca0 => { vm.decode(0x810724f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4ca4u64) }
        0x102d4ca4 => { vm.decode(0x2020212fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ca8u64) }
        0x102d4ca8 => { vm.decode(0x501019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4cacu64) }
        0x102d4cac => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4cb0u64) }
        0x102d4cb0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4cb4u64) }
        0x102d4cb4 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cb8u64) }
        0x102d4cb8 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cbcu64) }
        0x102d4cbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4cc0u64 + 4);
        vm.wr(37, 0x102d4cc0u64 + 4);
        Ok(0x102d4cc0u64) }
        0x102d4cc0 => { vm.decode(0x661b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4cc4u64) }
        0x102d4cc4 => { vm.decode(0x6a8c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cc8u64) }
        0x102d4cc8 => { vm.decode(0x81400530u32); let d = 18i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4cccu64) }
        0x102d4ccc => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4cd0u64) }
        0x102d4cd0 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cd4u64) }
        0x102d4cd4 => { vm.decode(0x2204212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cd8u64) }
        0x102d4cd8 => { vm.decode(0xb109430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4cdcu64) }
        0x102d4cdc => { vm.decode(0x60841019u32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4ce0u64) }
        0x102d4ce0 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ce4u64) }
        0x102d4ce4 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ce8u64) }
        0x102d4ce8 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4cecu64) }
        0x102d4cec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4cf0u64 + 4);
        vm.wr(37, 0x102d4cf0u64 + 4);
        Ok(0x102d4cf0u64) }
        0x102d4cf0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4cf4u64) }
        0x102d4cf4 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d4cf8u64) }
        0x102d4cf8 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4cfcu64) }
        0x102d4cfc => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4d00u64) }
        0x102d4d00 => { vm.decode(0x90000530u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d04u64) }
        0x102d4d04 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d08u64) }
        0x102d4d08 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d0cu64) }
        0x102d4d0c => { vm.decode(0x8b3814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d10u64) }
        0x102d4d10 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d14u64) }
        0x102d4d14 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d18u64) }
        0x102d4d18 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d1cu64) }
        0x102d4d1c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4d20u64 + 4);
        vm.wr(37, 0x102d4d20u64 + 4);
        Ok(0x102d4d20u64) }
        0x102d4d20 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4d24u64) }
        0x102d4d24 => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d28u64) }
        0x102d4d28 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d2cu64) }
        0x102d4d2c => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d30u64) }
        0x102d4d30 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d34u64) }
        0x102d4d34 => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d38u64) }
        0x102d4d38 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d3cu64) }
        0x102d4d3c => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d40u64) }
        0x102d4d40 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d44u64) }
        0x102d4d44 => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d48u64) }
        0x102d4d48 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d4cu64) }
        0x102d4d4c => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d50u64) }
        0x102d4d50 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d54u64) }
        0x102d4d54 => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d58u64) }
        0x102d4d58 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d5cu64) }
        0x102d4d5c => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4d60u64) }
        0x102d4d60 => { vm.decode(0x8820001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 200i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d4d64u64) }
        0x102d4d64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4d68u64) } }
        0x102d4d68 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d4d6cu64) }
        0x102d4d70 => { vm.decode(0x7b3bcf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4d74u64) }
        0x102d4d74 => { vm.decode(0x9b185d70u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d78u64) }
        0x102d4d78 => { vm.decode(0x8b105d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d7cu64) }
        0x102d4d7c => { vm.decode(0x9b085d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d80u64) }
        0x102d4d80 => { vm.decode(0x8b005d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d84u64) }
        0x102d4d84 => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d88u64) }
        0x102d4d88 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d8cu64) }
        0x102d4d8c => { vm.decode(0x9b2895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d90u64) }
        0x102d4d90 => { vm.decode(0x8b2095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d94u64) }
        0x102d4d94 => { vm.decode(0x9b389430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d98u64) }
        0x102d4d98 => { vm.decode(0x8b309430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4d9cu64) }
        0x102d4d9c => { vm.decode(0x1395b2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12845056i64;
        vm.st(d, imm as u64);
        Ok(0x102d4da0u64) }
        0x102d4da0 => { vm.decode(0x27592918u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 43256i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4da4u64) }
        0x102d4da4 => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4da8u64) }
        0x102d4da8 => { vm.decode(0x830404f5u32); let d = 16i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4dacu64) }
        0x102d4dac => { vm.decode(0x286096fu32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4db0u64) }
        0x102d4db0 => { vm.decode(0x208c296fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4db4u64) }
        0x102d4db4 => { vm.decode(0x8c096fu32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4db8u64) }
        0x102d4db8 => { vm.decode(0x2280296fu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dbcu64) }
        0x102d4dbc => { vm.decode(0x280096fu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dc0u64) }
        0x102d4dc0 => { vm.decode(0x2082296fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dc4u64) }
        0x102d4dc4 => { vm.decode(0x208a096fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dc8u64) }
        0x102d4dc8 => { vm.decode(0x4284292fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dccu64) }
        0x102d4dcc => { vm.decode(0x6286012fu32); let d = 21i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dd0u64) }
        0x102d4dd0 => { vm.decode(0x8a082fu32); let d = 14i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dd4u64) }
        0x102d4dd4 => { vm.decode(0x2088212fu32); let d = 3i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dd8u64) }
        0x102d4dd8 => { vm.decode(0x6084092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ddcu64) }
        0x102d4ddc => { vm.decode(0x608e292fu32); let d = 31i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4de0u64) }
        0x102d4de0 => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4de4u64) }
        0x102d4de4 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4de8u64) }
        0x102d4de8 => { vm.decode(0xa8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4decu64) }
        0x102d4dec => { vm.decode(0xa2202fu32); let d = 4i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4df0u64) }
        0x102d4df0 => { vm.decode(0x42a2002fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4df4u64) }
        0x102d4df4 => { vm.decode(0x62a8202fu32); let d = 19i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4df8u64) }
        0x102d4df8 => { vm.decode(0x60ae002fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4dfcu64) }
        0x102d4dfc => { vm.decode(0x40ac202fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e00u64) }
        0x102d4e00 => { vm.decode(0x893015b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e04u64) }
        0x102d4e04 => { vm.decode(0x42ac002fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e08u64) }
        0x102d4e08 => { vm.decode(0x892815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e0cu64) }
        0x102d4e0c => { vm.decode(0x2ae202fu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e10u64) }
        0x102d4e10 => { vm.decode(0x4288012fu32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e14u64) }
        0x102d4e14 => { vm.decode(0x8470a958u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 43272i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4e18u64) }
        0x102d4e18 => { vm.decode(0x428a212fu32); let d = 22i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e1cu64) }
        0x102d4e1c => { vm.decode(0x30000037u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102d4e20u64) }
        0x102d4e20 => { vm.decode(0x810224f5u32); let d = 4i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e24u64) }
        0x102d4e24 => { vm.decode(0xab4004f5u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e28u64) }
        0x102d4e28 => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d4e2cu64) }
        0x102d4e2c => { vm.decode(0x8f5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        
        Ok(0x102d4e30u64) }
        0x102d4e30 => { vm.decode(0x700400a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2496i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d4e34u64).wrapping_add((imm * 4) as u64) } else { 0x102d4e34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4e34u64) }
        0x102d4e34 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4e38u64) } }
        0x102d4e38 => { vm.decode(0x814184f5u32); let d = 1i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e3cu64) }
        0x102d4e3c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e40u64) }
        0x102d4e40 => { vm.decode(0x2610471u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2452i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4e44u64).wrapping_add((imm * 4) as u64) } else { 0x102d4e44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4e44u64) }
        0x102d4e44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4e48u64) } }
        0x102d4e48 => { vm.decode(0x48005875u32); let d = 18i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4e4cu64) }
        0x102d4e4c => { vm.decode(0x4c30d875u32); let d = 19i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4e50u64) }
        0x102d4e50 => { vm.decode(0xcb00875u32); let d = 20i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4e54u64) }
        0x102d4e54 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e58u64) }
        0x102d4e58 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e5cu64) }
        0x102d4e5c => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e60u64) }
        0x102d4e60 => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e64u64) }
        0x102d4e64 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e68u64) }
        0x102d4e68 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e6cu64) }
        0x102d4e6c => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e70u64) }
        0x102d4e70 => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e74u64) }
        0x102d4e74 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e78u64) }
        0x102d4e78 => { vm.decode(0x4b2734f5u32); let d = 22i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e7cu64) }
        0x102d4e7c => { vm.decode(0xab4724f5u32); let d = 23i16; let s = 17i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e80u64) }
        0x102d4e80 => { vm.decode(0x1395b2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12845056i64;
        vm.st(d, imm as u64);
        Ok(0x102d4e84u64) }
        0x102d4e84 => { vm.decode(0x7592918u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 43256i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d4e88u64) }
        0x102d4e88 => { vm.decode(0x214124f5u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4e8cu64) }
        0x102d4e8c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4e90u64) }
        0x102d4e90 => { vm.decode(0xa00dcbb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -244i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d4e94u64) }
        0x102d4e94 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d4e98u64) }
        0x102d4e98 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4e9cu64) }
        0x102d4e9c => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ea0u64) }
        0x102d4ea0 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4ea4u64) }
        0x102d4ea4 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ea8u64) }
        0x102d4ea8 => { vm.decode(0x28016fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4eacu64) }
        0x102d4eac => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4eb0u64) }
        0x102d4eb0 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4eb4u64) }
        0x102d4eb4 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4eb8u64) }
        0x102d4eb8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4ebcu64) }
        0x102d4ebc => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ec0u64) }
        0x102d4ec0 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4ec4u64) }
        0x102d4ec4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4ec8u64 + 4);
        vm.wr(37, 0x102d4ec8u64 + 4);
        Ok(0x102d4ec8u64) }
        0x102d4ec8 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4eccu64) }
        0x102d4ecc => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ed0u64) }
        0x102d4ed0 => { vm.decode(0x14184f5u32); let d = 1i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4ed4u64) }
        0x102d4ed4 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d4ed8u64) }
        0x102d4ed8 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d4edcu64) }
        0x102d4edc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ee0u64) }
        0x102d4ee0 => { vm.decode(0x2210571u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2388i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d4ee4u64).wrapping_add((imm * 4) as u64) } else { 0x102d4ee4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d4ee4u64) }
        0x102d4ee4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d4ee8u64) } }
        0x102d4ee8 => { vm.decode(0x89085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4eecu64) }
        0x102d4eec => { vm.decode(0x99105530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ef0u64) }
        0x102d4ef0 => { vm.decode(0x834104f5u32); let d = 1i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4ef4u64) }
        0x102d4ef4 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4ef8u64) }
        0x102d4ef8 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4efcu64) }
        0x102d4efc => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4f00u64) }
        0x102d4f00 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f04u64) }
        0x102d4f04 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f08u64) }
        0x102d4f08 => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4f0cu64) }
        0x102d4f0c => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f10u64) }
        0x102d4f10 => { vm.decode(0xa98604f5u32); let d = 30i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4f14u64) }
        0x102d4f14 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4f18u64) }
        0x102d4f18 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f1cu64) }
        0x102d4f1c => { vm.decode(0x814124f5u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4f20u64) }
        0x102d4f20 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f24u64) }
        0x102d4f24 => { vm.decode(0x4286632fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f28u64) }
        0x102d4f28 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f2cu64) }
        0x102d4f2c => { vm.decode(0x10009419u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 2048i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4f30u64) }
        0x102d4f30 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f34u64) }
        0x102d4f34 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f38u64) }
        0x102d4f38 => { vm.decode(0x761b8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4f3cu64) }
        0x102d4f3c => { vm.decode(0x608e432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f40u64) }
        0x102d4f40 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f44u64) }
        0x102d4f44 => { vm.decode(0x408c632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f48u64) }
        0x102d4f48 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f4cu64) }
        0x102d4f4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4f50u64 + 4);
        vm.wr(37, 0x102d4f50u64 + 4);
        Ok(0x102d4f50u64) }
        0x102d4f50 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f54u64) }
        0x102d4f54 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f58u64) }
        0x102d4f58 => { vm.decode(0x9b281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f5cu64) }
        0x102d4f5c => { vm.decode(0x6a8443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f60u64) }
        0x102d4f60 => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f64u64) }
        0x102d4f64 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f68u64) }
        0x102d4f68 => { vm.decode(0x420d1022u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d4f6cu64) }
        0x102d4f6c => { vm.decode(0x66598019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d4f70u64) }
        0x102d4f70 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f74u64) }
        0x102d4f74 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4f78u64 + 4);
        vm.wr(37, 0x102d4f78u64 + 4);
        Ok(0x102d4f78u64) }
        0x102d4f78 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f7cu64) }
        0x102d4f7c => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f80u64) }
        0x102d4f80 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f84u64) }
        0x102d4f84 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4f88u64) }
        0x102d4f88 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f8cu64) }
        0x102d4f8c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f90u64) }
        0x102d4f90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4f94u64 + 4);
        vm.wr(37, 0x102d4f94u64 + 4);
        Ok(0x102d4f94u64) }
        0x102d4f94 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4f98u64) }
        0x102d4f98 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4f9cu64) }
        0x102d4f9c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4fa0u64) }
        0x102d4fa0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4fa4u64) }
        0x102d4fa4 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fa8u64) }
        0x102d4fa8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4facu64) }
        0x102d4fac => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fb0u64) }
        0x102d4fb0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fb4u64) }
        0x102d4fb4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4fb8u64) }
        0x102d4fb8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4fbcu64) }
        0x102d4fbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4fc0u64 + 4);
        vm.wr(37, 0x102d4fc0u64 + 4);
        Ok(0x102d4fc0u64) }
        0x102d4fc0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4fc4u64) }
        0x102d4fc4 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4fc8u64) }
        0x102d4fc8 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4fccu64) }
        0x102d4fcc => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4fd0u64) }
        0x102d4fd0 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fd4u64) }
        0x102d4fd4 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4fd8u64) }
        0x102d4fd8 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fdcu64) }
        0x102d4fdc => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d4fe0u64) }
        0x102d4fe0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4fe4u64) }
        0x102d4fe4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4fe8u64) }
        0x102d4fe8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d4fecu64 + 4);
        vm.wr(37, 0x102d4fecu64 + 4);
        Ok(0x102d4fecu64) }
        0x102d4fec => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d4ff0u64) }
        0x102d4ff0 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ff4u64) }
        0x102d4ff4 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d4ff8u64) }
        0x102d4ff8 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d4ffcu64) }
        0x102d4ffc => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5000u64) }
        0x102d5000 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5004u64) }
        0x102d5004 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5008u64) }
        0x102d5008 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d500cu64) }
        0x102d500c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5010u64) }
        0x102d5010 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5014u64) }
        0x102d5014 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5018u64 + 4);
        vm.wr(37, 0x102d5018u64 + 4);
        Ok(0x102d5018u64) }
        0x102d5018 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d501cu64) }
        0x102d501c => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5020u64) }
        0x102d5020 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5024u64) }
        0x102d5024 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5028u64) }
        0x102d5028 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d502cu64) }
        0x102d502c => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5030u64) }
        0x102d5030 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5034u64) }
        0x102d5034 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5038u64) }
        0x102d5038 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d503cu64) }
        0x102d503c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5040u64) }
        0x102d5040 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5044u64 + 4);
        vm.wr(37, 0x102d5044u64 + 4);
        Ok(0x102d5044u64) }
        0x102d5044 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5048u64) }
        0x102d5048 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d504cu64) }
        0x102d504c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5050u64) }
        0x102d5050 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5054u64) }
        0x102d5054 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5058u64) }
        0x102d5058 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d505cu64) }
        0x102d505c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5060u64) }
        0x102d5060 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5064u64) }
        0x102d5064 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5068u64) }
        0x102d5068 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d506cu64) }
        0x102d506c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5070u64 + 4);
        vm.wr(37, 0x102d5070u64 + 4);
        Ok(0x102d5070u64) }
        0x102d5070 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5074u64) }
        0x102d5074 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5078u64) }
        0x102d5078 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d507cu64) }
        0x102d507c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5080u64) }
        0x102d5080 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5084u64) }
        0x102d5084 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5088u64) }
        0x102d5088 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d508cu64) }
        0x102d508c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5090u64) }
        0x102d5090 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5094u64) }
        0x102d5094 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5098u64) }
        0x102d5098 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d509cu64 + 4);
        vm.wr(37, 0x102d509cu64 + 4);
        Ok(0x102d509cu64) }
        0x102d509c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d50a0u64) }
        0x102d50a0 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d50a4u64) }
        0x102d50a4 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d50a8u64) }
        0x102d50a8 => { vm.decode(0x76499019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d50acu64) }
        0x102d50ac => { vm.decode(0x721b1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d50b0u64) }
        0x102d50b0 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d50b4u64) }
        0x102d50b4 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d50b8u64) }
        0x102d50b8 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d50bcu64) }
        0x102d50bc => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d50c0u64) }
        0x102d50c0 => { vm.decode(0x890c875u32); let d = 21i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d50c4u64) }
        0x102d50c4 => { vm.decode(0x2084402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d50c8u64) }
        0x102d50c8 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d50ccu64) }
        0x102d50cc => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d50d0u64) }
        0x102d50d0 => { vm.decode(0x8b185530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d50d4u64) }
        0x102d50d4 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d50d8u64) }
        0x102d50d8 => { vm.decode(0x4086636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d50dcu64) }
        0x102d50dc => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d50e0u64) }
        0x102d50e0 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d50e4u64) }
        0x102d50e4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d50e8u64) }
        0x102d50e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d50ecu64 + 4);
        vm.wr(37, 0x102d50ecu64 + 4);
        Ok(0x102d50ecu64) }
        0x102d50ec => { vm.decode(0x990055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d50f0u64) }
        0x102d50f0 => { vm.decode(0x704b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d50f4u64) }
        0x102d50f4 => { vm.decode(0x2084412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d50f8u64) }
        0x102d50f8 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d50fcu64) }
        0x102d50fc => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5100u64) }
        0x102d5100 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5104u64) }
        0x102d5104 => { vm.decode(0x8b181430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5108u64) }
        0x102d5108 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d510cu64) }
        0x102d510c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5110u64) }
        0x102d5110 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5114u64) }
        0x102d5114 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5118u64 + 4);
        vm.wr(37, 0x102d5118u64 + 4);
        Ok(0x102d5118u64) }
        0x102d5118 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d511cu64) }
        0x102d511c => { vm.decode(0x724b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5120u64) }
        0x102d5120 => { vm.decode(0x8059u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5124u64) }
        0x102d5124 => { vm.decode(0x628e632fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5128u64) }
        0x102d5128 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d512cu64) }
        0x102d512c => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5130u64) }
        0x102d5130 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5134u64) }
        0x102d5134 => { vm.decode(0x9b185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5138u64) }
        0x102d5138 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d513cu64) }
        0x102d513c => { vm.decode(0x2086432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5140u64) }
        0x102d5140 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5144u64) }
        0x102d5144 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5148u64 + 4);
        vm.wr(37, 0x102d5148u64 + 4);
        Ok(0x102d5148u64) }
        0x102d5148 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d514cu64) }
        0x102d514c => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5150u64) }
        0x102d5150 => { vm.decode(0x192854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5154u64) }
        0x102d5154 => { vm.decode(0x720b0119u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5158u64) }
        0x102d5158 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d515cu64) }
        0x102d515c => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5160u64) }
        0x102d5160 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5164u64) }
        0x102d5164 => { vm.decode(0x28c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5168u64) }
        0x102d5168 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d516cu64) }
        0x102d516c => { vm.decode(0x8938dcb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5170u64) }
        0x102d5170 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5174u64) }
        0x102d5174 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5178u64) }
        0x102d5178 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d517cu64) }
        0x102d517c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d5180u64 + 4);
        vm.wr(37, 0x102d5180u64 + 4);
        Ok(0x102d5180u64) }
        0x102d5180 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5184u64) } }
        0x102d5184 => { vm.decode(0x725b9019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5188u64) }
        0x102d5188 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d518cu64) }
        0x102d518c => { vm.decode(0x2000216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5190u64) }
        0x102d5190 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5194u64) }
        0x102d5194 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5198u64) }
        0x102d5198 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d519cu64) }
        0x102d519c => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d51a0u64) }
        0x102d51a0 => { vm.decode(0x9938ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d51a4u64) }
        0x102d51a4 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d51a8u64) }
        0x102d51a8 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d51acu64) }
        0x102d51ac => { vm.decode(0x2086636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d51b0u64) }
        0x102d51b0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d51b4u64) }
        0x102d51b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d51b8u64 + 4);
        vm.wr(37, 0x102d51b8u64 + 4);
        Ok(0x102d51b8u64) }
        0x102d51b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d51bcu64) } }
        0x102d51bc => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d51c0u64) }
        0x102d51c0 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d51c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d51c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d51c4u64) }
        0x102d51c4 => { vm.decode(0xa00c4bb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -256i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d51c8u64) }
        0x102d51c8 => { vm.decode(0x400141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d51ccu64) }
        0x102d51d0 => { vm.decode(0x110019b5u32); let d = 1i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d51d4u64) }
        0x102d51d4 => { vm.decode(0x4360bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d51d8u64) }
        0x102d51d8 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d51dcu64) }
        0x102d51dc => { vm.decode(0x20ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d51e0u64) }
        0x102d51e0 => { vm.decode(0x560207f5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d51e4u64) }
        0x102d51e4 => { vm.decode(0x4089875u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d51e8u64) }
        0x102d51e8 => { vm.decode(0x40c30bb5u32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d51ecu64) }
        0x102d51ec => { vm.decode(0xe84124f5u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d51f0u64) }
        0x102d51f0 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d51f4u64) }
        0x102d51f4 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d51f8u64) }
        0x102d51f8 => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d51fcu64) }
        0x102d51fc => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5200u64).wrapping_add((imm * 4) as u64) } else { 0x102d5200u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5200u64) }
        0x102d5200 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5204u64) } }
        0x102d5204 => { vm.decode(0x7818e7b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d5208u64) }
        0x102d5208 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d520cu64) }
        0x102d520c => { vm.decode(0x44290fb5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5210u64) }
        0x102d5210 => { vm.decode(0x40217f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5214u64) }
        0x102d5214 => { vm.decode(0xa8088b35u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5218u64) }
        0x102d5218 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d521cu64) }
        0x102d521c => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5220u64) }
        0x102d5220 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5224u64) }
        0x102d5224 => { vm.decode(0x21020446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5228u64) }
        0x102d5228 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d522cu64).wrapping_add((imm * 4) as u64) } else { 0x102d522cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d522cu64) }
        0x102d522c => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5230u64) }
        0x102d5230 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5234u64) }
        0x102d5234 => { vm.decode(0xd2cac12du32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5238u64) }
        0x102d5238 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d523cu64) }
        0x102d523c => { vm.decode(0xa0004118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5240u64) }
        0x102d5240 => { vm.decode(0x21020446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5244u64) }
        0x102d5244 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5248u64).wrapping_add((imm * 4) as u64) } else { 0x102d5248u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5248u64) }
        0x102d5248 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d524cu64) } }
        0x102d524c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5250u64) }
        0x102d5250 => { vm.decode(0x3020446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5254u64) }
        0x102d5254 => { vm.decode(0x739c3ea7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -34i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5258u64).wrapping_add((imm * 4) as u64) } else { 0x102d5258u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5258u64) }
        0x102d5258 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d525cu64) }
        0x102d525c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5260u64) }
        0x102d5260 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5264u64) }
        0x102d5264 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5268u64) }
        0x102d5268 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d526cu64) }
        0x102d526c => { vm.decode(0x3020446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5270u64) }
        0x102d5270 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5274u64).wrapping_add((imm * 4) as u64) } else { 0x102d5274u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5274u64) }
        0x102d5274 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5278u64) } }
        0x102d5278 => { vm.decode(0x520001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d527cu64) }
        0x102d527c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5280u64) } }
        0x102d5280 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5284u64) }
        0x102d5284 => { vm.decode(0x408e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5288u64) }
        0x102d5288 => { vm.decode(0x628a6b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d528cu64) }
        0x102d528c => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5290u64) }
        0x102d5290 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5294u64).wrapping_add((imm * 4) as u64) } else { 0x102d5294u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5294u64) }
        0x102d5294 => { vm.decode(0xb40c6ff6u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5298u64) }
        0x102d5298 => { vm.decode(0x25a0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1636i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d529cu64) }
        0x102d52a0 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d52a4u64) }
        0x102d52a4 => { vm.decode(0x4760bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d52a8u64) }
        0x102d52a8 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d52acu64) }
        0x102d52ac => { vm.decode(0x20ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d52b0u64) }
        0x102d52b0 => { vm.decode(0x140147f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d52b4u64) }
        0x102d52b4 => { vm.decode(0x8c875u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d52b8u64) }
        0x102d52b8 => { vm.decode(0x624b0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d52bcu64) }
        0x102d52bc => { vm.decode(0x802224f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d52c0u64) }
        0x102d52c0 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d52c4u64) }
        0x102d52c4 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d52c8u64) }
        0x102d52c8 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d52ccu64) }
        0x102d52cc => { vm.decode(0x10048027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 65i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d52d0u64).wrapping_add((imm * 4) as u64) } else { 0x102d52d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d52d0u64) }
        0x102d52d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d52d4u64) } }
        0x102d52d4 => { vm.decode(0x3018e7b5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d52d8u64) }
        0x102d52d8 => { vm.decode(0x50920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d52dcu64) }
        0x102d52dc => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d52e0u64) }
        0x102d52e0 => { vm.decode(0x60217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d52e4u64) }
        0x102d52e4 => { vm.decode(0x200c0b35u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d52e8u64) }
        0x102d52e8 => { vm.decode(0x6124f5u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d52ecu64) }
        0x102d52ec => { vm.decode(0xa00023eu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d52f0u64) }
        0x102d52f0 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d52f4u64) }
        0x102d52f4 => { vm.decode(0x23020446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d52f8u64) }
        0x102d52f8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d52fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d52fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d52fcu64) }
        0x102d52fc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5300u64) }
        0x102d5300 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5304u64) }
        0x102d5304 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5308u64) }
        0x102d5308 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d530cu64) }
        0x102d530c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5310u64) }
        0x102d5310 => { vm.decode(0x23020446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5314u64) }
        0x102d5314 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5318u64).wrapping_add((imm * 4) as u64) } else { 0x102d5318u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5318u64) }
        0x102d5318 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d531cu64) } }
        0x102d531c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5320u64) }
        0x102d5320 => { vm.decode(0x80020446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5324u64) }
        0x102d5324 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5328u64).wrapping_add((imm * 4) as u64) } else { 0x102d5328u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5328u64) }
        0x102d5328 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d532cu64) }
        0x102d532c => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5330u64) }
        0x102d5330 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5334u64) }
        0x102d5334 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5338u64) }
        0x102d5338 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d533cu64) }
        0x102d533c => { vm.decode(0x80020446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5340u64) }
        0x102d5340 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5344u64).wrapping_add((imm * 4) as u64) } else { 0x102d5344u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5344u64) }
        0x102d5344 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5348u64) } }
        0x102d5348 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d534cu64) }
        0x102d534c => { vm.decode(0x4770bb5u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5350u64) }
        0x102d5350 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5354u64) }
        0x102d5354 => { vm.decode(0x2021ad35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d5358u64) }
        0x102d5358 => { vm.decode(0x121347f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d535cu64) }
        0x102d535c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5360u64) }
        0x102d5360 => { vm.decode(0x70bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5364u64) }
        0x102d5364 => { vm.decode(0x8a2324f5u32); let d = 6i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5368u64) }
        0x102d5368 => { vm.decode(0x40220033u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d536cu64) }
        0x102d536c => { vm.decode(0x226124f5u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5370u64) }
        0x102d5370 => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5374u64) }
        0x102d5374 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5378u64) }
        0x102d5378 => { vm.decode(0x23020446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d537cu64) }
        0x102d537c => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5380u64).wrapping_add((imm * 4) as u64) } else { 0x102d5380u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5380u64) }
        0x102d5380 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5384u64) }
        0x102d5384 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5388u64) }
        0x102d5388 => { vm.decode(0xd2ca416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d538cu64) }
        0x102d538c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5390u64) }
        0x102d5390 => { vm.decode(0x80204118u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5394u64) }
        0x102d5394 => { vm.decode(0x23020446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5398u64) }
        0x102d5398 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d539cu64).wrapping_add((imm * 4) as u64) } else { 0x102d539cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d539cu64) }
        0x102d539c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d53a0u64) } }
        0x102d53a0 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d53a4u64) }
        0x102d53a4 => { vm.decode(0x80020446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d53a8u64) }
        0x102d53a8 => { vm.decode(0x679cb6a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -67i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d53acu64).wrapping_add((imm * 4) as u64) } else { 0x102d53acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d53acu64) }
        0x102d53ac => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d53b0u64) }
        0x102d53b0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d53b4u64) }
        0x102d53b4 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d53b8u64) }
        0x102d53b8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d53bcu64) }
        0x102d53bc => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d53c0u64) }
        0x102d53c0 => { vm.decode(0x80020446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d53c4u64) }
        0x102d53c4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d53c8u64).wrapping_add((imm * 4) as u64) } else { 0x102d53c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d53c8u64) }
        0x102d53c8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d53ccu64) } }
        0x102d53cc => { vm.decode(0x500141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1328i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d53d0u64) }
        0x102d53d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d53d4u64) } }
        0x102d53d4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d53d8u64) }
        0x102d53d8 => { vm.decode(0x628a432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d53dcu64) }
        0x102d53dc => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d53e0u64) }
        0x102d53e0 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d53e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d53e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d53e4u64) }
        0x102d53e4 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d53e8u64) }
        0x102d53e8 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d53ecu64) }
        0x102d53ec => { vm.decode(0x848103eu32); let d = 4i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d53f0u64) }
        0x102d53f0 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d53f4u64) }
        0x102d53f4 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d53f8u64) }
        0x102d53f8 => { vm.decode(0x32811099u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d53fcu64) }
        0x102d53fc => { vm.decode(0x30000099u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5400u64) }
        0x102d5400 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5404u64) }
        0x102d5404 => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d5408u64) }
        0x102d5408 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d540cu64).wrapping_add((imm * 4) as u64) } else { 0x102d540cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d540cu64) }
        0x102d540c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5410u64) }
        0x102d5410 => { vm.decode(0xc0801118u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5414u64) }
        0x102d5414 => { vm.decode(0xe057b119u32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 254i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5418u64) }
        0x102d5418 => { vm.decode(0xe2041099u32); let d = 6i16; let s = 17i16; let s2 = -1i16; let imm = 11i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d541cu64) }
        0x102d541c => { vm.decode(0x2020036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5420u64) }
        0x102d5420 => { vm.decode(0x200408eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -2139095040i64;
        vm.st(d, imm as u64);
        Ok(0x102d5424u64) }
        0x102d5424 => { vm.decode(0xa0384118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 32897i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5428u64) }
        0x102d5428 => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d542cu64) }
        0x102d542c => { vm.decode(0x82002171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 78i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5430u64).wrapping_add((imm * 4) as u64) } else { 0x102d5430u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5430u64) }
        0x102d5430 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5434u64) }
        0x102d5434 => { vm.decode(0xa800441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2204i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d5438u64) }
        0x102d543c => { vm.decode(0x108a016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 15i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5440u64) }
        0x102d5440 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d5444u64) }
        0x102d5444 => { vm.decode(0x714b0159u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5448u64) }
        0x102d5448 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d544cu64) }
        0x102d544c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5450u64) }
        0x102d5450 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5454u64) }
        0x102d5454 => { vm.decode(0xd5bfcdfeu32); let d = 24i16; let s = 2i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5458u64) }
        0x102d5458 => { vm.decode(0x95140fb5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d545cu64) }
        0x102d545c => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5460u64) }
        0x102d5460 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d5464u64) }
        0x102d5464 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5468u64) }
        0x102d5468 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d546cu64) }
        0x102d546c => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5470u64) }
        0x102d5470 => { vm.decode(0x21880135u32); let d = 32i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let p = (vm.ld(s) as u32 as u64).wrapping_mul(vm.ld(s2) as u32 as u64);
        vm.st(d, (p as u32 as i32 as i64) as u64);
        vm.st(d.wrapping_add(1), ((p >> 32) as u32 as i32 as i64) as u64);
        Ok(0x102d5474u64) }
        0x102d5474 => { vm.decode(0x100401f5u32); let d = 24i16; let s = 33i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5478u64) }
        0x102d5478 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d547cu64) }
        0x102d547c => { vm.decode(0xccf6ffd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5480u64) }
        0x102d5480 => { vm.decode(0x11858019u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5484u64) }
        0x102d5484 => { vm.decode(0x2413036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5488u64) }
        0x102d5488 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d548cu64) }
        0x102d548c => { vm.decode(0x44684d35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 10i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d5490u64) }
        0x102d5490 => { vm.decode(0xc400ebb5u32); let d = 16i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5494u64) }
        0x102d5494 => { vm.decode(0xc500abb5u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d5498u64) }
        0x102d5498 => { vm.decode(0x4f000b35u32); let d = 24i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d549cu64) }
        0x102d549c => { vm.decode(0xd4000fb5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d54a0u64) }
        0x102d54a0 => { vm.decode(0x9129036u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54a4u64) }
        0x102d54a4 => { vm.decode(0x129036u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54a8u64) }
        0x102d54a8 => { vm.decode(0x8109236u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54acu64) }
        0x102d54ac => { vm.decode(0x10b236u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54b0u64) }
        0x102d54b0 => { vm.decode(0xa121036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54b4u64) }
        0x102d54b4 => { vm.decode(0x8a65c909u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54b8u64) }
        0x102d54b8 => { vm.decode(0x692704f5u32); let d = 22i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d54bcu64) }
        0x102d54bc => { vm.decode(0xa4c103eu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d54c0u64) }
        0x102d54c0 => { vm.decode(0x535da835u32); let d = 15i16; let s = 15i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d54c4u64) }
        0x102d54c4 => { vm.decode(0xe0630433u32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d54c8u64) }
        0x102d54c8 => { vm.decode(0xeb274909u32); let d = 15i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54ccu64) }
        0x102d54cc => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54d0u64) }
        0x102d54d0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54d4u64) }
        0x102d54d4 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54d8u64) }
        0x102d54d8 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54dcu64) }
        0x102d54dc => { vm.decode(0xa101036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d54e0u64) }
        0x102d54e0 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d54e4u64) }
        0x102d54e4 => { vm.decode(0x212704f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d54e8u64) }
        0x102d54e8 => { vm.decode(0x844103eu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d54ecu64) }
        0x102d54ec => { vm.decode(0x131d8835u32); let d = 14i16; let s = 14i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d54f0u64) }
        0x102d54f0 => { vm.decode(0x606104b3u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d54f4u64) }
        0x102d54f4 => { vm.decode(0x298314f5u32); let d = 14i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d54f8u64) }
        0x102d54f8 => { vm.decode(0x218504f5u32); let d = 24i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d54fcu64) }
        0x102d54fc => { vm.decode(0x610504f5u32); let d = 16i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5500u64) }
        0x102d5500 => { vm.decode(0xe90504f5u32); let d = 18i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5504u64) }
        0x102d5504 => { vm.decode(0xe8e314f5u32); let d = 15i16; let s = 15i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5508u64) }
        0x102d5508 => { vm.decode(0x23036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d550cu64) }
        0x102d550c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d5510u64) }
        0x102d5510 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5514u64) }
        0x102d5514 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5518u64) }
        0x102d5518 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d551cu64) }
        0x102d551c => { vm.decode(0xc02204b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5520u64) }
        0x102d5520 => { vm.decode(0x2008323eu32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5524u64) }
        0x102d5524 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5528u64) }
        0x102d5528 => { vm.decode(0x40220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d552cu64) }
        0x102d552c => { vm.decode(0x2048003eu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5530u64) }
        0x102d5530 => { vm.decode(0x70002835u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5534u64) }
        0x102d5534 => { vm.decode(0xc0220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5538u64) }
        0x102d5538 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d553cu64) }
        0x102d553c => { vm.decode(0x70042835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5540u64) }
        0x102d5540 => { vm.decode(0x402201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5544u64) }
        0x102d5544 => { vm.decode(0x2040203eu32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5548u64) }
        0x102d5548 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d554cu64) }
        0x102d554c => { vm.decode(0xc02201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d5550u64) }
        0x102d5550 => { vm.decode(0x2008303eu32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5554u64) }
        0x102d5554 => { vm.decode(0x30002835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5558u64) }
        0x102d5558 => { vm.decode(0x40220533u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d555cu64) }
        0x102d555c => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d5560u64) }
        0x102d5560 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5564u64).wrapping_add((imm * 4) as u64) } else { 0x102d5564u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5564u64) }
        0x102d5564 => { vm.decode(0x44b00875u32); let d = 6i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5568u64) }
        0x102d5568 => { vm.decode(0xd5bbddfeu32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d556cu64) }
        0x102d556c => { vm.decode(0xd7bbdd7eu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5570u64) }
        0x102d5570 => { vm.decode(0x320a103eu32); let d = 11i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5574u64) }
        0x102d5574 => { vm.decode(0x200a10beu32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5578u64) }
        0x102d5578 => { vm.decode(0x80a103eu32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d557cu64) }
        0x102d557c => { vm.decode(0xcfbbddfeu32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5580u64) }
        0x102d5580 => { vm.decode(0x380a10beu32); let d = 13i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5584u64) }
        0x102d5584 => { vm.decode(0x2a0a107eu32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5588u64) }
        0x102d5588 => { vm.decode(0x82016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d558cu64) }
        0x102d558c => { vm.decode(0xa7d32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -85i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5590u64).wrapping_add((imm * 4) as u64) } else { 0x102d5590u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5590u64) }
        0x102d5590 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5594u64) } }
        0x102d5594 => { vm.decode(0xd2ca112du32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d5598u64) }
        0x102d5598 => { vm.decode(0xda5a512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d559cu64) }
        0x102d559c => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55a0u64) }
        0x102d55a0 => { vm.decode(0xda4a91adu32); let d = 24i16; let s = 9i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55a4u64) }
        0x102d55a4 => { vm.decode(0xda4e116du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55a8u64) }
        0x102d55a8 => { vm.decode(0xda4a916du32); let d = 8i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55acu64) }
        0x102d55ac => { vm.decode(0xdaca516du32); let d = 9i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55b0u64) }
        0x102d55b0 => { vm.decode(0xdaced16du32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d55b4u64) }
        0x102d55b4 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55b8u64) }
        0x102d55b8 => { vm.decode(0x208134f5u32); let d = 8i16; let s = 8i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55bcu64) }
        0x102d55bc => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55c0u64) }
        0x102d55c0 => { vm.decode(0xa0a003eu32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55c4u64) }
        0x102d55c4 => { vm.decode(0xa0c534f5u32); let d = 25i16; let s = 9i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55c8u64) }
        0x102d55c8 => { vm.decode(0x3204a835u32); let d = 9i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d55ccu64) }
        0x102d55cc => { vm.decode(0x2203eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55d0u64) }
        0x102d55d0 => { vm.decode(0xe04134f5u32); let d = 1i16; let s = 11i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55d4u64) }
        0x102d55d4 => { vm.decode(0x688334f5u32); let d = 14i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55d8u64) }
        0x102d55d8 => { vm.decode(0xa8e134f5u32); let d = 11i16; let s = 13i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55dcu64) }
        0x102d55dc => { vm.decode(0x28a134f5u32); let d = 10i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d55e0u64) }
        0x102d55e0 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55e4u64) }
        0x102d55e4 => { vm.decode(0x80a023eu32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55e8u64) }
        0x102d55e8 => { vm.decode(0x134aa835u32); let d = 15i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d55ecu64) }
        0x102d55ec => { vm.decode(0x242223eu32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55f0u64) }
        0x102d55f0 => { vm.decode(0x220a223eu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55f4u64) }
        0x102d55f4 => { vm.decode(0x80a303eu32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55f8u64) }
        0x102d55f8 => { vm.decode(0x2802023eu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d55fcu64) }
        0x102d55fc => { vm.decode(0x214134f5u32); let d = 1i16; let s = 21i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5600u64) }
        0x102d5600 => { vm.decode(0xa02023eu32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d5604u64) }
        0x102d5604 => { vm.decode(0xaca0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1740i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d5608u64) }
        0x102d5608 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d560cu64) } }
        0x102d560c => { vm.decode(0x1395b2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -12845056i64;
        vm.st(d, imm as u64);
        Ok(0x102d5610u64) }
        0x102d5610 => { vm.decode(0x23592918u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 43256i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5614u64) }
        0x102d5614 => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5618u64) }
        0x102d5618 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d561cu64) }
        0x102d561c => { vm.decode(0x8012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5620u64) }
        0x102d5620 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5624u64) }
        0x102d5624 => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5628u64) }
        0x102d5628 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d562cu64).wrapping_add((imm * 4) as u64) } else { 0x102d562cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d562cu64) }
        0x102d562c => { vm.decode(0x8e766fd9u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5630u64) }
        0x102d5630 => { vm.decode(0x8520401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d5634u64) }
        0x102d5638 => { vm.decode(0x2002a835u32); let d = 1i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d563cu64) }
        0x102d563c => { vm.decode(0x98082d75u32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5640u64) }
        0x102d5640 => { vm.decode(0x20a07f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5644u64) }
        0x102d5644 => { vm.decode(0xc22260f5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1594i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d5648u64) }
        0x102d5648 => { vm.decode(0x140357f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d564cu64) }
        0x102d564c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5650u64) }
        0x102d5650 => { vm.decode(0xc26204f5u32); let d = 5i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5654u64) }
        0x102d5654 => { vm.decode(0x10001430u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5658u64) }
        0x102d5658 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d565cu64) }
        0x102d565c => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5660u64) }
        0x102d5660 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5664u64).wrapping_add((imm * 4) as u64) } else { 0x102d5664u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5664u64) }
        0x102d5664 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5668u64) } }
        0x102d5668 => { vm.decode(0x5818bb5u32); let d = 8i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d566cu64) }
        0x102d566c => { vm.decode(0xa1409d75u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5670u64) }
        0x102d5670 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d5674u64) }
        0x102d5674 => { vm.decode(0x2002012fu32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5678u64) }
        0x102d5678 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d567cu64) }
        0x102d567c => { vm.decode(0xa0020446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5680u64) }
        0x102d5680 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5684u64).wrapping_add((imm * 4) as u64) } else { 0x102d5684u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5684u64) }
        0x102d5684 => { vm.decode(0x2001019u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5688u64) }
        0x102d5688 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d568cu64) }
        0x102d568c => { vm.decode(0x32400d75u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5690u64) }
        0x102d5690 => { vm.decode(0x80204118u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5694u64) }
        0x102d5694 => { vm.decode(0xa0020446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5698u64) }
        0x102d5698 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d569cu64).wrapping_add((imm * 4) as u64) } else { 0x102d569cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d569cu64) }
        0x102d569c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d56a0u64) } }
        0x102d56a0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d56a4u64) }
        0x102d56a4 => { vm.decode(0xa0020446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d56a8u64) }
        0x102d56a8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d56acu64).wrapping_add((imm * 4) as u64) } else { 0x102d56acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d56acu64) }
        0x102d56ac => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d56b0u64) }
        0x102d56b0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d56b4u64) }
        0x102d56b4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d56b8u64) }
        0x102d56b8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d56bcu64) }
        0x102d56bc => { vm.decode(0xa0020446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d56c0u64) }
        0x102d56c0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d56c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d56c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d56c4u64) }
        0x102d56c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d56c8u64) } }
        0x102d56c8 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d56ccu64) }
        0x102d56cc => { vm.decode(0x98482d75u32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d56d0u64) }
        0x102d56d0 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d56d4u64) }
        0x102d56d4 => { vm.decode(0xc22260f5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1594i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d56d8u64) }
        0x102d56d8 => { vm.decode(0x140357f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d56dcu64) }
        0x102d56dc => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d56e0u64) }
        0x102d56e0 => { vm.decode(0x4a02a036u32); let d = 5i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d56e4u64) }
        0x102d56e4 => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d56e8u64) }
        0x102d56e8 => { vm.decode(0x8a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d56ecu64) }
        0x102d56ec => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d56f0u64) }
        0x102d56f0 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d56f4u64) }
        0x102d56f4 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d56f8u64) }
        0x102d56f8 => { vm.decode(0x606104f5u32); let d = 1i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d56fcu64) }
        0x102d56fc => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5700u64) }
        0x102d5700 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5704u64) }
        0x102d5704 => { vm.decode(0xa2020446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5708u64) }
        0x102d5708 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d570cu64).wrapping_add((imm * 4) as u64) } else { 0x102d570cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d570cu64) }
        0x102d570c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5710u64) }
        0x102d5710 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5714u64) }
        0x102d5714 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5718u64) }
        0x102d5718 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d571cu64) }
        0x102d571c => { vm.decode(0xa2020446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5720u64) }
        0x102d5720 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d5724u64).wrapping_add((imm * 4) as u64) } else { 0x102d5724u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5724u64) }
        0x102d5724 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5728u64) } }
        0x102d5728 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d572cu64) }
        0x102d572c => { vm.decode(0xa2020446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5730u64) }
        0x102d5730 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d5734u64).wrapping_add((imm * 4) as u64) } else { 0x102d5734u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d5734u64) }
        0x102d5734 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d5738u64) }
        0x102d5738 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d573cu64) }
        0x102d573c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5740u64) }
        0x102d5740 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d5744u64) }
        0x102d5744 => { vm.decode(0xa2020446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d5748u64) }
        0x102d5748 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d574cu64).wrapping_add((imm * 4) as u64) } else { 0x102d574cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d574cu64) }
        0x102d574c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d5750u64) } }
        0x102d5750 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5754u64) }
        0x102d5754 => { vm.decode(0x89483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 6i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5758u64) }
        0x102d5758 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d575cu64) }
        0x102d575c => { vm.decode(0xa23220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1594i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d5760u64) }
        0x102d5760 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5764u64) }
        0x102d5764 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d5768u64) }
        0x102d5768 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d576cu64) }
        0x102d576c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d5770u64) }
        0x102d5770 => { vm.decode(0x40022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d5774u64) }
        0x102d5774 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d5778u64) }
        0x102d5778 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d577cu64) }
        0x102d577c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d5780u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
