#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102cf610 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf614u64) }
        0x102cf614 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf618u64) }
        0x102cf618 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf61cu64) }
        0x102cf61c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf620u64).wrapping_add((imm * 4) as u64) } else { 0x102cf620u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf620u64) }
        0x102cf620 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf624u64) }
        0x102cf624 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf628u64) }
        0x102cf628 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf62cu64) }
        0x102cf62c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf630u64) }
        0x102cf630 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf634u64) }
        0x102cf634 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf638u64).wrapping_add((imm * 4) as u64) } else { 0x102cf638u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf638u64) }
        0x102cf638 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf63cu64) } }
        0x102cf63c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf640u64) }
        0x102cf640 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf644u64) }
        0x102cf644 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf648u64).wrapping_add((imm * 4) as u64) } else { 0x102cf648u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf648u64) }
        0x102cf648 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf64cu64) }
        0x102cf64c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf650u64) }
        0x102cf650 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf654u64) }
        0x102cf654 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf658u64) }
        0x102cf658 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf65cu64) }
        0x102cf65c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf660u64).wrapping_add((imm * 4) as u64) } else { 0x102cf660u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf660u64) }
        0x102cf660 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf664u64) } }
        0x102cf664 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf668u64) }
        0x102cf668 => { vm.decode(0x30a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf66cu64) }
        0x102cf66c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf670u64) }
        0x102cf670 => { vm.decode(0x241120f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 4144i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102cf674u64) }
        0x102cf674 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf678u64) }
        0x102cf678 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf67cu64) }
        0x102cf67c => { vm.decode(0x9609236u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf680u64) }
        0x102cf680 => { vm.decode(0xa3481d75u32); let d = 11i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf684u64) }
        0x102cf684 => { vm.decode(0xca8014f5u32); let d = 10i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf688u64) }
        0x102cf688 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf68cu64) }
        0x102cf68c => { vm.decode(0x60b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf690u64) }
        0x102cf690 => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf694u64) }
        0x102cf694 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf698u64) }
        0x102cf698 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf69cu64) }
        0x102cf69c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6a0u64) }
        0x102cf6a0 => { vm.decode(0x83002406u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6a4u64) }
        0x102cf6a4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf6a8u64).wrapping_add((imm * 4) as u64) } else { 0x102cf6a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf6a8u64) }
        0x102cf6a8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf6acu64) }
        0x102cf6ac => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6b0u64) }
        0x102cf6b0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf6b4u64) }
        0x102cf6b4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf6b8u64) }
        0x102cf6b8 => { vm.decode(0x83002406u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6bcu64) }
        0x102cf6bc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf6c0u64).wrapping_add((imm * 4) as u64) } else { 0x102cf6c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf6c0u64) }
        0x102cf6c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf6c4u64) } }
        0x102cf6c4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6c8u64) }
        0x102cf6c8 => { vm.decode(0x83002406u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6ccu64) }
        0x102cf6cc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf6d0u64).wrapping_add((imm * 4) as u64) } else { 0x102cf6d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf6d0u64) }
        0x102cf6d0 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf6d4u64) }
        0x102cf6d4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6d8u64) }
        0x102cf6d8 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf6dcu64) }
        0x102cf6dc => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf6e0u64) }
        0x102cf6e0 => { vm.decode(0x83002406u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf6e4u64) }
        0x102cf6e4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf6e8u64).wrapping_add((imm * 4) as u64) } else { 0x102cf6e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf6e8u64) }
        0x102cf6e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf6ecu64) } }
        0x102cf6ec => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf6f0u64) }
        0x102cf6f0 => { vm.decode(0x110a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf6f4u64) }
        0x102cf6f4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf6f8u64) }
        0x102cf6f8 => { vm.decode(0x601060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 4144i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102cf6fcu64) }
        0x102cf6fc => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf700u64) }
        0x102cf700 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf704u64) }
        0x102cf704 => { vm.decode(0x82c014f5u32); let d = 9i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf708u64) }
        0x102cf708 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf70cu64) }
        0x102cf70c => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf710u64) }
        0x102cf710 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf714u64) }
        0x102cf714 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf718u64) }
        0x102cf718 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf71cu64) }
        0x102cf71c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf720u64).wrapping_add((imm * 4) as u64) } else { 0x102cf720u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf720u64) }
        0x102cf720 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf724u64) }
        0x102cf724 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf728u64) }
        0x102cf728 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf72cu64) }
        0x102cf72c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf730u64) }
        0x102cf730 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf734u64) }
        0x102cf734 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf738u64).wrapping_add((imm * 4) as u64) } else { 0x102cf738u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf738u64) }
        0x102cf738 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf73cu64) } }
        0x102cf73c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf740u64) }
        0x102cf740 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf744u64) }
        0x102cf744 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf748u64).wrapping_add((imm * 4) as u64) } else { 0x102cf748u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf748u64) }
        0x102cf748 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf74cu64) }
        0x102cf74c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf750u64) }
        0x102cf750 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf754u64) }
        0x102cf754 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf758u64) }
        0x102cf758 => { vm.decode(0x81002406u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf75cu64) }
        0x102cf75c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf760u64).wrapping_add((imm * 4) as u64) } else { 0x102cf760u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf760u64) }
        0x102cf760 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf764u64) } }
        0x102cf764 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf768u64) }
        0x102cf768 => { vm.decode(0x10aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf76cu64) }
        0x102cf76c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf770u64) }
        0x102cf770 => { vm.decode(0x203220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 4144i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102cf774u64) }
        0x102cf774 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf778u64) }
        0x102cf778 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf77cu64) }
        0x102cf77c => { vm.decode(0x42c014f5u32); let d = 9i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf780u64) }
        0x102cf780 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf784u64) }
        0x102cf784 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf788u64) }
        0x102cf788 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf78cu64) }
        0x102cf78c => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf790u64) }
        0x102cf790 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf794u64) }
        0x102cf794 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf798u64) }
        0x102cf798 => { vm.decode(0xa2002406u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf79cu64) }
        0x102cf79c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf7a0u64).wrapping_add((imm * 4) as u64) } else { 0x102cf7a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf7a0u64) }
        0x102cf7a0 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf7a4u64) }
        0x102cf7a4 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7a8u64) }
        0x102cf7a8 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf7acu64) }
        0x102cf7ac => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf7b0u64) }
        0x102cf7b0 => { vm.decode(0xa2002406u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7b4u64) }
        0x102cf7b4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf7b8u64).wrapping_add((imm * 4) as u64) } else { 0x102cf7b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf7b8u64) }
        0x102cf7b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf7bcu64) } }
        0x102cf7bc => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7c0u64) }
        0x102cf7c0 => { vm.decode(0xa2002406u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7c4u64) }
        0x102cf7c4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf7c8u64).wrapping_add((imm * 4) as u64) } else { 0x102cf7c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf7c8u64) }
        0x102cf7c8 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf7ccu64) }
        0x102cf7cc => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7d0u64) }
        0x102cf7d0 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf7d4u64) }
        0x102cf7d4 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf7d8u64) }
        0x102cf7d8 => { vm.decode(0xa2002406u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf7dcu64) }
        0x102cf7dc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf7e0u64).wrapping_add((imm * 4) as u64) } else { 0x102cf7e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf7e0u64) }
        0x102cf7e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf7e4u64) } }
        0x102cf7e4 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf7e8u64) }
        0x102cf7e8 => { vm.decode(0x14a3d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf7ecu64) }
        0x102cf7ec => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf7f0u64) }
        0x102cf7f0 => { vm.decode(0x203220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 4144i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102cf7f4u64) }
        0x102cf7f4 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf7f8u64) }
        0x102cf7f8 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf7fcu64) }
        0x102cf7fc => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf800u64) }
        0x102cf800 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf804u64) }
        0x102cf804 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf808u64) }
        0x102cf808 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf80cu64) }
        0x102cf80c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf810u64) }
        0x102cf810 => { vm.decode(0xa0002406u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf814u64) }
        0x102cf814 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf818u64).wrapping_add((imm * 4) as u64) } else { 0x102cf818u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf818u64) }
        0x102cf818 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf81cu64) }
        0x102cf81c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf820u64) }
        0x102cf820 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf824u64) }
        0x102cf824 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf828u64) }
        0x102cf828 => { vm.decode(0xa0002406u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf82cu64) }
        0x102cf82c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf830u64).wrapping_add((imm * 4) as u64) } else { 0x102cf830u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf830u64) }
        0x102cf830 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf834u64) } }
        0x102cf834 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf838u64) }
        0x102cf838 => { vm.decode(0xa0002406u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf83cu64) }
        0x102cf83c => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf840u64).wrapping_add((imm * 4) as u64) } else { 0x102cf840u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf840u64) }
        0x102cf840 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf844u64) }
        0x102cf844 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cf848u64) }
        0x102cf848 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cf84cu64) }
        0x102cf84c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf850u64) }
        0x102cf850 => { vm.decode(0xa0002406u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cf854u64) }
        0x102cf854 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf858u64).wrapping_add((imm * 4) as u64) } else { 0x102cf858u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf858u64) }
        0x102cf858 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf85cu64) } }
        0x102cf85c => { vm.decode(0x28a0051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8796i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cf860u64) }
        0x102cf860 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf864u64) } }
        0x102cf864 => { vm.decode(0x138df2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -5439488i64;
        vm.st(d, imm as u64);
        Ok(0x102cf868u64) }
        0x102cf868 => { vm.decode(0x20592f58u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 63912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf86cu64) }
        0x102cf86c => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf870u64) }
        0x102cf870 => { vm.decode(0x90000430u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf874u64) }
        0x102cf874 => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf878u64) }
        0x102cf878 => { vm.decode(0xd1cc011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cf87cu64) }
        0x102cf87c => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf880u64) }
        0x102cf880 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cf884u64).wrapping_add((imm * 4) as u64) } else { 0x102cf884u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf884u64) }
        0x102cf884 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf888u64) } }
        0x102cf888 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf88cu64) }
        0x102cf88c => { vm.decode(0x2200212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf890u64) }
        0x102cf890 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf894u64) }
        0x102cf894 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf898u64) }
        0x102cf898 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf89cu64) }
        0x102cf89c => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf8a0u64) }
        0x102cf8a0 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf8a4u64) }
        0x102cf8a4 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8a8u64) }
        0x102cf8a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cf8acu64 + 4);
        vm.wr(37, 0x102cf8acu64 + 4);
        Ok(0x102cf8acu64) }
        0x102cf8ac => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cf8b0u64) }
        0x102cf8b0 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8b4u64) }
        0x102cf8b4 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8b8u64) }
        0x102cf8b8 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8bcu64) }
        0x102cf8bc => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8c0u64) }
        0x102cf8c0 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8c4u64) }
        0x102cf8c4 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8c8u64) }
        0x102cf8c8 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8ccu64) }
        0x102cf8cc => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8d0u64) }
        0x102cf8d0 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8d4u64) }
        0x102cf8d4 => { vm.decode(0x8098b27u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cf8d8u64).wrapping_add((imm * 4) as u64) } else { 0x102cf8d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cf8d8u64) }
        0x102cf8d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cf8dcu64) } }
        0x102cf8dc => { vm.decode(0x138df2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -5439488i64;
        vm.st(d, imm as u64);
        Ok(0x102cf8e0u64) }
        0x102cf8e0 => { vm.decode(0x20592f58u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 63912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cf8e4u64) }
        0x102cf8e4 => { vm.decode(0x880184f5u32); let d = 2i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf8e8u64) }
        0x102cf8e8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8ecu64) }
        0x102cf8ec => { vm.decode(0x84891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf8f0u64) }
        0x102cf8f0 => { vm.decode(0xaa00a4f5u32); let d = 2i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf8f4u64) }
        0x102cf8f4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf8f8u64) }
        0x102cf8f8 => { vm.decode(0x85091b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf8fcu64) }
        0x102cf8fc => { vm.decode(0xa800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf900u64) }
        0x102cf900 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf904u64) }
        0x102cf904 => { vm.decode(0x85891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf908u64) }
        0x102cf908 => { vm.decode(0x8800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf90cu64) }
        0x102cf90c => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf910u64) }
        0x102cf910 => { vm.decode(0x820284f5u32); let d = 4i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf914u64) }
        0x102cf914 => { vm.decode(0xa04284f5u32); let d = 5i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf918u64) }
        0x102cf918 => { vm.decode(0x804084f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf91cu64) }
        0x102cf91c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf920u64) }
        0x102cf920 => { vm.decode(0x18609030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf924u64) }
        0x102cf924 => { vm.decode(0x2080012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf928u64) }
        0x102cf928 => { vm.decode(0x18689030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf92cu64) }
        0x102cf92c => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf930u64) }
        0x102cf930 => { vm.decode(0x18709030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf934u64) }
        0x102cf934 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf938u64) }
        0x102cf938 => { vm.decode(0x18789030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf93cu64) }
        0x102cf93c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf940u64) }
        0x102cf940 => { vm.decode(0x186090b0u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf944u64) }
        0x102cf944 => { vm.decode(0x4a8043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf948u64) }
        0x102cf948 => { vm.decode(0x6a8063efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf94cu64) }
        0x102cf94c => { vm.decode(0x488c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf950u64) }
        0x102cf950 => { vm.decode(0x688c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf954u64) }
        0x102cf954 => { vm.decode(0x4a8643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf958u64) }
        0x102cf958 => { vm.decode(0x6a8663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf95cu64) }
        0x102cf95c => { vm.decode(0x408ac3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf960u64) }
        0x102cf960 => { vm.decode(0x608ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf964u64) }
        0x102cf964 => { vm.decode(0x428acbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf968u64) }
        0x102cf968 => { vm.decode(0x628aebafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf96cu64) }
        0x102cf96c => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102cf970u64) }
        0x102cf970 => { vm.decode(0x77198059u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf974u64) }
        0x102cf974 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf978u64) }
        0x102cf978 => { vm.decode(0x830204f5u32); let d = 4i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cf97cu64) }
        0x102cf97c => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf980u64) }
        0x102cf980 => { vm.decode(0x8b209430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf984u64) }
        0x102cf984 => { vm.decode(0x1b1894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf988u64) }
        0x102cf988 => { vm.decode(0x665f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cf98cu64) }
        0x102cf98c => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf990u64) }
        0x102cf990 => { vm.decode(0x99181470u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf994u64) }
        0x102cf994 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf998u64) }
        0x102cf998 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf99cu64) }
        0x102cf99c => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9a0u64) }
        0x102cf9a0 => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9a4u64) }
        0x102cf9a4 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9a8u64) }
        0x102cf9a8 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9acu64) }
        0x102cf9ac => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9b0u64) }
        0x102cf9b0 => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9b4u64) }
        0x102cf9b4 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9b8u64) }
        0x102cf9b8 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9bcu64) }
        0x102cf9bc => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9c0u64) }
        0x102cf9c0 => { vm.decode(0x99105570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9c4u64) }
        0x102cf9c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cf9c8u64 + 4);
        vm.wr(37, 0x102cf9c8u64 + 4);
        Ok(0x102cf9c8u64) }
        0x102cf9c8 => { vm.decode(0x9085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cf9ccu64) }
        0x102cf9cc => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9d0u64) }
        0x102cf9d0 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9d4u64) }
        0x102cf9d4 => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9d8u64) }
        0x102cf9d8 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9dcu64) }
        0x102cf9dc => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9e0u64) }
        0x102cf9e0 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9e4u64) }
        0x102cf9e4 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9e8u64) }
        0x102cf9e8 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9ecu64) }
        0x102cf9ec => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9f0u64) }
        0x102cf9f0 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9f4u64) }
        0x102cf9f4 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9f8u64) }
        0x102cf9f8 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cf9fcu64) }
        0x102cf9fc => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa00u64) }
        0x102cfa00 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa04u64) }
        0x102cfa04 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa08u64) }
        0x102cfa08 => { vm.decode(0xd98c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cfa0cu64) }
        0x102cfa0c => { vm.decode(0x7143e27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -2518i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cfa10u64).wrapping_add((imm * 4) as u64) } else { 0x102cfa10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cfa10u64) }
        0x102cfa10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfa14u64) } }
        0x102cfa14 => { vm.decode(0x4b2514f5u32); let d = 18i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfa18u64) }
        0x102cfa18 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfa1cu64) }
        0x102cfa1c => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa20u64) }
        0x102cfa20 => { vm.decode(0x810724f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfa24u64) }
        0x102cfa24 => { vm.decode(0x2020212fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa28u64) }
        0x102cfa28 => { vm.decode(0x501019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfa2cu64) }
        0x102cfa2c => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa30u64) }
        0x102cfa30 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa34u64) }
        0x102cfa34 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa38u64) }
        0x102cfa38 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa3cu64) }
        0x102cfa3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfa40u64 + 4);
        vm.wr(37, 0x102cfa40u64 + 4);
        Ok(0x102cfa40u64) }
        0x102cfa40 => { vm.decode(0x661b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfa44u64) }
        0x102cfa44 => { vm.decode(0x6a8c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa48u64) }
        0x102cfa48 => { vm.decode(0x81400530u32); let d = 18i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa4cu64) }
        0x102cfa4c => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfa50u64) }
        0x102cfa50 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa54u64) }
        0x102cfa54 => { vm.decode(0x2204212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa58u64) }
        0x102cfa58 => { vm.decode(0xb109430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa5cu64) }
        0x102cfa5c => { vm.decode(0x60841019u32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfa60u64) }
        0x102cfa60 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa64u64) }
        0x102cfa64 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa68u64) }
        0x102cfa68 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa6cu64) }
        0x102cfa6c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfa70u64 + 4);
        vm.wr(37, 0x102cfa70u64 + 4);
        Ok(0x102cfa70u64) }
        0x102cfa70 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfa74u64) }
        0x102cfa74 => { vm.decode(0x138df2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -5439488i64;
        vm.st(d, imm as u64);
        Ok(0x102cfa78u64) }
        0x102cfa78 => { vm.decode(0x20592f58u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 63912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cfa7cu64) }
        0x102cfa7c => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfa80u64) }
        0x102cfa80 => { vm.decode(0x90000530u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa84u64) }
        0x102cfa84 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa88u64) }
        0x102cfa88 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa8cu64) }
        0x102cfa8c => { vm.decode(0x8b3814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa90u64) }
        0x102cfa90 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfa94u64) }
        0x102cfa94 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa98u64) }
        0x102cfa98 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfa9cu64) }
        0x102cfa9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfaa0u64 + 4);
        vm.wr(37, 0x102cfaa0u64 + 4);
        Ok(0x102cfaa0u64) }
        0x102cfaa0 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfaa4u64) }
        0x102cfaa4 => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfaa8u64) }
        0x102cfaa8 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfaacu64) }
        0x102cfaac => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfab0u64) }
        0x102cfab0 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfab4u64) }
        0x102cfab4 => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfab8u64) }
        0x102cfab8 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfabcu64) }
        0x102cfabc => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfac0u64) }
        0x102cfac0 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfac4u64) }
        0x102cfac4 => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfac8u64) }
        0x102cfac8 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfaccu64) }
        0x102cfacc => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfad0u64) }
        0x102cfad0 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfad4u64) }
        0x102cfad4 => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfad8u64) }
        0x102cfad8 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfadcu64) }
        0x102cfadc => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfae0u64) }
        0x102cfae0 => { vm.decode(0x8820001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 200i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cfae4u64) }
        0x102cfae4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfae8u64) } }
        0x102cfae8 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102cfaecu64) }
        0x102cfaf0 => { vm.decode(0x7b3bcf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfaf4u64) }
        0x102cfaf4 => { vm.decode(0x9b185d70u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfaf8u64) }
        0x102cfaf8 => { vm.decode(0x8b105d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfafcu64) }
        0x102cfafc => { vm.decode(0x9b085d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb00u64) }
        0x102cfb00 => { vm.decode(0x8b005d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb04u64) }
        0x102cfb04 => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb08u64) }
        0x102cfb08 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb0cu64) }
        0x102cfb0c => { vm.decode(0x9b2895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb10u64) }
        0x102cfb10 => { vm.decode(0x8b2095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb14u64) }
        0x102cfb14 => { vm.decode(0x9b389430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb18u64) }
        0x102cfb18 => { vm.decode(0x8b309430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb1cu64) }
        0x102cfb1c => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102cfb20u64) }
        0x102cfb20 => { vm.decode(0x254829d8u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cfb24u64) }
        0x102cfb24 => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb28u64) }
        0x102cfb28 => { vm.decode(0x830404f5u32); let d = 16i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfb2cu64) }
        0x102cfb2c => { vm.decode(0x286096fu32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb30u64) }
        0x102cfb30 => { vm.decode(0x208c296fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb34u64) }
        0x102cfb34 => { vm.decode(0x8c096fu32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb38u64) }
        0x102cfb38 => { vm.decode(0x2280296fu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb3cu64) }
        0x102cfb3c => { vm.decode(0x280096fu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb40u64) }
        0x102cfb40 => { vm.decode(0x2082296fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb44u64) }
        0x102cfb44 => { vm.decode(0x208a096fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb48u64) }
        0x102cfb48 => { vm.decode(0x4284292fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb4cu64) }
        0x102cfb4c => { vm.decode(0x6286012fu32); let d = 21i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb50u64) }
        0x102cfb50 => { vm.decode(0x8a082fu32); let d = 14i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb54u64) }
        0x102cfb54 => { vm.decode(0x2088212fu32); let d = 3i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb58u64) }
        0x102cfb58 => { vm.decode(0x6084092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb5cu64) }
        0x102cfb5c => { vm.decode(0x608e292fu32); let d = 31i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb60u64) }
        0x102cfb60 => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb64u64) }
        0x102cfb64 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb68u64) }
        0x102cfb68 => { vm.decode(0xa8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb6cu64) }
        0x102cfb6c => { vm.decode(0xa2202fu32); let d = 4i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb70u64) }
        0x102cfb70 => { vm.decode(0x42a2002fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb74u64) }
        0x102cfb74 => { vm.decode(0x62a8202fu32); let d = 19i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb78u64) }
        0x102cfb78 => { vm.decode(0x60ae002fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb7cu64) }
        0x102cfb7c => { vm.decode(0x40ac202fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb80u64) }
        0x102cfb80 => { vm.decode(0x893015b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb84u64) }
        0x102cfb84 => { vm.decode(0x42ac002fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb88u64) }
        0x102cfb88 => { vm.decode(0x892815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfb8cu64) }
        0x102cfb8c => { vm.decode(0x2ae202fu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb90u64) }
        0x102cfb90 => { vm.decode(0x4288012fu32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb94u64) }
        0x102cfb94 => { vm.decode(0x8768a9d8u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 11736i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cfb98u64) }
        0x102cfb98 => { vm.decode(0x428a212fu32); let d = 22i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfb9cu64) }
        0x102cfb9c => { vm.decode(0x30000037u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102cfba0u64) }
        0x102cfba0 => { vm.decode(0x810224f5u32); let d = 4i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfba4u64) }
        0x102cfba4 => { vm.decode(0xab4004f5u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfba8u64) }
        0x102cfba8 => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cfbacu64) }
        0x102cfbac => { vm.decode(0x8f5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        
        Ok(0x102cfbb0u64) }
        0x102cfbb0 => { vm.decode(0x700400a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2496i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cfbb4u64).wrapping_add((imm * 4) as u64) } else { 0x102cfbb4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cfbb4u64) }
        0x102cfbb4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfbb8u64) } }
        0x102cfbb8 => { vm.decode(0x814184f5u32); let d = 1i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfbbcu64) }
        0x102cfbbc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfbc0u64) }
        0x102cfbc0 => { vm.decode(0x2610471u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2452i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cfbc4u64).wrapping_add((imm * 4) as u64) } else { 0x102cfbc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cfbc4u64) }
        0x102cfbc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfbc8u64) } }
        0x102cfbc8 => { vm.decode(0x48005875u32); let d = 18i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfbccu64) }
        0x102cfbcc => { vm.decode(0x4c30d875u32); let d = 19i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfbd0u64) }
        0x102cfbd0 => { vm.decode(0xcb00875u32); let d = 20i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfbd4u64) }
        0x102cfbd4 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbd8u64) }
        0x102cfbd8 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbdcu64) }
        0x102cfbdc => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbe0u64) }
        0x102cfbe0 => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbe4u64) }
        0x102cfbe4 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbe8u64) }
        0x102cfbe8 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbecu64) }
        0x102cfbec => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbf0u64) }
        0x102cfbf0 => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbf4u64) }
        0x102cfbf4 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfbf8u64) }
        0x102cfbf8 => { vm.decode(0x4b2734f5u32); let d = 22i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfbfcu64) }
        0x102cfbfc => { vm.decode(0xab4724f5u32); let d = 23i16; let s = 17i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc00u64) }
        0x102cfc00 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102cfc04u64) }
        0x102cfc04 => { vm.decode(0x54829d8u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cfc08u64) }
        0x102cfc08 => { vm.decode(0x214124f5u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc0cu64) }
        0x102cfc0c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc10u64) }
        0x102cfc10 => { vm.decode(0xa00dcbb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -244i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cfc14u64) }
        0x102cfc14 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cfc18u64) }
        0x102cfc18 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc1cu64) }
        0x102cfc1c => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc20u64) }
        0x102cfc20 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfc24u64) }
        0x102cfc24 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc28u64) }
        0x102cfc28 => { vm.decode(0x28016fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc2cu64) }
        0x102cfc2c => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc30u64) }
        0x102cfc30 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc34u64) }
        0x102cfc34 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfc38u64) }
        0x102cfc38 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfc3cu64) }
        0x102cfc3c => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc40u64) }
        0x102cfc40 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfc44u64) }
        0x102cfc44 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfc48u64 + 4);
        vm.wr(37, 0x102cfc48u64 + 4);
        Ok(0x102cfc48u64) }
        0x102cfc48 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc4cu64) }
        0x102cfc4c => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc50u64) }
        0x102cfc50 => { vm.decode(0x14184f5u32); let d = 1i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc54u64) }
        0x102cfc54 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cfc58u64) }
        0x102cfc58 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cfc5cu64) }
        0x102cfc5c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc60u64) }
        0x102cfc60 => { vm.decode(0x2210571u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2388i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cfc64u64).wrapping_add((imm * 4) as u64) } else { 0x102cfc64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cfc64u64) }
        0x102cfc64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfc68u64) } }
        0x102cfc68 => { vm.decode(0x89085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc6cu64) }
        0x102cfc6c => { vm.decode(0x99105530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc70u64) }
        0x102cfc70 => { vm.decode(0x834104f5u32); let d = 1i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc74u64) }
        0x102cfc74 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc78u64) }
        0x102cfc78 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc7cu64) }
        0x102cfc7c => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc80u64) }
        0x102cfc80 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc84u64) }
        0x102cfc84 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc88u64) }
        0x102cfc88 => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc8cu64) }
        0x102cfc8c => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfc90u64) }
        0x102cfc90 => { vm.decode(0xa98604f5u32); let d = 30i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc94u64) }
        0x102cfc94 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfc98u64) }
        0x102cfc98 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfc9cu64) }
        0x102cfc9c => { vm.decode(0x814124f5u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfca0u64) }
        0x102cfca0 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfca4u64) }
        0x102cfca4 => { vm.decode(0x4286632fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfca8u64) }
        0x102cfca8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfcacu64) }
        0x102cfcac => { vm.decode(0x10009419u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 2048i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfcb0u64) }
        0x102cfcb0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfcb4u64) }
        0x102cfcb4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfcb8u64) }
        0x102cfcb8 => { vm.decode(0x761b8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfcbcu64) }
        0x102cfcbc => { vm.decode(0x608e432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfcc0u64) }
        0x102cfcc0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfcc4u64) }
        0x102cfcc4 => { vm.decode(0x408c632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfcc8u64) }
        0x102cfcc8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfcccu64) }
        0x102cfccc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfcd0u64 + 4);
        vm.wr(37, 0x102cfcd0u64 + 4);
        Ok(0x102cfcd0u64) }
        0x102cfcd0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfcd4u64) }
        0x102cfcd4 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfcd8u64) }
        0x102cfcd8 => { vm.decode(0x9b281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfcdcu64) }
        0x102cfcdc => { vm.decode(0x6a8443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfce0u64) }
        0x102cfce0 => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfce4u64) }
        0x102cfce4 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfce8u64) }
        0x102cfce8 => { vm.decode(0x420d1022u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cfcecu64) }
        0x102cfcec => { vm.decode(0x66598019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfcf0u64) }
        0x102cfcf0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfcf4u64) }
        0x102cfcf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfcf8u64 + 4);
        vm.wr(37, 0x102cfcf8u64 + 4);
        Ok(0x102cfcf8u64) }
        0x102cfcf8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfcfcu64) }
        0x102cfcfc => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd00u64) }
        0x102cfd00 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd04u64) }
        0x102cfd04 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd08u64) }
        0x102cfd08 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd0cu64) }
        0x102cfd0c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd10u64) }
        0x102cfd10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfd14u64 + 4);
        vm.wr(37, 0x102cfd14u64 + 4);
        Ok(0x102cfd14u64) }
        0x102cfd14 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd18u64) }
        0x102cfd18 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd1cu64) }
        0x102cfd1c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfd20u64) }
        0x102cfd20 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd24u64) }
        0x102cfd24 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd28u64) }
        0x102cfd28 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd2cu64) }
        0x102cfd2c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd30u64) }
        0x102cfd30 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd34u64) }
        0x102cfd34 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd38u64) }
        0x102cfd38 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd3cu64) }
        0x102cfd3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfd40u64 + 4);
        vm.wr(37, 0x102cfd40u64 + 4);
        Ok(0x102cfd40u64) }
        0x102cfd40 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd44u64) }
        0x102cfd44 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd48u64) }
        0x102cfd48 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfd4cu64) }
        0x102cfd4c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd50u64) }
        0x102cfd50 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd54u64) }
        0x102cfd54 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd58u64) }
        0x102cfd58 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd5cu64) }
        0x102cfd5c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd60u64) }
        0x102cfd60 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd64u64) }
        0x102cfd64 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd68u64) }
        0x102cfd68 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfd6cu64 + 4);
        vm.wr(37, 0x102cfd6cu64 + 4);
        Ok(0x102cfd6cu64) }
        0x102cfd6c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd70u64) }
        0x102cfd70 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd74u64) }
        0x102cfd74 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfd78u64) }
        0x102cfd78 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd7cu64) }
        0x102cfd7c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd80u64) }
        0x102cfd80 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfd84u64) }
        0x102cfd84 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd88u64) }
        0x102cfd88 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfd8cu64) }
        0x102cfd8c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd90u64) }
        0x102cfd90 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd94u64) }
        0x102cfd94 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfd98u64 + 4);
        vm.wr(37, 0x102cfd98u64 + 4);
        Ok(0x102cfd98u64) }
        0x102cfd98 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfd9cu64) }
        0x102cfd9c => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfda0u64) }
        0x102cfda0 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfda4u64) }
        0x102cfda4 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfda8u64) }
        0x102cfda8 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfdacu64) }
        0x102cfdac => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfdb0u64) }
        0x102cfdb0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfdb4u64) }
        0x102cfdb4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfdb8u64) }
        0x102cfdb8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfdbcu64) }
        0x102cfdbc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfdc0u64) }
        0x102cfdc0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfdc4u64 + 4);
        vm.wr(37, 0x102cfdc4u64 + 4);
        Ok(0x102cfdc4u64) }
        0x102cfdc4 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfdc8u64) }
        0x102cfdc8 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfdccu64) }
        0x102cfdcc => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfdd0u64) }
        0x102cfdd0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfdd4u64) }
        0x102cfdd4 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfdd8u64) }
        0x102cfdd8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfddcu64) }
        0x102cfddc => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfde0u64) }
        0x102cfde0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfde4u64) }
        0x102cfde4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfde8u64) }
        0x102cfde8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfdecu64) }
        0x102cfdec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfdf0u64 + 4);
        vm.wr(37, 0x102cfdf0u64 + 4);
        Ok(0x102cfdf0u64) }
        0x102cfdf0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfdf4u64) }
        0x102cfdf4 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfdf8u64) }
        0x102cfdf8 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfdfcu64) }
        0x102cfdfc => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe00u64) }
        0x102cfe00 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe04u64) }
        0x102cfe04 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe08u64) }
        0x102cfe08 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe0cu64) }
        0x102cfe0c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe10u64) }
        0x102cfe10 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe14u64) }
        0x102cfe14 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe18u64) }
        0x102cfe18 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfe1cu64 + 4);
        vm.wr(37, 0x102cfe1cu64 + 4);
        Ok(0x102cfe1cu64) }
        0x102cfe1c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe20u64) }
        0x102cfe20 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe24u64) }
        0x102cfe24 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cfe28u64) }
        0x102cfe28 => { vm.decode(0x76499019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfe2cu64) }
        0x102cfe2c => { vm.decode(0x721b1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfe30u64) }
        0x102cfe30 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cfe34u64) }
        0x102cfe34 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfe38u64) }
        0x102cfe38 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe3cu64) }
        0x102cfe3c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe40u64) }
        0x102cfe40 => { vm.decode(0x890c875u32); let d = 21i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe44u64) }
        0x102cfe44 => { vm.decode(0x2084402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe48u64) }
        0x102cfe48 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe4cu64) }
        0x102cfe4c => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cfe50u64) }
        0x102cfe50 => { vm.decode(0x8b185530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe54u64) }
        0x102cfe54 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe58u64) }
        0x102cfe58 => { vm.decode(0x4086636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe5cu64) }
        0x102cfe5c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe60u64) }
        0x102cfe60 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe64u64) }
        0x102cfe64 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe68u64) }
        0x102cfe68 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfe6cu64 + 4);
        vm.wr(37, 0x102cfe6cu64 + 4);
        Ok(0x102cfe6cu64) }
        0x102cfe6c => { vm.decode(0x990055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe70u64) }
        0x102cfe70 => { vm.decode(0x704b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfe74u64) }
        0x102cfe74 => { vm.decode(0x2084412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfe78u64) }
        0x102cfe78 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfe7cu64) }
        0x102cfe7c => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe80u64) }
        0x102cfe80 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cfe84u64) }
        0x102cfe84 => { vm.decode(0x8b181430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe88u64) }
        0x102cfe88 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfe8cu64) }
        0x102cfe8c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe90u64) }
        0x102cfe90 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe94u64) }
        0x102cfe94 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfe98u64 + 4);
        vm.wr(37, 0x102cfe98u64 + 4);
        Ok(0x102cfe98u64) }
        0x102cfe98 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfe9cu64) }
        0x102cfe9c => { vm.decode(0x724b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfea0u64) }
        0x102cfea0 => { vm.decode(0x8059u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfea4u64) }
        0x102cfea4 => { vm.decode(0x628e632fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfea8u64) }
        0x102cfea8 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfeacu64) }
        0x102cfeac => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfeb0u64) }
        0x102cfeb0 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cfeb4u64) }
        0x102cfeb4 => { vm.decode(0x9b185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfeb8u64) }
        0x102cfeb8 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfebcu64) }
        0x102cfebc => { vm.decode(0x2086432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfec0u64) }
        0x102cfec0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfec4u64) }
        0x102cfec4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cfec8u64 + 4);
        vm.wr(37, 0x102cfec8u64 + 4);
        Ok(0x102cfec8u64) }
        0x102cfec8 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfeccu64) }
        0x102cfecc => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfed0u64) }
        0x102cfed0 => { vm.decode(0x192854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfed4u64) }
        0x102cfed4 => { vm.decode(0x720b0119u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfed8u64) }
        0x102cfed8 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfedcu64) }
        0x102cfedc => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cfee0u64) }
        0x102cfee0 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfee4u64) }
        0x102cfee4 => { vm.decode(0x28c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfee8u64) }
        0x102cfee8 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfeecu64) }
        0x102cfeec => { vm.decode(0x8938dcb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfef0u64) }
        0x102cfef0 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cfef4u64) }
        0x102cfef4 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cfef8u64) }
        0x102cfef8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cfefcu64) }
        0x102cfefc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cff00u64 + 4);
        vm.wr(37, 0x102cff00u64 + 4);
        Ok(0x102cff00u64) }
        0x102cff00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cff04u64) } }
        0x102cff04 => { vm.decode(0x725b9019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cff08u64) }
        0x102cff08 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cff0cu64) }
        0x102cff0c => { vm.decode(0x2000216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cff10u64) }
        0x102cff10 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cff14u64) }
        0x102cff14 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cff18u64) }
        0x102cff18 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cff1cu64) }
        0x102cff1c => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cff20u64) }
        0x102cff20 => { vm.decode(0x9938ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cff24u64) }
        0x102cff24 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cff28u64) }
        0x102cff28 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cff2cu64) }
        0x102cff2c => { vm.decode(0x2086636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cff30u64) }
        0x102cff30 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cff34u64) }
        0x102cff34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cff38u64 + 4);
        vm.wr(37, 0x102cff38u64 + 4);
        Ok(0x102cff38u64) }
        0x102cff38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cff3cu64) } }
        0x102cff3c => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cff40u64) }
        0x102cff40 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cff44u64).wrapping_add((imm * 4) as u64) } else { 0x102cff44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cff44u64) }
        0x102cff44 => { vm.decode(0xa00c4bb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -256i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cff48u64) }
        0x102cff50 => { vm.decode(0x110019b5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102cff54u64) }
        0x102cff54 => { vm.decode(0x368bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cff58u64) }
        0x102cff58 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cff5cu64) }
        0x102cff5c => { vm.decode(0x24ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102cff60u64) }
        0x102cff60 => { vm.decode(0x560207f5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cff64u64) }
        0x102cff64 => { vm.decode(0x4089875u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cff68u64) }
        0x102cff68 => { vm.decode(0x40c30bb5u32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cff6cu64) }
        0x102cff6c => { vm.decode(0xe84124f5u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cff70u64) }
        0x102cff70 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cff74u64) }
        0x102cff74 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cff78u64) }
        0x102cff78 => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cff7cu64) }
        0x102cff7c => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cff80u64).wrapping_add((imm * 4) as u64) } else { 0x102cff80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cff80u64) }
        0x102cff80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cff84u64) } }
        0x102cff84 => { vm.decode(0x7818e7b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102cff88u64) }
        0x102cff88 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102cff8cu64) }
        0x102cff8c => { vm.decode(0x44290fb5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cff90u64) }
        0x102cff90 => { vm.decode(0x40217f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cff94u64) }
        0x102cff94 => { vm.decode(0xa8088b35u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cff98u64) }
        0x102cff98 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cff9cu64) }
        0x102cff9c => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cffa0u64) }
        0x102cffa0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffa4u64) }
        0x102cffa4 => { vm.decode(0x21100446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cffa8u64) }
        0x102cffa8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cffacu64).wrapping_add((imm * 4) as u64) } else { 0x102cffacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cffacu64) }
        0x102cffac => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffb0u64) }
        0x102cffb0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffb4u64) }
        0x102cffb4 => { vm.decode(0xd2cac12du32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cffb8u64) }
        0x102cffb8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cffbcu64) }
        0x102cffbc => { vm.decode(0xa0004118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cffc0u64) }
        0x102cffc0 => { vm.decode(0x21100446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cffc4u64) }
        0x102cffc4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cffc8u64).wrapping_add((imm * 4) as u64) } else { 0x102cffc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cffc8u64) }
        0x102cffc8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cffccu64) } }
        0x102cffcc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffd0u64) }
        0x102cffd0 => { vm.decode(0x3100446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cffd4u64) }
        0x102cffd4 => { vm.decode(0x739c3ea7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -34i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cffd8u64).wrapping_add((imm * 4) as u64) } else { 0x102cffd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cffd8u64) }
        0x102cffd8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffdcu64) }
        0x102cffdc => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cffe0u64) }
        0x102cffe0 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cffe4u64) }
        0x102cffe4 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cffe8u64) }
        0x102cffe8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cffecu64) }
        0x102cffec => { vm.decode(0x3100446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cfff0u64) }
        0x102cfff0 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cfff4u64).wrapping_add((imm * 4) as u64) } else { 0x102cfff4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cfff4u64) }
        0x102cfff4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cfff8u64) } }
        0x102cfff8 => { vm.decode(0x520001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cfffcu64) }
        0x102cfffc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0000u64) } }
        0x102d0000 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0004u64) }
        0x102d0004 => { vm.decode(0x408e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0008u64) }
        0x102d0008 => { vm.decode(0x628a6b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d000cu64) }
        0x102d000c => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0010u64) }
        0x102d0010 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0014u64).wrapping_add((imm * 4) as u64) } else { 0x102d0014u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0014u64) }
        0x102d0014 => { vm.decode(0xb40c6ff6u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0018u64) }
        0x102d0020 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0024u64) }
        0x102d0024 => { vm.decode(0x768bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0028u64) }
        0x102d0028 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d002cu64) }
        0x102d002c => { vm.decode(0x24ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d0030u64) }
        0x102d0030 => { vm.decode(0x140147f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0034u64) }
        0x102d0034 => { vm.decode(0x8c875u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0038u64) }
        0x102d0038 => { vm.decode(0x624b0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d003cu64) }
        0x102d003c => { vm.decode(0x802224f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0040u64) }
        0x102d0040 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d0044u64) }
        0x102d0044 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0048u64) }
        0x102d0048 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d004cu64) }
        0x102d004c => { vm.decode(0x10048027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 65i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0050u64).wrapping_add((imm * 4) as u64) } else { 0x102d0050u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0050u64) }
        0x102d0050 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0054u64) } }
        0x102d0054 => { vm.decode(0x3018e7b5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d0058u64) }
        0x102d0058 => { vm.decode(0x50920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d005cu64) }
        0x102d005c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0060u64) }
        0x102d0060 => { vm.decode(0x60217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0064u64) }
        0x102d0064 => { vm.decode(0x200c0b35u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0068u64) }
        0x102d0068 => { vm.decode(0x6124f5u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d006cu64) }
        0x102d006c => { vm.decode(0xa00023eu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0070u64) }
        0x102d0070 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0074u64) }
        0x102d0074 => { vm.decode(0x23100446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0078u64) }
        0x102d0078 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d007cu64).wrapping_add((imm * 4) as u64) } else { 0x102d007cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d007cu64) }
        0x102d007c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0080u64) }
        0x102d0080 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0084u64) }
        0x102d0084 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0088u64) }
        0x102d0088 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d008cu64) }
        0x102d008c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0090u64) }
        0x102d0090 => { vm.decode(0x23100446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0094u64) }
        0x102d0094 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0098u64).wrapping_add((imm * 4) as u64) } else { 0x102d0098u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0098u64) }
        0x102d0098 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d009cu64) } }
        0x102d009c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d00a0u64) }
        0x102d00a0 => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d00a4u64) }
        0x102d00a4 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d00a8u64).wrapping_add((imm * 4) as u64) } else { 0x102d00a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d00a8u64) }
        0x102d00a8 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d00acu64) }
        0x102d00ac => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d00b0u64) }
        0x102d00b0 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d00b4u64) }
        0x102d00b4 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d00b8u64) }
        0x102d00b8 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d00bcu64) }
        0x102d00bc => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d00c0u64) }
        0x102d00c0 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d00c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d00c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d00c4u64) }
        0x102d00c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d00c8u64) } }
        0x102d00c8 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102d00ccu64) }
        0x102d00cc => { vm.decode(0x778bb5u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d00d0u64) }
        0x102d00d0 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d00d4u64) }
        0x102d00d4 => { vm.decode(0x2025ad35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d00d8u64) }
        0x102d00d8 => { vm.decode(0x121347f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d00dcu64) }
        0x102d00dc => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d00e0u64) }
        0x102d00e0 => { vm.decode(0x70bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d00e4u64) }
        0x102d00e4 => { vm.decode(0x8a2324f5u32); let d = 6i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d00e8u64) }
        0x102d00e8 => { vm.decode(0x40220033u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d00ecu64) }
        0x102d00ec => { vm.decode(0x226124f5u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d00f0u64) }
        0x102d00f0 => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d00f4u64) }
        0x102d00f4 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d00f8u64) }
        0x102d00f8 => { vm.decode(0x23100446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d00fcu64) }
        0x102d00fc => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0100u64).wrapping_add((imm * 4) as u64) } else { 0x102d0100u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0100u64) }
        0x102d0100 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0104u64) }
        0x102d0104 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0108u64) }
        0x102d0108 => { vm.decode(0xd2ca416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d010cu64) }
        0x102d010c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0110u64) }
        0x102d0110 => { vm.decode(0x80204118u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0114u64) }
        0x102d0114 => { vm.decode(0x23100446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0118u64) }
        0x102d0118 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d011cu64).wrapping_add((imm * 4) as u64) } else { 0x102d011cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d011cu64) }
        0x102d011c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0120u64) } }
        0x102d0120 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0124u64) }
        0x102d0124 => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0128u64) }
        0x102d0128 => { vm.decode(0x679cb6a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -67i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d012cu64).wrapping_add((imm * 4) as u64) } else { 0x102d012cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d012cu64) }
        0x102d012c => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0130u64) }
        0x102d0130 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0134u64) }
        0x102d0134 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0138u64) }
        0x102d0138 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d013cu64) }
        0x102d013c => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0140u64) }
        0x102d0140 => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0144u64) }
        0x102d0144 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0148u64).wrapping_add((imm * 4) as u64) } else { 0x102d0148u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0148u64) }
        0x102d0148 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d014cu64) } }
        0x102d014c => { vm.decode(0x500141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1328i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d0150u64) }
        0x102d0150 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0154u64) } }
        0x102d0154 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0158u64) }
        0x102d0158 => { vm.decode(0x628a432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d015cu64) }
        0x102d015c => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d0160u64) }
        0x102d0160 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0164u64).wrapping_add((imm * 4) as u64) } else { 0x102d0164u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0164u64) }
        0x102d0164 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0168u64) }
        0x102d0168 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d016cu64) }
        0x102d016c => { vm.decode(0x848103eu32); let d = 4i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0170u64) }
        0x102d0170 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0174u64) }
        0x102d0174 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d0178u64) }
        0x102d0178 => { vm.decode(0x32811099u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d017cu64) }
        0x102d017c => { vm.decode(0x30000099u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0180u64) }
        0x102d0180 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0184u64) }
        0x102d0184 => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d0188u64) }
        0x102d0188 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d018cu64).wrapping_add((imm * 4) as u64) } else { 0x102d018cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d018cu64) }
        0x102d018c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0190u64) }
        0x102d0190 => { vm.decode(0xc0801118u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0194u64) }
        0x102d0194 => { vm.decode(0xe057b119u32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 254i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0198u64) }
        0x102d0198 => { vm.decode(0xe2041099u32); let d = 6i16; let s = 17i16; let s2 = -1i16; let imm = 11i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d019cu64) }
        0x102d019c => { vm.decode(0x2020036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d01a0u64) }
        0x102d01a0 => { vm.decode(0x200408eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -2139095040i64;
        vm.st(d, imm as u64);
        Ok(0x102d01a4u64) }
        0x102d01a4 => { vm.decode(0xa0384118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 32897i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d01a8u64) }
        0x102d01a8 => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d01acu64) }
        0x102d01ac => { vm.decode(0x82002171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 78i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d01b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d01b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d01b0u64) }
        0x102d01b0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d01b4u64) }
        0x102d01bc => { vm.decode(0x108a016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 15i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d01c0u64) }
        0x102d01c0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d01c4u64) }
        0x102d01c4 => { vm.decode(0x714b0159u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d01c8u64) }
        0x102d01c8 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d01ccu64) }
        0x102d01cc => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d01d0u64) }
        0x102d01d0 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d01d4u64) }
        0x102d01d4 => { vm.decode(0xd5bfcdfeu32); let d = 24i16; let s = 2i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d01d8u64) }
        0x102d01d8 => { vm.decode(0x95140fb5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d01dcu64) }
        0x102d01dc => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d01e0u64) }
        0x102d01e0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d01e4u64) }
        0x102d01e4 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d01e8u64) }
        0x102d01e8 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d01ecu64) }
        0x102d01ec => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d01f0u64) }
        0x102d01f0 => { vm.decode(0x21880135u32); let d = 32i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let p = (vm.ld(s) as u32 as u64).wrapping_mul(vm.ld(s2) as u32 as u64);
        vm.st(d, (p as u32 as i32 as i64) as u64);
        vm.st(d.wrapping_add(1), ((p >> 32) as u32 as i32 as i64) as u64);
        Ok(0x102d01f4u64) }
        0x102d01f4 => { vm.decode(0x100401f5u32); let d = 24i16; let s = 33i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d01f8u64) }
        0x102d01f8 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d01fcu64) }
        0x102d01fc => { vm.decode(0xccf6ffd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0200u64) }
        0x102d0200 => { vm.decode(0x11858019u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0204u64) }
        0x102d0204 => { vm.decode(0x2413036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0208u64) }
        0x102d0208 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d020cu64) }
        0x102d020c => { vm.decode(0x44684d35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 10i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d0210u64) }
        0x102d0210 => { vm.decode(0xc400ebb5u32); let d = 16i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0214u64) }
        0x102d0214 => { vm.decode(0xc500abb5u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0218u64) }
        0x102d0218 => { vm.decode(0x4f000b35u32); let d = 24i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d021cu64) }
        0x102d021c => { vm.decode(0xd4000fb5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0220u64) }
        0x102d0220 => { vm.decode(0x9129036u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0224u64) }
        0x102d0224 => { vm.decode(0x129036u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0228u64) }
        0x102d0228 => { vm.decode(0x8109236u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d022cu64) }
        0x102d022c => { vm.decode(0x10b236u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0230u64) }
        0x102d0230 => { vm.decode(0xa121036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0234u64) }
        0x102d0234 => { vm.decode(0x8a65c909u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0238u64) }
        0x102d0238 => { vm.decode(0x692704f5u32); let d = 22i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d023cu64) }
        0x102d023c => { vm.decode(0xa4c103eu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0240u64) }
        0x102d0240 => { vm.decode(0x535da835u32); let d = 15i16; let s = 15i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0244u64) }
        0x102d0244 => { vm.decode(0xe0630433u32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d0248u64) }
        0x102d0248 => { vm.decode(0xeb274909u32); let d = 15i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d024cu64) }
        0x102d024c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0250u64) }
        0x102d0250 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0254u64) }
        0x102d0254 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0258u64) }
        0x102d0258 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d025cu64) }
        0x102d025c => { vm.decode(0xa101036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0260u64) }
        0x102d0260 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0264u64) }
        0x102d0264 => { vm.decode(0x212704f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0268u64) }
        0x102d0268 => { vm.decode(0x844103eu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d026cu64) }
        0x102d026c => { vm.decode(0x131d8835u32); let d = 14i16; let s = 14i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0270u64) }
        0x102d0270 => { vm.decode(0x606104b3u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d0274u64) }
        0x102d0274 => { vm.decode(0x298314f5u32); let d = 14i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0278u64) }
        0x102d0278 => { vm.decode(0x218504f5u32); let d = 24i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d027cu64) }
        0x102d027c => { vm.decode(0x610504f5u32); let d = 16i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0280u64) }
        0x102d0280 => { vm.decode(0xe90504f5u32); let d = 18i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0284u64) }
        0x102d0284 => { vm.decode(0xe8e314f5u32); let d = 15i16; let s = 15i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0288u64) }
        0x102d0288 => { vm.decode(0x23036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d028cu64) }
        0x102d028c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d0290u64) }
        0x102d0290 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0294u64) }
        0x102d0294 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0298u64) }
        0x102d0298 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d029cu64) }
        0x102d029c => { vm.decode(0xc02204b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02a0u64) }
        0x102d02a0 => { vm.decode(0x2008323eu32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02a4u64) }
        0x102d02a4 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02a8u64) }
        0x102d02a8 => { vm.decode(0x40220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02acu64) }
        0x102d02ac => { vm.decode(0x2048003eu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02b0u64) }
        0x102d02b0 => { vm.decode(0x70002835u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02b4u64) }
        0x102d02b4 => { vm.decode(0xc0220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02b8u64) }
        0x102d02b8 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02bcu64) }
        0x102d02bc => { vm.decode(0x70042835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02c0u64) }
        0x102d02c0 => { vm.decode(0x402201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02c4u64) }
        0x102d02c4 => { vm.decode(0x2040203eu32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02c8u64) }
        0x102d02c8 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02ccu64) }
        0x102d02cc => { vm.decode(0xc02201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02d0u64) }
        0x102d02d0 => { vm.decode(0x2008303eu32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02d4u64) }
        0x102d02d4 => { vm.decode(0x30002835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02d8u64) }
        0x102d02d8 => { vm.decode(0x40220533u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d02dcu64) }
        0x102d02dc => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d02e0u64) }
        0x102d02e0 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d02e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d02e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d02e4u64) }
        0x102d02e4 => { vm.decode(0x44b00875u32); let d = 6i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d02e8u64) }
        0x102d02e8 => { vm.decode(0xc5bbddfeu32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02ecu64) }
        0x102d02ec => { vm.decode(0x120a10beu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02f0u64) }
        0x102d02f0 => { vm.decode(0x220a103eu32); let d = 11i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02f4u64) }
        0x102d02f4 => { vm.decode(0xf5bbdd7eu32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02f8u64) }
        0x102d02f8 => { vm.decode(0x80a10beu32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d02fcu64) }
        0x102d02fc => { vm.decode(0xa0a107eu32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0300u64) }
        0x102d0300 => { vm.decode(0xfdbbddfeu32); let d = 13i16; let s = 6i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0304u64) }
        0x102d0304 => { vm.decode(0x3a0a103eu32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0308u64) }
        0x102d0308 => { vm.decode(0x82016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d030cu64) }
        0x102d030c => { vm.decode(0xa7d32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -85i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0310u64).wrapping_add((imm * 4) as u64) } else { 0x102d0310u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0310u64) }
        0x102d0310 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0314u64) } }
        0x102d0314 => { vm.decode(0xd2ca112du32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0318u64) }
        0x102d0318 => { vm.decode(0xda5a512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d031cu64) }
        0x102d031c => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0320u64) }
        0x102d0320 => { vm.decode(0xda4a91adu32); let d = 24i16; let s = 9i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0324u64) }
        0x102d0324 => { vm.decode(0xda4e116du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0328u64) }
        0x102d0328 => { vm.decode(0xda4a916du32); let d = 8i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d032cu64) }
        0x102d032c => { vm.decode(0xdaca516du32); let d = 9i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0330u64) }
        0x102d0330 => { vm.decode(0xdaced16du32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d0334u64) }
        0x102d0334 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0338u64) }
        0x102d0338 => { vm.decode(0x208134f5u32); let d = 8i16; let s = 8i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d033cu64) }
        0x102d033c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0340u64) }
        0x102d0340 => { vm.decode(0xa0a003eu32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0344u64) }
        0x102d0344 => { vm.decode(0xa0c534f5u32); let d = 25i16; let s = 9i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0348u64) }
        0x102d0348 => { vm.decode(0x3204a835u32); let d = 9i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d034cu64) }
        0x102d034c => { vm.decode(0x2203eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0350u64) }
        0x102d0350 => { vm.decode(0xe04134f5u32); let d = 1i16; let s = 11i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0354u64) }
        0x102d0354 => { vm.decode(0x688334f5u32); let d = 14i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0358u64) }
        0x102d0358 => { vm.decode(0xa8e134f5u32); let d = 11i16; let s = 13i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d035cu64) }
        0x102d035c => { vm.decode(0x28a134f5u32); let d = 10i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0360u64) }
        0x102d0360 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0364u64) }
        0x102d0364 => { vm.decode(0x80a023eu32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0368u64) }
        0x102d0368 => { vm.decode(0x134aa835u32); let d = 15i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d036cu64) }
        0x102d036c => { vm.decode(0x242223eu32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0370u64) }
        0x102d0370 => { vm.decode(0x220a223eu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0374u64) }
        0x102d0374 => { vm.decode(0x80a303eu32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0378u64) }
        0x102d0378 => { vm.decode(0x2802023eu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d037cu64) }
        0x102d037c => { vm.decode(0x214134f5u32); let d = 1i16; let s = 21i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0380u64) }
        0x102d0380 => { vm.decode(0xa02023eu32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d0384u64) }
        0x102d0384 => { vm.decode(0xaca0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1740i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d0388u64) }
        0x102d0388 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d038cu64) } }
        0x102d038c => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d0390u64) }
        0x102d0390 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0394u64) }
        0x102d0394 => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0398u64) }
        0x102d0398 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d039cu64) }
        0x102d039c => { vm.decode(0x8012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d03a0u64) }
        0x102d03a0 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d03a4u64) }
        0x102d03a4 => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d03a8u64) }
        0x102d03a8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d03acu64).wrapping_add((imm * 4) as u64) } else { 0x102d03acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d03acu64) }
        0x102d03ac => { vm.decode(0x8e766fd9u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d03b0u64) }
        0x102d03b8 => { vm.decode(0x2002a835u32); let d = 1i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d03bcu64) }
        0x102d03bc => { vm.decode(0x3a080d75u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d03c0u64) }
        0x102d03c0 => { vm.decode(0x20a07f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d03c4u64) }
        0x102d03c4 => { vm.decode(0xc68160f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d03c8u64) }
        0x102d03c8 => { vm.decode(0x561147f5u32); let d = 6i16; let s = 6i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d03ccu64) }
        0x102d03cc => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d03d0u64) }
        0x102d03d0 => { vm.decode(0x826204f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d03d4u64) }
        0x102d03d4 => { vm.decode(0x10001430u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d03d8u64) }
        0x102d03d8 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d03dcu64) }
        0x102d03dc => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d03e0u64) }
        0x102d03e0 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d03e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d03e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d03e4u64) }
        0x102d03e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d03e8u64) } }
        0x102d03e8 => { vm.decode(0x4c58bb5u32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d03ecu64) }
        0x102d03ec => { vm.decode(0xb2408d75u32); let d = 5i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d03f0u64) }
        0x102d03f0 => { vm.decode(0xa06104f5u32); let d = 1i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d03f4u64) }
        0x102d03f4 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d03f8u64) }
        0x102d03f8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d03fcu64) }
        0x102d03fc => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0400u64) }
        0x102d0400 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0404u64).wrapping_add((imm * 4) as u64) } else { 0x102d0404u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0404u64) }
        0x102d0404 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0408u64) }
        0x102d0408 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d040cu64) }
        0x102d040c => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0410u64) }
        0x102d0410 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0414u64) }
        0x102d0414 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0418u64) }
        0x102d0418 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d041cu64).wrapping_add((imm * 4) as u64) } else { 0x102d041cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d041cu64) }
        0x102d041c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0420u64) } }
        0x102d0420 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0424u64) }
        0x102d0424 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0428u64) }
        0x102d0428 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d042cu64).wrapping_add((imm * 4) as u64) } else { 0x102d042cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d042cu64) }
        0x102d042c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0430u64) }
        0x102d0430 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0434u64) }
        0x102d0434 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0438u64) }
        0x102d0438 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d043cu64) }
        0x102d043c => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0440u64) }
        0x102d0440 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0444u64).wrapping_add((imm * 4) as u64) } else { 0x102d0444u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0444u64) }
        0x102d0444 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0448u64) } }
        0x102d0448 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d044cu64) }
        0x102d044c => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0450u64) }
        0x102d0450 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0454u64) }
        0x102d0454 => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0458u64) }
        0x102d0458 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d045cu64) }
        0x102d045c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0460u64) }
        0x102d0460 => { vm.decode(0xa60a236u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0464u64) }
        0x102d0464 => { vm.decode(0xb2082d75u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0468u64) }
        0x102d0468 => { vm.decode(0xa4214f5u32); let d = 7i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d046cu64) }
        0x102d046c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0470u64) }
        0x102d0470 => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0474u64) }
        0x102d0474 => { vm.decode(0xa0482d75u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0478u64) }
        0x102d0478 => { vm.decode(0xa04114f5u32); let d = 1i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d047cu64) }
        0x102d047c => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0480u64) }
        0x102d0480 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0484u64) }
        0x102d0484 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0488u64) }
        0x102d0488 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d048cu64).wrapping_add((imm * 4) as u64) } else { 0x102d048cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d048cu64) }
        0x102d048c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0490u64) }
        0x102d0490 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0494u64) }
        0x102d0494 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0498u64) }
        0x102d0498 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d049cu64) }
        0x102d049c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d04a0u64) }
        0x102d04a0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d04a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d04a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d04a4u64) }
        0x102d04a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d04a8u64) } }
        0x102d04a8 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d04acu64) }
        0x102d04ac => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d04b0u64) }
        0x102d04b0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d04b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d04b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d04b4u64) }
        0x102d04b4 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d04b8u64) }
        0x102d04b8 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d04bcu64) }
        0x102d04bc => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d04c0u64) }
        0x102d04c0 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d04c4u64) }
        0x102d04c4 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d04c8u64) }
        0x102d04c8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d04ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d04ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d04ccu64) }
        0x102d04cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d04d0u64) } }
        0x102d04d0 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d04d4u64) }
        0x102d04d4 => { vm.decode(0x3a483d75u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d04d8u64) }
        0x102d04d8 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d04dcu64) }
        0x102d04dc => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d04e0u64) }
        0x102d04e0 => { vm.decode(0x545257f5u32); let d = 7i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d04e4u64) }
        0x102d04e4 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d04e8u64) }
        0x102d04e8 => { vm.decode(0xa62a236u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d04ecu64) }
        0x102d04ec => { vm.decode(0xb240bd75u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d04f0u64) }
        0x102d04f0 => { vm.decode(0xcaa004f5u32); let d = 10i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d04f4u64) }
        0x102d04f4 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d04f8u64) }
        0x102d04f8 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d04fcu64) }
        0x102d04fc => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0500u64) }
        0x102d0500 => { vm.decode(0x604114f5u32); let d = 1i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0504u64) }
        0x102d0504 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0508u64) }
        0x102d0508 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d050cu64) }
        0x102d050c => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0510u64) }
        0x102d0510 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0514u64).wrapping_add((imm * 4) as u64) } else { 0x102d0514u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0514u64) }
        0x102d0514 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0518u64) }
        0x102d0518 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d051cu64) }
        0x102d051c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0520u64) }
        0x102d0520 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0524u64) }
        0x102d0524 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0528u64) }
        0x102d0528 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d052cu64).wrapping_add((imm * 4) as u64) } else { 0x102d052cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d052cu64) }
        0x102d052c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0530u64) } }
        0x102d0530 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0534u64) }
        0x102d0534 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0538u64) }
        0x102d0538 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d053cu64).wrapping_add((imm * 4) as u64) } else { 0x102d053cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d053cu64) }
        0x102d053c => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0540u64) }
        0x102d0540 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0544u64) }
        0x102d0544 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0548u64) }
        0x102d0548 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d054cu64) }
        0x102d054c => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0550u64) }
        0x102d0550 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0554u64).wrapping_add((imm * 4) as u64) } else { 0x102d0554u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0554u64) }
        0x102d0554 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0558u64) } }
        0x102d0558 => { vm.decode(0x43a835u32); let d = 1i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d055cu64) }
        0x102d055c => { vm.decode(0x2b48ad75u32); let d = 13i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0560u64) }
        0x102d0560 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0564u64) }
        0x102d0564 => { vm.decode(0xa6b320f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0568u64) }
        0x102d0568 => { vm.decode(0x405f17f5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d056cu64) }
        0x102d056c => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0570u64) }
        0x102d0570 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0574u64) }
        0x102d0574 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0578u64) }
        0x102d0578 => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d057cu64) }
        0x102d057c => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0580u64) }
        0x102d0580 => { vm.decode(0x604114f5u32); let d = 1i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0584u64) }
        0x102d0584 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0588u64) }
        0x102d0588 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d058cu64) }
        0x102d058c => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0590u64) }
        0x102d0590 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0594u64).wrapping_add((imm * 4) as u64) } else { 0x102d0594u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0594u64) }
        0x102d0594 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0598u64) }
        0x102d0598 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d059cu64) }
        0x102d059c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d05a0u64) }
        0x102d05a0 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d05a4u64) }
        0x102d05a4 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d05a8u64) }
        0x102d05a8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d05acu64).wrapping_add((imm * 4) as u64) } else { 0x102d05acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d05acu64) }
        0x102d05ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d05b0u64) } }
        0x102d05b0 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d05b4u64) }
        0x102d05b4 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d05b8u64) }
        0x102d05b8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d05bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d05bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d05bcu64) }
        0x102d05bc => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d05c0u64) }
        0x102d05c0 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d05c4u64) }
        0x102d05c4 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d05c8u64) }
        0x102d05c8 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d05ccu64) }
        0x102d05cc => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d05d0u64) }
        0x102d05d0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d05d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d05d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d05d4u64) }
        0x102d05d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d05d8u64) } }
        0x102d05d8 => { vm.decode(0x43a835u32); let d = 1i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d05dcu64) }
        0x102d05dc => { vm.decode(0x2b48ad75u32); let d = 13i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d05e0u64) }
        0x102d05e0 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d05e4u64) }
        0x102d05e4 => { vm.decode(0xa6b320f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d05e8u64) }
        0x102d05e8 => { vm.decode(0x405f17f5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d05ecu64) }
        0x102d05ec => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d05f0u64) }
        0x102d05f0 => { vm.decode(0x160a236u32); let d = 11i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d05f4u64) }
        0x102d05f4 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d05f8u64) }
        0x102d05f8 => { vm.decode(0xc28214f5u32); let d = 12i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d05fcu64) }
        0x102d05fc => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0600u64) }
        0x102d0600 => { vm.decode(0xe04114f5u32); let d = 1i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0604u64) }
        0x102d0604 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0608u64) }
        0x102d0608 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d060cu64) }
        0x102d060c => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0610u64) }
        0x102d0610 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0614u64).wrapping_add((imm * 4) as u64) } else { 0x102d0614u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0614u64) }
        0x102d0614 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0618u64) }
        0x102d0618 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d061cu64) }
        0x102d061c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0620u64) }
        0x102d0620 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0624u64) }
        0x102d0624 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0628u64) }
        0x102d0628 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d062cu64).wrapping_add((imm * 4) as u64) } else { 0x102d062cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d062cu64) }
        0x102d062c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0630u64) } }
        0x102d0630 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0634u64) }
        0x102d0634 => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0638u64) }
        0x102d0638 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d063cu64).wrapping_add((imm * 4) as u64) } else { 0x102d063cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d063cu64) }
        0x102d063c => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0640u64) }
        0x102d0640 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0644u64) }
        0x102d0644 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0648u64) }
        0x102d0648 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d064cu64) }
        0x102d064c => { vm.decode(0xa1100446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0650u64) }
        0x102d0650 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0654u64).wrapping_add((imm * 4) as u64) } else { 0x102d0654u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0654u64) }
        0x102d0654 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0658u64) } }
        0x102d0658 => { vm.decode(0x43a835u32); let d = 1i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d065cu64) }
        0x102d065c => { vm.decode(0x2b48ad75u32); let d = 13i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0660u64) }
        0x102d0660 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0664u64) }
        0x102d0664 => { vm.decode(0xa6b320f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0668u64) }
        0x102d0668 => { vm.decode(0x405f17f5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d066cu64) }
        0x102d066c => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0670u64) }
        0x102d0670 => { vm.decode(0x82c014f5u32); let d = 9i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0674u64) }
        0x102d0674 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0678u64) }
        0x102d0678 => { vm.decode(0x204114f5u32); let d = 1i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d067cu64) }
        0x102d067c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0680u64) }
        0x102d0680 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0684u64) }
        0x102d0684 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0688u64) }
        0x102d0688 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d068cu64).wrapping_add((imm * 4) as u64) } else { 0x102d068cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d068cu64) }
        0x102d068c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0690u64) }
        0x102d0690 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0694u64) }
        0x102d0694 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0698u64) }
        0x102d0698 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d069cu64) }
        0x102d069c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d06a0u64) }
        0x102d06a0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d06a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d06a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d06a4u64) }
        0x102d06a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d06a8u64) } }
        0x102d06a8 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d06acu64) }
        0x102d06ac => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d06b0u64) }
        0x102d06b0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d06b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d06b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d06b4u64) }
        0x102d06b4 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d06b8u64) }
        0x102d06b8 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d06bcu64) }
        0x102d06bc => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d06c0u64) }
        0x102d06c0 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d06c4u64) }
        0x102d06c4 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d06c8u64) }
        0x102d06c8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d06ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d06ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d06ccu64) }
        0x102d06cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d06d0u64) } }
        0x102d06d0 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d06d4u64) }
        0x102d06d4 => { vm.decode(0x29481d75u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d06d8u64) }
        0x102d06d8 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d06dcu64) }
        0x102d06dc => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d06e0u64) }
        0x102d06e0 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d06e4u64) }
        0x102d06e4 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d06e8u64) }
        0x102d06e8 => { vm.decode(0x428014f5u32); let d = 8i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d06ecu64) }
        0x102d06ec => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d06f0u64) }
        0x102d06f0 => { vm.decode(0x62a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d06f4u64) }
        0x102d06f4 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d06f8u64) }
        0x102d06f8 => { vm.decode(0x606104f5u32); let d = 1i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d06fcu64) }
        0x102d06fc => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0700u64) }
        0x102d0700 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0704u64) }
        0x102d0704 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0708u64) }
        0x102d0708 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d070cu64).wrapping_add((imm * 4) as u64) } else { 0x102d070cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d070cu64) }
        0x102d070c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0710u64) }
        0x102d0710 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0714u64) }
        0x102d0714 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0718u64) }
        0x102d0718 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d071cu64) }
        0x102d071c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0720u64) }
        0x102d0720 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0724u64).wrapping_add((imm * 4) as u64) } else { 0x102d0724u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0724u64) }
        0x102d0724 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0728u64) } }
        0x102d0728 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d072cu64) }
        0x102d072c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0730u64) }
        0x102d0730 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0734u64).wrapping_add((imm * 4) as u64) } else { 0x102d0734u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0734u64) }
        0x102d0734 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0738u64) }
        0x102d0738 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d073cu64) }
        0x102d073c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0740u64) }
        0x102d0740 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0744u64) }
        0x102d0744 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0748u64) }
        0x102d0748 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d074cu64).wrapping_add((imm * 4) as u64) } else { 0x102d074cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d074cu64) }
        0x102d074c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0750u64) } }
        0x102d0750 => { vm.decode(0x2042a835u32); let d = 1i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0754u64) }
        0x102d0754 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0758u64) }
        0x102d0758 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d075cu64) }
        0x102d075c => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0760u64) }
        0x102d0760 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0764u64) }
        0x102d0764 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0768u64) }
        0x102d0768 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d076cu64) }
        0x102d076c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0770u64) }
        0x102d0770 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0774u64) }
        0x102d0774 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0778u64) }
        0x102d0778 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d077cu64) }
        0x102d077c => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0780u64) }
        0x102d0780 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0784u64).wrapping_add((imm * 4) as u64) } else { 0x102d0784u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0784u64) }
        0x102d0784 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0788u64) }
        0x102d0788 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d078cu64) }
        0x102d078c => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0790u64) }
        0x102d0790 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0794u64) }
        0x102d0794 => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0798u64) }
        0x102d0798 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d079cu64).wrapping_add((imm * 4) as u64) } else { 0x102d079cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d079cu64) }
        0x102d079c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d07a0u64) } }
        0x102d07a0 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d07a4u64) }
        0x102d07a4 => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d07a8u64) }
        0x102d07a8 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d07acu64).wrapping_add((imm * 4) as u64) } else { 0x102d07acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d07acu64) }
        0x102d07ac => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d07b0u64) }
        0x102d07b0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d07b4u64) }
        0x102d07b4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d07b8u64) }
        0x102d07b8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d07bcu64) }
        0x102d07bc => { vm.decode(0x80100446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d07c0u64) }
        0x102d07c0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d07c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d07c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d07c4u64) }
        0x102d07c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d07c8u64) } }
        0x102d07c8 => { vm.decode(0x8820401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2248i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d07ccu64) }
        0x102d07cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d07d0u64) } }
        0x102d07d0 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d07d4u64) }
        0x102d07d4 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d07d8u64) }
        0x102d07d8 => { vm.decode(0x888632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d07dcu64) }
        0x102d07dc => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d07e0u64) }
        0x102d07e0 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d07e4u64) }
        0x102d07e4 => { vm.decode(0x2888432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d07e8u64) }
        0x102d07e8 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d07ecu64) }
        0x102d07ec => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d07f0u64) }
        0x102d07f0 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d07f4u64) }
        0x102d07f4 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d07f8u64) }
        0x102d07f8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d07fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d07fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d07fcu64) }
        0x102d07fc => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0800u64) }
        0x102d0808 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d080cu64) }
        0x102d080c => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0810u64) }
        0x102d0810 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0814u64) }
        0x102d0814 => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0818u64) }
        0x102d0818 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d081cu64) }
        0x102d081c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0820u64) }
        0x102d0820 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0824u64) }
        0x102d0824 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0828u64) }
        0x102d0828 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d082cu64) }
        0x102d082c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0830u64) }
        0x102d0830 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0834u64).wrapping_add((imm * 4) as u64) } else { 0x102d0834u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0834u64) }
        0x102d0834 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0838u64) } }
        0x102d0838 => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d083cu64) }
        0x102d083c => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0840u64) }
        0x102d0840 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0844u64) }
        0x102d0844 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0848u64) }
        0x102d0848 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d084cu64) }
        0x102d084c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0850u64) }
        0x102d0850 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0854u64).wrapping_add((imm * 4) as u64) } else { 0x102d0854u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0854u64) }
        0x102d0854 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0858u64) }
        0x102d0858 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d085cu64) }
        0x102d085c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0860u64) }
        0x102d0860 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0864u64) }
        0x102d0864 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0868u64) }
        0x102d0868 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d086cu64).wrapping_add((imm * 4) as u64) } else { 0x102d086cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d086cu64) }
        0x102d086c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0870u64) } }
        0x102d0870 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0874u64) }
        0x102d0874 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0878u64) }
        0x102d0878 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d087cu64).wrapping_add((imm * 4) as u64) } else { 0x102d087cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d087cu64) }
        0x102d087c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0880u64) }
        0x102d0880 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0884u64) }
        0x102d0884 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0888u64) }
        0x102d0888 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d088cu64) }
        0x102d088c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0890u64) }
        0x102d0890 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0894u64).wrapping_add((imm * 4) as u64) } else { 0x102d0894u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0894u64) }
        0x102d0894 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0898u64) } }
        0x102d0898 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d089cu64) }
        0x102d089c => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d08a0u64) }
        0x102d08a0 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d08a4u64) }
        0x102d08a4 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d08a8u64) }
        0x102d08a8 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d08acu64) }
        0x102d08ac => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d08b0u64) }
        0x102d08b0 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d08b4u64) }
        0x102d08b4 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d08b8u64) }
        0x102d08b8 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d08bcu64) }
        0x102d08bc => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d08c0u64) }
        0x102d08c0 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d08c4u64) }
        0x102d08c4 => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d08c8u64) }
        0x102d08c8 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d08ccu64) }
        0x102d08cc => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d08d0u64) }
        0x102d08d0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d08d4u64) }
        0x102d08d4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d08d8u64) }
        0x102d08d8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d08dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d08dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d08dcu64) }
        0x102d08dc => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d08e0u64) }
        0x102d08e0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d08e4u64) }
        0x102d08e4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d08e8u64) }
        0x102d08e8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d08ecu64) }
        0x102d08ec => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d08f0u64) }
        0x102d08f0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d08f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d08f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d08f4u64) }
        0x102d08f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d08f8u64) } }
        0x102d08f8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d08fcu64) }
        0x102d08fc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0900u64) }
        0x102d0900 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0904u64).wrapping_add((imm * 4) as u64) } else { 0x102d0904u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0904u64) }
        0x102d0904 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0908u64) }
        0x102d0908 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d090cu64) }
        0x102d090c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0910u64) }
        0x102d0910 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0914u64) }
        0x102d0914 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0918u64) }
        0x102d0918 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d091cu64).wrapping_add((imm * 4) as u64) } else { 0x102d091cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d091cu64) }
        0x102d091c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0920u64) } }
        0x102d0920 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0924u64) }
        0x102d0924 => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0928u64) }
        0x102d0928 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d092cu64) }
        0x102d092c => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0930u64) }
        0x102d0930 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0934u64) }
        0x102d0934 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0938u64) }
        0x102d0938 => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d093cu64) }
        0x102d093c => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0940u64) }
        0x102d0940 => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0944u64) }
        0x102d0944 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0948u64) }
        0x102d0948 => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d094cu64) }
        0x102d094c => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0950u64) }
        0x102d0950 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0954u64) }
        0x102d0954 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0958u64) }
        0x102d0958 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d095cu64) }
        0x102d095c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0960u64) }
        0x102d0960 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0964u64).wrapping_add((imm * 4) as u64) } else { 0x102d0964u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0964u64) }
        0x102d0964 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0968u64) }
        0x102d0968 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d096cu64) }
        0x102d096c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0970u64) }
        0x102d0970 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0974u64) }
        0x102d0974 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0978u64) }
        0x102d0978 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d097cu64).wrapping_add((imm * 4) as u64) } else { 0x102d097cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d097cu64) }
        0x102d097c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0980u64) } }
        0x102d0980 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0984u64) }
        0x102d0984 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0988u64) }
        0x102d0988 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d098cu64).wrapping_add((imm * 4) as u64) } else { 0x102d098cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d098cu64) }
        0x102d098c => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0990u64) }
        0x102d0990 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0994u64) }
        0x102d0994 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0998u64) }
        0x102d0998 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d099cu64) }
        0x102d099c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d09a0u64) }
        0x102d09a0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d09a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d09a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d09a4u64) }
        0x102d09a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d09a8u64) } }
        0x102d09a8 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d09acu64) }
        0x102d09ac => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d09b0u64) }
        0x102d09b0 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d09b4u64) }
        0x102d09b4 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d09b8u64) }
        0x102d09b8 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d09bcu64) }
        0x102d09bc => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d09c0u64) }
        0x102d09c0 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d09c4u64) }
        0x102d09c4 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d09c8u64) }
        0x102d09c8 => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d09ccu64) }
        0x102d09cc => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d09d0u64) }
        0x102d09d0 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d09d4u64) }
        0x102d09d4 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d09d8u64) }
        0x102d09d8 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d09dcu64) }
        0x102d09dc => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d09e0u64) }
        0x102d09e0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d09e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d09e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d09e4u64) }
        0x102d09e4 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d09e8u64) }
        0x102d09e8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d09ecu64) }
        0x102d09ec => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d09f0u64) }
        0x102d09f0 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d09f4u64) }
        0x102d09f4 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d09f8u64) }
        0x102d09f8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d09fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d09fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d09fcu64) }
        0x102d09fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0a00u64) } }
        0x102d0a00 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a04u64) }
        0x102d0a04 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a08u64) }
        0x102d0a08 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0a0cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0a0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0a0cu64) }
        0x102d0a0c => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0a10u64) }
        0x102d0a10 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a14u64) }
        0x102d0a14 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a18u64) }
        0x102d0a18 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0a1cu64) }
        0x102d0a1c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a20u64) }
        0x102d0a20 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0a24u64).wrapping_add((imm * 4) as u64) } else { 0x102d0a24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0a24u64) }
        0x102d0a24 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0a28u64) } }
        0x102d0a28 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0a2cu64) }
        0x102d0a2c => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a30u64) }
        0x102d0a30 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0a34u64) }
        0x102d0a34 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a38u64) }
        0x102d0a38 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0a3cu64) }
        0x102d0a3c => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0a40u64) }
        0x102d0a40 => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a44u64) }
        0x102d0a44 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a48u64) }
        0x102d0a48 => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0a4cu64) }
        0x102d0a4c => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0a50u64) }
        0x102d0a50 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0a54u64) }
        0x102d0a54 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0a58u64) }
        0x102d0a58 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a5cu64) }
        0x102d0a5c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a60u64) }
        0x102d0a60 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0a64u64).wrapping_add((imm * 4) as u64) } else { 0x102d0a64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0a64u64) }
        0x102d0a64 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0a68u64) }
        0x102d0a68 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a6cu64) }
        0x102d0a6c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a70u64) }
        0x102d0a70 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0a74u64) }
        0x102d0a74 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a78u64) }
        0x102d0a78 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0a7cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0a7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0a7cu64) }
        0x102d0a7c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0a80u64) } }
        0x102d0a80 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a84u64) }
        0x102d0a84 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a88u64) }
        0x102d0a88 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0a8cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0a8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0a8cu64) }
        0x102d0a8c => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0a90u64) }
        0x102d0a90 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0a94u64) }
        0x102d0a94 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0a98u64) }
        0x102d0a98 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0a9cu64) }
        0x102d0a9c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0aa0u64) }
        0x102d0aa0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0aa4u64).wrapping_add((imm * 4) as u64) } else { 0x102d0aa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0aa4u64) }
        0x102d0aa4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0aa8u64) } }
        0x102d0aa8 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0aacu64) }
        0x102d0aac => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ab0u64) }
        0x102d0ab0 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ab4u64) }
        0x102d0ab4 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ab8u64) }
        0x102d0ab8 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0abcu64) }
        0x102d0abc => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ac0u64) }
        0x102d0ac0 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0ac4u64) }
        0x102d0ac4 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0ac8u64) }
        0x102d0ac8 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0accu64) }
        0x102d0acc => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0ad0u64) }
        0x102d0ad0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ad4u64) }
        0x102d0ad4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ad8u64) }
        0x102d0ad8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0adcu64).wrapping_add((imm * 4) as u64) } else { 0x102d0adcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0adcu64) }
        0x102d0adc => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0ae0u64) }
        0x102d0ae0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ae4u64) }
        0x102d0ae4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ae8u64) }
        0x102d0ae8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0aecu64) }
        0x102d0aec => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0af0u64) }
        0x102d0af0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0af4u64).wrapping_add((imm * 4) as u64) } else { 0x102d0af4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0af4u64) }
        0x102d0af4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0af8u64) } }
        0x102d0af8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0afcu64) }
        0x102d0afc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b00u64) }
        0x102d0b00 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0b04u64).wrapping_add((imm * 4) as u64) } else { 0x102d0b04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b04u64) }
        0x102d0b04 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0b08u64) }
        0x102d0b08 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b0cu64) }
        0x102d0b0c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b10u64) }
        0x102d0b10 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0b14u64) }
        0x102d0b14 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b18u64) }
        0x102d0b18 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0b1cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0b1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b1cu64) }
        0x102d0b1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0b20u64) } }
        0x102d0b20 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0b24u64) }
        0x102d0b24 => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b28u64) }
        0x102d0b28 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0b2cu64) }
        0x102d0b2c => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b30u64) }
        0x102d0b30 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0b34u64) }
        0x102d0b34 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0b38u64) }
        0x102d0b38 => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0b3cu64) }
        0x102d0b3c => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0b40u64) }
        0x102d0b40 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b44u64) }
        0x102d0b44 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b48u64) }
        0x102d0b48 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0b4cu64) }
        0x102d0b4c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0b50u64) }
        0x102d0b50 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b54u64) }
        0x102d0b54 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b58u64) }
        0x102d0b58 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0b5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0b5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b5cu64) }
        0x102d0b5c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0b60u64) }
        0x102d0b60 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b64u64) }
        0x102d0b64 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b68u64) }
        0x102d0b68 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0b6cu64) }
        0x102d0b6c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b70u64) }
        0x102d0b70 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0b74u64).wrapping_add((imm * 4) as u64) } else { 0x102d0b74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b74u64) }
        0x102d0b74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0b78u64) } }
        0x102d0b78 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b7cu64) }
        0x102d0b7c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b80u64) }
        0x102d0b80 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0b84u64).wrapping_add((imm * 4) as u64) } else { 0x102d0b84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b84u64) }
        0x102d0b84 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0b88u64) }
        0x102d0b88 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b8cu64) }
        0x102d0b8c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0b90u64) }
        0x102d0b90 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0b94u64) }
        0x102d0b94 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0b98u64) }
        0x102d0b98 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0b9cu64).wrapping_add((imm * 4) as u64) } else { 0x102d0b9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0b9cu64) }
        0x102d0b9c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0ba0u64) } }
        0x102d0ba0 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ba4u64) }
        0x102d0ba4 => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ba8u64) }
        0x102d0ba8 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0bacu64) }
        0x102d0bac => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0bb0u64) }
        0x102d0bb0 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0bb4u64) }
        0x102d0bb4 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0bb8u64) }
        0x102d0bb8 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0bbcu64) }
        0x102d0bbc => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0bc0u64) }
        0x102d0bc0 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0bc4u64) }
        0x102d0bc4 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0bc8u64) }
        0x102d0bc8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0bccu64) }
        0x102d0bcc => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0bd0u64) }
        0x102d0bd0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0bd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d0bd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0bd4u64) }
        0x102d0bd4 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0bd8u64) }
        0x102d0bd8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0bdcu64) }
        0x102d0bdc => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0be0u64) }
        0x102d0be0 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0be4u64) }
        0x102d0be4 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0be8u64) }
        0x102d0be8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0becu64).wrapping_add((imm * 4) as u64) } else { 0x102d0becu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0becu64) }
        0x102d0bec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0bf0u64) } }
        0x102d0bf0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0bf4u64) }
        0x102d0bf4 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0bf8u64) }
        0x102d0bf8 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0bfcu64).wrapping_add((imm * 4) as u64) } else { 0x102d0bfcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0bfcu64) }
        0x102d0bfc => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0c00u64) }
        0x102d0c00 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c04u64) }
        0x102d0c04 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0c08u64) }
        0x102d0c08 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0c0cu64) }
        0x102d0c0c => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c10u64) }
        0x102d0c10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0c14u64).wrapping_add((imm * 4) as u64) } else { 0x102d0c14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0c14u64) }
        0x102d0c14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0c18u64) } }
        0x102d0c18 => { vm.decode(0xc00541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d0c1cu64) }
        0x102d0c1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0c20u64) } }
        0x102d0c20 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d0c24u64) }
        0x102d0c24 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0c28u64) }
        0x102d0c28 => { vm.decode(0x8a01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0c2cu64) }
        0x102d0c2c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0c30u64) }
        0x102d0c30 => { vm.decode(0x2888632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0c34u64) }
        0x102d0c34 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0c38u64) }
        0x102d0c38 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0c3cu64) }
        0x102d0c3c => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c40u64) }
        0x102d0c40 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c44u64) }
        0x102d0c44 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0c48u64).wrapping_add((imm * 4) as u64) } else { 0x102d0c48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0c48u64) }
        0x102d0c48 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0c4cu64) }
        0x102d0c54 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0c58u64) }
        0x102d0c58 => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0c5cu64) }
        0x102d0c5c => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0c60u64) }
        0x102d0c60 => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0c64u64) }
        0x102d0c64 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0c68u64) }
        0x102d0c68 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0c6cu64) }
        0x102d0c6c => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0c70u64) }
        0x102d0c70 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0c74u64) }
        0x102d0c74 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c78u64) }
        0x102d0c78 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c7cu64) }
        0x102d0c7c => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0c80u64).wrapping_add((imm * 4) as u64) } else { 0x102d0c80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0c80u64) }
        0x102d0c80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0c84u64) } }
        0x102d0c84 => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d0c88u64) }
        0x102d0c88 => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0c8cu64) }
        0x102d0c8c => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0c90u64) }
        0x102d0c90 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0c94u64) }
        0x102d0c94 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c98u64) }
        0x102d0c98 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0c9cu64) }
        0x102d0c9c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0ca0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0ca0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0ca0u64) }
        0x102d0ca0 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0ca4u64) }
        0x102d0ca4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ca8u64) }
        0x102d0ca8 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0cacu64) }
        0x102d0cac => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0cb0u64) }
        0x102d0cb0 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0cb4u64) }
        0x102d0cb4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0cb8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0cb8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0cb8u64) }
        0x102d0cb8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0cbcu64) } }
        0x102d0cbc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0cc0u64) }
        0x102d0cc0 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0cc4u64) }
        0x102d0cc4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0cc8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0cc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0cc8u64) }
        0x102d0cc8 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0cccu64) }
        0x102d0ccc => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0cd0u64) }
        0x102d0cd0 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0cd4u64) }
        0x102d0cd4 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0cd8u64) }
        0x102d0cd8 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0cdcu64) }
        0x102d0cdc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0ce0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0ce0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0ce0u64) }
        0x102d0ce0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0ce4u64) } }
        0x102d0ce4 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ce8u64) }
        0x102d0ce8 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0cecu64) }
        0x102d0cec => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0cf0u64) }
        0x102d0cf0 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0cf4u64) }
        0x102d0cf4 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0cf8u64) }
        0x102d0cf8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0cfcu64) }
        0x102d0cfc => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d00u64) }
        0x102d0d00 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d04u64) }
        0x102d0d04 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0d08u64) }
        0x102d0d08 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0d0cu64) }
        0x102d0d0c => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d10u64) }
        0x102d0d10 => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d14u64) }
        0x102d0d14 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0d18u64) }
        0x102d0d18 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0d1cu64) }
        0x102d0d1c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d20u64) }
        0x102d0d20 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d24u64) }
        0x102d0d24 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0d28u64).wrapping_add((imm * 4) as u64) } else { 0x102d0d28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0d28u64) }
        0x102d0d28 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0d2cu64) }
        0x102d0d2c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d30u64) }
        0x102d0d30 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d34u64) }
        0x102d0d34 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0d38u64) }
        0x102d0d38 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d3cu64) }
        0x102d0d3c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0d40u64).wrapping_add((imm * 4) as u64) } else { 0x102d0d40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0d40u64) }
        0x102d0d40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0d44u64) } }
        0x102d0d44 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d48u64) }
        0x102d0d48 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d4cu64) }
        0x102d0d4c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0d50u64).wrapping_add((imm * 4) as u64) } else { 0x102d0d50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0d50u64) }
        0x102d0d50 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0d54u64) }
        0x102d0d54 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d58u64) }
        0x102d0d58 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d5cu64) }
        0x102d0d5c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0d60u64) }
        0x102d0d60 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d64u64) }
        0x102d0d64 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0d68u64).wrapping_add((imm * 4) as u64) } else { 0x102d0d68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0d68u64) }
        0x102d0d68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0d6cu64) } }
        0x102d0d6c => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0d70u64) }
        0x102d0d70 => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d74u64) }
        0x102d0d74 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0d78u64) }
        0x102d0d78 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d7cu64) }
        0x102d0d7c => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0d80u64) }
        0x102d0d80 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0d84u64) }
        0x102d0d84 => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d88u64) }
        0x102d0d88 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d8cu64) }
        0x102d0d8c => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0d90u64) }
        0x102d0d90 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0d94u64) }
        0x102d0d94 => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0d98u64) }
        0x102d0d98 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0d9cu64) }
        0x102d0d9c => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0da0u64) }
        0x102d0da0 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0da4u64) }
        0x102d0da4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0da8u64) }
        0x102d0da8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0dacu64) }
        0x102d0dac => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0db0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0db0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0db0u64) }
        0x102d0db0 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0db4u64) }
        0x102d0db4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0db8u64) }
        0x102d0db8 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0dbcu64) }
        0x102d0dbc => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0dc0u64) }
        0x102d0dc0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0dc4u64) }
        0x102d0dc4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0dc8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0dc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0dc8u64) }
        0x102d0dc8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0dccu64) } }
        0x102d0dcc => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0dd0u64) }
        0x102d0dd0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0dd4u64) }
        0x102d0dd4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0dd8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0dd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0dd8u64) }
        0x102d0dd8 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0ddcu64) }
        0x102d0ddc => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0de0u64) }
        0x102d0de0 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0de4u64) }
        0x102d0de4 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0de8u64) }
        0x102d0de8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0decu64) }
        0x102d0dec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0df0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0df0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0df0u64) }
        0x102d0df0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0df4u64) } }
        0x102d0df4 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0df8u64) }
        0x102d0df8 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0dfcu64) }
        0x102d0dfc => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e00u64) }
        0x102d0e00 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e04u64) }
        0x102d0e04 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e08u64) }
        0x102d0e08 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e0cu64) }
        0x102d0e0c => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0e10u64) }
        0x102d0e10 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0e14u64) }
        0x102d0e14 => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e18u64) }
        0x102d0e18 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e1cu64) }
        0x102d0e1c => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0e20u64) }
        0x102d0e20 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0e24u64) }
        0x102d0e24 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e28u64) }
        0x102d0e28 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e2cu64) }
        0x102d0e2c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0e30u64).wrapping_add((imm * 4) as u64) } else { 0x102d0e30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0e30u64) }
        0x102d0e30 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0e34u64) }
        0x102d0e34 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e38u64) }
        0x102d0e38 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e3cu64) }
        0x102d0e3c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0e40u64) }
        0x102d0e40 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e44u64) }
        0x102d0e44 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0e48u64).wrapping_add((imm * 4) as u64) } else { 0x102d0e48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0e48u64) }
        0x102d0e48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0e4cu64) } }
        0x102d0e4c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e50u64) }
        0x102d0e50 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e54u64) }
        0x102d0e54 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0e58u64).wrapping_add((imm * 4) as u64) } else { 0x102d0e58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0e58u64) }
        0x102d0e58 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0e5cu64) }
        0x102d0e5c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e60u64) }
        0x102d0e60 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e64u64) }
        0x102d0e64 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0e68u64) }
        0x102d0e68 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e6cu64) }
        0x102d0e6c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0e70u64).wrapping_add((imm * 4) as u64) } else { 0x102d0e70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0e70u64) }
        0x102d0e70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0e74u64) } }
        0x102d0e74 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e78u64) }
        0x102d0e78 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e7cu64) }
        0x102d0e7c => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e80u64) }
        0x102d0e80 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e84u64) }
        0x102d0e84 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e88u64) }
        0x102d0e88 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0e8cu64) }
        0x102d0e8c => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0e90u64) }
        0x102d0e90 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0e94u64) }
        0x102d0e94 => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0e98u64) }
        0x102d0e98 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0e9cu64) }
        0x102d0e9c => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0ea0u64) }
        0x102d0ea0 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0ea4u64) }
        0x102d0ea4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ea8u64) }
        0x102d0ea8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0eacu64) }
        0x102d0eac => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0eb0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0eb0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0eb0u64) }
        0x102d0eb0 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0eb4u64) }
        0x102d0eb4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0eb8u64) }
        0x102d0eb8 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ebcu64) }
        0x102d0ebc => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0ec0u64) }
        0x102d0ec0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ec4u64) }
        0x102d0ec4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0ec8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0ec8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0ec8u64) }
        0x102d0ec8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0eccu64) } }
        0x102d0ecc => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ed0u64) }
        0x102d0ed0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ed4u64) }
        0x102d0ed4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0ed8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0ed8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0ed8u64) }
        0x102d0ed8 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0edcu64) }
        0x102d0edc => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0ee0u64) }
        0x102d0ee0 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ee4u64) }
        0x102d0ee4 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0ee8u64) }
        0x102d0ee8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0eecu64) }
        0x102d0eec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0ef0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0ef0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0ef0u64) }
        0x102d0ef0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0ef4u64) } }
        0x102d0ef4 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ef8u64) }
        0x102d0ef8 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0efcu64) }
        0x102d0efc => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f00u64) }
        0x102d0f00 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f04u64) }
        0x102d0f04 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f08u64) }
        0x102d0f08 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f0cu64) }
        0x102d0f0c => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0f10u64) }
        0x102d0f10 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0f14u64) }
        0x102d0f14 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0f18u64) }
        0x102d0f18 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0f1cu64) }
        0x102d0f1c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f20u64) }
        0x102d0f20 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f24u64) }
        0x102d0f24 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0f28u64).wrapping_add((imm * 4) as u64) } else { 0x102d0f28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0f28u64) }
        0x102d0f28 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0f2cu64) }
        0x102d0f2c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f30u64) }
        0x102d0f30 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f34u64) }
        0x102d0f34 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0f38u64) }
        0x102d0f38 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f3cu64) }
        0x102d0f3c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0f40u64).wrapping_add((imm * 4) as u64) } else { 0x102d0f40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0f40u64) }
        0x102d0f40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0f44u64) } }
        0x102d0f44 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f48u64) }
        0x102d0f48 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f4cu64) }
        0x102d0f4c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0f50u64).wrapping_add((imm * 4) as u64) } else { 0x102d0f50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0f50u64) }
        0x102d0f50 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0f54u64) }
        0x102d0f54 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f58u64) }
        0x102d0f58 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f5cu64) }
        0x102d0f5c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0f60u64) }
        0x102d0f60 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f64u64) }
        0x102d0f64 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0f68u64).wrapping_add((imm * 4) as u64) } else { 0x102d0f68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0f68u64) }
        0x102d0f68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0f6cu64) } }
        0x102d0f6c => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f70u64) }
        0x102d0f70 => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f74u64) }
        0x102d0f74 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f78u64) }
        0x102d0f78 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f7cu64) }
        0x102d0f7c => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f80u64) }
        0x102d0f80 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0f84u64) }
        0x102d0f84 => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0f88u64) }
        0x102d0f88 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d0f8cu64) }
        0x102d0f8c => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0f90u64) }
        0x102d0f90 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0f94u64) }
        0x102d0f94 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d0f98u64) }
        0x102d0f98 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d0f9cu64) }
        0x102d0f9c => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fa0u64) }
        0x102d0fa0 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fa4u64) }
        0x102d0fa4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0fa8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0fa8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0fa8u64) }
        0x102d0fa8 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0facu64) }
        0x102d0fac => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fb0u64) }
        0x102d0fb0 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0fb4u64) }
        0x102d0fb4 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0fb8u64) }
        0x102d0fb8 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fbcu64) }
        0x102d0fbc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0fc0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0fc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0fc0u64) }
        0x102d0fc0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0fc4u64) } }
        0x102d0fc4 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fc8u64) }
        0x102d0fc8 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fccu64) }
        0x102d0fcc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d0fd0u64).wrapping_add((imm * 4) as u64) } else { 0x102d0fd0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0fd0u64) }
        0x102d0fd0 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d0fd4u64) }
        0x102d0fd4 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fd8u64) }
        0x102d0fd8 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0fdcu64) }
        0x102d0fdc => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d0fe0u64) }
        0x102d0fe0 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d0fe4u64) }
        0x102d0fe4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d0fe8u64).wrapping_add((imm * 4) as u64) } else { 0x102d0fe8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d0fe8u64) }
        0x102d0fe8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d0fecu64) } }
        0x102d0fec => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ff0u64) }
        0x102d0ff0 => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ff4u64) }
        0x102d0ff4 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d0ff8u64) }
        0x102d0ff8 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d0ffcu64) }
        0x102d0ffc => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1000u64) }
        0x102d1000 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1004u64) }
        0x102d1004 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1008u64) }
        0x102d1008 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d100cu64) }
        0x102d100c => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1010u64) }
        0x102d1010 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1014u64) }
        0x102d1014 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1018u64) }
        0x102d1018 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d101cu64) }
        0x102d101c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1020u64).wrapping_add((imm * 4) as u64) } else { 0x102d1020u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1020u64) }
        0x102d1020 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1024u64) }
        0x102d1024 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1028u64) }
        0x102d1028 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d102cu64) }
        0x102d102c => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1030u64) }
        0x102d1030 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1034u64) }
        0x102d1034 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1038u64).wrapping_add((imm * 4) as u64) } else { 0x102d1038u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1038u64) }
        0x102d1038 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d103cu64) } }
        0x102d103c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1040u64) }
        0x102d1040 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1044u64) }
        0x102d1044 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1048u64).wrapping_add((imm * 4) as u64) } else { 0x102d1048u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1048u64) }
        0x102d1048 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d104cu64) }
        0x102d104c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1050u64) }
        0x102d1050 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1054u64) }
        0x102d1054 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1058u64) }
        0x102d1058 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d105cu64) }
        0x102d105c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1060u64).wrapping_add((imm * 4) as u64) } else { 0x102d1060u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1060u64) }
        0x102d1060 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1064u64) } }
        0x102d1064 => { vm.decode(0x2124101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4452i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d1068u64) }
        0x102d1068 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d106cu64) } }
        0x102d106c => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d1070u64) }
        0x102d1070 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1074u64) }
        0x102d1074 => { vm.decode(0xaa01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1078u64) }
        0x102d1078 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d107cu64) }
        0x102d107c => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1080u64) }
        0x102d1080 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1084u64) }
        0x102d1084 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1088u64) }
        0x102d1088 => { vm.decode(0x408a632fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d108cu64) }
        0x102d108c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1090u64) }
        0x102d1090 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1094u64).wrapping_add((imm * 4) as u64) } else { 0x102d1094u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1094u64) }
        0x102d1094 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1098u64) }
        0x102d10a0 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d10a4u64) }
        0x102d10a4 => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d10a8u64) }
        0x102d10a8 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d10acu64) }
        0x102d10ac => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d10b0u64) }
        0x102d10b0 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d10b4u64) }
        0x102d10b4 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d10b8u64) }
        0x102d10b8 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d10bcu64) }
        0x102d10bc => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d10c0u64) }
        0x102d10c0 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d10c4u64) }
        0x102d10c4 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d10c8u64) }
        0x102d10c8 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d10ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d10ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d10ccu64) }
        0x102d10cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d10d0u64) } }
        0x102d10d0 => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d10d4u64) }
        0x102d10d4 => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d10d8u64) }
        0x102d10d8 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d10dcu64) }
        0x102d10dc => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d10e0u64) }
        0x102d10e0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d10e4u64) }
        0x102d10e4 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d10e8u64) }
        0x102d10e8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d10ecu64).wrapping_add((imm * 4) as u64) } else { 0x102d10ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d10ecu64) }
        0x102d10ec => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d10f0u64) }
        0x102d10f0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d10f4u64) }
        0x102d10f4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d10f8u64) }
        0x102d10f8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d10fcu64) }
        0x102d10fc => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1100u64) }
        0x102d1100 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1104u64).wrapping_add((imm * 4) as u64) } else { 0x102d1104u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1104u64) }
        0x102d1104 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1108u64) } }
        0x102d1108 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d110cu64) }
        0x102d110c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1110u64) }
        0x102d1110 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1114u64).wrapping_add((imm * 4) as u64) } else { 0x102d1114u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1114u64) }
        0x102d1114 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1118u64) }
        0x102d1118 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d111cu64) }
        0x102d111c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1120u64) }
        0x102d1120 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1124u64) }
        0x102d1124 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1128u64) }
        0x102d1128 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d112cu64).wrapping_add((imm * 4) as u64) } else { 0x102d112cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d112cu64) }
        0x102d112c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1130u64) } }
        0x102d1130 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1134u64) }
        0x102d1134 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1138u64) }
        0x102d1138 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d113cu64) }
        0x102d113c => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1140u64) }
        0x102d1140 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1144u64) }
        0x102d1144 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1148u64) }
        0x102d1148 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d114cu64) }
        0x102d114c => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1150u64) }
        0x102d1150 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1154u64) }
        0x102d1154 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1158u64) }
        0x102d1158 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d115cu64) }
        0x102d115c => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1160u64) }
        0x102d1160 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1164u64) }
        0x102d1164 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1168u64) }
        0x102d1168 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d116cu64) }
        0x102d116c => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1170u64) }
        0x102d1170 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1174u64).wrapping_add((imm * 4) as u64) } else { 0x102d1174u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1174u64) }
        0x102d1174 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1178u64) }
        0x102d1178 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d117cu64) }
        0x102d117c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1180u64) }
        0x102d1180 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1184u64) }
        0x102d1184 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1188u64) }
        0x102d1188 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d118cu64).wrapping_add((imm * 4) as u64) } else { 0x102d118cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d118cu64) }
        0x102d118c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1190u64) } }
        0x102d1190 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1194u64) }
        0x102d1194 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1198u64) }
        0x102d1198 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d119cu64).wrapping_add((imm * 4) as u64) } else { 0x102d119cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d119cu64) }
        0x102d119c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d11a0u64) }
        0x102d11a0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d11a4u64) }
        0x102d11a4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d11a8u64) }
        0x102d11a8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d11acu64) }
        0x102d11ac => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d11b0u64) }
        0x102d11b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d11b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d11b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d11b4u64) }
        0x102d11b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d11b8u64) } }
        0x102d11b8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d11bcu64) }
        0x102d11bc => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d11c0u64) }
        0x102d11c0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d11c4u64) }
        0x102d11c4 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d11c8u64) }
        0x102d11c8 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d11ccu64) }
        0x102d11cc => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d11d0u64) }
        0x102d11d0 => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d11d4u64) }
        0x102d11d4 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d11d8u64) }
        0x102d11d8 => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d11dcu64) }
        0x102d11dc => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d11e0u64) }
        0x102d11e0 => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d11e4u64) }
        0x102d11e4 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d11e8u64) }
        0x102d11e8 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d11ecu64) }
        0x102d11ec => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d11f0u64) }
        0x102d11f0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d11f4u64) }
        0x102d11f4 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d11f8u64) }
        0x102d11f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d11fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d11fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d11fcu64) }
        0x102d11fc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1200u64) }
        0x102d1200 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1204u64) }
        0x102d1204 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1208u64) }
        0x102d1208 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d120cu64) }
        0x102d120c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1210u64) }
        0x102d1210 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1214u64).wrapping_add((imm * 4) as u64) } else { 0x102d1214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1214u64) }
        0x102d1214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1218u64) } }
        0x102d1218 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d121cu64) }
        0x102d121c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1220u64) }
        0x102d1220 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1224u64).wrapping_add((imm * 4) as u64) } else { 0x102d1224u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1224u64) }
        0x102d1224 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1228u64) }
        0x102d1228 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d122cu64) }
        0x102d122c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1230u64) }
        0x102d1230 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1234u64) }
        0x102d1234 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1238u64) }
        0x102d1238 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d123cu64).wrapping_add((imm * 4) as u64) } else { 0x102d123cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d123cu64) }
        0x102d123c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1240u64) } }
        0x102d1240 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1244u64) }
        0x102d1244 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1248u64) }
        0x102d1248 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d124cu64) }
        0x102d124c => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1250u64) }
        0x102d1250 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1254u64) }
        0x102d1254 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1258u64) }
        0x102d1258 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d125cu64) }
        0x102d125c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1260u64) }
        0x102d1260 => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1264u64) }
        0x102d1264 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1268u64) }
        0x102d1268 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d126cu64) }
        0x102d126c => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1270u64) }
        0x102d1270 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1274u64) }
        0x102d1274 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1278u64) }
        0x102d1278 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d127cu64).wrapping_add((imm * 4) as u64) } else { 0x102d127cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d127cu64) }
        0x102d127c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1280u64) }
        0x102d1280 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1284u64) }
        0x102d1284 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1288u64) }
        0x102d1288 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d128cu64) }
        0x102d128c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1290u64) }
        0x102d1290 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1294u64).wrapping_add((imm * 4) as u64) } else { 0x102d1294u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1294u64) }
        0x102d1294 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1298u64) } }
        0x102d1298 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d129cu64) }
        0x102d129c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d12a0u64) }
        0x102d12a0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d12a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d12a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d12a4u64) }
        0x102d12a4 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d12a8u64) }
        0x102d12a8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d12acu64) }
        0x102d12ac => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d12b0u64) }
        0x102d12b0 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d12b4u64) }
        0x102d12b4 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d12b8u64) }
        0x102d12b8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d12bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d12bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d12bcu64) }
        0x102d12bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d12c0u64) } }
        0x102d12c0 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d12c4u64) }
        0x102d12c4 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d12c8u64) }
        0x102d12c8 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d12ccu64) }
        0x102d12cc => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d12d0u64) }
        0x102d12d0 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d12d4u64) }
        0x102d12d4 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d12d8u64) }
        0x102d12d8 => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d12dcu64) }
        0x102d12dc => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d12e0u64) }
        0x102d12e0 => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d12e4u64) }
        0x102d12e4 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d12e8u64) }
        0x102d12e8 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d12ecu64) }
        0x102d12ec => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d12f0u64) }
        0x102d12f0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d12f4u64) }
        0x102d12f4 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d12f8u64) }
        0x102d12f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d12fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d12fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d12fcu64) }
        0x102d12fc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1300u64) }
        0x102d1300 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1304u64) }
        0x102d1304 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1308u64) }
        0x102d1308 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d130cu64) }
        0x102d130c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1310u64) }
        0x102d1310 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1314u64).wrapping_add((imm * 4) as u64) } else { 0x102d1314u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1314u64) }
        0x102d1314 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1318u64) } }
        0x102d1318 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d131cu64) }
        0x102d131c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1320u64) }
        0x102d1320 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1324u64).wrapping_add((imm * 4) as u64) } else { 0x102d1324u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1324u64) }
        0x102d1324 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1328u64) }
        0x102d1328 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d132cu64) }
        0x102d132c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1330u64) }
        0x102d1330 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1334u64) }
        0x102d1334 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1338u64) }
        0x102d1338 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d133cu64).wrapping_add((imm * 4) as u64) } else { 0x102d133cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d133cu64) }
        0x102d133c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1340u64) } }
        0x102d1340 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1344u64) }
        0x102d1344 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1348u64) }
        0x102d1348 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d134cu64) }
        0x102d134c => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1350u64) }
        0x102d1350 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1354u64) }
        0x102d1354 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1358u64) }
        0x102d1358 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d135cu64) }
        0x102d135c => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1360u64) }
        0x102d1360 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1364u64) }
        0x102d1364 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1368u64) }
        0x102d1368 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d136cu64) }
        0x102d136c => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1370u64) }
        0x102d1370 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1374u64).wrapping_add((imm * 4) as u64) } else { 0x102d1374u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1374u64) }
        0x102d1374 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1378u64) }
        0x102d1378 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d137cu64) }
        0x102d137c => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1380u64) }
        0x102d1380 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1384u64) }
        0x102d1384 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1388u64) }
        0x102d1388 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d138cu64).wrapping_add((imm * 4) as u64) } else { 0x102d138cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d138cu64) }
        0x102d138c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1390u64) } }
        0x102d1390 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1394u64) }
        0x102d1394 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1398u64) }
        0x102d1398 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d139cu64).wrapping_add((imm * 4) as u64) } else { 0x102d139cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d139cu64) }
        0x102d139c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d13a0u64) }
        0x102d13a0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d13a4u64) }
        0x102d13a4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d13a8u64) }
        0x102d13a8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d13acu64) }
        0x102d13ac => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d13b0u64) }
        0x102d13b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d13b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d13b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d13b4u64) }
        0x102d13b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d13b8u64) } }
        0x102d13b8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d13bcu64) }
        0x102d13bc => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d13c0u64) }
        0x102d13c0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d13c4u64) }
        0x102d13c4 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d13c8u64) }
        0x102d13c8 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d13ccu64) }
        0x102d13cc => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d13d0u64) }
        0x102d13d0 => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d13d4u64) }
        0x102d13d4 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d13d8u64) }
        0x102d13d8 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d13dcu64) }
        0x102d13dc => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d13e0u64) }
        0x102d13e0 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d13e4u64) }
        0x102d13e4 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d13e8u64) }
        0x102d13e8 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d13ecu64) }
        0x102d13ec => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d13f0u64) }
        0x102d13f0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d13f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d13f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d13f4u64) }
        0x102d13f4 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d13f8u64) }
        0x102d13f8 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d13fcu64) }
        0x102d13fc => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1400u64) }
        0x102d1400 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1404u64) }
        0x102d1404 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1408u64) }
        0x102d1408 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d140cu64).wrapping_add((imm * 4) as u64) } else { 0x102d140cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d140cu64) }
        0x102d140c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1410u64) } }
        0x102d1410 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1414u64) }
        0x102d1414 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1418u64) }
        0x102d1418 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d141cu64).wrapping_add((imm * 4) as u64) } else { 0x102d141cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d141cu64) }
        0x102d141c => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1420u64) }
        0x102d1420 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1424u64) }
        0x102d1424 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1428u64) }
        0x102d1428 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d142cu64) }
        0x102d142c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1430u64) }
        0x102d1430 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1434u64).wrapping_add((imm * 4) as u64) } else { 0x102d1434u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1434u64) }
        0x102d1434 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1438u64) } }
        0x102d1438 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d143cu64) }
        0x102d143c => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1440u64) }
        0x102d1440 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1444u64) }
        0x102d1444 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1448u64) }
        0x102d1448 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d144cu64) }
        0x102d144c => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1450u64) }
        0x102d1450 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1454u64) }
        0x102d1454 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1458u64) }
        0x102d1458 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d145cu64) }
        0x102d145c => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1460u64) }
        0x102d1460 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1464u64) }
        0x102d1464 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1468u64) }
        0x102d1468 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d146cu64).wrapping_add((imm * 4) as u64) } else { 0x102d146cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d146cu64) }
        0x102d146c => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1470u64) }
        0x102d1470 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1474u64) }
        0x102d1474 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1478u64) }
        0x102d1478 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d147cu64) }
        0x102d147c => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1480u64) }
        0x102d1480 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1484u64).wrapping_add((imm * 4) as u64) } else { 0x102d1484u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1484u64) }
        0x102d1484 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1488u64) } }
        0x102d1488 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d148cu64) }
        0x102d148c => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1490u64) }
        0x102d1490 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1494u64).wrapping_add((imm * 4) as u64) } else { 0x102d1494u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1494u64) }
        0x102d1494 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1498u64) }
        0x102d1498 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d149cu64) }
        0x102d149c => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d14a0u64) }
        0x102d14a0 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d14a4u64) }
        0x102d14a4 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d14a8u64) }
        0x102d14a8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d14acu64).wrapping_add((imm * 4) as u64) } else { 0x102d14acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d14acu64) }
        0x102d14ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d14b0u64) } }
        0x102d14b0 => { vm.decode(0x8504141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5552i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d14b4u64) }
        0x102d14b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d14b8u64) } }
        0x102d14b8 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d14bcu64) }
        0x102d14bc => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d14c0u64) }
        0x102d14c0 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d14c4u64) }
        0x102d14c4 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d14c8u64) }
        0x102d14c8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d14ccu64) }
        0x102d14cc => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d14d0u64) }
        0x102d14d0 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d14d4u64) }
        0x102d14d4 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d14d8u64) }
        0x102d14d8 => { vm.decode(0x42804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d14dcu64) }
        0x102d14dc => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d14e0u64) }
        0x102d14e0 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d14e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d14e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d14e4u64) }
        0x102d14e4 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d14e8u64) }
        0x102d14f0 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d14f4u64) }
        0x102d14f4 => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d14f8u64) }
        0x102d14f8 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d14fcu64) }
        0x102d14fc => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1500u64) }
        0x102d1500 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1504u64) }
        0x102d1504 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1508u64) }
        0x102d1508 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d150cu64) }
        0x102d150c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1510u64) }
        0x102d1510 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1514u64) }
        0x102d1514 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1518u64) }
        0x102d1518 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d151cu64).wrapping_add((imm * 4) as u64) } else { 0x102d151cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d151cu64) }
        0x102d151c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1520u64) } }
        0x102d1520 => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d1524u64) }
        0x102d1524 => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1528u64) }
        0x102d1528 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d152cu64) }
        0x102d152c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1530u64) }
        0x102d1530 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1534u64) }
        0x102d1534 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1538u64) }
        0x102d1538 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d153cu64).wrapping_add((imm * 4) as u64) } else { 0x102d153cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d153cu64) }
        0x102d153c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1540u64) }
        0x102d1540 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1544u64) }
        0x102d1544 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1548u64) }
        0x102d1548 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d154cu64) }
        0x102d154c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1550u64) }
        0x102d1550 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1554u64).wrapping_add((imm * 4) as u64) } else { 0x102d1554u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1554u64) }
        0x102d1554 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1558u64) } }
        0x102d1558 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d155cu64) }
        0x102d155c => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1560u64) }
        0x102d1560 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1564u64).wrapping_add((imm * 4) as u64) } else { 0x102d1564u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1564u64) }
        0x102d1564 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1568u64) }
        0x102d1568 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d156cu64) }
        0x102d156c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1570u64) }
        0x102d1570 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1574u64) }
        0x102d1574 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1578u64) }
        0x102d1578 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d157cu64).wrapping_add((imm * 4) as u64) } else { 0x102d157cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d157cu64) }
        0x102d157c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1580u64) } }
        0x102d1580 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1584u64) }
        0x102d1584 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1588u64) }
        0x102d1588 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d158cu64) }
        0x102d158c => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1590u64) }
        0x102d1590 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1594u64) }
        0x102d1594 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1598u64) }
        0x102d1598 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d159cu64) }
        0x102d159c => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d15a0u64) }
        0x102d15a0 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d15a4u64) }
        0x102d15a4 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d15a8u64) }
        0x102d15a8 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d15acu64) }
        0x102d15ac => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d15b0u64) }
        0x102d15b0 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d15b4u64) }
        0x102d15b4 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d15b8u64) }
        0x102d15b8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d15bcu64) }
        0x102d15bc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d15c0u64) }
        0x102d15c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d15c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d15c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d15c4u64) }
        0x102d15c4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d15c8u64) }
        0x102d15c8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d15ccu64) }
        0x102d15cc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d15d0u64) }
        0x102d15d0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d15d4u64) }
        0x102d15d4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d15d8u64) }
        0x102d15d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d15dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d15dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d15dcu64) }
        0x102d15dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d15e0u64) } }
        0x102d15e0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d15e4u64) }
        0x102d15e4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d15e8u64) }
        0x102d15e8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d15ecu64).wrapping_add((imm * 4) as u64) } else { 0x102d15ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d15ecu64) }
        0x102d15ec => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d15f0u64) }
        0x102d15f0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d15f4u64) }
        0x102d15f4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d15f8u64) }
        0x102d15f8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d15fcu64) }
        0x102d15fc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1600u64) }
        0x102d1600 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1604u64).wrapping_add((imm * 4) as u64) } else { 0x102d1604u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1604u64) }
        0x102d1604 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1608u64) } }
        0x102d1608 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d160cu64) }
        0x102d160c => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1610u64) }
        0x102d1610 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1614u64) }
        0x102d1614 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1618u64) }
        0x102d1618 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d161cu64) }
        0x102d161c => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1620u64) }
        0x102d1620 => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1624u64) }
        0x102d1624 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1628u64) }
        0x102d1628 => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d162cu64) }
        0x102d162c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1630u64) }
        0x102d1630 => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1634u64) }
        0x102d1634 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1638u64) }
        0x102d1638 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d163cu64) }
        0x102d163c => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1640u64) }
        0x102d1640 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1644u64) }
        0x102d1644 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1648u64) }
        0x102d1648 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d164cu64).wrapping_add((imm * 4) as u64) } else { 0x102d164cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d164cu64) }
        0x102d164c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1650u64) }
        0x102d1650 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1654u64) }
        0x102d1654 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1658u64) }
        0x102d1658 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d165cu64) }
        0x102d165c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1660u64) }
        0x102d1660 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1664u64).wrapping_add((imm * 4) as u64) } else { 0x102d1664u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1664u64) }
        0x102d1664 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1668u64) } }
        0x102d1668 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d166cu64) }
        0x102d166c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1670u64) }
        0x102d1670 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1674u64).wrapping_add((imm * 4) as u64) } else { 0x102d1674u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1674u64) }
        0x102d1674 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1678u64) }
        0x102d1678 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d167cu64) }
        0x102d167c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1680u64) }
        0x102d1680 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1684u64) }
        0x102d1684 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1688u64) }
        0x102d1688 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d168cu64).wrapping_add((imm * 4) as u64) } else { 0x102d168cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d168cu64) }
        0x102d168c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1690u64) } }
        0x102d1690 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1694u64) }
        0x102d1694 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1698u64) }
        0x102d1698 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d169cu64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
