#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102887a4 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102887a8u64) }
        0x102887a8 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102887acu64) }
        0x102887ac => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102887b0u64) }
        0x102887b0 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102887b4u64) }
        0x102887b4 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102887b8u64) }
        0x102887b8 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102887bcu64) }
        0x102887bc => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102887c0u64) }
        0x102887c0 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102887c4u64) }
        0x102887c4 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102887c8u64) }
        0x102887c8 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102887ccu64) }
        0x102887cc => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102887d0u64) }
        0x102887d0 => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102887d4u64) }
        0x102887d4 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102887d8u64) }
        0x102887d8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102887dcu64) }
        0x102887dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102887e0u64 + 4);
        vm.wr(37, 0x102887e0u64 + 4);
        Ok(0x102887e0u64) }
        0x102887e0 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102887e4u64) }
        0x102887e4 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102887e8u64) }
        0x102887e8 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102887ecu64) }
        0x102887ec => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102887f0u64) }
        0x102887f0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102887f4u64) }
        0x102887f4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102887f8u64).wrapping_add((imm * 4) as u64) } else { 0x102887f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102887f8u64) }
        0x102887f8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102887fcu64) }
        0x102887fc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288800u64) }
        0x10288800 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288804u64) }
        0x10288804 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288808u64) }
        0x10288808 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028880cu64) }
        0x1028880c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288810u64).wrapping_add((imm * 4) as u64) } else { 0x10288810u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288810u64) }
        0x10288810 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288814u64) } }
        0x10288814 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288818u64) }
        0x10288818 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028881cu64) }
        0x1028881c => { vm.decode(0x731cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288820u64).wrapping_add((imm * 4) as u64) } else { 0x10288820u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288820u64) }
        0x10288820 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288824u64) }
        0x10288824 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288828u64) }
        0x10288828 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028882cu64) }
        0x1028882c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288830u64) }
        0x10288830 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288834u64) }
        0x10288834 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288838u64).wrapping_add((imm * 4) as u64) } else { 0x10288838u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288838u64) }
        0x10288838 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028883cu64) } }
        0x1028883c => { vm.decode(0xa9a0111du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9196i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288840u64) }
        0x10288840 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288844u64) } }
        0x10288844 => { vm.decode(0xb381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288848u64) }
        0x10288848 => { vm.decode(0xb201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028884cu64) }
        0x1028884c => { vm.decode(0xb301c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288850u64) }
        0x10288850 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288854u64) }
        0x10288854 => { vm.decode(0x621d1062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288858u64) }
        0x10288858 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028885cu64) }
        0x1028885c => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288860u64) }
        0x10288860 => { vm.decode(0x8056e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8639i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288864u64) }
        0x10288864 => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288868u64) }
        0x10288868 => { vm.decode(0xdc565f59u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -1030i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028886cu64) }
        0x1028886c => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288870u64) }
        0x10288870 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10288874u64) }
        0x10288874 => { vm.decode(0x2202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288878u64).wrapping_add((imm * 4) as u64) } else { 0x10288878u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288878u64) }
        0x10288878 => { vm.decode(0x9c76ffd9u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028887cu64) }
        0x1028887c => { vm.decode(0x2d84551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 16188i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288880u64) }
        0x10288884 => { vm.decode(0x60453835u32); let d = 1i16; let s = 7i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288888u64) }
        0x10288888 => { vm.decode(0x44081ff5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x1028888cu64) }
        0x1028888c => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288890u64) }
        0x10288890 => { vm.decode(0x90350175u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x10288894u64) }
        0x10288894 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288898u64) }
        0x10288898 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028889cu64) }
        0x1028889c => { vm.decode(0x8b214909u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102888a0u64) }
        0x102888a0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102888a4u64) }
        0x102888a4 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888a8u64) }
        0x102888a8 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102888acu64) }
        0x102888ac => { vm.decode(0x106800b0u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102888b0u64) }
        0x102888b0 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102888b4u64) }
        0x102888b4 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102888b8u64) }
        0x102888b8 => { vm.decode(0x21400a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1664i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102888bcu64).wrapping_add((imm * 4) as u64) } else { 0x102888bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102888bcu64) }
        0x102888bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102888c0u64) } }
        0x102888c0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888c4u64) }
        0x102888c4 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888c8u64) }
        0x102888c8 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102888ccu64) }
        0x102888cc => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102888d0u64) }
        0x102888d0 => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888d4u64) }
        0x102888d4 => { vm.decode(0x414124f5u32); let d = 1i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102888d8u64) }
        0x102888d8 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102888dcu64) }
        0x102888dc => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888e0u64) }
        0x102888e0 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102888e4u64) }
        0x102888e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102888e8u64 + 4);
        vm.wr(37, 0x102888e8u64 + 4);
        Ok(0x102888e8u64) }
        0x102888e8 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102888ecu64) }
        0x102888ec => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102888f0u64) }
        0x102888f0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102888f4u64) }
        0x102888f4 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102888f8u64) }
        0x102888f8 => { vm.decode(0x90a0b75u32); let d = 4i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102888fcu64) }
        0x102888fc => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288900u64) }
        0x10288900 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288904u64) }
        0x10288904 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288908u64).wrapping_add((imm * 4) as u64) } else { 0x10288908u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288908u64) }
        0x10288908 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028890cu64) }
        0x1028890c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288910u64) }
        0x10288910 => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288914u64) }
        0x10288914 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288918u64) }
        0x10288918 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028891cu64) }
        0x1028891c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288920u64).wrapping_add((imm * 4) as u64) } else { 0x10288920u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288920u64) }
        0x10288920 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288924u64) } }
        0x10288924 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288928u64) }
        0x10288928 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028892cu64) }
        0x1028892c => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288930u64).wrapping_add((imm * 4) as u64) } else { 0x10288930u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288930u64) }
        0x10288930 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288934u64) }
        0x10288934 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288938u64) }
        0x10288938 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028893cu64) }
        0x1028893c => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288940u64) }
        0x10288940 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288944u64) }
        0x10288944 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288948u64).wrapping_add((imm * 4) as u64) } else { 0x10288948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288948u64) }
        0x10288948 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028894cu64) } }
        0x1028894c => { vm.decode(0x2400111du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9476i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288950u64) }
        0x10288950 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288954u64) } }
        0x10288954 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288958u64) }
        0x10288958 => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028895cu64) }
        0x1028895c => { vm.decode(0x824404f5u32); let d = 17i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288960u64) }
        0x10288960 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288964u64) }
        0x10288964 => { vm.decode(0x4c008191u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288968u64) }
        0x10288968 => { vm.decode(0x200092fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028896cu64) }
        0x1028896c => { vm.decode(0x42004b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10288970u64) }
        0x10288970 => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288974u64) }
        0x10288974 => { vm.decode(0x660d1059u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288978u64) }
        0x10288978 => { vm.decode(0x9119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028897cu64) }
        0x1028897c => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288980u64) }
        0x10288980 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288984u64) }
        0x10288984 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288988u64).wrapping_add((imm * 4) as u64) } else { 0x10288988u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288988u64) }
        0x10288988 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028898cu64) }
        0x1028898c => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288990u64) }
        0x10288990 => { vm.decode(0x41119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 72i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288994u64) }
        0x10288994 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288998u64) }
        0x10288998 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028899cu64) }
        0x1028899c => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102889a0u64) }
        0x102889a0 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102889a4u64) }
        0x102889a4 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102889a8u64).wrapping_add((imm * 4) as u64) } else { 0x102889a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102889a8u64) }
        0x102889a8 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889acu64) }
        0x102889ac => { vm.decode(0x9209470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889b0u64) }
        0x102889b0 => { vm.decode(0x43657022u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102889b4u64) }
        0x102889b4 => { vm.decode(0xb301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889b8u64) }
        0x102889b8 => { vm.decode(0x501119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 112i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102889bcu64) }
        0x102889bc => { vm.decode(0x169019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102889c0u64) }
        0x102889c0 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102889c4u64) }
        0x102889c4 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102889c8u64) }
        0x102889c8 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102889ccu64).wrapping_add((imm * 4) as u64) } else { 0x102889ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102889ccu64) }
        0x102889cc => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889d0u64) }
        0x102889d0 => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889d4u64) }
        0x102889d4 => { vm.decode(0xb209cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889d8u64) }
        0x102889d8 => { vm.decode(0xb389c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889dcu64) }
        0x102889dc => { vm.decode(0xb309c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889e0u64) }
        0x102889e0 => { vm.decode(0xb289c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889e4u64) }
        0x102889e4 => { vm.decode(0xb209c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889e8u64) }
        0x102889e8 => { vm.decode(0xb189cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889ecu64) }
        0x102889ec => { vm.decode(0xb109cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889f0u64) }
        0x102889f0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102889f4u64) }
        0x102889f4 => { vm.decode(0x2204016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102889f8u64) }
        0x102889f8 => { vm.decode(0x931dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102889fcu64) }
        0x102889fc => { vm.decode(0x19299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288a00u64) }
        0x10288a00 => { vm.decode(0x620f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a04u64) }
        0x10288a04 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a08u64) }
        0x10288a08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288a0cu64 + 4);
        vm.wr(37, 0x10288a0cu64 + 4);
        Ok(0x10288a0cu64) }
        0x10288a0c => { vm.decode(0x938d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288a10u64) }
        0x10288a10 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a14u64) }
        0x10288a14 => { vm.decode(0x39281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288a18u64) }
        0x10288a18 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a1cu64) }
        0x10288a1c => { vm.decode(0x50160119u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a20u64) }
        0x10288a20 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288a24u64) }
        0x10288a24 => { vm.decode(0x10009119u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a28u64) }
        0x10288a28 => { vm.decode(0x8044e499u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 10271i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a2cu64) }
        0x10288a2c => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288a30u64) }
        0x10288a30 => { vm.decode(0x56346999u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 26221i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a34u64) }
        0x10288a34 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a38u64) }
        0x10288a38 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288a3cu64) }
        0x10288a3c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288a40u64).wrapping_add((imm * 4) as u64) } else { 0x10288a40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288a40u64) }
        0x10288a40 => { vm.decode(0x9e76efd9u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288a44u64) }
        0x10288a44 => { vm.decode(0x84a4511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 16320i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288a48u64) }
        0x10288a4c => { vm.decode(0x6041b835u32); let d = 1i16; let s = 7i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288a50u64) }
        0x10288a50 => { vm.decode(0x4e081ff5u32); let d = 4i16; let s = 30i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x10288a54u64) }
        0x10288a54 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288a58u64) }
        0x10288a58 => { vm.decode(0x94352175u32); let d = 3i16; let s = 30i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x10288a5cu64) }
        0x10288a5c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288a60u64) }
        0x10288a60 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288a64u64) }
        0x10288a64 => { vm.decode(0x492104f5u32); let d = 2i16; let s = 16i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288a68u64) }
        0x10288a68 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288a6cu64) }
        0x10288a6c => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288a70u64) }
        0x10288a70 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288a74u64) }
        0x10288a74 => { vm.decode(0x61c0827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1586i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288a78u64).wrapping_add((imm * 4) as u64) } else { 0x10288a78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288a78u64) }
        0x10288a78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288a7cu64) } }
        0x10288a7c => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288a80u64) }
        0x10288a80 => { vm.decode(0xe000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288a84u64) }
        0x10288a84 => { vm.decode(0x616104f5u32); let d = 1i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288a88u64) }
        0x10288a88 => { vm.decode(0x400a092fu32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a8cu64) }
        0x10288a8c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a90u64) }
        0x10288a90 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a94u64) }
        0x10288a94 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288a98u64) }
        0x10288a98 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288a9cu64) }
        0x10288a9c => { vm.decode(0xb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288aa0u64) }
        0x10288aa0 => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288aa4u64) }
        0x10288aa4 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288aa8u64) }
        0x10288aa8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288aacu64) }
        0x10288aac => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288ab0u64) }
        0x10288ab0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288ab4u64 + 4);
        vm.wr(37, 0x10288ab4u64 + 4);
        Ok(0x10288ab4u64) }
        0x10288ab4 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288ab8u64) }
        0x10288ab8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288abcu64) }
        0x10288abc => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288ac0u64) }
        0x10288ac0 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288ac4u64) }
        0x10288ac4 => { vm.decode(0x92a0b75u32); let d = 4i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288ac8u64) }
        0x10288ac8 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288accu64) }
        0x10288acc => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288ad0u64) }
        0x10288ad0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288ad4u64).wrapping_add((imm * 4) as u64) } else { 0x10288ad4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288ad4u64) }
        0x10288ad4 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288ad8u64) }
        0x10288ad8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288adcu64) }
        0x10288adc => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288ae0u64) }
        0x10288ae0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288ae4u64) }
        0x10288ae4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288ae8u64) }
        0x10288ae8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288aecu64).wrapping_add((imm * 4) as u64) } else { 0x10288aecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288aecu64) }
        0x10288aec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288af0u64) } }
        0x10288af0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288af4u64) }
        0x10288af4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288af8u64) }
        0x10288af8 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288afcu64).wrapping_add((imm * 4) as u64) } else { 0x10288afcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288afcu64) }
        0x10288afc => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b00u64) }
        0x10288b00 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288b04u64) }
        0x10288b04 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288b08u64) }
        0x10288b08 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288b0cu64) }
        0x10288b0c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288b10u64) }
        0x10288b10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288b14u64).wrapping_add((imm * 4) as u64) } else { 0x10288b14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288b14u64) }
        0x10288b14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288b18u64) } }
        0x10288b18 => { vm.decode(0xaca0011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9932i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288b1cu64) }
        0x10288b1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288b20u64) } }
        0x10288b20 => { vm.decode(0x6a8063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b24u64) }
        0x10288b24 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b28u64) }
        0x10288b28 => { vm.decode(0x10001059u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b2cu64) }
        0x10288b2c => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b30u64) }
        0x10288b30 => { vm.decode(0x8b1954b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b34u64) }
        0x10288b34 => { vm.decode(0x742d0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b38u64) }
        0x10288b38 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b3cu64) }
        0x10288b3c => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b40u64) }
        0x10288b40 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b44u64) }
        0x10288b44 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b48u64) }
        0x10288b48 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288b4cu64) }
        0x10288b4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288b50u64 + 4);
        vm.wr(37, 0x10288b50u64 + 4);
        Ok(0x10288b50u64) }
        0x10288b50 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b54u64) }
        0x10288b54 => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b58u64) }
        0x10288b58 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b5cu64) }
        0x10288b5c => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b60u64) }
        0x10288b60 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10288b64u64) }
        0x10288b64 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b68u64) }
        0x10288b68 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288b6cu64) }
        0x10288b6c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b70u64) }
        0x10288b70 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288b74u64) }
        0x10288b74 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288b78u64 + 4);
        vm.wr(37, 0x10288b78u64 + 4);
        Ok(0x10288b78u64) }
        0x10288b78 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288b7cu64) }
        0x10288b7c => { vm.decode(0x8424a6d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 7951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b80u64) }
        0x10288b80 => { vm.decode(0xa0422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288b84u64) }
        0x10288b84 => { vm.decode(0x4c168f59u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -9240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b88u64) }
        0x10288b88 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288b8cu64) }
        0x10288b8c => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288b90u64) }
        0x10288b90 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b94u64) }
        0x10288b94 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288b98u64) }
        0x10288b98 => { vm.decode(0x4d184211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288b9cu64) }
        0x10288b9c => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288ba0u64) }
        0x10288ba0 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288ba4u64).wrapping_add((imm * 4) as u64) } else { 0x10288ba4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288ba4u64) }
        0x10288ba4 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288ba8u64) }
        0x10288bb0 => { vm.decode(0x8224735u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288bb4u64) }
        0x10288bb4 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288bb8u64) }
        0x10288bb8 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288bbcu64) }
        0x10288bbc => { vm.decode(0x64304675u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288bc0u64) }
        0x10288bc0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288bc4u64) }
        0x10288bc4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288bc8u64) }
        0x10288bc8 => { vm.decode(0xcb21a4f5u32); let d = 2i16; let s = 23i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288bccu64) }
        0x10288bcc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288bd0u64) }
        0x10288bd0 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288bd4u64) }
        0x10288bd4 => { vm.decode(0x4d184211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288bd8u64) }
        0x10288bd8 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288bdcu64) }
        0x10288bdc => { vm.decode(0x29c0227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1564i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288be0u64).wrapping_add((imm * 4) as u64) } else { 0x10288be0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288be0u64) }
        0x10288be0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288be4u64) } }
        0x10288be4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288be8u64) }
        0x10288be8 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288becu64) }
        0x10288bec => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288bf0u64) }
        0x10288bf0 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288bf4u64) }
        0x10288bf4 => { vm.decode(0x8b111530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288bf8u64) }
        0x10288bf8 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288bfcu64) }
        0x10288bfc => { vm.decode(0xc040bd75u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288c00u64) }
        0x10288c00 => { vm.decode(0xc16104f5u32); let d = 1i16; let s = 16i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288c04u64) }
        0x10288c04 => { vm.decode(0x14000111u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288c08u64) }
        0x10288c08 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c0cu64) }
        0x10288c0c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c10u64) }
        0x10288c10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288c14u64 + 4);
        vm.wr(37, 0x10288c14u64 + 4);
        Ok(0x10288c14u64) }
        0x10288c14 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288c18u64) }
        0x10288c18 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288c1cu64) }
        0x10288c1c => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288c20u64) }
        0x10288c20 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c24u64) }
        0x10288c24 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288c28u64) }
        0x10288c28 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288c2cu64).wrapping_add((imm * 4) as u64) } else { 0x10288c2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288c2cu64) }
        0x10288c2c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c30u64) }
        0x10288c30 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c34u64) }
        0x10288c34 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288c38u64) }
        0x10288c38 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288c3cu64) }
        0x10288c3c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288c40u64) }
        0x10288c40 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288c44u64).wrapping_add((imm * 4) as u64) } else { 0x10288c44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288c44u64) }
        0x10288c44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288c48u64) } }
        0x10288c48 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c4cu64) }
        0x10288c4c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288c50u64) }
        0x10288c50 => { vm.decode(0x739cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288c54u64).wrapping_add((imm * 4) as u64) } else { 0x10288c54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288c54u64) }
        0x10288c54 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c58u64) }
        0x10288c58 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c5cu64) }
        0x10288c5c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288c60u64) }
        0x10288c60 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288c64u64) }
        0x10288c64 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288c68u64) }
        0x10288c68 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288c6cu64).wrapping_add((imm * 4) as u64) } else { 0x10288c6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288c6cu64) }
        0x10288c6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288c70u64) } }
        0x10288c70 => { vm.decode(0x100451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 10288i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288c74u64) }
        0x10288c74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288c78u64) } }
        0x10288c78 => { vm.decode(0x803a036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 15i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288c7cu64) }
        0x10288c7c => { vm.decode(0x28c4befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c80u64) }
        0x10288c80 => { vm.decode(0x86584531u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 5064i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288c84u64).wrapping_add((imm * 4) as u64) } else { 0x10288c84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288c84u64) }
        0x10288c84 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288c88u64) } }
        0x10288c88 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c8cu64) }
        0x10288c8c => { vm.decode(0x88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c90u64) }
        0x10288c90 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288c94u64) }
        0x10288c94 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288c98u64) }
        0x10288c98 => { vm.decode(0x88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288c9cu64) }
        0x10288c9c => { vm.decode(0x820604f5u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288ca0u64) }
        0x10288ca0 => { vm.decode(0x288432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288ca4u64) }
        0x10288ca4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288ca8u64) }
        0x10288ca8 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288cacu64) }
        0x10288cac => { vm.decode(0xb1815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cb0u64) }
        0x10288cb0 => { vm.decode(0x60491519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288cb4u64) }
        0x10288cb4 => { vm.decode(0xb3015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cb8u64) }
        0x10288cb8 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288cbcu64) }
        0x10288cbc => { vm.decode(0xb0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cc0u64) }
        0x10288cc0 => { vm.decode(0x646b1159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288cc4u64) }
        0x10288cc4 => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cc8u64) }
        0x10288cc8 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288cccu64) }
        0x10288ccc => { vm.decode(0x58408091u32); let d = 19i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288cd0u64) }
        0x10288cd0 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288cd4u64) }
        0x10288cd4 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cd8u64) }
        0x10288cd8 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288cdcu64) }
        0x10288cdc => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288ce0u64) }
        0x10288ce0 => { vm.decode(0x10100019u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288ce4u64) }
        0x10288ce4 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288ce8u64) }
        0x10288ce8 => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cecu64) }
        0x10288cec => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288cf0u64) }
        0x10288cf0 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cf4u64) }
        0x10288cf4 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288cf8u64) }
        0x10288cf8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288cfcu64) }
        0x10288cfc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d00u64) }
        0x10288d00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288d04u64 + 4);
        vm.wr(37, 0x10288d04u64 + 4);
        Ok(0x10288d04u64) }
        0x10288d04 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288d08u64) }
        0x10288d08 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288d0cu64) }
        0x10288d0c => { vm.decode(0x23554058u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 33657i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288d10u64) }
        0x10288d10 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288d14u64) }
        0x10288d14 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288d18u64) }
        0x10288d18 => { vm.decode(0xc074c419u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11322i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288d1cu64) }
        0x10288d1c => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d20u64) }
        0x10288d20 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288d24u64) }
        0x10288d24 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288d28u64) }
        0x10288d28 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288d2cu64) }
        0x10288d2c => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288d30u64).wrapping_add((imm * 4) as u64) } else { 0x10288d30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288d30u64) }
        0x10288d30 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288d34u64) }
        0x10288d3c => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288d40u64) }
        0x10288d40 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288d44u64) }
        0x10288d44 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288d48u64) }
        0x10288d48 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288d4cu64) }
        0x10288d4c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288d50u64) }
        0x10288d50 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288d54u64) }
        0x10288d54 => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d58u64) }
        0x10288d58 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288d5cu64) }
        0x10288d5c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288d60u64) }
        0x10288d60 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288d64u64) }
        0x10288d64 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d68u64) }
        0x10288d68 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288d6cu64) }
        0x10288d6c => { vm.decode(0x14941027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 5220i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288d70u64).wrapping_add((imm * 4) as u64) } else { 0x10288d70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288d70u64) }
        0x10288d70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288d74u64) } }
        0x10288d74 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d78u64) }
        0x10288d78 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d7cu64) }
        0x10288d7c => { vm.decode(0x8b191430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288d80u64) }
        0x10288d80 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288d84u64) }
        0x10288d84 => { vm.decode(0x9b115430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288d88u64) }
        0x10288d88 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288d8cu64) }
        0x10288d8c => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288d90u64) }
        0x10288d90 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288d94u64) }
        0x10288d94 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288d98u64) }
        0x10288d98 => { vm.decode(0x1c000011u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288d9cu64) }
        0x10288d9c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288da0u64) }
        0x10288da0 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288da4u64) }
        0x10288da4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288da8u64 + 4);
        vm.wr(37, 0x10288da8u64 + 4);
        Ok(0x10288da8u64) }
        0x10288da8 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288dacu64) }
        0x10288dac => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288db0u64) }
        0x10288db0 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288db4u64) }
        0x10288db4 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288db8u64) }
        0x10288db8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288dbcu64) }
        0x10288dbc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288dc0u64).wrapping_add((imm * 4) as u64) } else { 0x10288dc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288dc0u64) }
        0x10288dc0 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288dc4u64) }
        0x10288dc4 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288dc8u64) }
        0x10288dc8 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288dccu64) }
        0x10288dcc => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288dd0u64) }
        0x10288dd0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288dd4u64) }
        0x10288dd4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288dd8u64).wrapping_add((imm * 4) as u64) } else { 0x10288dd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288dd8u64) }
        0x10288dd8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288ddcu64) } }
        0x10288ddc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288de0u64) }
        0x10288de0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288de4u64) }
        0x10288de4 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288de8u64).wrapping_add((imm * 4) as u64) } else { 0x10288de8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288de8u64) }
        0x10288de8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288decu64) }
        0x10288dec => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288df0u64) }
        0x10288df0 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288df4u64) }
        0x10288df4 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288df8u64) }
        0x10288df8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288dfcu64) }
        0x10288dfc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288e00u64).wrapping_add((imm * 4) as u64) } else { 0x10288e00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288e00u64) }
        0x10288e00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288e04u64) } }
        0x10288e04 => { vm.decode(0xa900551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 10684i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288e08u64) }
        0x10288e08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288e0cu64) } }
        0x10288e0c => { vm.decode(0xb081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288e10u64) }
        0x10288e10 => { vm.decode(0xe08bd04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1295i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x10288e14u64) }
        0x10288e14 => { vm.decode(0x51148302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10288e18u64) }
        0x10288e18 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288e1cu64) }
        0x10288e1c => { vm.decode(0xe88bc04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1303i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x10288e20u64) }
        0x10288e20 => { vm.decode(0x41168302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10288e24u64) }
        0x10288e24 => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288e28u64) }
        0x10288e28 => { vm.decode(0x4a8ae32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288e2cu64) }
        0x10288e2c => { vm.decode(0x6a80c32fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288e30u64) }
        0x10288e30 => { vm.decode(0x2026031u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288e34u64).wrapping_add((imm * 4) as u64) } else { 0x10288e34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288e34u64) }
        0x10288e34 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288e38u64) } }
        0x10288e38 => { vm.decode(0x26c0941du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 182036i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288e3cu64) }
        0x10288e40 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288e44u64) }
        0x10288e44 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288e48u64) }
        0x10288e48 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288e4cu64) }
        0x10288e4c => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x10288e50u64) }
        0x10288e50 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288e54u64).wrapping_add((imm * 4) as u64) } else { 0x10288e54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288e54u64) }
        0x10288e54 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10288e58u64) }
        0x10288e58 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288e5cu64) }
        0x10288e64 => { vm.decode(0xa022031u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288e68u64).wrapping_add((imm * 4) as u64) } else { 0x10288e68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288e68u64) }
        0x10288e68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288e6cu64) } }
        0x10288e6c => { vm.decode(0xa784511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 147364i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288e70u64) }
        0x10288e70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288e74u64) } }
        0x10288e74 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288e78u64) }
        0x102890bc => { vm.decode(0x9b1015b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102890c0u64) }
        0x102890c0 => { vm.decode(0x6a8063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890c4u64) }
        0x102890c4 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890c8u64) }
        0x102890c8 => { vm.decode(0x10001059u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102890ccu64) }
        0x102890cc => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102890d0u64) }
        0x102890d0 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890d4u64) }
        0x102890d4 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102890d8u64) }
        0x102890d8 => { vm.decode(0x742d0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102890dcu64) }
        0x102890dc => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102890e0u64) }
        0x102890e0 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102890e4u64) }
        0x102890e4 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890e8u64) }
        0x102890e8 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890ecu64) }
        0x102890ec => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102890f0u64) }
        0x102890f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102890f4u64 + 4);
        vm.wr(37, 0x102890f4u64 + 4);
        Ok(0x102890f4u64) }
        0x102890f4 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102890f8u64) }
        0x102890f8 => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102890fcu64) }
        0x102890fc => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289100u64) }
        0x10289100 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289104u64) }
        0x10289104 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10289108u64) }
        0x10289108 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028910cu64) }
        0x1028910c => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289110u64) }
        0x10289110 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289114u64) }
        0x10289114 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289118u64) }
        0x10289118 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028911cu64 + 4);
        vm.wr(37, 0x1028911cu64 + 4);
        Ok(0x1028911cu64) }
        0x1028911c => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289120u64) }
        0x10289120 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289124u64) }
        0x10289124 => { vm.decode(0x40708c99u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19505i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289128u64) }
        0x10289128 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028912cu64) }
        0x1028912c => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289130u64) }
        0x10289130 => { vm.decode(0x12108019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289134u64) }
        0x10289134 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289138u64) }
        0x10289138 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028913cu64) }
        0x1028913c => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10289140u64) }
        0x10289140 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289144u64).wrapping_add((imm * 4) as u64) } else { 0x10289144u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289144u64) }
        0x10289144 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289148u64) }
        0x10289148 => { vm.decode(0x900d01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 18728i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028914cu64) }
        0x10289150 => { vm.decode(0xa424735u32); let d = 1i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289154u64) }
        0x10289154 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10289158u64) }
        0x10289158 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028915cu64) }
        0x1028915c => { vm.decode(0x74114675u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289160u64) }
        0x10289160 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289164u64) }
        0x10289164 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289168u64) }
        0x10289168 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028916cu64) }
        0x1028916c => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289170u64) }
        0x10289170 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10289174u64) }
        0x10289174 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289178u64) }
        0x10289178 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028917cu64) }
        0x1028917c => { vm.decode(0x12140aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1738i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289180u64).wrapping_add((imm * 4) as u64) } else { 0x10289180u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289180u64) }
        0x10289180 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289184u64) }
        0x10289184 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289188u64) }
        0x10289188 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028918cu64) }
        0x1028918c => { vm.decode(0x9b195430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289190u64) }
        0x10289190 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289194u64) }
        0x10289194 => { vm.decode(0x8b111530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289198u64) }
        0x10289198 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028919cu64) }
        0x1028919c => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102891a0u64) }
        0x102891a0 => { vm.decode(0x416114f5u32); let d = 1i16; let s = 16i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102891a4u64) }
        0x102891a4 => { vm.decode(0x5c000011u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102891a8u64) }
        0x102891a8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102891acu64) }
        0x102891ac => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102891b0u64) }
        0x102891b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102891b4u64 + 4);
        vm.wr(37, 0x102891b4u64 + 4);
        Ok(0x102891b4u64) }
        0x102891b4 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102891b8u64) }
        0x102891b8 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102891bcu64) }
        0x102891bc => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102891c0u64) }
        0x102891c0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891c4u64) }
        0x102891c4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102891c8u64) }
        0x102891c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102891ccu64).wrapping_add((imm * 4) as u64) } else { 0x102891ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102891ccu64) }
        0x102891cc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891d0u64) }
        0x102891d0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891d4u64) }
        0x102891d4 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102891d8u64) }
        0x102891d8 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102891dcu64) }
        0x102891dc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102891e0u64) }
        0x102891e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102891e4u64).wrapping_add((imm * 4) as u64) } else { 0x102891e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102891e4u64) }
        0x102891e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102891e8u64) } }
        0x102891e8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891ecu64) }
        0x102891ec => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102891f0u64) }
        0x102891f0 => { vm.decode(0x739cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102891f4u64).wrapping_add((imm * 4) as u64) } else { 0x102891f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102891f4u64) }
        0x102891f4 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891f8u64) }
        0x102891f8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102891fcu64) }
        0x102891fc => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289200u64) }
        0x10289200 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289204u64) }
        0x10289204 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289208u64) }
        0x10289208 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028920cu64).wrapping_add((imm * 4) as u64) } else { 0x1028920cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028920cu64) }
        0x1028920c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289210u64) } }
        0x10289210 => { vm.decode(0x8420551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 11728i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289214u64) }
        0x10289214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289218u64) } }
        0x10289218 => { vm.decode(0xb081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028921cu64) }
        0x1028921c => { vm.decode(0xe08bd04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1295i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x10289220u64) }
        0x10289220 => { vm.decode(0x51148302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10289224u64) }
        0x10289224 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289228u64) }
        0x10289228 => { vm.decode(0xe88bc04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1303i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x1028922cu64) }
        0x1028922c => { vm.decode(0x41168302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10289230u64) }
        0x10289230 => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289234u64) }
        0x10289234 => { vm.decode(0x6884c32fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289238u64) }
        0x10289238 => { vm.decode(0x2026031u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028923cu64).wrapping_add((imm * 4) as u64) } else { 0x1028923cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028923cu64) }
        0x1028923c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289240u64) } }
        0x10289248 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028924cu64) }
        0x1028924c => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289250u64) }
        0x10289250 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10289254u64) }
        0x10289254 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x10289258u64) }
        0x10289258 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028925cu64).wrapping_add((imm * 4) as u64) } else { 0x1028925cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028925cu64) }
        0x1028925c => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10289260u64) }
        0x1028926c => { vm.decode(0xa022031u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289270u64).wrapping_add((imm * 4) as u64) } else { 0x10289270u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289270u64) }
        0x10289270 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289274u64) } }
        0x10289274 => { vm.decode(0xaf84511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 147372i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289278u64) }
        0x10289278 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028927cu64) } }
        0x102894d0 => { vm.decode(0x4d94e211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102894d4u64) }
        0x102894d4 => { vm.decode(0x288c8627u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 10269i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102894d8u64).wrapping_add((imm * 4) as u64) } else { 0x102894d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102894d8u64) }
        0x102894d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102894dcu64) } }
        0x10289554 => { vm.decode(0xb381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289558u64) }
        0x10289558 => { vm.decode(0xb201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028955cu64) }
        0x1028955c => { vm.decode(0xb301c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289560u64) }
        0x10289560 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289564u64) }
        0x10289564 => { vm.decode(0x621d1062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10289568u64) }
        0x10289568 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028956cu64) }
        0x1028956c => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289570u64) }
        0x10289570 => { vm.decode(0x8450a999u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 17015i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289574u64) }
        0x10289574 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10289578u64) }
        0x10289578 => { vm.decode(0x58327499u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -21339i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028957cu64) }
        0x1028957c => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289580u64) }
        0x10289580 => { vm.decode(0x6a82636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289584u64) }
        0x10289584 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10289588u64) }
        0x10289588 => { vm.decode(0x2202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028958cu64).wrapping_add((imm * 4) as u64) } else { 0x1028958cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028958cu64) }
        0x1028958c => { vm.decode(0x9c76ffd9u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289590u64) }
        0x10289590 => { vm.decode(0x124941du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 20848i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289594u64) }
        0x10289598 => { vm.decode(0x60453835u32); let d = 1i16; let s = 7i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028959cu64) }
        0x1028959c => { vm.decode(0x44081ff5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x102895a0u64) }
        0x102895a0 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102895a4u64) }
        0x102895a4 => { vm.decode(0x90350175u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x102895a8u64) }
        0x102895a8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102895acu64) }
        0x102895ac => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102895b0u64) }
        0x102895b0 => { vm.decode(0x8b214909u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102895b4u64) }
        0x102895b4 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102895b8u64) }
        0x102895b8 => { vm.decode(0x2a88436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895bcu64) }
        0x102895bc => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102895c0u64) }
        0x102895c0 => { vm.decode(0x106800b0u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102895c4u64) }
        0x102895c4 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102895c8u64) }
        0x102895c8 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102895ccu64) }
        0x102895cc => { vm.decode(0x521402a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1992i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102895d0u64).wrapping_add((imm * 4) as u64) } else { 0x102895d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102895d0u64) }
        0x102895d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102895d4u64) } }
        0x102895d4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895d8u64) }
        0x102895d8 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895dcu64) }
        0x102895dc => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102895e0u64) }
        0x102895e0 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102895e4u64) }
        0x102895e4 => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895e8u64) }
        0x102895e8 => { vm.decode(0x414124f5u32); let d = 1i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102895ecu64) }
        0x102895ec => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102895f0u64) }
        0x102895f0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895f4u64) }
        0x102895f4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102895f8u64) }
        0x102895f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102895fcu64 + 4);
        vm.wr(37, 0x102895fcu64 + 4);
        Ok(0x102895fcu64) }
        0x102895fc => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289600u64) }
        0x10289600 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289604u64) }
        0x10289604 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289608u64) }
        0x10289608 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028960cu64) }
        0x1028960c => { vm.decode(0x90a0b75u32); let d = 4i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10289610u64) }
        0x10289610 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289614u64) }
        0x10289614 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289618u64) }
        0x10289618 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028961cu64).wrapping_add((imm * 4) as u64) } else { 0x1028961cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028961cu64) }
        0x1028961c => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289620u64) }
        0x10289620 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289624u64) }
        0x10289624 => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10289628u64) }
        0x10289628 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028962cu64) }
        0x1028962c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289630u64) }
        0x10289630 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289634u64).wrapping_add((imm * 4) as u64) } else { 0x10289634u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289634u64) }
        0x10289634 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289638u64) } }
        0x10289638 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028963cu64) }
        0x1028963c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289640u64) }
        0x10289640 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289644u64).wrapping_add((imm * 4) as u64) } else { 0x10289644u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289644u64) }
        0x10289644 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289648u64) }
        0x10289648 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028964cu64) }
        0x1028964c => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10289650u64) }
        0x10289650 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289654u64) }
        0x10289654 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289658u64) }
        0x10289658 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028965cu64).wrapping_add((imm * 4) as u64) } else { 0x1028965cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028965cu64) }
        0x1028965c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289660u64) } }
        0x10289660 => { vm.decode(0x884051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 12824i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289664u64) }
        0x10289664 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289668u64) } }
        0x10289668 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028966cu64) }
        0x1028966c => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289670u64) }
        0x10289670 => { vm.decode(0x40008119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289674u64) }
        0x10289674 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289678u64) }
        0x10289678 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028967cu64) }
        0x1028967c => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289680u64) }
        0x10289680 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289684u64) }
        0x10289684 => { vm.decode(0x12008019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289688u64) }
        0x10289688 => { vm.decode(0x604b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028968cu64) }
        0x1028968c => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289690u64) }
        0x10289690 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289694u64) }
        0x10289694 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289698u64) }
        0x10289698 => { vm.decode(0x8019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028969cu64) }
        0x1028969c => { vm.decode(0x8a020071u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102896a0u64).wrapping_add((imm * 4) as u64) } else { 0x102896a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102896a0u64) }
        0x102896a0 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102896a4u64) }
        0x102896a4 => { vm.decode(0x8024551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 14800i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102896a8u64) }
        0x102896cc => { vm.decode(0x4c0400a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 416i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102896d0u64).wrapping_add((imm * 4) as u64) } else { 0x102896d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102896d0u64) }
        0x102896d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102896d4u64) } }
        0x102896d4 => { vm.decode(0x12404171u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 196i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102896d8u64).wrapping_add((imm * 4) as u64) } else { 0x102896d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102896d8u64) }
        0x102896d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102896dcu64) } }
        0x102896dc => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102896e0u64) }
        0x102896e0 => { vm.decode(0x2420131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 193i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102896e4u64).wrapping_add((imm * 4) as u64) } else { 0x102896e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102896e4u64) }
        0x102896e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102896e8u64) } }
        0x102896e8 => { vm.decode(0x9b2054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102896ecu64) }
        0x102899e8 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102899ecu64) }
        0x102899ec => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102899f0u64) }
        0x102899f0 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102899f4u64) }
        0x102899f4 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102899f8u64) }
        0x102899f8 => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102899fcu64) }
        0x102899fc => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289a00u64) }
        0x10289a00 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289a04u64) }
        0x10289a04 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289a08u64) }
        0x10289a08 => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289a0cu64) }
        0x10289a0c => { vm.decode(0xc1422b75u32); let d = 16i16; let s = 20i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10289a10u64) }
        0x10289a10 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10289a14u64) }
        0x10289a14 => { vm.decode(0x24040e35u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x10289a18u64) }
        0x10289a18 => { vm.decode(0x840042f5u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x10289a1cu64) }
        0x10289a1c => { vm.decode(0x8b614909u32); let d = 18i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10289a20u64) }
        0x10289a20 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289a24u64) }
        0x10289a24 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289a28u64) }
        0x10289a28 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289a2cu64) }
        0x10289a2c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289a30u64) }
        0x10289a30 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289a34u64 + 4);
        vm.wr(37, 0x10289a34u64 + 4);
        Ok(0x10289a34u64) }
        0x10289a34 => { vm.decode(0xc009875u32); let d = 17i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289a38u64) }
        0x10289a38 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289a3cu64) }
        0x10289a3c => { vm.decode(0x6b4504f5u32); let d = 19i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289a40u64) }
        0x10289a40 => { vm.decode(0x9b009530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289a44u64) }
        0x10289a44 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289a48u64) }
        0x10289a48 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289a4cu64) }
        0x10289a4c => { vm.decode(0xf6b37cf1u32); let d = 20i16; let s = 21i16; let s2 = -1i16; let imm = -225i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289a50u64).wrapping_add((imm * 4) as u64) } else { 0x10289a50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289a50u64) }
        0x10289a50 => { vm.decode(0xa8208b35u32); let d = 3i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289a54u64) }
        0x10289a54 => { vm.decode(0x630d7022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10289a58u64) }
        0x10289d50 => { vm.decode(0x9b2054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289d54u64) }
        0x10289d54 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d58u64) }
        0x10289d58 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d5cu64) }
        0x10289d5c => { vm.decode(0x10000119u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289d60u64) }
        0x10289d60 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289d64u64) }
        0x10289d64 => { vm.decode(0x28843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d68u64) }
        0x10289d68 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289d6cu64) }
        0x10289d6c => { vm.decode(0x740f9119u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289d70u64) }
        0x10289d70 => { vm.decode(0x9b1115b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289d74u64) }
        0x10289d74 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289d78u64) }
        0x10289d78 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d7cu64) }
        0x10289d7c => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d80u64) }
        0x10289d80 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289d84u64) }
        0x10289d84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289d88u64 + 4);
        vm.wr(37, 0x10289d88u64 + 4);
        Ok(0x10289d88u64) }
        0x10289d88 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289d8cu64) }
        0x10289d8c => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289d90u64) }
        0x10289d90 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289d94u64) }
        0x10289d94 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289d98u64) }
        0x10289d98 => { vm.decode(0x2102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289d9cu64) }
        0x10289d9c => { vm.decode(0x8b2114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289da0u64) }
        0x10289da0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10289da4u64) }
        0x10289da4 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289da8u64) }
        0x10289da8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289dacu64) }
        0x10289dac => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289db0u64) }
        0x10289db0 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289db4u64) }
        0x10289db4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289db8u64 + 4);
        vm.wr(37, 0x10289db8u64 + 4);
        Ok(0x10289db8u64) }
        0x10289db8 => { vm.decode(0x8001875u32); let d = 16i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289dbcu64) }
        0x10289dbc => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289dc0u64) }
        0x10289dc0 => { vm.decode(0x40428019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 144i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289dc4u64) }
        0x10289dc4 => { vm.decode(0x12041019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289dc8u64) }
        0x10289dc8 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289dccu64) }
        0x10289dcc => { vm.decode(0x8a4604f5u32); let d = 23i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289dd0u64) }
        0x10289dd0 => { vm.decode(0x760d0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289dd4u64) }
        0x10289dd4 => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10289dd8u64) }
        0x10289dd8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289ddcu64).wrapping_add((imm * 4) as u64) } else { 0x10289ddcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289ddcu64) }
        0x10289ddc => { vm.decode(0x3110236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289de0u64) }
        0x10289de0 => { vm.decode(0xa924901du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 20972i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289de4u64) }
        0x10289de8 => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289decu64) }
        0x10289dec => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10289df0u64) }
        0x10289df0 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289df4u64) }
        0x10289df4 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289df8u64) }
        0x10289df8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289dfcu64) }
        0x10289dfc => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289e00u64) }
        0x10289e00 => { vm.decode(0x490104f5u32); let d = 2i16; let s = 18i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289e04u64) }
        0x10289e04 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10289e08u64) }
        0x10289e08 => { vm.decode(0xad03036u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e0cu64) }
        0x10289e0c => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10289e10u64) }
        0x10289e10 => { vm.decode(0x509c08a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1494i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289e14u64).wrapping_add((imm * 4) as u64) } else { 0x10289e14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289e14u64) }
        0x10289e14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289e18u64) } }
        0x10289e18 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289e1cu64) }
        0x10289e1c => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289e20u64) }
        0x10289e20 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289e24u64) }
        0x10289e24 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289e28u64) }
        0x10289e28 => { vm.decode(0x9b115530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289e2cu64) }
        0x10289e2c => { vm.decode(0xc0e00bb5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289e30u64) }
        0x10289e30 => { vm.decode(0x634104f5u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289e34u64) }
        0x10289e34 => { vm.decode(0x2004023eu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10289e38u64) }
        0x10289e38 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289e3cu64) }
        0x10289e3c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289e40u64) }
        0x10289e40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289e44u64 + 4);
        vm.wr(37, 0x10289e44u64 + 4);
        Ok(0x10289e44u64) }
        0x10289e44 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289e48u64) }
        0x10289e48 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289e4cu64) }
        0x10289e4c => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289e50u64) }
        0x10289e50 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e54u64) }
        0x10289e54 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289e58u64) }
        0x10289e58 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289e5cu64).wrapping_add((imm * 4) as u64) } else { 0x10289e5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289e5cu64) }
        0x10289e5c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e60u64) }
        0x10289e60 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e64u64) }
        0x10289e64 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10289e68u64) }
        0x10289e68 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289e6cu64) }
        0x10289e6c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289e70u64) }
        0x10289e70 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289e74u64) }
        0x10289e74 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289e78u64).wrapping_add((imm * 4) as u64) } else { 0x10289e78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289e78u64) }
        0x10289e78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289e7cu64) } }
        0x10289e7c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e80u64) }
        0x10289e80 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289e84u64) }
        0x10289e84 => { vm.decode(0x731c36a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289e88u64).wrapping_add((imm * 4) as u64) } else { 0x10289e88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289e88u64) }
        0x10289e88 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e8cu64) }
        0x10289e8c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289e90u64) }
        0x10289e90 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10289e94u64) }
        0x10289e94 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289e98u64) }
        0x10289e98 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289e9cu64) }
        0x10289e9c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289ea0u64) }
        0x10289ea0 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289ea4u64).wrapping_add((imm * 4) as u64) } else { 0x10289ea4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289ea4u64) }
        0x10289ea4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289ea8u64) } }
        0x10289ea8 => { vm.decode(0x9a4411du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 14952i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289eacu64) }
        0x10289eac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289eb0u64) } }
        0x10289eb0 => { vm.decode(0x6a8063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289eb4u64) }
        0x10289eb4 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289eb8u64) }
        0x10289eb8 => { vm.decode(0x10001059u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289ebcu64) }
        0x10289ebc => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289ec0u64) }
        0x10289ec0 => { vm.decode(0x8b1954b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289ec4u64) }
        0x10289ec4 => { vm.decode(0x742d0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289ec8u64) }
        0x10289ec8 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289eccu64) }
        0x10289ecc => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289ed0u64) }
        0x10289ed0 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289ed4u64) }
        0x10289ed4 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289ed8u64) }
        0x10289ed8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289edcu64) }
        0x10289edc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289ee0u64 + 4);
        vm.wr(37, 0x10289ee0u64 + 4);
        Ok(0x10289ee0u64) }
        0x10289ee0 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289ee4u64) }
        0x10289ee4 => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289ee8u64) }
        0x10289ee8 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289eecu64) }
        0x10289eec => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289ef0u64) }
        0x10289ef0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10289ef4u64) }
        0x10289ef4 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289ef8u64) }
        0x10289ef8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289efcu64) }
        0x10289efc => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289f00u64) }
        0x10289f00 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289f04u64) }
        0x10289f04 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289f08u64 + 4);
        vm.wr(37, 0x10289f08u64 + 4);
        Ok(0x10289f08u64) }
        0x10289f08 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289f0cu64) }
        0x10289f0c => { vm.decode(0x84028cd9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 19331i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289f10u64) }
        0x10289f10 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10289f14u64) }
        0x10289f14 => { vm.decode(0xc050e9d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289f18u64) }
        0x10289f18 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289f1cu64) }
        0x10289f1c => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289f20u64) }
        0x10289f20 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289f24u64) }
        0x10289f24 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10289f28u64) }
        0x10289f28 => { vm.decode(0x4d184211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289f2cu64) }
        0x10289f2c => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10289f30u64) }
        0x10289f30 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289f34u64).wrapping_add((imm * 4) as u64) } else { 0x10289f34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289f34u64) }
        0x10289f34 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289f38u64) }
        0x10289f38 => { vm.decode(0x2904d41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 22844i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10289f3cu64) }
        0x10289f40 => { vm.decode(0x8224735u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289f44u64) }
        0x10289f44 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10289f48u64) }
        0x10289f48 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289f4cu64) }
        0x10289f4c => { vm.decode(0x64304675u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289f50u64) }
        0x10289f50 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289f54u64) }
        0x10289f54 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289f58u64) }
        0x10289f58 => { vm.decode(0xcb21a4f5u32); let d = 2i16; let s = 23i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289f5cu64) }
        0x10289f5c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10289f60u64) }
        0x10289f60 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289f64u64) }
        0x10289f64 => { vm.decode(0x4d184211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289f68u64) }
        0x10289f68 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10289f6cu64) }
        0x10289f6c => { vm.decode(0x521c8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1875i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289f70u64).wrapping_add((imm * 4) as u64) } else { 0x10289f70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289f70u64) }
        0x10289f70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289f74u64) } }
        0x10289f74 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289f78u64) }
        0x10289f78 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289f7cu64) }
        0x10289f7c => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289f80u64) }
        0x10289f80 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289f84u64) }
        0x10289f84 => { vm.decode(0x8b111530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10289f88u64) }
        0x10289f88 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289f8cu64) }
        0x10289f8c => { vm.decode(0xc040bd75u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10289f90u64) }
        0x10289f90 => { vm.decode(0xc16104f5u32); let d = 1i16; let s = 16i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10289f94u64) }
        0x10289f94 => { vm.decode(0x14000111u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289f98u64) }
        0x10289f98 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289f9cu64) }
        0x10289f9c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10289fa0u64) }
        0x10289fa0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10289fa4u64 + 4);
        vm.wr(37, 0x10289fa4u64 + 4);
        Ok(0x10289fa4u64) }
        0x10289fa4 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10289fa8u64) }
        0x10289fa8 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10289facu64) }
        0x10289fac => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289fb0u64) }
        0x10289fb0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fb4u64) }
        0x10289fb4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289fb8u64) }
        0x10289fb8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289fbcu64).wrapping_add((imm * 4) as u64) } else { 0x10289fbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289fbcu64) }
        0x10289fbc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fc0u64) }
        0x10289fc0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fc4u64) }
        0x10289fc4 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289fc8u64) }
        0x10289fc8 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289fccu64) }
        0x10289fcc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289fd0u64) }
        0x10289fd0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289fd4u64).wrapping_add((imm * 4) as u64) } else { 0x10289fd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289fd4u64) }
        0x10289fd4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10289fd8u64) } }
        0x10289fd8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fdcu64) }
        0x10289fdc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289fe0u64) }
        0x10289fe0 => { vm.decode(0x739cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10289fe4u64).wrapping_add((imm * 4) as u64) } else { 0x10289fe4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289fe4u64) }
        0x10289fe4 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fe8u64) }
        0x10289fe8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10289fecu64) }
        0x10289fec => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10289ff0u64) }
        0x10289ff0 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10289ff4u64) }
        0x10289ff4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10289ff8u64) }
        0x10289ff8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10289ffcu64).wrapping_add((imm * 4) as u64) } else { 0x10289ffcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10289ffcu64) }
        0x10289ffc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a000u64) } }
        0x1028a000 => { vm.decode(0x80a4511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 15296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028a004u64) }
        0x1028a004 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a008u64) } }
        0x1028a008 => { vm.decode(0xb081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a00cu64) }
        0x1028a00c => { vm.decode(0xe08bd04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1295i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x1028a010u64) }
        0x1028a010 => { vm.decode(0x51148302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x1028a014u64) }
        0x1028a014 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a018u64) }
        0x1028a018 => { vm.decode(0xe88bc04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1303i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x1028a01cu64) }
        0x1028a01c => { vm.decode(0x41168302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x1028a020u64) }
        0x1028a020 => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a024u64) }
        0x1028a024 => { vm.decode(0x9938d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a028u64) }
        0x1028a028 => { vm.decode(0x4a82e32fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a02cu64) }
        0x1028a02c => { vm.decode(0x10026031u32); let d = 0i16; let s = 20i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a030u64).wrapping_add((imm * 4) as u64) } else { 0x1028a030u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a030u64) }
        0x1028a030 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a034u64) } }
        0x1028a034 => { vm.decode(0x8364d11du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 195040i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028a038u64) }
        0x1028a03c => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028a040u64) }
        0x1028a040 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028a044u64) }
        0x1028a044 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028a048u64) }
        0x1028a048 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x1028a04cu64) }
        0x1028a04c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a050u64).wrapping_add((imm * 4) as u64) } else { 0x1028a050u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a050u64) }
        0x1028a050 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028a054u64) }
        0x1028a054 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028a058u64) }
        0x1028a060 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a064u64) }
        0x1028a064 => { vm.decode(0xa022031u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a068u64).wrapping_add((imm * 4) as u64) } else { 0x1028a068u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a068u64) }
        0x1028a068 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a06cu64) } }
        0x1028a06c => { vm.decode(0xa784551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 147380i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028a070u64) }
        0x1028a070 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a074u64) } }
        0x1028a074 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a078u64) }
        0x1028a2bc => { vm.decode(0x4d94e211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028a2c0u64) }
        0x1028a2c0 => { vm.decode(0xc940ca7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 9382i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a2c4u64).wrapping_add((imm * 4) as u64) } else { 0x1028a2c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a2c4u64) }
        0x1028a2c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a2c8u64) } }
        0x1028a2c8 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028a2ccu64) }
        0x1028a340 => { vm.decode(0xb381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a344u64) }
        0x1028a344 => { vm.decode(0xb201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a348u64) }
        0x1028a348 => { vm.decode(0xb301c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a34cu64) }
        0x1028a34c => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a350u64) }
        0x1028a350 => { vm.decode(0x621d1062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028a354u64) }
        0x1028a354 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a358u64) }
        0x1028a358 => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a35cu64) }
        0x1028a35c => { vm.decode(0x8044e499u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 10271i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a360u64) }
        0x1028a360 => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028a364u64) }
        0x1028a364 => { vm.decode(0x54347919u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 26220i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a368u64) }
        0x1028a368 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a36cu64) }
        0x1028a36c => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028a370u64) }
        0x1028a370 => { vm.decode(0x2202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a374u64).wrapping_add((imm * 4) as u64) } else { 0x1028a374u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a374u64) }
        0x1028a374 => { vm.decode(0x9c76ffd9u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a378u64) }
        0x1028a378 => { vm.decode(0xa000951du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 24980i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028a37cu64) }
        0x1028a380 => { vm.decode(0x60453835u32); let d = 1i16; let s = 7i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a384u64) }
        0x1028a384 => { vm.decode(0x44081ff5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x1028a388u64) }
        0x1028a388 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a38cu64) }
        0x1028a38c => { vm.decode(0x90350175u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x1028a390u64) }
        0x1028a390 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a394u64) }
        0x1028a394 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a398u64) }
        0x1028a398 => { vm.decode(0x8b214909u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028a39cu64) }
        0x1028a39c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028a3a0u64) }
        0x1028a3a0 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3a4u64) }
        0x1028a3a4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028a3a8u64) }
        0x1028a3a8 => { vm.decode(0x106800b0u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a3acu64) }
        0x1028a3ac => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a3b0u64) }
        0x1028a3b0 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028a3b4u64) }
        0x1028a3b4 => { vm.decode(0x308c8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2135i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a3b8u64).wrapping_add((imm * 4) as u64) } else { 0x1028a3b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a3b8u64) }
        0x1028a3b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a3bcu64) } }
        0x1028a3bc => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3c0u64) }
        0x1028a3c0 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3c4u64) }
        0x1028a3c4 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a3c8u64) }
        0x1028a3c8 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a3ccu64) }
        0x1028a3cc => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3d0u64) }
        0x1028a3d0 => { vm.decode(0x414124f5u32); let d = 1i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028a3d4u64) }
        0x1028a3d4 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a3d8u64) }
        0x1028a3d8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3dcu64) }
        0x1028a3dc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a3e0u64) }
        0x1028a3e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028a3e4u64 + 4);
        vm.wr(37, 0x1028a3e4u64 + 4);
        Ok(0x1028a3e4u64) }
        0x1028a3e4 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a3e8u64) }
        0x1028a3e8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a3ecu64) }
        0x1028a3ec => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028a3f0u64) }
        0x1028a3f0 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028a3f4u64) }
        0x1028a3f4 => { vm.decode(0x90a0b75u32); let d = 4i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028a3f8u64) }
        0x1028a3f8 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028a3fcu64) }
        0x1028a3fc => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028a400u64) }
        0x1028a400 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a404u64).wrapping_add((imm * 4) as u64) } else { 0x1028a404u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a404u64) }
        0x1028a404 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a408u64) }
        0x1028a408 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a40cu64) }
        0x1028a40c => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028a410u64) }
        0x1028a410 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028a414u64) }
        0x1028a414 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028a418u64) }
        0x1028a418 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a41cu64).wrapping_add((imm * 4) as u64) } else { 0x1028a41cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a41cu64) }
        0x1028a41c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a420u64) } }
        0x1028a420 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a424u64) }
        0x1028a424 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028a428u64) }
        0x1028a428 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a42cu64).wrapping_add((imm * 4) as u64) } else { 0x1028a42cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a42cu64) }
        0x1028a42c => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a430u64) }
        0x1028a430 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028a434u64) }
        0x1028a434 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028a438u64) }
        0x1028a438 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028a43cu64) }
        0x1028a43c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028a440u64) }
        0x1028a440 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a444u64).wrapping_add((imm * 4) as u64) } else { 0x1028a444u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a444u64) }
        0x1028a444 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a448u64) } }
        0x1028a448 => { vm.decode(0x801du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 16384i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028a44cu64) }
        0x1028a44c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a450u64) } }
        0x1028a450 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a454u64) }
        0x1028a454 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a458u64) }
        0x1028a458 => { vm.decode(0x40008119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a45cu64) }
        0x1028a45c => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a460u64) }
        0x1028a460 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a464u64) }
        0x1028a464 => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028a468u64) }
        0x1028a468 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a46cu64) }
        0x1028a46c => { vm.decode(0x12008019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a470u64) }
        0x1028a470 => { vm.decode(0x604b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a474u64) }
        0x1028a474 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a478u64) }
        0x1028a478 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a47cu64) }
        0x1028a47c => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a480u64) }
        0x1028a480 => { vm.decode(0x8424a6d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 7951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a484u64) }
        0x1028a484 => { vm.decode(0xa0422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028a488u64) }
        0x1028a488 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a48cu64) }
        0x1028a48c => { vm.decode(0x8019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a490u64) }
        0x1028a490 => { vm.decode(0x8a020071u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a494u64).wrapping_add((imm * 4) as u64) } else { 0x1028a494u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a494u64) }
        0x1028a494 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a498u64) }
        0x1028a4c0 => { vm.decode(0x4c0400a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 416i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028a4c4u64).wrapping_add((imm * 4) as u64) } else { 0x1028a4c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a4c4u64) }
        0x1028a4c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a4c8u64) } }
        0x1028a4c8 => { vm.decode(0x12404171u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 196i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a4ccu64).wrapping_add((imm * 4) as u64) } else { 0x1028a4ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a4ccu64) }
        0x1028a4cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a4d0u64) } }
        0x1028a4d0 => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028a4d4u64) }
        0x1028a4d4 => { vm.decode(0x2420131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 193i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a4d8u64).wrapping_add((imm * 4) as u64) } else { 0x1028a4d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a4d8u64) }
        0x1028a4d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028a4dcu64) } }
        0x1028a7dc => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a7e0u64) }
        0x1028a7e0 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028a7e4u64) }
        0x1028a7e4 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a7e8u64) }
        0x1028a7e8 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a7ecu64) }
        0x1028a7ec => { vm.decode(0x628c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a7f0u64) }
        0x1028a7f0 => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a7f4u64) }
        0x1028a7f4 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a7f8u64) }
        0x1028a7f8 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a7fcu64) }
        0x1028a7fc => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a800u64) }
        0x1028a800 => { vm.decode(0xc1422b75u32); let d = 16i16; let s = 20i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028a804u64) }
        0x1028a804 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028a808u64) }
        0x1028a808 => { vm.decode(0x24040e35u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x1028a80cu64) }
        0x1028a80c => { vm.decode(0x840042f5u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x1028a810u64) }
        0x1028a810 => { vm.decode(0x8b614909u32); let d = 18i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028a814u64) }
        0x1028a814 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a818u64) }
        0x1028a818 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028a81cu64) }
        0x1028a81c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a820u64) }
        0x1028a820 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028a824u64) }
        0x1028a824 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028a828u64 + 4);
        vm.wr(37, 0x1028a828u64 + 4);
        Ok(0x1028a828u64) }
        0x1028a828 => { vm.decode(0xc009875u32); let d = 17i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a82cu64) }
        0x1028a82c => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028a830u64) }
        0x1028a830 => { vm.decode(0x6b4504f5u32); let d = 19i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028a834u64) }
        0x1028a834 => { vm.decode(0x9b3815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028a838u64) }
        0x1028a838 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028a83cu64) }
        0x1028a83c => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028a840u64) }
        0x1028a840 => { vm.decode(0xf6b37cf1u32); let d = 20i16; let s = 21i16; let s2 = -1i16; let imm = -225i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028a844u64).wrapping_add((imm * 4) as u64) } else { 0x1028a844u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028a844u64) }
        0x1028a844 => { vm.decode(0xa8208b35u32); let d = 3i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028a848u64) }
        0x1028ab44 => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab48u64) }
        0x1028ab48 => { vm.decode(0x6a8863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab4cu64) }
        0x1028ab4c => { vm.decode(0x2824412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab50u64) }
        0x1028ab50 => { vm.decode(0x10000119u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ab54u64) }
        0x1028ab54 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab58u64) }
        0x1028ab58 => { vm.decode(0x28843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab5cu64) }
        0x1028ab5c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab60u64) }
        0x1028ab60 => { vm.decode(0x740f1119u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ab64u64) }
        0x1028ab64 => { vm.decode(0x8b1115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab68u64) }
        0x1028ab68 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab6cu64) }
        0x1028ab6c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab70u64) }
        0x1028ab70 => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab74u64) }
        0x1028ab74 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ab78u64) }
        0x1028ab78 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028ab7cu64 + 4);
        vm.wr(37, 0x1028ab7cu64 + 4);
        Ok(0x1028ab7cu64) }
        0x1028ab7c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ab80u64) }
        0x1028ab80 => { vm.decode(0x2220416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ab84u64) }
        0x1028ab84 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ab88u64) }
        0x1028ab88 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ab8cu64) }
        0x1028ab8c => { vm.decode(0x2102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ab90u64) }
        0x1028ab90 => { vm.decode(0x8b2114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab94u64) }
        0x1028ab94 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028ab98u64) }
        0x1028ab98 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ab9cu64) }
        0x1028ab9c => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028aba0u64) }
        0x1028aba0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028aba4u64) }
        0x1028aba4 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028aba8u64) }
        0x1028aba8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028abacu64 + 4);
        vm.wr(37, 0x1028abacu64 + 4);
        Ok(0x1028abacu64) }
        0x1028abac => { vm.decode(0x8001875u32); let d = 16i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028abb0u64) }
        0x1028abb0 => { vm.decode(0x8424a6d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 7951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028abb4u64) }
        0x1028abb4 => { vm.decode(0xa0422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028abb8u64) }
        0x1028abb8 => { vm.decode(0x4c168fd9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -9239i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028abbcu64) }
        0x1028abbc => { vm.decode(0x12041019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028abc0u64) }
        0x1028abc0 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028abc4u64) }
        0x1028abc4 => { vm.decode(0x8a4604f5u32); let d = 23i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028abc8u64) }
        0x1028abc8 => { vm.decode(0x760d0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028abccu64) }
        0x1028abcc => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028abd0u64) }
        0x1028abd0 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028abd4u64).wrapping_add((imm * 4) as u64) } else { 0x1028abd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028abd4u64) }
        0x1028abd4 => { vm.decode(0x3110236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028abd8u64) }
        0x1028abe0 => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028abe4u64) }
        0x1028abe4 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028abe8u64) }
        0x1028abe8 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028abecu64) }
        0x1028abec => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028abf0u64) }
        0x1028abf0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028abf4u64) }
        0x1028abf4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028abf8u64) }
        0x1028abf8 => { vm.decode(0xc90104f5u32); let d = 2i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028abfcu64) }
        0x1028abfc => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028ac00u64) }
        0x1028ac00 => { vm.decode(0xad03036u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac04u64) }
        0x1028ac04 => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac08u64) }
        0x1028ac08 => { vm.decode(0x16148827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1635i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ac0cu64).wrapping_add((imm * 4) as u64) } else { 0x1028ac0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ac0cu64) }
        0x1028ac0c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ac10u64) } }
        0x1028ac10 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ac14u64) }
        0x1028ac14 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ac18u64) }
        0x1028ac18 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ac1cu64) }
        0x1028ac1c => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ac20u64) }
        0x1028ac20 => { vm.decode(0x9b115530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ac24u64) }
        0x1028ac24 => { vm.decode(0xc0e20bb5u32); let d = 19i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ac28u64) }
        0x1028ac28 => { vm.decode(0xc34104f5u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028ac2cu64) }
        0x1028ac2c => { vm.decode(0x2004023eu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028ac30u64) }
        0x1028ac30 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ac34u64) }
        0x1028ac34 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ac38u64) }
        0x1028ac38 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028ac3cu64 + 4);
        vm.wr(37, 0x1028ac3cu64 + 4);
        Ok(0x1028ac3cu64) }
        0x1028ac3c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ac40u64) }
        0x1028ac40 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ac44u64) }
        0x1028ac44 => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ac48u64) }
        0x1028ac48 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac4cu64) }
        0x1028ac4c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028ac50u64) }
        0x1028ac50 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ac54u64).wrapping_add((imm * 4) as u64) } else { 0x1028ac54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ac54u64) }
        0x1028ac54 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac58u64) }
        0x1028ac58 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac5cu64) }
        0x1028ac5c => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028ac60u64) }
        0x1028ac60 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ac64u64) }
        0x1028ac64 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028ac68u64) }
        0x1028ac68 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028ac6cu64) }
        0x1028ac6c => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ac70u64).wrapping_add((imm * 4) as u64) } else { 0x1028ac70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ac70u64) }
        0x1028ac70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ac74u64) } }
        0x1028ac74 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac78u64) }
        0x1028ac78 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028ac7cu64) }
        0x1028ac7c => { vm.decode(0x731c36a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ac80u64).wrapping_add((imm * 4) as u64) } else { 0x1028ac80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ac80u64) }
        0x1028ac80 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac84u64) }
        0x1028ac84 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ac88u64) }
        0x1028ac88 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028ac8cu64) }
        0x1028ac8c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ac90u64) }
        0x1028ac90 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028ac94u64) }
        0x1028ac94 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028ac98u64) }
        0x1028ac98 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ac9cu64).wrapping_add((imm * 4) as u64) } else { 0x1028ac9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ac9cu64) }
        0x1028ac9c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028aca0u64) } }
        0x1028aca0 => { vm.decode(0x120c01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 18528i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028aca4u64) }
        0x1028aca4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028aca8u64) } }
        0x1028aca8 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acacu64) }
        0x1028acac => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028acb0u64) }
        0x1028acb0 => { vm.decode(0x40008119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acb4u64) }
        0x1028acb4 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028acb8u64) }
        0x1028acb8 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028acbcu64) }
        0x1028acbc => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028acc0u64) }
        0x1028acc0 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028acc4u64) }
        0x1028acc4 => { vm.decode(0x12008019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acc8u64) }
        0x1028acc8 => { vm.decode(0x604b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acccu64) }
        0x1028accc => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028acd0u64) }
        0x1028acd0 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acd4u64) }
        0x1028acd4 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028acd8u64) }
        0x1028acd8 => { vm.decode(0x8019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028acdcu64) }
        0x1028acdc => { vm.decode(0x8a020071u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ace0u64).wrapping_add((imm * 4) as u64) } else { 0x1028ace0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ace0u64) }
        0x1028ace0 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ace4u64) }
        0x1028ace4 => { vm.decode(0x4841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 20496i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028ace8u64) }
        0x1028ad0c => { vm.decode(0x4c0400a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 416i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ad10u64).wrapping_add((imm * 4) as u64) } else { 0x1028ad10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ad10u64) }
        0x1028ad10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ad14u64) } }
        0x1028ad14 => { vm.decode(0x12404171u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 196i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ad18u64).wrapping_add((imm * 4) as u64) } else { 0x1028ad18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ad18u64) }
        0x1028ad18 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ad1cu64) } }
        0x1028ad1c => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028ad20u64) }
        0x1028ad20 => { vm.decode(0x2420131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 193i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ad24u64).wrapping_add((imm * 4) as u64) } else { 0x1028ad24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ad24u64) }
        0x1028ad24 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ad28u64) } }
        0x1028ad28 => { vm.decode(0x9b2054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ad2cu64) }
        0x1028b028 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b02cu64) }
        0x1028b02c => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b030u64) }
        0x1028b030 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b034u64) }
        0x1028b034 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b038u64) }
        0x1028b038 => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b03cu64) }
        0x1028b03c => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b040u64) }
        0x1028b040 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b044u64) }
        0x1028b044 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b048u64) }
        0x1028b048 => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b04cu64) }
        0x1028b04c => { vm.decode(0xc1422b75u32); let d = 16i16; let s = 20i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028b050u64) }
        0x1028b050 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028b054u64) }
        0x1028b054 => { vm.decode(0x24040e35u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x1028b058u64) }
        0x1028b058 => { vm.decode(0x840042f5u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x1028b05cu64) }
        0x1028b05c => { vm.decode(0x8b614909u32); let d = 18i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028b060u64) }
        0x1028b060 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b064u64) }
        0x1028b064 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b068u64) }
        0x1028b068 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b06cu64) }
        0x1028b06c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b070u64) }
        0x1028b070 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b074u64 + 4);
        vm.wr(37, 0x1028b074u64 + 4);
        Ok(0x1028b074u64) }
        0x1028b074 => { vm.decode(0xc009875u32); let d = 17i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b078u64) }
        0x1028b078 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b07cu64) }
        0x1028b07c => { vm.decode(0x6b4504f5u32); let d = 19i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b080u64) }
        0x1028b080 => { vm.decode(0x9b009530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b084u64) }
        0x1028b084 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b088u64) }
        0x1028b088 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b08cu64) }
        0x1028b08c => { vm.decode(0xf6b37cf1u32); let d = 20i16; let s = 21i16; let s2 = -1i16; let imm = -225i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b090u64).wrapping_add((imm * 4) as u64) } else { 0x1028b090u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b090u64) }
        0x1028b090 => { vm.decode(0xa8208b35u32); let d = 3i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b094u64) }
        0x1028b094 => { vm.decode(0x630d7022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028b098u64) }
        0x1028b390 => { vm.decode(0x9b2054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b394u64) }
        0x1028b394 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b398u64) }
        0x1028b398 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b39cu64) }
        0x1028b39c => { vm.decode(0x10000119u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b3a0u64) }
        0x1028b3a0 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3a4u64) }
        0x1028b3a4 => { vm.decode(0x28843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b3a8u64) }
        0x1028b3a8 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3acu64) }
        0x1028b3ac => { vm.decode(0x740f9119u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b3b0u64) }
        0x1028b3b0 => { vm.decode(0x9b1115b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3b4u64) }
        0x1028b3b4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3b8u64) }
        0x1028b3b8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b3bcu64) }
        0x1028b3bc => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b3c0u64) }
        0x1028b3c0 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b3c4u64) }
        0x1028b3c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b3c8u64 + 4);
        vm.wr(37, 0x1028b3c8u64 + 4);
        Ok(0x1028b3c8u64) }
        0x1028b3c8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b3ccu64) }
        0x1028b3cc => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b3d0u64) }
        0x1028b3d0 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b3d4u64) }
        0x1028b3d4 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b3d8u64) }
        0x1028b3d8 => { vm.decode(0x2102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b3dcu64) }
        0x1028b3dc => { vm.decode(0x8b2114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3e0u64) }
        0x1028b3e0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b3e4u64) }
        0x1028b3e4 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3e8u64) }
        0x1028b3e8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b3ecu64) }
        0x1028b3ec => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b3f0u64) }
        0x1028b3f0 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b3f4u64) }
        0x1028b3f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b3f8u64 + 4);
        vm.wr(37, 0x1028b3f8u64 + 4);
        Ok(0x1028b3f8u64) }
        0x1028b3f8 => { vm.decode(0x8001875u32); let d = 16i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b3fcu64) }
        0x1028b3fc => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b400u64) }
        0x1028b400 => { vm.decode(0xc0708c19u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19506i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b404u64) }
        0x1028b404 => { vm.decode(0x12041019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b408u64) }
        0x1028b408 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b40cu64) }
        0x1028b40c => { vm.decode(0x8a4604f5u32); let d = 23i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b410u64) }
        0x1028b410 => { vm.decode(0x760d0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b414u64) }
        0x1028b414 => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028b418u64) }
        0x1028b418 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b41cu64).wrapping_add((imm * 4) as u64) } else { 0x1028b41cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b41cu64) }
        0x1028b41c => { vm.decode(0x3110236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b420u64) }
        0x1028b420 => { vm.decode(0x120d51du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 26992i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028b424u64) }
        0x1028b428 => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b42cu64) }
        0x1028b42c => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028b430u64) }
        0x1028b430 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b434u64) }
        0x1028b434 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b438u64) }
        0x1028b438 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b43cu64) }
        0x1028b43c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b440u64) }
        0x1028b440 => { vm.decode(0x490104f5u32); let d = 2i16; let s = 18i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b444u64) }
        0x1028b444 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b448u64) }
        0x1028b448 => { vm.decode(0xad03036u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b44cu64) }
        0x1028b44c => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028b450u64) }
        0x1028b450 => { vm.decode(0x6948827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1575i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b454u64).wrapping_add((imm * 4) as u64) } else { 0x1028b454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b454u64) }
        0x1028b454 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b458u64) } }
        0x1028b458 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b45cu64) }
        0x1028b45c => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b460u64) }
        0x1028b460 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b464u64) }
        0x1028b464 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b468u64) }
        0x1028b468 => { vm.decode(0x9b115530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b46cu64) }
        0x1028b46c => { vm.decode(0xc0e00bb5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b470u64) }
        0x1028b470 => { vm.decode(0x634104f5u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b474u64) }
        0x1028b474 => { vm.decode(0x2004023eu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028b478u64) }
        0x1028b478 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b47cu64) }
        0x1028b47c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b480u64) }
        0x1028b480 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b484u64 + 4);
        vm.wr(37, 0x1028b484u64 + 4);
        Ok(0x1028b484u64) }
        0x1028b484 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b488u64) }
        0x1028b488 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028b48cu64) }
        0x1028b48c => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b490u64) }
        0x1028b490 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b494u64) }
        0x1028b494 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028b498u64) }
        0x1028b498 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b49cu64).wrapping_add((imm * 4) as u64) } else { 0x1028b49cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b49cu64) }
        0x1028b49c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b4a0u64) }
        0x1028b4a0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b4a4u64) }
        0x1028b4a4 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028b4a8u64) }
        0x1028b4a8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b4acu64) }
        0x1028b4ac => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028b4b0u64) }
        0x1028b4b0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028b4b4u64) }
        0x1028b4b4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b4b8u64).wrapping_add((imm * 4) as u64) } else { 0x1028b4b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b4b8u64) }
        0x1028b4b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b4bcu64) } }
        0x1028b4bc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b4c0u64) }
        0x1028b4c0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028b4c4u64) }
        0x1028b4c4 => { vm.decode(0x731c36a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b4c8u64).wrapping_add((imm * 4) as u64) } else { 0x1028b4c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b4c8u64) }
        0x1028b4c8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b4ccu64) }
        0x1028b4cc => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b4d0u64) }
        0x1028b4d0 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028b4d4u64) }
        0x1028b4d4 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b4d8u64) }
        0x1028b4d8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028b4dcu64) }
        0x1028b4dc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028b4e0u64) }
        0x1028b4e0 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b4e4u64).wrapping_add((imm * 4) as u64) } else { 0x1028b4e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b4e4u64) }
        0x1028b4e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b4e8u64) } }
        0x1028b4e8 => { vm.decode(0x8904801du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 20648i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028b4ecu64) }
        0x1028b4ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b4f0u64) } }
        0x1028b4f0 => { vm.decode(0x4d94e211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028b4f4u64) }
        0x1028b4f4 => { vm.decode(0x88c8627u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 8221i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b4f8u64).wrapping_add((imm * 4) as u64) } else { 0x1028b4f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b4f8u64) }
        0x1028b4f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b4fcu64) } }
        0x1028b4fc => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028b500u64) }
        0x1028b56c => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b570u64) }
        0x1028b570 => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028b574u64) }
        0x1028b574 => { vm.decode(0x508480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 453i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b578u64).wrapping_add((imm * 4) as u64) } else { 0x1028b578u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b578u64) }
        0x1028b578 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b57cu64) } }
        0x1028b57c => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b580u64) }
        0x1028b580 => { vm.decode(0xe04004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b584u64) }
        0x1028b584 => { vm.decode(0x8740323eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028b588u64) }
        0x1028b588 => { vm.decode(0x20000833u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b58cu64) }
        0x1028b58c => { vm.decode(0x40801099u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b590u64) }
        0x1028b590 => { vm.decode(0xa0006600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 57i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028b594u64) }
        0x1028b594 => { vm.decode(0x82422171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 207i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b598u64).wrapping_add((imm * 4) as u64) } else { 0x1028b598u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b598u64) }
        0x1028b598 => { vm.decode(0xa200ad75u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028b59cu64) }
        0x1028b59c => { vm.decode(0xb305430u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b5a0u64) }
        0x1028b8d4 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b8d8u64) }
        0x1028b8d8 => { vm.decode(0x286004f5u32); let d = 3i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b8dcu64) }
        0x1028b8dc => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028b8e0u64) }
        0x1028b8e0 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b8e4u64).wrapping_add((imm * 4) as u64) } else { 0x1028b8e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b8e4u64) }
        0x1028b8e4 => { vm.decode(0x40204018u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028b8e8u64) }
        0x1028b8e8 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b8ecu64) }
        0x1028b8ec => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028b8f0u64) }
        0x1028b8f0 => { vm.decode(0x2800103eu32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028b8f4u64) }
        0x1028b8f4 => { vm.decode(0x420833u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b8f8u64) }
        0x1028b8f8 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b8fcu64) }
        0x1028b8fc => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028b900u64) }
        0x1028b900 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b904u64).wrapping_add((imm * 4) as u64) } else { 0x1028b904u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b904u64) }
        0x1028b904 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b908u64) }
        0x1028b908 => { vm.decode(0x2300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b90cu64) }
        0x1028b90c => { vm.decode(0x930403b5u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b910u64) }
        0x1028b910 => { vm.decode(0x834003b5u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b914u64) }
        0x1028b914 => { vm.decode(0x81c403b5u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b918u64) }
        0x1028b918 => { vm.decode(0xf20a4ef3u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1863i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b91cu64) }
        0x1028b91c => { vm.decode(0x524a4ef3u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1862i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b920u64) }
        0x1028b920 => { vm.decode(0xd2484e73u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1861i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b924u64) }
        0x1028b924 => { vm.decode(0x72084e73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b928u64) }
        0x1028b928 => { vm.decode(0xd20a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1859i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b92cu64) }
        0x1028b92c => { vm.decode(0x22300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b930u64) }
        0x1028b930 => { vm.decode(0x520a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1858i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b934u64) }
        0x1028b934 => { vm.decode(0x2320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b938u64) }
        0x1028b938 => { vm.decode(0xd20a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1857i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b93cu64) }
        0x1028b93c => { vm.decode(0x22320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028b940u64) }
        0x1028b940 => { vm.decode(0x520a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b944u64) }
        0x1028b944 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b948u64) }
        0x1028b948 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b94cu64) }
        0x1028b94c => { vm.decode(0x101019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b950u64) }
        0x1028b950 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b954u64) }
        0x1028b954 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b958u64) }
        0x1028b958 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b95cu64) }
        0x1028b95c => { vm.decode(0x746b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b960u64) }
        0x1028b960 => { vm.decode(0x8b311530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b964u64) }
        0x1028b964 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b968u64) }
        0x1028b968 => { vm.decode(0x760d0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b96cu64) }
        0x1028b96c => { vm.decode(0x408ec32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b970u64) }
        0x1028b970 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b974u64) }
        0x1028b974 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b978u64) }
        0x1028b978 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b97cu64) }
        0x1028b97c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b980u64 + 4);
        vm.wr(37, 0x1028b980u64 + 4);
        Ok(0x1028b980u64) }
        0x1028b980 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b984u64) }
        0x1028b984 => { vm.decode(0x740d0059u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b988u64) }
        0x1028b988 => { vm.decode(0x4008019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b98cu64) }
        0x1028b98c => { vm.decode(0x288416fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b990u64) }
        0x1028b990 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b994u64) }
        0x1028b994 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028b998u64) }
        0x1028b998 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b99cu64) }
        0x1028b99c => { vm.decode(0xb291530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028b9a0u64) }
        0x1028b9a0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b9a4u64) }
        0x1028b9a4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b9a8u64) }
        0x1028b9a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028b9acu64 + 4);
        vm.wr(37, 0x1028b9acu64 + 4);
        Ok(0x1028b9acu64) }
        0x1028b9ac => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028b9b0u64) }
        0x1028b9b0 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028b9b4u64) }
        0x1028b9b4 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028b9b8u64) }
        0x1028b9b8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b9bcu64).wrapping_add((imm * 4) as u64) } else { 0x1028b9bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b9bcu64) }
        0x1028b9bc => { vm.decode(0x60491519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b9c0u64) }
        0x1028b9c0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028b9c4u64) }
        0x1028b9c4 => { vm.decode(0x5040022u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028b9c8u64) }
        0x1028b9c8 => { vm.decode(0x803019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b9ccu64) }
        0x1028b9cc => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b9d0u64) }
        0x1028b9d0 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028b9d4u64) }
        0x1028b9d4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028b9d8u64).wrapping_add((imm * 4) as u64) } else { 0x1028b9d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b9d8u64) }
        0x1028b9d8 => { vm.decode(0x30803019u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028b9dcu64) }
        0x1028b9dc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028b9e0u64) }
        0x1028b9e0 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028b9e4u64) }
        0x1028b9e4 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028b9e8u64).wrapping_add((imm * 4) as u64) } else { 0x1028b9e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028b9e8u64) }
        0x1028b9e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028b9ecu64) } }
        0x1028b9ec => { vm.decode(0x8120002eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64).wrapping_add(3) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 24 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shl(24 - sh) | (x & (0x00ff_ffffu32 >> sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028b9f0u64) }
        0x1028b9f0 => { vm.decode(0x2ff57e5fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 0 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shr(sh) | (x & 0xffff_ffffu32.wrapping_shl(32 - sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028b9f4u64) }
        0x1028b9f4 => { vm.decode(0x4003a649u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let x = vm.ld(s) as u32;
        let v = ((x >> 8) & 0x00ff_00ff) | ((x << 8) & 0xff00_ff00);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1028b9f8u64) }
        0x1028b9f8 => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028b9fcu64) }
        0x1028b9fc => { vm.decode(0x1040022u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028ba00u64) }
        0x1028ba00 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ba04u64) }
        0x1028ba04 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba08u64) }
        0x1028ba08 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba0cu64) }
        0x1028ba0c => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ba10u64).wrapping_add((imm * 4) as u64) } else { 0x1028ba10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ba10u64) }
        0x1028ba10 => { vm.decode(0x30002019u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ba14u64) }
        0x1028ba14 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ba18u64) }
        0x1028ba18 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ba1cu64) }
        0x1028ba1c => { vm.decode(0x40001119u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ba20u64) }
        0x1028ba20 => { vm.decode(0x2636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba24u64) }
        0x1028ba24 => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba28u64) }
        0x1028ba28 => { vm.decode(0xc8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ba2cu64).wrapping_add((imm * 4) as u64) } else { 0x1028ba2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ba2cu64) }
        0x1028ba2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ba30u64) } }
        0x1028ba30 => { vm.decode(0xaaecae51u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = -8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba34u64) }
        0x1028ba34 => { vm.decode(0xa2ec27d1u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -28i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba38u64) }
        0x1028ba38 => { vm.decode(0x1c05cd35u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba3cu64) }
        0x1028ba3c => { vm.decode(0x815cd35u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba40u64) }
        0x1028ba40 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ba44u64) }
        0x1028ba44 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028ba48u64) }
        0x1028ba48 => { vm.decode(0x80a403b5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028ba4cu64) }
        0x1028ba4c => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ba50u64) }
        0x1028ba50 => { vm.decode(0xaae827d1u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = -60i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba54u64) }
        0x1028ba54 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028ba58u64) }
        0x1028ba58 => { vm.decode(0xc254d35u32); let d = 4i16; let s = 5i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba5cu64) }
        0x1028ba5c => { vm.decode(0xcb4cd35u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba60u64) }
        0x1028ba60 => { vm.decode(0x1418835u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ba64u64) }
        0x1028ba64 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028ba68u64) }
        0x1028ba68 => { vm.decode(0x80e843b5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028ba6cu64) }
        0x1028ba6c => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ba70u64) }
        0x1028ba70 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028ba74u64) }
        0x1028ba74 => { vm.decode(0xa2e82751u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba78u64) }
        0x1028ba78 => { vm.decode(0x280a019u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ba7cu64) }
        0x1028ba7c => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba80u64) }
        0x1028ba80 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028ba84u64) }
        0x1028ba84 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ba88u64) }
        0x1028ba88 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028ba8cu64) }
        0x1028ba8c => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ba90u64) }
        0x1028ba90 => { vm.decode(0x27d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -25i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ba94u64).wrapping_add((imm * 4) as u64) } else { 0x1028ba94u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ba94u64) }
        0x1028ba94 => { vm.decode(0x44004875u32); let d = 2i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ba98u64) }
        0x1028ba98 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ba9cu64) }
        0x1028ba9c => { vm.decode(0xd006a019u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 142i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028baa0u64) }
        0x1028baa0 => { vm.decode(0x2100236u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028baa4u64) }
        0x1028baa4 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028baa8u64) }
        0x1028baa8 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028baacu64) }
        0x1028baac => { vm.decode(0x6a8843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bab0u64) }
        0x1028bab0 => { vm.decode(0x428c63efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bab4u64) }
        0x1028bab4 => { vm.decode(0x488443afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bab8u64) }
        0x1028bab8 => { vm.decode(0x79004c6u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 101i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028babcu64) }
        0x1028babc => { vm.decode(0x10848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 79i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028bac0u64).wrapping_add((imm * 4) as u64) } else { 0x1028bac0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bac0u64) }
        0x1028bac0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028bac4u64) } }
        0x1028bac4 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bac8u64) }
        0x1028bac8 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028baccu64) }
        0x1028bacc => { vm.decode(0x1119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bad0u64) }
        0x1028bad0 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bad4u64) }
        0x1028bad4 => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bad8u64) }
        0x1028bad8 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028badcu64) }
        0x1028badc => { vm.decode(0xd98ea51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bae0u64) }
        0x1028bae0 => { vm.decode(0x33d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bae4u64) }
        0x1028bae4 => { vm.decode(0x5d9ce351u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bae8u64) }
        0x1028bae8 => { vm.decode(0x5598fbd1u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028baecu64) }
        0x1028baec => { vm.decode(0x159ce3d1u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028baf0u64) }
        0x1028baf0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028baf4u64) }
        0x1028baf4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028baf8u64) }
        0x1028baf8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028bafcu64 + 4);
        vm.wr(37, 0x1028bafcu64 + 4);
        Ok(0x1028bafcu64) }
        0x1028bafc => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb00u64) }
        0x1028bb00 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bb04u64) }
        0x1028bb04 => { vm.decode(0x21019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bb08u64) }
        0x1028bb08 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bb0cu64) }
        0x1028bb0c => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bb10u64) }
        0x1028bb10 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bb14u64) }
        0x1028bb14 => { vm.decode(0x20c443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bb18u64) }
        0x1028bb18 => { vm.decode(0x80402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028bb1cu64) }
        0x1028bb1c => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bb20u64) }
        0x1028bb20 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028bb24u64) }
        0x1028bb24 => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb28u64) }
        0x1028bb28 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb2cu64) }
        0x1028bb2c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb30u64) }
        0x1028bb30 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028bb34u64 + 4);
        vm.wr(37, 0x1028bb34u64 + 4);
        Ok(0x1028bb34u64) }
        0x1028bb34 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb38u64) }
        0x1028bb38 => { vm.decode(0x53b835u32); let d = 1i16; let s = 20i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb3cu64) }
        0x1028bb3c => { vm.decode(0x131217f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb40u64) }
        0x1028bb40 => { vm.decode(0x40112835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb44u64) }
        0x1028bb44 => { vm.decode(0x2f30cd35u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb48u64) }
        0x1028bb48 => { vm.decode(0x3db04d35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 6i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb4cu64) }
        0x1028bb4c => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb50u64) }
        0x1028bb50 => { vm.decode(0x3b31cd35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb54u64) }
        0x1028bb54 => { vm.decode(0x404c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb58u64) }
        0x1028bb58 => { vm.decode(0x4d18c211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb5cu64) }
        0x1028bb5c => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb60u64) }
        0x1028bb60 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb64u64) }
        0x1028bb64 => { vm.decode(0xaf00236u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028bb68u64) }
        0x1028bb68 => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb6cu64) }
        0x1028bb6c => { vm.decode(0xcc43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bb70u64) }
        0x1028bb70 => { vm.decode(0x82001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028bb74u64) }
        0x1028bb74 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bb78u64) }
        0x1028bb78 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028bb7cu64) }
        0x1028bb7c => { vm.decode(0x40400011u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb80u64) }
        0x1028bb80 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb84u64) }
        0x1028bb84 => { vm.decode(0x459c62d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb88u64) }
        0x1028bb88 => { vm.decode(0x4d9c6251u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb8cu64) }
        0x1028bb8c => { vm.decode(0x4c01875u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bb90u64) }
        0x1028bb90 => { vm.decode(0xd986b51u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb94u64) }
        0x1028bb94 => { vm.decode(0x45986bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb98u64) }
        0x1028bb98 => { vm.decode(0x20330fb5u32); let d = 6i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bb9cu64) }
        0x1028bb9c => { vm.decode(0x441257f5u32); let d = 7i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bba0u64) }
        0x1028bba0 => { vm.decode(0x120147f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bba4u64) }
        0x1028bba4 => { vm.decode(0x59cf2d1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bba8u64) }
        0x1028bba8 => { vm.decode(0xd98fad1u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbacu64) }
        0x1028bbac => { vm.decode(0x459cf251u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbb0u64) }
        0x1028bbb0 => { vm.decode(0x436d3c62u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbb4u64) }
        0x1028bbb4 => { vm.decode(0x623f5c62u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbb8u64) }
        0x1028bbb8 => { vm.decode(0x623d5862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbbcu64) }
        0x1028bbbc => { vm.decode(0x632d5c62u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbc0u64) }
        0x1028bbc0 => { vm.decode(0x632f1862u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbc4u64) }
        0x1028bbc4 => { vm.decode(0x422f3862u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbc8u64) }
        0x1028bbc8 => { vm.decode(0x427d7862u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbccu64) }
        0x1028bbcc => { vm.decode(0x50484875u32); let d = 2i16; let s = 4i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bbd0u64) }
        0x1028bbd0 => { vm.decode(0x2a84cd35u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbd4u64) }
        0x1028bbd4 => { vm.decode(0x3c044d35u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbd8u64) }
        0x1028bbd8 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bbdcu64) }
        0x1028bbdc => { vm.decode(0x3c914d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbe0u64) }
        0x1028bbe0 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bbe4u64) }
        0x1028bbe4 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbe8u64) }
        0x1028bbe8 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bbecu64) }
        0x1028bbec => { vm.decode(0x433d3c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bbf0u64) }
        0x1028bbf0 => { vm.decode(0x79004c6u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 101i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028bbf4u64) }
        0x1028bbf4 => { vm.decode(0x27f32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -77i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bbf8u64).wrapping_add((imm * 4) as u64) } else { 0x1028bbf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bbf8u64) }
        0x1028bbf8 => { vm.decode(0x70008099u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bbfcu64) }
        0x1028bbfc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc00u64) }
        0x1028bc00 => { vm.decode(0x646b9159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc04u64) }
        0x1028bc04 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc08u64) }
        0x1028bc08 => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028bc0cu64).wrapping_add((imm * 4) as u64) } else { 0x1028bc0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bc0cu64) }
        0x1028bc0c => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc10u64) }
        0x1028bc10 => { vm.decode(0xc400011u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bc14u64) }
        0x1028bc14 => { vm.decode(0x8100111u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bc18u64) }
        0x1028bc18 => { vm.decode(0x44110fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028bc1cu64) }
        0x1028bc1c => { vm.decode(0x1001022u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028bc20u64) }
        0x1028bc20 => { vm.decode(0x2012019u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc24u64) }
        0x1028bc24 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc28u64) }
        0x1028bc28 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc2cu64) }
        0x1028bc2c => { vm.decode(0xa7f10db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -8i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bc30u64).wrapping_add((imm * 4) as u64) } else { 0x1028bc30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bc30u64) }
        0x1028bc30 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc34u64) }
        0x1028bc34 => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bc38u64) }
        0x1028bc38 => { vm.decode(0xa0804118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028bc3cu64) }
        0x1028bc3c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc40u64) }
        0x1028bc40 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc44u64) }
        0x1028bc44 => { vm.decode(0xc8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028bc48u64).wrapping_add((imm * 4) as u64) } else { 0x1028bc48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bc48u64) }
        0x1028bc48 => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc4cu64) }
        0x1028bc4c => { vm.decode(0x300010beu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028bc50u64) }
        0x1028bc50 => { vm.decode(0x9dbe7f33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028bc54u64) }
        0x1028bc54 => { vm.decode(0x808823u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x1028bc58u64) }
        0x1028bc58 => { vm.decode(0x1dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028bc5cu64) }
        0x1028bc5c => { vm.decode(0x8100011u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bc60u64) }
        0x1028bc60 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1028bc64u64) }
        0x1028bc64 => { vm.decode(0x9dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028bc68u64) }
        0x1028bc68 => { vm.decode(0x1a019u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc6cu64) }
        0x1028bc6c => { vm.decode(0x4280a019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bc70u64) }
        0x1028bc70 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc74u64) }
        0x1028bc74 => { vm.decode(0x100111u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028bc78u64) }
        0x1028bc78 => { vm.decode(0x600833u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028bc7cu64) }
        0x1028bc7c => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bc80u64) }
        0x1028bc80 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028bc84u64) }
        0x1028bc84 => { vm.decode(0x27f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bc88u64).wrapping_add((imm * 4) as u64) } else { 0x1028bc88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bc88u64) }
        0x1028bc88 => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bc8cu64) }
        0x1028bc8c => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bc90u64) }
        0x1028bc90 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bc94u64) }
        0x1028bc94 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bc98u64) }
        0x1028bc98 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bc9cu64) }
        0x1028bc9c => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bca0u64) }
        0x1028bca0 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bca4u64) }
        0x1028bca4 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bca8u64) }
        0x1028bca8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028bcacu64) }
        0x1028bcac => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcb0u64) }
        0x1028bcb0 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bcb4u64) }
        0x1028bcb4 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028bcb8u64) }
        0x1028bcb8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028bcbcu64) } }
        0x1028bcbc => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bcc0u64) }
        0x1028bcc0 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcc4u64) }
        0x1028bcc4 => { vm.decode(0x40008119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bcc8u64) }
        0x1028bcc8 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcccu64) }
        0x1028bccc => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028bcd0u64) }
        0x1028bcd0 => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028bcd4u64) }
        0x1028bcd4 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcd8u64) }
        0x1028bcd8 => { vm.decode(0x12008019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bcdcu64) }
        0x1028bcdc => { vm.decode(0x604b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bce0u64) }
        0x1028bce0 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bce4u64) }
        0x1028bce4 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bce8u64) }
        0x1028bce8 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcecu64) }
        0x1028bcec => { vm.decode(0x84028cd9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 19331i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bcf0u64) }
        0x1028bcf0 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028bcf4u64) }
        0x1028bcf4 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bcf8u64) }
        0x1028bcf8 => { vm.decode(0x8019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028bcfcu64) }
        0x1028bcfc => { vm.decode(0x8a020071u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bd00u64).wrapping_add((imm * 4) as u64) } else { 0x1028bd00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bd00u64) }
        0x1028bd00 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bd04u64) }
        0x1028bd04 => { vm.decode(0x100851du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 24624i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028bd08u64) }
        0x1028bd2c => { vm.decode(0x4c0400a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 416i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028bd30u64).wrapping_add((imm * 4) as u64) } else { 0x1028bd30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bd30u64) }
        0x1028bd30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028bd34u64) } }
        0x1028bd34 => { vm.decode(0x12404171u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 196i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bd38u64).wrapping_add((imm * 4) as u64) } else { 0x1028bd38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bd38u64) }
        0x1028bd38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028bd3cu64) } }
        0x1028bd3c => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028bd40u64) }
        0x1028bd40 => { vm.decode(0x2420131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 193i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028bd44u64).wrapping_add((imm * 4) as u64) } else { 0x1028bd44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028bd44u64) }
        0x1028bd44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028bd48u64) } }
        0x1028bd48 => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028bd4cu64) }
        0x1028c048 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c04cu64) }
        0x1028c04c => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c050u64) }
        0x1028c050 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c054u64) }
        0x1028c054 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c058u64) }
        0x1028c058 => { vm.decode(0x628c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c05cu64) }
        0x1028c05c => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c060u64) }
        0x1028c060 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c064u64) }
        0x1028c064 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c068u64) }
        0x1028c068 => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c06cu64) }
        0x1028c06c => { vm.decode(0xc1422b75u32); let d = 16i16; let s = 20i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028c070u64) }
        0x1028c070 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028c074u64) }
        0x1028c074 => { vm.decode(0x24040e35u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x1028c078u64) }
        0x1028c078 => { vm.decode(0x840042f5u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x1028c07cu64) }
        0x1028c07c => { vm.decode(0x8b614909u32); let d = 18i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028c080u64) }
        0x1028c080 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c084u64) }
        0x1028c084 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c088u64) }
        0x1028c088 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c08cu64) }
        0x1028c08c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c090u64) }
        0x1028c090 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c094u64 + 4);
        vm.wr(37, 0x1028c094u64 + 4);
        Ok(0x1028c094u64) }
        0x1028c094 => { vm.decode(0xc009875u32); let d = 17i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c098u64) }
        0x1028c098 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c09cu64) }
        0x1028c09c => { vm.decode(0x6b4504f5u32); let d = 19i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c0a0u64) }
        0x1028c0a0 => { vm.decode(0x9b3815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c0a4u64) }
        0x1028c0a4 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c0a8u64) }
        0x1028c0a8 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c0acu64) }
        0x1028c0ac => { vm.decode(0xf6b37cf1u32); let d = 20i16; let s = 21i16; let s2 = -1i16; let imm = -225i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c0b0u64).wrapping_add((imm * 4) as u64) } else { 0x1028c0b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c0b0u64) }
        0x1028c0b0 => { vm.decode(0xa8208b35u32); let d = 3i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c0b4u64) }
        0x1028c0b4 => { vm.decode(0x32d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028c0b8u64) }
        0x1028c3b0 => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c3b4u64) }
        0x1028c3b4 => { vm.decode(0x6a8863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3b8u64) }
        0x1028c3b8 => { vm.decode(0x2824412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3bcu64) }
        0x1028c3bc => { vm.decode(0x10000119u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c3c0u64) }
        0x1028c3c0 => { vm.decode(0x8b211430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c3c4u64) }
        0x1028c3c4 => { vm.decode(0x28843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3c8u64) }
        0x1028c3c8 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c3ccu64) }
        0x1028c3cc => { vm.decode(0x740f1119u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c3d0u64) }
        0x1028c3d0 => { vm.decode(0x8b1115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c3d4u64) }
        0x1028c3d4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c3d8u64) }
        0x1028c3d8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3dcu64) }
        0x1028c3dc => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3e0u64) }
        0x1028c3e0 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c3e4u64) }
        0x1028c3e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c3e8u64 + 4);
        vm.wr(37, 0x1028c3e8u64 + 4);
        Ok(0x1028c3e8u64) }
        0x1028c3e8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c3ecu64) }
        0x1028c3ec => { vm.decode(0x2220416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c3f0u64) }
        0x1028c3f0 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c3f4u64) }
        0x1028c3f4 => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c3f8u64) }
        0x1028c3f8 => { vm.decode(0x2102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c3fcu64) }
        0x1028c3fc => { vm.decode(0x8b2114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c400u64) }
        0x1028c400 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c404u64) }
        0x1028c404 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c408u64) }
        0x1028c408 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c40cu64) }
        0x1028c40c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c410u64) }
        0x1028c410 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c414u64) }
        0x1028c414 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c418u64 + 4);
        vm.wr(37, 0x1028c418u64 + 4);
        Ok(0x1028c418u64) }
        0x1028c418 => { vm.decode(0x8001875u32); let d = 16i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c41cu64) }
        0x1028c41c => { vm.decode(0x84028cd9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 19331i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c420u64) }
        0x1028c420 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028c424u64) }
        0x1028c424 => { vm.decode(0x4054c959u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c428u64) }
        0x1028c428 => { vm.decode(0x12041019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c42cu64) }
        0x1028c42c => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c430u64) }
        0x1028c430 => { vm.decode(0x8a4604f5u32); let d = 23i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c434u64) }
        0x1028c434 => { vm.decode(0x760d0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c438u64) }
        0x1028c438 => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028c43cu64) }
        0x1028c43c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c440u64).wrapping_add((imm * 4) as u64) } else { 0x1028c440u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c440u64) }
        0x1028c440 => { vm.decode(0x3110236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c444u64) }
        0x1028c444 => { vm.decode(0x8024811du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 28864i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028c448u64) }
        0x1028c44c => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c450u64) }
        0x1028c450 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x1028c454u64) }
        0x1028c454 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c458u64) }
        0x1028c458 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c45cu64) }
        0x1028c45c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c460u64) }
        0x1028c460 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c464u64) }
        0x1028c464 => { vm.decode(0xc90104f5u32); let d = 2i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c468u64) }
        0x1028c468 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c46cu64) }
        0x1028c46c => { vm.decode(0xad03036u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c470u64) }
        0x1028c470 => { vm.decode(0x25000486u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028c474u64) }
        0x1028c474 => { vm.decode(0x560c08a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1010i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c478u64).wrapping_add((imm * 4) as u64) } else { 0x1028c478u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c478u64) }
        0x1028c478 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c47cu64) } }
        0x1028c47c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c480u64) }
        0x1028c480 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c484u64) }
        0x1028c484 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c488u64) }
        0x1028c488 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c48cu64) }
        0x1028c48c => { vm.decode(0x9b115530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c490u64) }
        0x1028c490 => { vm.decode(0xc0e20bb5u32); let d = 19i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c494u64) }
        0x1028c494 => { vm.decode(0xc34104f5u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c498u64) }
        0x1028c498 => { vm.decode(0x2004023eu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028c49cu64) }
        0x1028c49c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c4a0u64) }
        0x1028c4a0 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c4a4u64) }
        0x1028c4a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c4a8u64 + 4);
        vm.wr(37, 0x1028c4a8u64 + 4);
        Ok(0x1028c4a8u64) }
        0x1028c4a8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c4acu64) }
        0x1028c4ac => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028c4b0u64) }
        0x1028c4b0 => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c4b4u64) }
        0x1028c4b4 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4b8u64) }
        0x1028c4b8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028c4bcu64) }
        0x1028c4bc => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c4c0u64).wrapping_add((imm * 4) as u64) } else { 0x1028c4c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c4c0u64) }
        0x1028c4c0 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4c4u64) }
        0x1028c4c4 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4c8u64) }
        0x1028c4c8 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028c4ccu64) }
        0x1028c4cc => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c4d0u64) }
        0x1028c4d0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028c4d4u64) }
        0x1028c4d4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028c4d8u64) }
        0x1028c4d8 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c4dcu64).wrapping_add((imm * 4) as u64) } else { 0x1028c4dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c4dcu64) }
        0x1028c4dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c4e0u64) } }
        0x1028c4e0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4e4u64) }
        0x1028c4e4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028c4e8u64) }
        0x1028c4e8 => { vm.decode(0x731c36a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c4ecu64).wrapping_add((imm * 4) as u64) } else { 0x1028c4ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c4ecu64) }
        0x1028c4ec => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4f0u64) }
        0x1028c4f0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c4f4u64) }
        0x1028c4f4 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028c4f8u64) }
        0x1028c4f8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028c4fcu64) }
        0x1028c4fc => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028c500u64) }
        0x1028c500 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028c504u64) }
        0x1028c504 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c508u64).wrapping_add((imm * 4) as u64) } else { 0x1028c508u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c508u64) }
        0x1028c508 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c50cu64) } }
        0x1028c50c => { vm.decode(0xa820811du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 24780i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028c510u64) }
        0x1028c510 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c514u64) } }
        0x1028c514 => { vm.decode(0x4d94e211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028c518u64) }
        0x1028c518 => { vm.decode(0x281c1227u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 7192i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c51cu64).wrapping_add((imm * 4) as u64) } else { 0x1028c51cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c51cu64) }
        0x1028c51c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c520u64) } }
        0x1028c520 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028c524u64) }
        0x1028c598 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c59cu64) }
        0x1028c59c => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028c5a0u64) }
        0x1028c5a0 => { vm.decode(0x508488a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 455i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c5a4u64).wrapping_add((imm * 4) as u64) } else { 0x1028c5a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c5a4u64) }
        0x1028c5a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028c5a8u64) } }
        0x1028c5a8 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c5acu64) }
        0x1028c5ac => { vm.decode(0xe04004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c5b0u64) }
        0x1028c5b0 => { vm.decode(0x8740323eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028c5b4u64) }
        0x1028c5b4 => { vm.decode(0x20000833u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c5b8u64) }
        0x1028c5b8 => { vm.decode(0x40801099u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c5bcu64) }
        0x1028c5bc => { vm.decode(0xa0006600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 57i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028c5c0u64) }
        0x1028c5c0 => { vm.decode(0x2600131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 208i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c5c4u64).wrapping_add((imm * 4) as u64) } else { 0x1028c5c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c5c4u64) }
        0x1028c5c4 => { vm.decode(0xa200ad75u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028c5c8u64) }
        0x1028c904 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c908u64) }
        0x1028c908 => { vm.decode(0x286004f5u32); let d = 3i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c90cu64) }
        0x1028c90c => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028c910u64) }
        0x1028c910 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c914u64).wrapping_add((imm * 4) as u64) } else { 0x1028c914u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c914u64) }
        0x1028c914 => { vm.decode(0x40204018u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028c918u64) }
        0x1028c918 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c91cu64) }
        0x1028c91c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028c920u64) }
        0x1028c920 => { vm.decode(0x2800103eu32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028c924u64) }
        0x1028c924 => { vm.decode(0x420833u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c928u64) }
        0x1028c928 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c92cu64) }
        0x1028c92c => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028c930u64) }
        0x1028c930 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028c934u64).wrapping_add((imm * 4) as u64) } else { 0x1028c934u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c934u64) }
        0x1028c934 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c938u64) }
        0x1028c938 => { vm.decode(0x2300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c93cu64) }
        0x1028c93c => { vm.decode(0x930403b5u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c940u64) }
        0x1028c940 => { vm.decode(0x834003b5u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c944u64) }
        0x1028c944 => { vm.decode(0x81c403b5u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c948u64) }
        0x1028c948 => { vm.decode(0xf20a4ef3u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1863i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c94cu64) }
        0x1028c94c => { vm.decode(0x524a4ef3u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1862i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c950u64) }
        0x1028c950 => { vm.decode(0xd2484e73u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1861i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c954u64) }
        0x1028c954 => { vm.decode(0x72084e73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c958u64) }
        0x1028c958 => { vm.decode(0xd20a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1859i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c95cu64) }
        0x1028c95c => { vm.decode(0x22300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c960u64) }
        0x1028c960 => { vm.decode(0x520a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1858i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c964u64) }
        0x1028c964 => { vm.decode(0x2320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c968u64) }
        0x1028c968 => { vm.decode(0xd20a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1857i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c96cu64) }
        0x1028c96c => { vm.decode(0x22320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028c970u64) }
        0x1028c970 => { vm.decode(0x520a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c974u64) }
        0x1028c974 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c978u64) }
        0x1028c978 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c97cu64) }
        0x1028c97c => { vm.decode(0x101019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c980u64) }
        0x1028c980 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c984u64) }
        0x1028c984 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028c988u64) }
        0x1028c988 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c98cu64) }
        0x1028c98c => { vm.decode(0x746b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c990u64) }
        0x1028c990 => { vm.decode(0x8b311530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c994u64) }
        0x1028c994 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c998u64) }
        0x1028c998 => { vm.decode(0x760d0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c99cu64) }
        0x1028c99c => { vm.decode(0x408ec32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c9a0u64) }
        0x1028c9a0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9a4u64) }
        0x1028c9a4 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c9a8u64) }
        0x1028c9a8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9acu64) }
        0x1028c9ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c9b0u64 + 4);
        vm.wr(37, 0x1028c9b0u64 + 4);
        Ok(0x1028c9b0u64) }
        0x1028c9b0 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9b4u64) }
        0x1028c9b4 => { vm.decode(0x740d0059u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c9b8u64) }
        0x1028c9b8 => { vm.decode(0x4008019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c9bcu64) }
        0x1028c9bc => { vm.decode(0x288416fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c9c0u64) }
        0x1028c9c0 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c9c4u64) }
        0x1028c9c4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028c9c8u64) }
        0x1028c9c8 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c9ccu64) }
        0x1028c9cc => { vm.decode(0xb291530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028c9d0u64) }
        0x1028c9d0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9d4u64) }
        0x1028c9d4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9d8u64) }
        0x1028c9d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028c9dcu64 + 4);
        vm.wr(37, 0x1028c9dcu64 + 4);
        Ok(0x1028c9dcu64) }
        0x1028c9dc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028c9e0u64) }
        0x1028c9e0 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028c9e4u64) }
        0x1028c9e4 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028c9e8u64) }
        0x1028c9e8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028c9ecu64).wrapping_add((imm * 4) as u64) } else { 0x1028c9ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028c9ecu64) }
        0x1028c9ec => { vm.decode(0x60491519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c9f0u64) }
        0x1028c9f0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028c9f4u64) }
        0x1028c9f4 => { vm.decode(0x5040022u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028c9f8u64) }
        0x1028c9f8 => { vm.decode(0x803019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028c9fcu64) }
        0x1028c9fc => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca00u64) }
        0x1028ca00 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca04u64) }
        0x1028ca04 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ca08u64).wrapping_add((imm * 4) as u64) } else { 0x1028ca08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ca08u64) }
        0x1028ca08 => { vm.decode(0x30803019u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ca0cu64) }
        0x1028ca0c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca10u64) }
        0x1028ca10 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca14u64) }
        0x1028ca14 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ca18u64).wrapping_add((imm * 4) as u64) } else { 0x1028ca18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ca18u64) }
        0x1028ca18 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ca1cu64) } }
        0x1028ca1c => { vm.decode(0x8120002eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64).wrapping_add(3) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 24 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shl(24 - sh) | (x & (0x00ff_ffffu32 >> sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028ca20u64) }
        0x1028ca20 => { vm.decode(0x2ff57e5fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 0 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shr(sh) | (x & 0xffff_ffffu32.wrapping_shl(32 - sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028ca24u64) }
        0x1028ca24 => { vm.decode(0x4003a649u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let x = vm.ld(s) as u32;
        let v = ((x >> 8) & 0x00ff_00ff) | ((x << 8) & 0xff00_ff00);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1028ca28u64) }
        0x1028ca28 => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca2cu64) }
        0x1028ca2c => { vm.decode(0x1040022u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028ca30u64) }
        0x1028ca30 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ca34u64) }
        0x1028ca34 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca38u64) }
        0x1028ca38 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca3cu64) }
        0x1028ca3c => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ca40u64).wrapping_add((imm * 4) as u64) } else { 0x1028ca40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ca40u64) }
        0x1028ca40 => { vm.decode(0x30002019u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ca44u64) }
        0x1028ca44 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ca48u64) }
        0x1028ca48 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ca4cu64) }
        0x1028ca4c => { vm.decode(0x40001119u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028ca50u64) }
        0x1028ca50 => { vm.decode(0x2636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca54u64) }
        0x1028ca54 => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ca58u64) }
        0x1028ca58 => { vm.decode(0xc8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ca5cu64).wrapping_add((imm * 4) as u64) } else { 0x1028ca5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ca5cu64) }
        0x1028ca5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ca60u64) } }
        0x1028ca60 => { vm.decode(0xaaecae51u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = -8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca64u64) }
        0x1028ca64 => { vm.decode(0xa2ec27d1u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -28i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca68u64) }
        0x1028ca68 => { vm.decode(0x1c05cd35u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca6cu64) }
        0x1028ca6c => { vm.decode(0x815cd35u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca70u64) }
        0x1028ca70 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ca74u64) }
        0x1028ca74 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028ca78u64) }
        0x1028ca78 => { vm.decode(0x80a403b5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028ca7cu64) }
        0x1028ca7c => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ca80u64) }
        0x1028ca80 => { vm.decode(0xaae827d1u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = -60i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca84u64) }
        0x1028ca84 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028ca88u64) }
        0x1028ca88 => { vm.decode(0xc254d35u32); let d = 4i16; let s = 5i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca8cu64) }
        0x1028ca8c => { vm.decode(0xcb4cd35u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028ca90u64) }
        0x1028ca90 => { vm.decode(0x1418835u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ca94u64) }
        0x1028ca94 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028ca98u64) }
        0x1028ca98 => { vm.decode(0x80e843b5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028ca9cu64) }
        0x1028ca9c => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028caa0u64) }
        0x1028caa0 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028caa4u64) }
        0x1028caa4 => { vm.decode(0xa2e82751u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028caa8u64) }
        0x1028caa8 => { vm.decode(0x280a019u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028caacu64) }
        0x1028caac => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cab0u64) }
        0x1028cab0 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028cab4u64) }
        0x1028cab4 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cab8u64) }
        0x1028cab8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cabcu64) }
        0x1028cabc => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028cac0u64) }
        0x1028cac0 => { vm.decode(0x27d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -25i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028cac4u64).wrapping_add((imm * 4) as u64) } else { 0x1028cac4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cac4u64) }
        0x1028cac4 => { vm.decode(0x44004875u32); let d = 2i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cac8u64) }
        0x1028cac8 => { vm.decode(0x8424a6d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 7951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028caccu64) }
        0x1028cacc => { vm.decode(0xa0422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028cad0u64) }
        0x1028cad0 => { vm.decode(0xdc12afd9u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = -9241i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cad4u64) }
        0x1028cad4 => { vm.decode(0x2100236u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cad8u64) }
        0x1028cad8 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cadcu64) }
        0x1028cadc => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cae0u64) }
        0x1028cae0 => { vm.decode(0x6a8843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cae4u64) }
        0x1028cae4 => { vm.decode(0x428c63efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cae8u64) }
        0x1028cae8 => { vm.decode(0x488443afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028caecu64) }
        0x1028caec => { vm.decode(0x79004c6u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 101i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028caf0u64) }
        0x1028caf0 => { vm.decode(0x10848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 79i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028caf4u64).wrapping_add((imm * 4) as u64) } else { 0x1028caf4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028caf4u64) }
        0x1028caf4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028caf8u64) } }
        0x1028caf8 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cafcu64) }
        0x1028cafc => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cb00u64) }
        0x1028cb00 => { vm.decode(0x1119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cb04u64) }
        0x1028cb04 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb08u64) }
        0x1028cb08 => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb0cu64) }
        0x1028cb0c => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb10u64) }
        0x1028cb10 => { vm.decode(0xd9ce2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb14u64) }
        0x1028cb14 => { vm.decode(0x33d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cb18u64) }
        0x1028cb18 => { vm.decode(0x5d986b51u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb1cu64) }
        0x1028cb1c => { vm.decode(0x559c7351u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb20u64) }
        0x1028cb20 => { vm.decode(0x159ce351u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb24u64) }
        0x1028cb24 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb28u64) }
        0x1028cb28 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb2cu64) }
        0x1028cb2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028cb30u64 + 4);
        vm.wr(37, 0x1028cb30u64 + 4);
        Ok(0x1028cb30u64) }
        0x1028cb30 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb34u64) }
        0x1028cb34 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cb38u64) }
        0x1028cb38 => { vm.decode(0x21019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cb3cu64) }
        0x1028cb3c => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb40u64) }
        0x1028cb40 => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb44u64) }
        0x1028cb44 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cb48u64) }
        0x1028cb48 => { vm.decode(0x20c443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cb4cu64) }
        0x1028cb4c => { vm.decode(0x80402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028cb50u64) }
        0x1028cb50 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cb54u64) }
        0x1028cb54 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028cb58u64) }
        0x1028cb58 => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb5cu64) }
        0x1028cb5c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb60u64) }
        0x1028cb60 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb64u64) }
        0x1028cb64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028cb68u64 + 4);
        vm.wr(37, 0x1028cb68u64 + 4);
        Ok(0x1028cb68u64) }
        0x1028cb68 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb6cu64) }
        0x1028cb6c => { vm.decode(0x53b835u32); let d = 1i16; let s = 20i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb70u64) }
        0x1028cb70 => { vm.decode(0x131217f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb74u64) }
        0x1028cb74 => { vm.decode(0x40112835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb78u64) }
        0x1028cb78 => { vm.decode(0x2f30cd35u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb7cu64) }
        0x1028cb7c => { vm.decode(0x3db04d35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 6i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb80u64) }
        0x1028cb80 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb84u64) }
        0x1028cb84 => { vm.decode(0x3b31cd35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb88u64) }
        0x1028cb88 => { vm.decode(0x404c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cb8cu64) }
        0x1028cb8c => { vm.decode(0x4d18c211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb90u64) }
        0x1028cb90 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb94u64) }
        0x1028cb94 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cb98u64) }
        0x1028cb98 => { vm.decode(0xaf00236u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cb9cu64) }
        0x1028cb9c => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cba0u64) }
        0x1028cba0 => { vm.decode(0xcc43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cba4u64) }
        0x1028cba4 => { vm.decode(0x82001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028cba8u64) }
        0x1028cba8 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cbacu64) }
        0x1028cbac => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028cbb0u64) }
        0x1028cbb0 => { vm.decode(0x40400011u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbb4u64) }
        0x1028cbb4 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbb8u64) }
        0x1028cbb8 => { vm.decode(0x4598ead1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbbcu64) }
        0x1028cbbc => { vm.decode(0x4d9c62d1u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbc0u64) }
        0x1028cbc0 => { vm.decode(0x4c01875u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cbc4u64) }
        0x1028cbc4 => { vm.decode(0xd98eb51u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbc8u64) }
        0x1028cbc8 => { vm.decode(0x45986bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbccu64) }
        0x1028cbcc => { vm.decode(0x20330fb5u32); let d = 6i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbd0u64) }
        0x1028cbd0 => { vm.decode(0x441257f5u32); let d = 7i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cbd4u64) }
        0x1028cbd4 => { vm.decode(0x120147f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cbd8u64) }
        0x1028cbd8 => { vm.decode(0x59c7251u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbdcu64) }
        0x1028cbdc => { vm.decode(0xd9cf251u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbe0u64) }
        0x1028cbe0 => { vm.decode(0x45987a51u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cbe4u64) }
        0x1028cbe4 => { vm.decode(0x422f7c62u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbe8u64) }
        0x1028cbe8 => { vm.decode(0x633f1c62u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbecu64) }
        0x1028cbec => { vm.decode(0x633d5862u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbf0u64) }
        0x1028cbf0 => { vm.decode(0x622f1c62u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbf4u64) }
        0x1028cbf4 => { vm.decode(0x622d5862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbf8u64) }
        0x1028cbf8 => { vm.decode(0x437d3c62u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cbfcu64) }
        0x1028cbfc => { vm.decode(0x427d7862u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cc00u64) }
        0x1028cc00 => { vm.decode(0x50484875u32); let d = 2i16; let s = 4i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cc04u64) }
        0x1028cc04 => { vm.decode(0x2a84cd35u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc08u64) }
        0x1028cc08 => { vm.decode(0x3c044d35u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc0cu64) }
        0x1028cc0c => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cc10u64) }
        0x1028cc10 => { vm.decode(0x3c914d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc14u64) }
        0x1028cc14 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cc18u64) }
        0x1028cc18 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc1cu64) }
        0x1028cc1c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc20u64) }
        0x1028cc20 => { vm.decode(0x432d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cc24u64) }
        0x1028cc24 => { vm.decode(0x79004c6u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 101i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc28u64) }
        0x1028cc28 => { vm.decode(0x27f32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -77i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028cc2cu64).wrapping_add((imm * 4) as u64) } else { 0x1028cc2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cc2cu64) }
        0x1028cc2c => { vm.decode(0x70008099u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc30u64) }
        0x1028cc30 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc34u64) }
        0x1028cc34 => { vm.decode(0x646b9159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc38u64) }
        0x1028cc38 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc3cu64) }
        0x1028cc3c => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028cc40u64).wrapping_add((imm * 4) as u64) } else { 0x1028cc40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cc40u64) }
        0x1028cc40 => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc44u64) }
        0x1028cc44 => { vm.decode(0xc400011u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc48u64) }
        0x1028cc48 => { vm.decode(0x8100111u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc4cu64) }
        0x1028cc4c => { vm.decode(0x44110fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc50u64) }
        0x1028cc50 => { vm.decode(0x1001022u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028cc54u64) }
        0x1028cc54 => { vm.decode(0x2012019u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc58u64) }
        0x1028cc58 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc5cu64) }
        0x1028cc5c => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc60u64) }
        0x1028cc60 => { vm.decode(0xa7f10db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -8i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028cc64u64).wrapping_add((imm * 4) as u64) } else { 0x1028cc64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cc64u64) }
        0x1028cc64 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc68u64) }
        0x1028cc68 => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cc6cu64) }
        0x1028cc6c => { vm.decode(0xa0804118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028cc70u64) }
        0x1028cc70 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc74u64) }
        0x1028cc74 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028cc78u64) }
        0x1028cc78 => { vm.decode(0xc8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028cc7cu64).wrapping_add((imm * 4) as u64) } else { 0x1028cc7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cc7cu64) }
        0x1028cc7c => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cc80u64) }
        0x1028cc80 => { vm.decode(0x300010beu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028cc84u64) }
        0x1028cc84 => { vm.decode(0x9dbe7f33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028cc88u64) }
        0x1028cc88 => { vm.decode(0x808823u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x1028cc8cu64) }
        0x1028cc8c => { vm.decode(0x1dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028cc90u64) }
        0x1028cc90 => { vm.decode(0x8100011u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028cc94u64) }
        0x1028cc94 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1028cc98u64) }
        0x1028cc98 => { vm.decode(0x9dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028cc9cu64) }
        0x1028cc9c => { vm.decode(0x1a019u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cca0u64) }
        0x1028cca0 => { vm.decode(0x4280a019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cca4u64) }
        0x1028cca4 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028cca8u64) }
        0x1028cca8 => { vm.decode(0x100111u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028ccacu64) }
        0x1028ccac => { vm.decode(0x600833u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028ccb0u64) }
        0x1028ccb0 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ccb4u64) }
        0x1028ccb4 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028ccb8u64) }
        0x1028ccb8 => { vm.decode(0x27f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028ccbcu64).wrapping_add((imm * 4) as u64) } else { 0x1028ccbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ccbcu64) }
        0x1028ccbc => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ccc0u64) }
        0x1028ccc0 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccc4u64) }
        0x1028ccc4 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccc8u64) }
        0x1028ccc8 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028ccccu64) }
        0x1028cccc => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccd0u64) }
        0x1028ccd0 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028ccd4u64) }
        0x1028ccd4 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccd8u64) }
        0x1028ccd8 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccdcu64) }
        0x1028ccdc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028cce0u64) }
        0x1028cce0 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cce4u64) }
        0x1028cce4 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cce8u64) }
        0x1028cce8 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028ccecu64) }
        0x1028ccec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028ccf0u64) } }
        0x1028ccf0 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028ccf4u64) }
        0x1028ccf4 => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028ccf8u64) }
        0x1028ccf8 => { vm.decode(0x508480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 453i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028ccfcu64).wrapping_add((imm * 4) as u64) } else { 0x1028ccfcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028ccfcu64) }
        0x1028ccfc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028cd00u64) } }
        0x1028cd00 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028cd04u64) }
        0x1028cd04 => { vm.decode(0xe04004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028cd08u64) }
        0x1028cd08 => { vm.decode(0x8740323eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028cd0cu64) }
        0x1028cd0c => { vm.decode(0x20000833u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028cd10u64) }
        0x1028cd10 => { vm.decode(0x40801099u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028cd14u64) }
        0x1028cd14 => { vm.decode(0xa0006600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 57i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028cd18u64) }
        0x1028cd18 => { vm.decode(0x82422171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 207i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028cd1cu64).wrapping_add((imm * 4) as u64) } else { 0x1028cd1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028cd1cu64) }
        0x1028cd1c => { vm.decode(0xa200ad75u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028cd20u64) }
        0x1028cd20 => { vm.decode(0xb305430u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028cd24u64) }
        0x1028d058 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d05cu64) }
        0x1028d05c => { vm.decode(0x286004f5u32); let d = 3i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d060u64) }
        0x1028d060 => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028d064u64) }
        0x1028d064 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028d068u64).wrapping_add((imm * 4) as u64) } else { 0x1028d068u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d068u64) }
        0x1028d068 => { vm.decode(0x40204018u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028d06cu64) }
        0x1028d06c => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d070u64) }
        0x1028d070 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d074u64) }
        0x1028d074 => { vm.decode(0x2800103eu32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028d078u64) }
        0x1028d078 => { vm.decode(0x420833u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d07cu64) }
        0x1028d07c => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d080u64) }
        0x1028d080 => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028d084u64) }
        0x1028d084 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028d088u64).wrapping_add((imm * 4) as u64) } else { 0x1028d088u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d088u64) }
        0x1028d088 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d08cu64) }
        0x1028d08c => { vm.decode(0x2300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d090u64) }
        0x1028d090 => { vm.decode(0x930403b5u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d094u64) }
        0x1028d094 => { vm.decode(0x834003b5u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d098u64) }
        0x1028d098 => { vm.decode(0x81c403b5u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d09cu64) }
        0x1028d09c => { vm.decode(0xf20a4ef3u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1863i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0a0u64) }
        0x1028d0a0 => { vm.decode(0x524a4ef3u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1862i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0a4u64) }
        0x1028d0a4 => { vm.decode(0xd2484e73u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1861i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0a8u64) }
        0x1028d0a8 => { vm.decode(0x72084e73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0acu64) }
        0x1028d0ac => { vm.decode(0xd20a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1859i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0b0u64) }
        0x1028d0b0 => { vm.decode(0x22300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d0b4u64) }
        0x1028d0b4 => { vm.decode(0x520a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1858i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0b8u64) }
        0x1028d0b8 => { vm.decode(0x2320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d0bcu64) }
        0x1028d0bc => { vm.decode(0xd20a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1857i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0c0u64) }
        0x1028d0c0 => { vm.decode(0x22320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d0c4u64) }
        0x1028d0c4 => { vm.decode(0x520a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0c8u64) }
        0x1028d0c8 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d0ccu64) }
        0x1028d0cc => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d0d0u64) }
        0x1028d0d0 => { vm.decode(0x101019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d0d4u64) }
        0x1028d0d4 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d0d8u64) }
        0x1028d0d8 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d0dcu64) }
        0x1028d0dc => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d0e0u64) }
        0x1028d0e0 => { vm.decode(0x746b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d0e4u64) }
        0x1028d0e4 => { vm.decode(0x8b311530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d0e8u64) }
        0x1028d0e8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d0ecu64) }
        0x1028d0ec => { vm.decode(0x760d0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d0f0u64) }
        0x1028d0f0 => { vm.decode(0x408ec32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d0f4u64) }
        0x1028d0f4 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d0f8u64) }
        0x1028d0f8 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d0fcu64) }
        0x1028d0fc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d100u64) }
        0x1028d100 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028d104u64 + 4);
        vm.wr(37, 0x1028d104u64 + 4);
        Ok(0x1028d104u64) }
        0x1028d104 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d108u64) }
        0x1028d108 => { vm.decode(0x740d0059u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d10cu64) }
        0x1028d10c => { vm.decode(0x4008019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d110u64) }
        0x1028d110 => { vm.decode(0x288416fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d114u64) }
        0x1028d114 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d118u64) }
        0x1028d118 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028d11cu64) }
        0x1028d11c => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d120u64) }
        0x1028d120 => { vm.decode(0xb291530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d124u64) }
        0x1028d124 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d128u64) }
        0x1028d128 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d12cu64) }
        0x1028d12c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028d130u64 + 4);
        vm.wr(37, 0x1028d130u64 + 4);
        Ok(0x1028d130u64) }
        0x1028d130 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d134u64) }
        0x1028d134 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d138u64) }
        0x1028d138 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d13cu64) }
        0x1028d13c => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028d140u64).wrapping_add((imm * 4) as u64) } else { 0x1028d140u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d140u64) }
        0x1028d140 => { vm.decode(0x60491519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d144u64) }
        0x1028d144 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d148u64) }
        0x1028d148 => { vm.decode(0x5040022u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028d14cu64) }
        0x1028d14c => { vm.decode(0x803019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d150u64) }
        0x1028d150 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d154u64) }
        0x1028d154 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d158u64) }
        0x1028d158 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028d15cu64).wrapping_add((imm * 4) as u64) } else { 0x1028d15cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d15cu64) }
        0x1028d15c => { vm.decode(0x30803019u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d160u64) }
        0x1028d160 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d164u64) }
        0x1028d164 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d168u64) }
        0x1028d168 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028d16cu64).wrapping_add((imm * 4) as u64) } else { 0x1028d16cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d16cu64) }
        0x1028d16c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028d170u64) } }
        0x1028d170 => { vm.decode(0x8120002eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64).wrapping_add(3) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 24 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shl(24 - sh) | (x & (0x00ff_ffffu32 >> sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028d174u64) }
        0x1028d174 => { vm.decode(0x2ff57e5fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 0 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shr(sh) | (x & 0xffff_ffffu32.wrapping_shl(32 - sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x1028d178u64) }
        0x1028d178 => { vm.decode(0x4003a649u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let x = vm.ld(s) as u32;
        let v = ((x >> 8) & 0x00ff_00ff) | ((x << 8) & 0xff00_ff00);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1028d17cu64) }
        0x1028d17c => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d180u64) }
        0x1028d180 => { vm.decode(0x1040022u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028d184u64) }
        0x1028d184 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d188u64) }
        0x1028d188 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d18cu64) }
        0x1028d18c => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d190u64) }
        0x1028d190 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028d194u64).wrapping_add((imm * 4) as u64) } else { 0x1028d194u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d194u64) }
        0x1028d194 => { vm.decode(0x30002019u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d198u64) }
        0x1028d198 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d19cu64) }
        0x1028d19c => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d1a0u64) }
        0x1028d1a0 => { vm.decode(0x40001119u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d1a4u64) }
        0x1028d1a4 => { vm.decode(0x2636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d1a8u64) }
        0x1028d1a8 => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d1acu64) }
        0x1028d1ac => { vm.decode(0xc8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028d1b0u64).wrapping_add((imm * 4) as u64) } else { 0x1028d1b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d1b0u64) }
        0x1028d1b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028d1b4u64) } }
        0x1028d1b4 => { vm.decode(0xaaecae51u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = -8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1b8u64) }
        0x1028d1b8 => { vm.decode(0xa2ec27d1u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -28i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1bcu64) }
        0x1028d1bc => { vm.decode(0x1c05cd35u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1c0u64) }
        0x1028d1c0 => { vm.decode(0x815cd35u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1c4u64) }
        0x1028d1c4 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d1c8u64) }
        0x1028d1c8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028d1ccu64) }
        0x1028d1cc => { vm.decode(0x80a403b5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d1d0u64) }
        0x1028d1d0 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d1d4u64) }
        0x1028d1d4 => { vm.decode(0xaae827d1u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = -60i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1d8u64) }
        0x1028d1d8 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d1dcu64) }
        0x1028d1dc => { vm.decode(0xc254d35u32); let d = 4i16; let s = 5i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1e0u64) }
        0x1028d1e0 => { vm.decode(0xcb4cd35u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1e4u64) }
        0x1028d1e4 => { vm.decode(0x1418835u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d1e8u64) }
        0x1028d1e8 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1028d1ecu64) }
        0x1028d1ec => { vm.decode(0x80e843b5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1028d1f0u64) }
        0x1028d1f0 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d1f4u64) }
        0x1028d1f4 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d1f8u64) }
        0x1028d1f8 => { vm.decode(0xa2e82751u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d1fcu64) }
        0x1028d1fc => { vm.decode(0x280a019u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d200u64) }
        0x1028d200 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d204u64) }
        0x1028d204 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d208u64) }
        0x1028d208 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d20cu64) }
        0x1028d20c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028d210u64) }
        0x1028d210 => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d214u64) }
        0x1028d214 => { vm.decode(0x27d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -25i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028d218u64).wrapping_add((imm * 4) as u64) } else { 0x1028d218u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d218u64) }
        0x1028d218 => { vm.decode(0x44004875u32); let d = 2i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d21cu64) }
        0x1028d21c => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d220u64) }
        0x1028d220 => { vm.decode(0x50708c19u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 19504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d224u64) }
        0x1028d224 => { vm.decode(0x2100236u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d228u64) }
        0x1028d228 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d22cu64) }
        0x1028d22c => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d230u64) }
        0x1028d230 => { vm.decode(0x6a8843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d234u64) }
        0x1028d234 => { vm.decode(0x428c63efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d238u64) }
        0x1028d238 => { vm.decode(0x488443afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d23cu64) }
        0x1028d23c => { vm.decode(0x7920486u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 99i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028d240u64) }
        0x1028d240 => { vm.decode(0x10848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 79i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028d244u64).wrapping_add((imm * 4) as u64) } else { 0x1028d244u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028d244u64) }
        0x1028d244 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028d248u64) } }
        0x1028d248 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d24cu64) }
        0x1028d24c => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d250u64) }
        0x1028d250 => { vm.decode(0x1119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d254u64) }
        0x1028d254 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d258u64) }
        0x1028d258 => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d25cu64) }
        0x1028d25c => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d260u64) }
        0x1028d260 => { vm.decode(0xd9ce251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d264u64) }
        0x1028d264 => { vm.decode(0x33d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028d268u64) }
        0x1028d268 => { vm.decode(0x5d98eb51u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d26cu64) }
        0x1028d26c => { vm.decode(0x559c7351u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d270u64) }
        0x1028d270 => { vm.decode(0x15986b51u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d274u64) }
        0x1028d274 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d278u64) }
        0x1028d278 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d27cu64) }
        0x1028d27c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028d280u64 + 4);
        vm.wr(37, 0x1028d280u64 + 4);
        Ok(0x1028d280u64) }
        0x1028d280 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d284u64) }
        0x1028d284 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d288u64) }
        0x1028d288 => { vm.decode(0x21019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028d28cu64) }
        0x1028d28c => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d290u64) }
        0x1028d290 => { vm.decode(0x9b111430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d294u64) }
        0x1028d294 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028d298u64) }
        0x1028d298 => { vm.decode(0x20c443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d29cu64) }
        0x1028d29c => { vm.decode(0x80402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028d2a0u64) }
        0x1028d2a0 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028d2a4u64) }
        0x1028d2a4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028d2a8u64) }
        0x1028d2a8 => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2acu64) }
        0x1028d2ac => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2b0u64) }
        0x1028d2b0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2b4u64) }
        0x1028d2b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028d2b8u64 + 4);
        vm.wr(37, 0x1028d2b8u64 + 4);
        Ok(0x1028d2b8u64) }
        0x1028d2b8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2bcu64) }
        0x1028d2bc => { vm.decode(0x53b835u32); let d = 1i16; let s = 20i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2c0u64) }
        0x1028d2c0 => { vm.decode(0x131217f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2c4u64) }
        0x1028d2c4 => { vm.decode(0x2f30cd35u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2c8u64) }
        0x1028d2c8 => { vm.decode(0x3db04d35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 6i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2ccu64) }
        0x1028d2cc => { vm.decode(0x40112835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2d0u64) }
        0x1028d2d0 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2d4u64) }
        0x1028d2d4 => { vm.decode(0x3b31cd35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2d8u64) }
        0x1028d2d8 => { vm.decode(0x404c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2dcu64) }
        0x1028d2dc => { vm.decode(0x4d18c211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2e0u64) }
        0x1028d2e0 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2e4u64) }
        0x1028d2e4 => { vm.decode(0xaf00236u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028d2e8u64) }
        0x1028d2e8 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2ecu64) }
        0x1028d2ec => { vm.decode(0x459c62d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2f0u64) }
        0x1028d2f0 => { vm.decode(0x3a908d35u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 13i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2f4u64) }
        0x1028d2f4 => { vm.decode(0x2c040d35u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028d2f8u64) }
        0x1028d2f8 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d2fcu64) }
        0x1028d2fc => { vm.decode(0x598ebd1u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d300u64) }
        0x1028d300 => { vm.decode(0x1347f5u32); let d = 5i16; let s = 4i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d304u64) }
        0x1028d304 => { vm.decode(0x50801875u32); let d = 6i16; let s = 4i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028d308u64) }
        0x1028d308 => { vm.decode(0x4d9ce3d1u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028d30cu64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
