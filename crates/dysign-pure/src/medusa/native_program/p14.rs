#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102bc59c => { vm.decode(0x110229b5u32); let d = 1i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc5a0u64) }
        0x102bc5a0 => { vm.decode(0x1d98f811u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5a4u64) }
        0x102bc5a4 => { vm.decode(0x4388875u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc5a8u64) }
        0x102bc5a8 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc5acu64) }
        0x102bc5ac => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5b0u64) }
        0x102bc5b0 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5b4u64) }
        0x102bc5b4 => { vm.decode(0x45906a51u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5b8u64) }
        0x102bc5b8 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5bcu64) }
        0x102bc5bc => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc5c0u64) }
        0x102bc5c0 => { vm.decode(0x40190fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5c4u64) }
        0x102bc5c4 => { vm.decode(0x2a88c36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc5c8u64) }
        0x102bc5c8 => { vm.decode(0x48320b75u32); let d = 20i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc5ccu64) }
        0x102bc5cc => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc5d0u64) }
        0x102bc5d0 => { vm.decode(0xca0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc5d4u64) }
        0x102bc5d4 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc5d8u64) }
        0x102bc5d8 => { vm.decode(0x8229b5u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc5dcu64) }
        0x102bc5dc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc5e0u64) }
        0x102bc5e0 => { vm.decode(0x62255022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc5e4u64) }
        0x102bc5e4 => { vm.decode(0x4d90eb51u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5e8u64) }
        0x102bc5e8 => { vm.decode(0x590fad1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1884i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5ecu64) }
        0x102bc5ec => { vm.decode(0xd947251u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5f0u64) }
        0x102bc5f0 => { vm.decode(0xe114d35u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5f4u64) }
        0x102bc5f4 => { vm.decode(0x411129b5u32); let d = 10i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc5f8u64) }
        0x102bc5f8 => { vm.decode(0x45906bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1876i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc5fcu64) }
        0x102bc5fc => { vm.decode(0xd18e2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1932i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc600u64) }
        0x102bc600 => { vm.decode(0x91230fb5u32); let d = 12i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc604u64) }
        0x102bc604 => { vm.decode(0x4518e251u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc608u64) }
        0x102bc608 => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc60cu64) }
        0x102bc60c => { vm.decode(0x10a04875u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc610u64) }
        0x102bc610 => { vm.decode(0x2112a835u32); let d = 5i16; let s = 4i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc614u64) }
        0x102bc614 => { vm.decode(0x51863d1u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1924i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc618u64) }
        0x102bc618 => { vm.decode(0x4d9c7811u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc61cu64) }
        0x102bc61c => { vm.decode(0x75112fb5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc620u64) }
        0x102bc620 => { vm.decode(0xd186351u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc624u64) }
        0x102bc624 => { vm.decode(0x65a06fb5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc628u64) }
        0x102bc628 => { vm.decode(0x4594ebd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc62cu64) }
        0x102bc62c => { vm.decode(0x75292fb5u32); let d = 11i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc630u64) }
        0x102bc630 => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc634u64) }
        0x102bc634 => { vm.decode(0x55a24fb5u32); let d = 13i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc638u64) }
        0x102bc638 => { vm.decode(0x47604875u32); let d = 10i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc63cu64) }
        0x102bc63c => { vm.decode(0x120f2835u32); let d = 11i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc640u64) }
        0x102bc640 => { vm.decode(0x4594fa51u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc644u64) }
        0x102bc644 => { vm.decode(0x459cf911u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc648u64) }
        0x102bc648 => { vm.decode(0x35a26fb5u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc64cu64) }
        0x102bc64c => { vm.decode(0x4d947ad1u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1908i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc650u64) }
        0x102bc650 => { vm.decode(0x60b36fb5u32); let d = 7i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc654u64) }
        0x102bc654 => { vm.decode(0x409119b5u32); let d = 14i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc658u64) }
        0x102bc658 => { vm.decode(0x60ab2fb5u32); let d = 7i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc65cu64) }
        0x102bc65c => { vm.decode(0x3835cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc660u64) }
        0x102bc660 => { vm.decode(0x75b30fb5u32); let d = 15i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc664u64) }
        0x102bc664 => { vm.decode(0x55e0d875u32); let d = 7i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc668u64) }
        0x102bc668 => { vm.decode(0x610fa835u32); let d = 7i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc66cu64) }
        0x102bc66c => { vm.decode(0x45947b51u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc670u64) }
        0x102bc670 => { vm.decode(0x15105211u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc674u64) }
        0x102bc674 => { vm.decode(0xf4320fb5u32); let d = 7i16; let s = 7i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc678u64) }
        0x102bc678 => { vm.decode(0x1594f2d1u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1900i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc67cu64) }
        0x102bc67c => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc680u64) }
        0x102bc680 => { vm.decode(0x411019b5u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc684u64) }
        0x102bc684 => { vm.decode(0x648b6fb5u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc688u64) }
        0x102bc688 => { vm.decode(0x3e34cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc68cu64) }
        0x102bc68c => { vm.decode(0x55b52fb5u32); let d = 25i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc690u64) }
        0x102bc690 => { vm.decode(0x45b08875u32); let d = 7i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc694u64) }
        0x102bc694 => { vm.decode(0x614fa835u32); let d = 7i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc698u64) }
        0x102bc698 => { vm.decode(0x1594e251u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc69cu64) }
        0x102bc69c => { vm.decode(0x5105a11u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6a0u64) }
        0x102bc6a0 => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6a4u64) }
        0x102bc6a4 => { vm.decode(0x59472d1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1892i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6a8u64) }
        0x102bc6a8 => { vm.decode(0x74330fb5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6acu64) }
        0x102bc6ac => { vm.decode(0x511219b5u32); let d = 9i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc6b0u64) }
        0x102bc6b0 => { vm.decode(0x60936fb5u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6b4u64) }
        0x102bc6b4 => { vm.decode(0x3e354d35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6b8u64) }
        0x102bc6b8 => { vm.decode(0x95330fb5u32); let d = 12i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6bcu64) }
        0x102bc6bc => { vm.decode(0x41a8c875u32); let d = 7i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc6c0u64) }
        0x102bc6c0 => { vm.decode(0x611ea835u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc6c4u64) }
        0x102bc6c4 => { vm.decode(0xd10fa91u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6c8u64) }
        0x102bc6c8 => { vm.decode(0x1d9c6011u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6ccu64) }
        0x102bc6cc => { vm.decode(0xc4350fb5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6d0u64) }
        0x102bc6d0 => { vm.decode(0x4d10eb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6d4u64) }
        0x102bc6d4 => { vm.decode(0x15080fb5u32); let d = 8i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6d8u64) }
        0x102bc6d8 => { vm.decode(0x1110a9b5u32); let d = 17i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc6dcu64) }
        0x102bc6dc => { vm.decode(0x15104fb5u32); let d = 8i16; let s = 8i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6e0u64) }
        0x102bc6e0 => { vm.decode(0xc813d35u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6e4u64) }
        0x102bc6e4 => { vm.decode(0x51824fb5u32); let d = 13i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6e8u64) }
        0x102bc6e8 => { vm.decode(0x6286875u32); let d = 8i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc6ecu64) }
        0x102bc6ec => { vm.decode(0x14032835u32); let d = 17i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc6f0u64) }
        0x102bc6f0 => { vm.decode(0x5107a91u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6f4u64) }
        0x102bc6f4 => { vm.decode(0x5510ca11u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6f8u64) }
        0x102bc6f8 => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc6fcu64) }
        0x102bc6fc => { vm.decode(0x1199b5u32); let d = 18i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc700u64) }
        0x102bc700 => { vm.decode(0x840c0fb5u32); let d = 16i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc704u64) }
        0x102bc704 => { vm.decode(0xe5324fb5u32); let d = 15i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc708u64) }
        0x102bc708 => { vm.decode(0x3835fd35u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc70cu64) }
        0x102bc70c => { vm.decode(0x75b34fb5u32); let d = 15i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc710u64) }
        0x102bc710 => { vm.decode(0x11099b5u32); let d = 16i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc714u64) }
        0x102bc714 => { vm.decode(0xc60f875u32); let d = 17i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc718u64) }
        0x102bc718 => { vm.decode(0x64033835u32); let d = 17i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc71cu64) }
        0x102bc71c => { vm.decode(0x55144211u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc720u64) }
        0x102bc720 => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc724u64) }
        0x102bc724 => { vm.decode(0x950c0fb5u32); let d = 24i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc728u64) }
        0x102bc728 => { vm.decode(0x950c4fb5u32); let d = 24i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc72cu64) }
        0x102bc72c => { vm.decode(0x8f00bd35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc730u64) }
        0x102bc730 => { vm.decode(0x118d6fb5u32); let d = 24i16; let s = 15i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc734u64) }
        0x102bc734 => { vm.decode(0x511299b5u32); let d = 25i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc738u64) }
        0x102bc738 => { vm.decode(0x8302875u32); let d = 16i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc73cu64) }
        0x102bc73c => { vm.decode(0x24431835u32); let d = 16i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc740u64) }
        0x102bc740 => { vm.decode(0x800c2fb5u32); let d = 16i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc744u64) }
        0x102bc744 => { vm.decode(0x318a2fb5u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc748u64) }
        0x102bc748 => { vm.decode(0x11926fb5u32); let d = 12i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc74cu64) }
        0x102bc74c => { vm.decode(0xe253d35u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc750u64) }
        0x102bc750 => { vm.decode(0x91124fb5u32); let d = 12i16; let s = 12i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc754u64) }
        0x102bc754 => { vm.decode(0x409129b5u32); let d = 14i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc758u64) }
        0x102bc758 => { vm.decode(0xb28e875u32); let d = 25i16; let s = 12i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc75cu64) }
        0x102bc75c => { vm.decode(0x56123835u32); let d = 25i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc760u64) }
        0x102bc760 => { vm.decode(0x15144a11u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc764u64) }
        0x102bc764 => { vm.decode(0xc50c4fb5u32); let d = 25i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc768u64) }
        0x102bc768 => { vm.decode(0x75096fb5u32); let d = 11i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc76cu64) }
        0x102bc76c => { vm.decode(0x75116fb5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc770u64) }
        0x102bc770 => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc774u64) }
        0x102bc774 => { vm.decode(0x75a04fb5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc778u64) }
        0x102bc778 => { vm.decode(0x509019b5u32); let d = 13i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc77cu64) }
        0x102bc77c => { vm.decode(0x57e01875u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc780u64) }
        0x102bc780 => { vm.decode(0x130f8835u32); let d = 14i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc784u64) }
        0x102bc784 => { vm.decode(0x1d14d211u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc788u64) }
        0x102bc788 => { vm.decode(0xb1334fb5u32); let d = 14i16; let s = 14i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc78cu64) }
        0x102bc78c => { vm.decode(0x31306fb5u32); let d = 10i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc790u64) }
        0x102bc790 => { vm.decode(0x35306fb5u32); let d = 10i16; let s = 10i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc794u64) }
        0x102bc794 => { vm.decode(0x2811bd35u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc798u64) }
        0x102bc798 => { vm.decode(0x31216fb5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc79cu64) }
        0x102bc79c => { vm.decode(0x419309b5u32); let d = 14i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc7a0u64) }
        0x102bc7a0 => { vm.decode(0x13a89875u32); let d = 13i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7a4u64) }
        0x102bc7a4 => { vm.decode(0x7342a835u32); let d = 13i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7a8u64) }
        0x102bc7a8 => { vm.decode(0x4d14db11u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7acu64) }
        0x102bc7ac => { vm.decode(0x55936fb5u32); let d = 13i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7b0u64) }
        0x102bc7b0 => { vm.decode(0x24926fb5u32); let d = 6i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7b4u64) }
        0x102bc7b4 => { vm.decode(0x208a6fb5u32); let d = 6i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7b8u64) }
        0x102bc7b8 => { vm.decode(0x2e348d35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7bcu64) }
        0x102bc7bc => { vm.decode(0x30322fb5u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7c0u64) }
        0x102bc7c0 => { vm.decode(0x509209b5u32); let d = 13i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc7c4u64) }
        0x102bc7c4 => { vm.decode(0x53c05875u32); let d = 14i16; let s = 6i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7c8u64) }
        0x102bc7c8 => { vm.decode(0x134e8835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7ccu64) }
        0x102bc7cc => { vm.decode(0x4d905311u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7d0u64) }
        0x102bc7d0 => { vm.decode(0x31b36fb5u32); let d = 14i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7d4u64) }
        0x102bc7d4 => { vm.decode(0x40b34fb5u32); let d = 5i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7d8u64) }
        0x102bc7d8 => { vm.decode(0x40934fb5u32); let d = 5i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7dcu64) }
        0x102bc7dc => { vm.decode(0x1e254d35u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7e0u64) }
        0x102bc7e0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7e4u64) }
        0x102bc7e4 => { vm.decode(0x408219b5u32); let d = 12i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc7e8u64) }
        0x102bc7e8 => { vm.decode(0x1788c875u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7ecu64) }
        0x102bc7ec => { vm.decode(0x5341a835u32); let d = 13i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc7f0u64) }
        0x102bc7f0 => { vm.decode(0x4590d311u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7f4u64) }
        0x102bc7f4 => { vm.decode(0x55926fb5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7f8u64) }
        0x102bc7f8 => { vm.decode(0x4924fb5u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc7fcu64) }
        0x102bc7fc => { vm.decode(0x4a24fb5u32); let d = 4i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc800u64) }
        0x102bc800 => { vm.decode(0xca50d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc804u64) }
        0x102bc804 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc808u64) }
        0x102bc808 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc80cu64) }
        0x102bc80c => { vm.decode(0x510119b5u32); let d = 11i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc810u64) }
        0x102bc810 => { vm.decode(0x13804875u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc814u64) }
        0x102bc814 => { vm.decode(0x45804fb5u32); let d = 9i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc818u64) }
        0x102bc818 => { vm.decode(0x30d7062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc81cu64) }
        0x102bc81c => { vm.decode(0x3009a835u32); let d = 3i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc820u64) }
        0x102bc820 => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc824u64) }
        0x102bc824 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc828u64) }
        0x102bc828 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc82cu64) }
        0x102bc82c => { vm.decode(0x20206fb5u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc830u64) }
        0x102bc830 => { vm.decode(0x28118d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc834u64) }
        0x102bc834 => { vm.decode(0x20a00fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc838u64) }
        0x102bc838 => { vm.decode(0x41489875u32); let d = 3i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc83cu64) }
        0x102bc83c => { vm.decode(0x31f3462u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc840u64) }
        0x102bc840 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc844u64) }
        0x102bc844 => { vm.decode(0x4322fb5u32); let d = 4i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc848u64) }
        0x102bc848 => { vm.decode(0x25d3062u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc84cu64) }
        0x102bc84c => { vm.decode(0x5944311u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc850u64) }
        0x102bc850 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc854u64) }
        0x102bc854 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc858u64) }
        0x102bc858 => { vm.decode(0x40310fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc85cu64) }
        0x102bc85c => { vm.decode(0x1e00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc860u64) }
        0x102bc860 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc864u64) }
        0x102bc864 => { vm.decode(0x40014fb5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc868u64) }
        0x102bc868 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc86cu64) }
        0x102bc86c => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc870u64) }
        0x102bc870 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc874u64) }
        0x102bc874 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc878u64) }
        0x102bc878 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc87cu64) }
        0x102bc87c => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc880u64) }
        0x102bc880 => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc884u64) }
        0x102bc884 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc888u64) }
        0x102bc888 => { vm.decode(0x4422a7d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8133i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc88cu64) }
        0x102bc88c => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc890u64) }
        0x102bc890 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc894u64) }
        0x102bc894 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc898u64) }
        0x102bc898 => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc89cu64) }
        0x102bc89c => { vm.decode(0x22028000u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102bc8a0u64) }
        0x102bc8a0 => { vm.decode(0x40c82a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 185i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc8a4u64).wrapping_add((imm * 4) as u64) } else { 0x102bc8a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc8a4u64) }
        0x102bc8a4 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc8a8u64) }
        0x102bc8a8 => { vm.decode(0x268c911du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 227076i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bc8acu64) }
        0x102bc8ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc8b0u64) } }
        0x102bc8b0 => { vm.decode(0x100039b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc8b4u64) }
        0x102bda84 => { vm.decode(0xb8027u32); let d = 0i16; let s = 20i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bda88u64).wrapping_add((imm * 4) as u64) } else { 0x102bda88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bda88u64) }
        0x102bda88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bda8cu64) } }
        0x102bda8c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bda90u64) }
        0x102bdacc => { vm.decode(0x4a8063afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdad0u64) }
        0x102bdad0 => { vm.decode(0x2804402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdad4u64) }
        0x102bdad4 => { vm.decode(0x10408019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdad8u64) }
        0x102bdad8 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdadcu64) }
        0x102bdadc => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdae0u64) }
        0x102bdae0 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdae4u64) }
        0x102bdae4 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdae8u64) }
        0x102bdae8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdaecu64) }
        0x102bdaec => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdaf0u64) }
        0x102bdaf0 => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdaf4u64) }
        0x102bdaf4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdaf8u64) }
        0x102bdaf8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bdafcu64 + 4);
        vm.wr(37, 0x102bdafcu64 + 4);
        Ok(0x102bdafcu64) }
        0x102bdafc => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdb00u64) }
        0x102bdb00 => { vm.decode(0x2200406fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb04u64) }
        0x102bdb04 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdb08u64) }
        0x102bdb08 => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb0cu64) }
        0x102bdb0c => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdb10u64) }
        0x102bdb10 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb14u64) }
        0x102bdb14 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb18u64) }
        0x102bdb18 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb1cu64) }
        0x102bdb1c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdb20u64) }
        0x102bdb20 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bdb24u64 + 4);
        vm.wr(37, 0x102bdb24u64 + 4);
        Ok(0x102bdb24u64) }
        0x102bdb24 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdb28u64) }
        0x102bdb28 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb2cu64) }
        0x102bdb2c => { vm.decode(0xd4268759u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 8138i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdb30u64) }
        0x102bdb30 => { vm.decode(0x10101019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdb34u64) }
        0x102bdb34 => { vm.decode(0x740d9419u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdb38u64) }
        0x102bdb38 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bdb3cu64) }
        0x102bdb3c => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bdb40u64).wrapping_add((imm * 4) as u64) } else { 0x102bdb40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdb40u64) }
        0x102bdb40 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdb44u64) }
        0x102bdb44 => { vm.decode(0x830cc11du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 227488i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bdb48u64) }
        0x102bdb4c => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdb50u64) }
        0x102bdb50 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bdb54u64) }
        0x102bdb54 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdb58u64) }
        0x102bdb58 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdb5cu64) }
        0x102bdb5c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdb60u64) }
        0x102bdb60 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdb64u64) }
        0x102bdb64 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bdb68u64) }
        0x102bdb68 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bdb6cu64) }
        0x102bdb6c => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdb70u64) }
        0x102bdb70 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb74u64) }
        0x102bdb74 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bdb78u64) }
        0x102bdb78 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bdb7cu64).wrapping_add((imm * 4) as u64) } else { 0x102bdb7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdb7cu64) }
        0x102bdb7c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdb80u64) } }
        0x102bdb80 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb84u64) }
        0x102bdb84 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bdb88u64) }
        0x102bdb88 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb8cu64) }
        0x102bdb8c => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdb90u64) }
        0x102bdb90 => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb94u64) }
        0x102bdb94 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb98u64) }
        0x102bdb98 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bdb9cu64) }
        0x102bdb9c => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdba0u64) }
        0x102bdba0 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdba4u64) }
        0x102bdba4 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bdba8u64) }
        0x102bdba8 => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bdbacu64) }
        0x102bdbac => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdbb0u64) }
        0x102bdbb0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdbb4u64) }
        0x102bdbb4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdbb8u64) }
        0x102bdbb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bdbbcu64 + 4);
        vm.wr(37, 0x102bdbbcu64 + 4);
        Ok(0x102bdbbcu64) }
        0x102bdbbc => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdbc0u64) }
        0x102bdbc0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdbc4u64) }
        0x102bdbc4 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdbc8u64) }
        0x102bdbc8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdbccu64) }
        0x102bdbcc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bdbd0u64) }
        0x102bdbd0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bdbd4u64).wrapping_add((imm * 4) as u64) } else { 0x102bdbd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdbd4u64) }
        0x102bdbd4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdbd8u64) }
        0x102bdbd8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdbdcu64) }
        0x102bdbdc => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdbe0u64) }
        0x102bdbe0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bdbe4u64) }
        0x102bdbe4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bdbe8u64) }
        0x102bdbe8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bdbecu64).wrapping_add((imm * 4) as u64) } else { 0x102bdbecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdbecu64) }
        0x102bdbec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdbf0u64) } }
        0x102bdbf0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdbf4u64) }
        0x102bdbf4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bdbf8u64) }
        0x102bdbf8 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bdbfcu64).wrapping_add((imm * 4) as u64) } else { 0x102bdbfcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdbfcu64) }
        0x102bdbfc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdc00u64) }
        0x102bdc00 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bdc04u64) }
        0x102bdc04 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc08u64) }
        0x102bdc08 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bdc0cu64) }
        0x102bdc0c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bdc10u64) }
        0x102bdc10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bdc14u64).wrapping_add((imm * 4) as u64) } else { 0x102bdc14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdc14u64) }
        0x102bdc14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdc18u64) } }
        0x102bdc18 => { vm.decode(0xaeac911du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 227276i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bdc1cu64) }
        0x102bdc1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdc20u64) } }
        0x102bdc20 => { vm.decode(0xd10e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc24u64) }
        0x102bdc24 => { vm.decode(0x45106a91u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc28u64) }
        0x102bdc28 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdc2cu64) }
        0x102bdc2c => { vm.decode(0x4510e051u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc30u64) }
        0x102bdc30 => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc34u64) }
        0x102bdc34 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdc38u64) }
        0x102bdc38 => { vm.decode(0x4d106851u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc3cu64) }
        0x102bdc3c => { vm.decode(0x510eb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc40u64) }
        0x102bdc40 => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bdc44u64) }
        0x102bdc44 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bdc48u64) }
        0x102bdc48 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdc4cu64) }
        0x102bdc4c => { vm.decode(0x31d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bdc50u64) }
        0x102bdc50 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc54u64) }
        0x102bdc54 => { vm.decode(0x4d10ea91u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc58u64) }
        0x102bdc58 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bdc5cu64) }
        0x102bdc5c => { vm.decode(0x31d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bdc60u64) }
        0x102bdc60 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdc64u64) }
        0x102bdc64 => { vm.decode(0x20200833u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc68u64) }
        0x102bdc68 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc6cu64) }
        0x102bdc6c => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102bdc70u64) }
        0x102bdc70 => { vm.decode(0x80220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc74u64) }
        0x102bdc74 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdc78u64) }
        0x102bdc78 => { vm.decode(0xc2a08d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 786i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102bdc7cu64) }
        0x102bdc7c => { vm.decode(0x2208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc80u64) }
        0x102bdc80 => { vm.decode(0x714132beu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 787i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102bdc84u64) }
        0x102bdc84 => { vm.decode(0x802208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc88u64) }
        0x102bdc88 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc8cu64) }
        0x102bdc8c => { vm.decode(0x220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc90u64) }
        0x102bdc90 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdc94u64) }
        0x102bdc94 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102bdc98u64) }
        0x102bdc98 => { vm.decode(0x80220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdc9cu64) }
        0x102bdc9c => { vm.decode(0xc2a08da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 790i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102bdca0u64) }
        0x102bdca0 => { vm.decode(0x220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdca4u64) }
        0x102bdca4 => { vm.decode(0x714132feu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 791i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102bdca8u64) }
        0x102bdca8 => { vm.decode(0x80220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcacu64) }
        0x102bdcac => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdcb0u64) }
        0x102bdcb0 => { vm.decode(0x220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcb4u64) }
        0x102bdcb4 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdcb8u64) }
        0x102bdcb8 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102bdcbcu64) }
        0x102bdcbc => { vm.decode(0x80220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcc0u64) }
        0x102bdcc0 => { vm.decode(0xc2a18d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 794i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102bdcc4u64) }
        0x102bdcc4 => { vm.decode(0x2209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcc8u64) }
        0x102bdcc8 => { vm.decode(0x71413abeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 795i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102bdcccu64) }
        0x102bdccc => { vm.decode(0x802209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcd0u64) }
        0x102bdcd0 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdcd4u64) }
        0x102bdcd4 => { vm.decode(0x220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcd8u64) }
        0x102bdcd8 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bdcdcu64) }
        0x102bdcdc => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102bdce0u64) }
        0x102bdce0 => { vm.decode(0x80220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdce4u64) }
        0x102bdce4 => { vm.decode(0xc2a18da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 798i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102bdce8u64) }
        0x102bdce8 => { vm.decode(0x220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcecu64) }
        0x102bdcec => { vm.decode(0x71413afeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 799i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102bdcf0u64) }
        0x102bdcf0 => { vm.decode(0x80220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdcf4u64) }
        0x102bdcf4 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdcf8u64) }
        0x102bdcf8 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdcfcu64) }
        0x102bdcfc => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bdd00u64) }
        0x102bdd00 => { vm.decode(0x7ea8dca7u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 15351i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bdd04u64).wrapping_add((imm * 4) as u64) } else { 0x102bdd04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdd04u64) }
        0x102bdd04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdd08u64) } }
        0x102bdd08 => { vm.decode(0x404024f5u32); let d = 1i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bdd0cu64) }
        0x102bdd0c => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdd10u64) }
        0x102bdd10 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bdd14u64).wrapping_add((imm * 4) as u64) } else { 0x102bdd14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bdd14u64) }
        0x102bdd14 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bdd18u64) }
        0x102bdd18 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bdd1cu64) }
        0x102bdd1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bdd20u64) } }
        0x102bdd20 => { vm.decode(0x110229b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102bdd24u64) }
        0x102bdff8 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bdffcu64) }
        0x102bdffc => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102be000u64) }
        0x102be000 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102be004u64) }
        0x102be004 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102be008u64) }
        0x102be008 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102be00cu64) }
        0x102be00c => { vm.decode(0x405673d9u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 12797i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102be010u64) }
        0x102be010 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102be014u64) }
        0x102be014 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102be018u64) }
        0x102be018 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102be01cu64) }
        0x102be01c => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102be020u64) }
        0x102be020 => { vm.decode(0xa2028000u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102be024u64) }
        0x102be024 => { vm.decode(0x40c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 184i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102be028u64).wrapping_add((imm * 4) as u64) } else { 0x102be028u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102be028u64) }
        0x102be028 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102be02cu64) }
        0x102be02c => { vm.decode(0xa6c8401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 233092i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102be030u64) }
        0x102be030 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102be034u64) } }
        0x102bf204 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf208u64) }
        0x102bf208 => { vm.decode(0x80b0027u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf20cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf20cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf20cu64) }
        0x102bf20c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf210u64) } }
        0x102bf210 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf214u64) }
        0x102bf214 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf218u64) }
        0x102bf218 => { vm.decode(0x89215db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf21cu64) }
        0x102bf21c => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf220u64) }
        0x102bf220 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf224u64) }
        0x102bf224 => { vm.decode(0xd9c4011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf228u64) }
        0x102bf228 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bf22cu64) }
        0x102bf22c => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf230u64) }
        0x102bf230 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf234u64) }
        0x102bf234 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf238u64) }
        0x102bf238 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf23cu64) }
        0x102bf23c => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf240u64) }
        0x102bf240 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf244u64) }
        0x102bf244 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf248u64 + 4);
        vm.wr(37, 0x102bf248u64 + 4);
        Ok(0x102bf248u64) }
        0x102bf248 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf24cu64) }
        0x102bf24c => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf250u64) }
        0x102bf250 => { vm.decode(0x9236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf254u64) }
        0x102bf254 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf258u64) }
        0x102bf258 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102bf25cu64) }
        0x102bf25c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf260u64).wrapping_add((imm * 4) as u64) } else { 0x102bf260u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf260u64) }
        0x102bf260 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102bf264u64) }
        0x102bf270 => { vm.decode(0x12002019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf274u64) }
        0x102bf274 => { vm.decode(0x642d8159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf278u64) }
        0x102bf278 => { vm.decode(0x80c80a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 145i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf27cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf27cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf27cu64) }
        0x102bf27c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf280u64) }
        0x102bf280 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf284u64) }
        0x102bf284 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf288u64) }
        0x102bf288 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bf28cu64) }
        0x102bf28c => { vm.decode(0xa0002040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102bf290u64) }
        0x102bf290 => { vm.decode(0x8482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 141i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf294u64).wrapping_add((imm * 4) as u64) } else { 0x102bf294u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf294u64) }
        0x102bf294 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf298u64) } }
        0x102bf298 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf29cu64) }
        0x102bf29c => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf2a0u64) }
        0x102bf2a0 => { vm.decode(0x662d0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf2a4u64) }
        0x102bf2a4 => { vm.decode(0x9309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2a8u64) }
        0x102bf2a8 => { vm.decode(0x9211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2acu64) }
        0x102bf2ac => { vm.decode(0x928dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2b0u64) }
        0x102bf2b0 => { vm.decode(0x9395c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2b4u64) }
        0x102bf2b4 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2b8u64) }
        0x102bf2b8 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bf2bcu64) }
        0x102bf2bc => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf2c0u64) }
        0x102bf2c0 => { vm.decode(0xc41019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf2c4u64) }
        0x102bf2c4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf2c8u64) }
        0x102bf2c8 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf2ccu64) }
        0x102bf2cc => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf2d0u64) }
        0x102bf2d0 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf2d4u64) }
        0x102bf2d4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf2d8u64) }
        0x102bf2d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf2dcu64 + 4);
        vm.wr(37, 0x102bf2dcu64 + 4);
        Ok(0x102bf2dcu64) }
        0x102bf2dc => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf2e0u64) }
        0x102bf2e0 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf2e4u64) }
        0x102bf2e4 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf2e8u64) }
        0x102bf2e8 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf2ecu64) }
        0x102bf2ec => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf2f0u64) }
        0x102bf2f0 => { vm.decode(0x15106b91u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf2f4u64) }
        0x102bf2f4 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf2f8u64) }
        0x102bf2f8 => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf2fcu64) }
        0x102bf2fc => { vm.decode(0xd90ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf300u64) }
        0x102bf300 => { vm.decode(0x430d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf304u64) }
        0x102bf304 => { vm.decode(0xd90ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf308u64) }
        0x102bf308 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf30cu64) }
        0x102bf30c => { vm.decode(0xd906a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 852i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf310u64) }
        0x102bf310 => { vm.decode(0x63251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf314u64) }
        0x102bf314 => { vm.decode(0xd906a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf318u64) }
        0x102bf318 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf31cu64) }
        0x102bf31c => { vm.decode(0xd90e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 844i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf320u64) }
        0x102bf320 => { vm.decode(0x30d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf324u64) }
        0x102bf324 => { vm.decode(0xd90e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf328u64) }
        0x102bf328 => { vm.decode(0x630d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf32cu64) }
        0x102bf32c => { vm.decode(0xd906291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 836i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf330u64) }
        0x102bf330 => { vm.decode(0x230d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf334u64) }
        0x102bf334 => { vm.decode(0xd906211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf338u64) }
        0x102bf338 => { vm.decode(0x431d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf33cu64) }
        0x102bf33c => { vm.decode(0xd14ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 828i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf340u64) }
        0x102bf340 => { vm.decode(0x63353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf344u64) }
        0x102bf344 => { vm.decode(0xd14ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf348u64) }
        0x102bf348 => { vm.decode(0x431d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf34cu64) }
        0x102bf34c => { vm.decode(0xd146a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 820i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf350u64) }
        0x102bf350 => { vm.decode(0x231d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf354u64) }
        0x102bf354 => { vm.decode(0xd146a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf358u64) }
        0x102bf358 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf35cu64) }
        0x102bf35c => { vm.decode(0xd14e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 812i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf360u64) }
        0x102bf360 => { vm.decode(0x231d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf364u64) }
        0x102bf364 => { vm.decode(0xd14e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf368u64) }
        0x102bf368 => { vm.decode(0x230d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf36cu64) }
        0x102bf36c => { vm.decode(0xd146291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 804i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf370u64) }
        0x102bf370 => { vm.decode(0x30d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf374u64) }
        0x102bf374 => { vm.decode(0xd146211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf378u64) }
        0x102bf378 => { vm.decode(0x63351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf37cu64) }
        0x102bf37c => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf380u64) }
        0x102bf380 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf384u64) }
        0x102bf384 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf388u64) }
        0x102bf388 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf38cu64) }
        0x102bf38c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf390u64) }
        0x102bf390 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf394u64) }
        0x102bf394 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf398u64) }
        0x102bf398 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf39cu64) }
        0x102bf39c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf3a0u64) }
        0x102bf3a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf3a4u64 + 4);
        vm.wr(37, 0x102bf3a4u64 + 4);
        Ok(0x102bf3a4u64) }
        0x102bf3a4 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf3a8u64) }
        0x102bf3a8 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf3acu64) }
        0x102bf3ac => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf3b0u64) }
        0x102bf3b0 => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf3b4u64) }
        0x102bf3b4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bf3b8u64) }
        0x102bf3b8 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf3bcu64) }
        0x102bf3bc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf3c0u64) }
        0x102bf3c0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf3c4u64) }
        0x102bf3c4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf3c8u64) }
        0x102bf3c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf3ccu64 + 4);
        vm.wr(37, 0x102bf3ccu64 + 4);
        Ok(0x102bf3ccu64) }
        0x102bf3cc => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf3d0u64) }
        0x102bf3d0 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf3d4u64) }
        0x102bf3d4 => { vm.decode(0x5056e3d9u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 12797i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf3d8u64) }
        0x102bf3d8 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf3dcu64) }
        0x102bf3dc => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bf3e0u64) }
        0x102bf3e0 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf3e4u64).wrapping_add((imm * 4) as u64) } else { 0x102bf3e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf3e4u64) }
        0x102bf3e4 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf3e8u64) }
        0x102bf3f0 => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf3f4u64) }
        0x102bf3f4 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bf3f8u64) }
        0x102bf3f8 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf3fcu64) }
        0x102bf3fc => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf400u64) }
        0x102bf400 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf404u64) }
        0x102bf404 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf408u64) }
        0x102bf408 => { vm.decode(0x4b21a4f5u32); let d = 2i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf40cu64) }
        0x102bf40c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf410u64) }
        0x102bf410 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf414u64) }
        0x102bf414 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bf418u64) }
        0x102bf418 => { vm.decode(0x4848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf41cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf41cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf41cu64) }
        0x102bf41c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf420u64) } }
        0x102bf420 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf424u64) }
        0x102bf424 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf428u64) }
        0x102bf428 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf42cu64) }
        0x102bf42c => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf430u64) }
        0x102bf430 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf434u64) }
        0x102bf434 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf438u64) }
        0x102bf438 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf43cu64) }
        0x102bf43c => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf440u64) }
        0x102bf440 => { vm.decode(0xc000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bf444u64) }
        0x102bf444 => { vm.decode(0x228043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf448u64) }
        0x102bf448 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf44cu64) }
        0x102bf44c => { vm.decode(0x5c000111u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf450u64) }
        0x102bf450 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf454u64) }
        0x102bf454 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf458u64) }
        0x102bf458 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf45cu64 + 4);
        vm.wr(37, 0x102bf45cu64 + 4);
        Ok(0x102bf45cu64) }
        0x102bf45c => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf460u64) }
        0x102bf460 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf464u64) }
        0x102bf464 => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf468u64) }
        0x102bf468 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf46cu64) }
        0x102bf46c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf470u64) }
        0x102bf470 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf474u64).wrapping_add((imm * 4) as u64) } else { 0x102bf474u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf474u64) }
        0x102bf474 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf478u64) }
        0x102bf478 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf47cu64) }
        0x102bf47c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf480u64) }
        0x102bf480 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bf484u64) }
        0x102bf484 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf488u64) }
        0x102bf488 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf48cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf48cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf48cu64) }
        0x102bf48c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf490u64) } }
        0x102bf490 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf494u64) }
        0x102bf494 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf498u64) }
        0x102bf498 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf49cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf49cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf49cu64) }
        0x102bf49c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf4a0u64) }
        0x102bf4a0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf4a4u64) }
        0x102bf4a4 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4a8u64) }
        0x102bf4a8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bf4acu64) }
        0x102bf4ac => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf4b0u64) }
        0x102bf4b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf4b4u64).wrapping_add((imm * 4) as u64) } else { 0x102bf4b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf4b4u64) }
        0x102bf4b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf4b8u64) } }
        0x102bf4b8 => { vm.decode(0x36c041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 233584i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bf4bcu64) }
        0x102bf4bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf4c0u64) } }
        0x102bf4d0 => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4d4u64) }
        0x102bf4d4 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf4d8u64) }
        0x102bf4d8 => { vm.decode(0xd906051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4dcu64) }
        0x102bf4dc => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf4e0u64) }
        0x102bf4e0 => { vm.decode(0xd14e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1340i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4e4u64) }
        0x102bf4e4 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf4e8u64) }
        0x102bf4e8 => { vm.decode(0xd14e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4ecu64) }
        0x102bf4ec => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf4f0u64) }
        0x102bf4f0 => { vm.decode(0xd1468d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1332i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4f4u64) }
        0x102bf4f4 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf4f8u64) }
        0x102bf4f8 => { vm.decode(0xd146851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf4fcu64) }
        0x102bf4fc => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf500u64) }
        0x102bf500 => { vm.decode(0xd14e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1324i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf504u64) }
        0x102bf504 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf508u64) }
        0x102bf508 => { vm.decode(0xd14e051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf50cu64) }
        0x102bf50c => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf510u64) }
        0x102bf510 => { vm.decode(0xd1460d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1316i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf514u64) }
        0x102bf514 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf518u64) }
        0x102bf518 => { vm.decode(0xd146051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf51cu64) }
        0x102bf51c => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf520u64) }
        0x102bf520 => { vm.decode(0xd10e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf524u64) }
        0x102bf524 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf528u64) }
        0x102bf528 => { vm.decode(0xd10e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf52cu64) }
        0x102bf52c => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf530u64) }
        0x102bf530 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf534u64) }
        0x102bf534 => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf538u64) }
        0x102bf538 => { vm.decode(0xd106851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf53cu64) }
        0x102bf53c => { vm.decode(0x3153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf540u64) }
        0x102bf540 => { vm.decode(0x5510f1d1u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf544u64) }
        0x102bf544 => { vm.decode(0x5d10e051u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf548u64) }
        0x102bf548 => { vm.decode(0x4a8a63afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf54cu64) }
        0x102bf54c => { vm.decode(0x28a4402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf550u64) }
        0x102bf550 => { vm.decode(0x12008119u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf554u64) }
        0x102bf554 => { vm.decode(0x9b215430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf558u64) }
        0x102bf558 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf55cu64) }
        0x102bf55c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf560u64) }
        0x102bf560 => { vm.decode(0x704b0419u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf564u64) }
        0x102bf564 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf568u64) }
        0x102bf568 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf56cu64) }
        0x102bf56c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf570u64) }
        0x102bf570 => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf574u64) }
        0x102bf574 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf578u64) }
        0x102bf578 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf57cu64 + 4);
        vm.wr(37, 0x102bf57cu64 + 4);
        Ok(0x102bf57cu64) }
        0x102bf57c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf580u64) }
        0x102bf580 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf584u64) }
        0x102bf584 => { vm.decode(0x70498519u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf588u64) }
        0x102bf588 => { vm.decode(0x22a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf58cu64) }
        0x102bf58c => { vm.decode(0x9b2154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf590u64) }
        0x102bf590 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bf594u64) }
        0x102bf594 => { vm.decode(0x9b311430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf598u64) }
        0x102bf598 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf59cu64) }
        0x102bf59c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf5a0u64) }
        0x102bf5a0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5a4u64) }
        0x102bf5a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf5a8u64 + 4);
        vm.wr(37, 0x102bf5a8u64 + 4);
        Ok(0x102bf5a8u64) }
        0x102bf5a8 => { vm.decode(0x2102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf5acu64) }
        0x102bf5ac => { vm.decode(0xd986111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5b0u64) }
        0x102bf5b0 => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5b4u64) }
        0x102bf5b4 => { vm.decode(0x2001a835u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5b8u64) }
        0x102bf5b8 => { vm.decode(0x130207f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5bcu64) }
        0x102bf5bc => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5c0u64) }
        0x102bf5c0 => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5c4u64) }
        0x102bf5c4 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5c8u64) }
        0x102bf5c8 => { vm.decode(0xc4012fb5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5ccu64) }
        0x102bf5cc => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5d0u64) }
        0x102bf5d0 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5d4u64) }
        0x102bf5d4 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5d8u64) }
        0x102bf5d8 => { vm.decode(0xc4800fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5dcu64) }
        0x102bf5dc => { vm.decode(0x40198835u32); let d = 2i16; let s = 5i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5e0u64) }
        0x102bf5e0 => { vm.decode(0x440017f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5e4u64) }
        0x102bf5e4 => { vm.decode(0x200d0835u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf5e8u64) }
        0x102bf5e8 => { vm.decode(0x4d14c211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5ecu64) }
        0x102bf5ec => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5f0u64) }
        0x102bf5f0 => { vm.decode(0xb0a02fb5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5f4u64) }
        0x102bf5f4 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5f8u64) }
        0x102bf5f8 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf5fcu64) }
        0x102bf5fc => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf600u64) }
        0x102bf600 => { vm.decode(0x2008b835u32); let d = 3i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf604u64) }
        0x102bf604 => { vm.decode(0x461207f5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf608u64) }
        0x102bf608 => { vm.decode(0x401d2835u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf60cu64) }
        0x102bf60c => { vm.decode(0x5104b11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf610u64) }
        0x102bf610 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf614u64) }
        0x102bf614 => { vm.decode(0x510e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf618u64) }
        0x102bf618 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf61cu64) }
        0x102bf61c => { vm.decode(0x64112fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf620u64) }
        0x102bf620 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf624u64) }
        0x102bf624 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf628u64) }
        0x102bf628 => { vm.decode(0x41400835u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf62cu64) }
        0x102bf62c => { vm.decode(0x1157f5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf630u64) }
        0x102bf630 => { vm.decode(0x21008835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf634u64) }
        0x102bf634 => { vm.decode(0xd14cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf638u64) }
        0x102bf638 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf63cu64) }
        0x102bf63c => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf640u64) }
        0x102bf640 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf644u64) }
        0x102bf644 => { vm.decode(0x9a0fb5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf648u64) }
        0x102bf648 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf64cu64) }
        0x102bf64c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf650u64) }
        0x102bf650 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf654u64) }
        0x102bf654 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf658u64) }
        0x102bf658 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf65cu64) }
        0x102bf65c => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf660u64) }
        0x102bf660 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf664u64) }
        0x102bf664 => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf668u64) }
        0x102bf668 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf66cu64) }
        0x102bf66c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf670u64) }
        0x102bf670 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf674u64) }
        0x102bf674 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf678u64) }
        0x102bf678 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf67cu64) }
        0x102bf67c => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf680u64) }
        0x102bf680 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf684u64) }
        0x102bf684 => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf688u64) }
        0x102bf688 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf68cu64) }
        0x102bf68c => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf690u64) }
        0x102bf690 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf694u64) }
        0x102bf694 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf698u64) }
        0x102bf698 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf69cu64) }
        0x102bf69c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6a0u64) }
        0x102bf6a0 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6a4u64) }
        0x102bf6a4 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6a8u64) }
        0x102bf6a8 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6acu64) }
        0x102bf6ac => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6b0u64) }
        0x102bf6b0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6b4u64) }
        0x102bf6b4 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6b8u64) }
        0x102bf6b8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6bcu64) }
        0x102bf6bc => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6c0u64) }
        0x102bf6c0 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6c4u64) }
        0x102bf6c4 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6c8u64) }
        0x102bf6c8 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6ccu64) }
        0x102bf6cc => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6d0u64) }
        0x102bf6d0 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6d4u64) }
        0x102bf6d4 => { vm.decode(0x45144b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6d8u64) }
        0x102bf6d8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6dcu64) }
        0x102bf6dc => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6e0u64) }
        0x102bf6e0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6e4u64) }
        0x102bf6e4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6e8u64) }
        0x102bf6e8 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6ecu64) }
        0x102bf6ec => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf6f0u64) }
        0x102bf6f0 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6f4u64) }
        0x102bf6f4 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6f8u64) }
        0x102bf6f8 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf6fcu64) }
        0x102bf6fc => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf700u64) }
        0x102bf700 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf704u64) }
        0x102bf704 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf708u64) }
        0x102bf708 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf70cu64) }
        0x102bf70c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf710u64) }
        0x102bf710 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf714u64) }
        0x102bf714 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf718u64) }
        0x102bf718 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf71cu64) }
        0x102bf71c => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf720u64) }
        0x102bf720 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf724u64) }
        0x102bf724 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf728u64) }
        0x102bf728 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf72cu64) }
        0x102bf72c => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf730u64) }
        0x102bf730 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf734u64) }
        0x102bf734 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf738u64) }
        0x102bf738 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf73cu64) }
        0x102bf73c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf740u64) }
        0x102bf740 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf744u64) }
        0x102bf744 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf748u64) }
        0x102bf748 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf74cu64) }
        0x102bf74c => { vm.decode(0x45104311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf750u64) }
        0x102bf750 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf754u64) }
        0x102bf754 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf758u64) }
        0x102bf758 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf75cu64) }
        0x102bf75c => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf760u64) }
        0x102bf760 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf764u64) }
        0x102bf764 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf768u64) }
        0x102bf768 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf76cu64) }
        0x102bf76c => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf770u64) }
        0x102bf770 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf774u64) }
        0x102bf774 => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf778u64) }
        0x102bf778 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf77cu64) }
        0x102bf77c => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf780u64) }
        0x102bf780 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf784u64) }
        0x102bf784 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf788u64) }
        0x102bf788 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf78cu64) }
        0x102bf78c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf790u64) }
        0x102bf790 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf794u64) }
        0x102bf794 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf798u64) }
        0x102bf798 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf79cu64) }
        0x102bf79c => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7a0u64) }
        0x102bf7a0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7a4u64) }
        0x102bf7a4 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7a8u64) }
        0x102bf7a8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7acu64) }
        0x102bf7ac => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7b0u64) }
        0x102bf7b0 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7b4u64) }
        0x102bf7b4 => { vm.decode(0x4860fb5u32); let d = 20i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7b8u64) }
        0x102bf7b8 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7bcu64) }
        0x102bf7bc => { vm.decode(0x130207f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7c0u64) }
        0x102bf7c0 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7c4u64) }
        0x102bf7c4 => { vm.decode(0xd144311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7c8u64) }
        0x102bf7c8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7ccu64) }
        0x102bf7cc => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7d0u64) }
        0x102bf7d0 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7d4u64) }
        0x102bf7d4 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7d8u64) }
        0x102bf7d8 => { vm.decode(0x18814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7dcu64) }
        0x102bf7dc => { vm.decode(0xc4820fb5u32); let d = 5i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7e0u64) }
        0x102bf7e0 => { vm.decode(0x11a835u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7e4u64) }
        0x102bf7e4 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7e8u64) }
        0x102bf7e8 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf7ecu64) }
        0x102bf7ec => { vm.decode(0x4590c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7f0u64) }
        0x102bf7f0 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7f4u64) }
        0x102bf7f4 => { vm.decode(0x45186011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7f8u64) }
        0x102bf7f8 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf7fcu64) }
        0x102bf7fc => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf800u64) }
        0x102bf800 => { vm.decode(0x1e80cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf804u64) }
        0x102bf804 => { vm.decode(0x64810fb5u32); let d = 3i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf808u64) }
        0x102bf808 => { vm.decode(0x3751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf80cu64) }
        0x102bf80c => { vm.decode(0x2001b835u32); let d = 1i16; let s = 5i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf810u64) }
        0x102bf810 => { vm.decode(0x21217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf814u64) }
        0x102bf814 => { vm.decode(0x40112835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf818u64) }
        0x102bf818 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf81cu64) }
        0x102bf81c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf820u64) }
        0x102bf820 => { vm.decode(0x45186811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf824u64) }
        0x102bf824 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf828u64) }
        0x102bf828 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf82cu64) }
        0x102bf82c => { vm.decode(0x1e004d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf830u64) }
        0x102bf830 => { vm.decode(0x3257062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf834u64) }
        0x102bf834 => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf838u64) }
        0x102bf838 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf83cu64) }
        0x102bf83c => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf840u64) }
        0x102bf840 => { vm.decode(0xd256e359u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 12798i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf844u64) }
        0x102bf844 => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf848u64) }
        0x102bf848 => { vm.decode(0x27002406u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bf84cu64) }
        0x102bf84c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf850u64).wrapping_add((imm * 4) as u64) } else { 0x102bf850u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf850u64) }
        0x102bf850 => { vm.decode(0x7100236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf854u64) }
        0x102bf85c => { vm.decode(0xa424735u32); let d = 1i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf860u64) }
        0x102bf860 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bf864u64) }
        0x102bf864 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf868u64) }
        0x102bf868 => { vm.decode(0x74114675u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf86cu64) }
        0x102bf86c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf870u64) }
        0x102bf870 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf874u64) }
        0x102bf874 => { vm.decode(0x690104f5u32); let d = 2i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf878u64) }
        0x102bf878 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf87cu64) }
        0x102bf87c => { vm.decode(0xaf03236u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf880u64) }
        0x102bf880 => { vm.decode(0x27002406u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bf884u64) }
        0x102bf884 => { vm.decode(0x4040227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf888u64).wrapping_add((imm * 4) as u64) } else { 0x102bf888u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf888u64) }
        0x102bf888 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf88cu64) } }
        0x102bf88c => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf890u64) }
        0x102bf890 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf894u64) }
        0x102bf894 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf898u64) }
        0x102bf898 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf89cu64) }
        0x102bf89c => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf8a0u64) }
        0x102bf8a0 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf8a4u64) }
        0x102bf8a4 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf8a8u64) }
        0x102bf8a8 => { vm.decode(0x40a30bb5u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf8acu64) }
        0x102bf8ac => { vm.decode(0xc0003d75u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bf8b0u64) }
        0x102bf8b0 => { vm.decode(0x414104f5u32); let d = 1i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bf8b4u64) }
        0x102bf8b4 => { vm.decode(0x5c000011u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf8b8u64) }
        0x102bf8b8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf8bcu64) }
        0x102bf8bc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf8c0u64) }
        0x102bf8c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf8c4u64 + 4);
        vm.wr(37, 0x102bf8c4u64 + 4);
        Ok(0x102bf8c4u64) }
        0x102bf8c4 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf8c8u64) }
        0x102bf8c8 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf8ccu64) }
        0x102bf8cc => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf8d0u64) }
        0x102bf8d0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf8d4u64) }
        0x102bf8d4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf8d8u64) }
        0x102bf8d8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf8dcu64).wrapping_add((imm * 4) as u64) } else { 0x102bf8dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf8dcu64) }
        0x102bf8dc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf8e0u64) }
        0x102bf8e0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf8e4u64) }
        0x102bf8e4 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf8e8u64) }
        0x102bf8e8 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bf8ecu64) }
        0x102bf8ec => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf8f0u64) }
        0x102bf8f0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf8f4u64).wrapping_add((imm * 4) as u64) } else { 0x102bf8f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf8f4u64) }
        0x102bf8f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf8f8u64) } }
        0x102bf8f8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf8fcu64) }
        0x102bf8fc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf900u64) }
        0x102bf900 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bf904u64).wrapping_add((imm * 4) as u64) } else { 0x102bf904u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf904u64) }
        0x102bf904 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf908u64) }
        0x102bf908 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bf90cu64) }
        0x102bf90c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bf910u64) }
        0x102bf910 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bf914u64) }
        0x102bf914 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bf918u64) }
        0x102bf918 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bf91cu64).wrapping_add((imm * 4) as u64) } else { 0x102bf91cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bf91cu64) }
        0x102bf91c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf920u64) } }
        0x102bf920 => { vm.decode(0xae6c041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 234716i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bf924u64) }
        0x102bf924 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bf928u64) } }
        0x102bf928 => { vm.decode(0xf18c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2188i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf92cu64) }
        0x102bf92c => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf930u64) }
        0x102bf930 => { vm.decode(0xf18c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf934u64) }
        0x102bf934 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf938u64) }
        0x102bf938 => { vm.decode(0xf184091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2180i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf93cu64) }
        0x102bf93c => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf940u64) }
        0x102bf940 => { vm.decode(0xf184011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf944u64) }
        0x102bf944 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf948u64) }
        0x102bf948 => { vm.decode(0xf94c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2172i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf94cu64) }
        0x102bf94c => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf950u64) }
        0x102bf950 => { vm.decode(0xf94c811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf954u64) }
        0x102bf954 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf958u64) }
        0x102bf958 => { vm.decode(0xf944891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2164i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf95cu64) }
        0x102bf95c => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf960u64) }
        0x102bf960 => { vm.decode(0xf944811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf964u64) }
        0x102bf964 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf968u64) }
        0x102bf968 => { vm.decode(0xf94c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2156i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf96cu64) }
        0x102bf96c => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf970u64) }
        0x102bf970 => { vm.decode(0xf94c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf974u64) }
        0x102bf974 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf978u64) }
        0x102bf978 => { vm.decode(0xf944091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2148i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf97cu64) }
        0x102bf97c => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf980u64) }
        0x102bf980 => { vm.decode(0xf944011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf984u64) }
        0x102bf984 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf988u64) }
        0x102bf988 => { vm.decode(0xf90c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2140i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf98cu64) }
        0x102bf98c => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bf990u64) }
        0x102bf990 => { vm.decode(0x5f90c811u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf994u64) }
        0x102bf994 => { vm.decode(0x5f904991u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2132i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf998u64) }
        0x102bf998 => { vm.decode(0x1f904911u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bf99cu64) }
        0x102bf99c => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9a0u64) }
        0x102bf9a0 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9a4u64) }
        0x102bf9a4 => { vm.decode(0x12001119u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf9a8u64) }
        0x102bf9a8 => { vm.decode(0x8b215530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9acu64) }
        0x102bf9ac => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9b0u64) }
        0x102bf9b0 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9b4u64) }
        0x102bf9b4 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9b8u64) }
        0x102bf9b8 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9bcu64) }
        0x102bf9bc => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9c0u64) }
        0x102bf9c0 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9c4u64) }
        0x102bf9c4 => { vm.decode(0x488e4bafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9c8u64) }
        0x102bf9c8 => { vm.decode(0xa30d875u32); let d = 25i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf9ccu64) }
        0x102bf9cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf9d0u64 + 4);
        vm.wr(37, 0x102bf9d0u64 + 4);
        Ok(0x102bf9d0u64) }
        0x102bf9d0 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf9d4u64) }
        0x102bf9d4 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf9d8u64) }
        0x102bf9d8 => { vm.decode(0x746b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bf9dcu64) }
        0x102bf9dc => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9e0u64) }
        0x102bf9e0 => { vm.decode(0x8b2155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9e4u64) }
        0x102bf9e4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bf9e8u64) }
        0x102bf9e8 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9ecu64) }
        0x102bf9ec => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bf9f0u64) }
        0x102bf9f0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bf9f4u64) }
        0x102bf9f4 => { vm.decode(0xa30d875u32); let d = 25i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bf9f8u64) }
        0x102bf9f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bf9fcu64 + 4);
        vm.wr(37, 0x102bf9fcu64 + 4);
        Ok(0x102bf9fcu64) }
        0x102bf9fc => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfa00u64) }
        0x102bfa00 => { vm.decode(0xd186911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa04u64) }
        0x102bfa04 => { vm.decode(0x45186111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa08u64) }
        0x102bfa08 => { vm.decode(0x4041a835u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa0cu64) }
        0x102bfa0c => { vm.decode(0x518e111u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa10u64) }
        0x102bfa10 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa14u64) }
        0x102bfa14 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa18u64) }
        0x102bfa18 => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa1cu64) }
        0x102bfa1c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa20u64) }
        0x102bfa20 => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa24u64) }
        0x102bfa24 => { vm.decode(0x40190fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa28u64) }
        0x102bfa28 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa2cu64) }
        0x102bfa2c => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa30u64) }
        0x102bfa30 => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa34u64) }
        0x102bfa34 => { vm.decode(0x540017f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa38u64) }
        0x102bfa38 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa3cu64) }
        0x102bfa3c => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa40u64) }
        0x102bfa40 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa44u64) }
        0x102bfa44 => { vm.decode(0xa0a12fb5u32); let d = 2i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa48u64) }
        0x102bfa48 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa4cu64) }
        0x102bfa4c => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa50u64) }
        0x102bfa50 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa54u64) }
        0x102bfa54 => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa58u64) }
        0x102bfa58 => { vm.decode(0x561207f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa5cu64) }
        0x102bfa5c => { vm.decode(0x600c2835u32); let d = 3i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa60u64) }
        0x102bfa60 => { vm.decode(0x510cb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa64u64) }
        0x102bfa64 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa68u64) }
        0x102bfa68 => { vm.decode(0xe4212fb5u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa6cu64) }
        0x102bfa6c => { vm.decode(0x64112fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa70u64) }
        0x102bfa70 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa74u64) }
        0x102bfa74 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa78u64) }
        0x102bfa78 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa7cu64) }
        0x102bfa7c => { vm.decode(0x157f5u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa80u64) }
        0x102bfa80 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfa84u64) }
        0x102bfa84 => { vm.decode(0xd90c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa88u64) }
        0x102bfa88 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa8cu64) }
        0x102bfa8c => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa90u64) }
        0x102bfa90 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa94u64) }
        0x102bfa94 => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa98u64) }
        0x102bfa98 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfa9cu64) }
        0x102bfa9c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfaa0u64) }
        0x102bfaa0 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaa4u64) }
        0x102bfaa4 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaa8u64) }
        0x102bfaa8 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaacu64) }
        0x102bfaac => { vm.decode(0x4514cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfab0u64) }
        0x102bfab0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfab4u64) }
        0x102bfab4 => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfab8u64) }
        0x102bfab8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfabcu64) }
        0x102bfabc => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfac0u64) }
        0x102bfac0 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfac4u64) }
        0x102bfac4 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfac8u64) }
        0x102bfac8 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaccu64) }
        0x102bfacc => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfad0u64) }
        0x102bfad0 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfad4u64) }
        0x102bfad4 => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfad8u64) }
        0x102bfad8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfadcu64) }
        0x102bfadc => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfae0u64) }
        0x102bfae0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfae4u64) }
        0x102bfae4 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfae8u64) }
        0x102bfae8 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfaecu64) }
        0x102bfaec => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfaf0u64) }
        0x102bfaf0 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaf4u64) }
        0x102bfaf4 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfaf8u64) }
        0x102bfaf8 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfafcu64) }
        0x102bfafc => { vm.decode(0x459ce911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb00u64) }
        0x102bfb00 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb04u64) }
        0x102bfb04 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb08u64) }
        0x102bfb08 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb0cu64) }
        0x102bfb0c => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb10u64) }
        0x102bfb10 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb14u64) }
        0x102bfb14 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb18u64) }
        0x102bfb18 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb1cu64) }
        0x102bfb1c => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb20u64) }
        0x102bfb20 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb24u64) }
        0x102bfb24 => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb28u64) }
        0x102bfb28 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb2cu64) }
        0x102bfb2c => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb30u64) }
        0x102bfb30 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb34u64) }
        0x102bfb34 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb38u64) }
        0x102bfb38 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb3cu64) }
        0x102bfb3c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb40u64) }
        0x102bfb40 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb44u64) }
        0x102bfb44 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb48u64) }
        0x102bfb48 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb4cu64) }
        0x102bfb4c => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb50u64) }
        0x102bfb50 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb54u64) }
        0x102bfb54 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb58u64) }
        0x102bfb58 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb5cu64) }
        0x102bfb5c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb60u64) }
        0x102bfb60 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb64u64) }
        0x102bfb64 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb68u64) }
        0x102bfb68 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb6cu64) }
        0x102bfb6c => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb70u64) }
        0x102bfb70 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb74u64) }
        0x102bfb74 => { vm.decode(0x4514c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb78u64) }
        0x102bfb78 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb7cu64) }
        0x102bfb7c => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb80u64) }
        0x102bfb80 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb84u64) }
        0x102bfb84 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb88u64) }
        0x102bfb88 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb8cu64) }
        0x102bfb8c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfb90u64) }
        0x102bfb90 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb94u64) }
        0x102bfb94 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb98u64) }
        0x102bfb98 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfb9cu64) }
        0x102bfb9c => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfba0u64) }
        0x102bfba0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfba4u64) }
        0x102bfba4 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfba8u64) }
        0x102bfba8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbacu64) }
        0x102bfbac => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbb0u64) }
        0x102bfbb0 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbb4u64) }
        0x102bfbb4 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbb8u64) }
        0x102bfbb8 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbbcu64) }
        0x102bfbbc => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbc0u64) }
        0x102bfbc0 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbc4u64) }
        0x102bfbc4 => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbc8u64) }
        0x102bfbc8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbccu64) }
        0x102bfbcc => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbd0u64) }
        0x102bfbd0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbd4u64) }
        0x102bfbd4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbd8u64) }
        0x102bfbd8 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbdcu64) }
        0x102bfbdc => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbe0u64) }
        0x102bfbe0 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbe4u64) }
        0x102bfbe4 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbe8u64) }
        0x102bfbe8 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfbecu64) }
        0x102bfbec => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbf0u64) }
        0x102bfbf0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbf4u64) }
        0x102bfbf4 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbf8u64) }
        0x102bfbf8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfbfcu64) }
        0x102bfbfc => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc00u64) }
        0x102bfc00 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc04u64) }
        0x102bfc04 => { vm.decode(0x24840fb5u32); let d = 18i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc08u64) }
        0x102bfc08 => { vm.decode(0x50a835u32); let d = 1i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc0cu64) }
        0x102bfc0c => { vm.decode(0x21217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc10u64) }
        0x102bfc10 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc14u64) }
        0x102bfc14 => { vm.decode(0xd144b11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc18u64) }
        0x102bfc18 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc1cu64) }
        0x102bfc1c => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc20u64) }
        0x102bfc20 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc24u64) }
        0x102bfc24 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc28u64) }
        0x102bfc28 => { vm.decode(0x1c81cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc2cu64) }
        0x102bfc2c => { vm.decode(0xe4042fb5u32); let d = 19i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc30u64) }
        0x102bfc30 => { vm.decode(0x20543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc34u64) }
        0x102bfc34 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc38u64) }
        0x102bfc38 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc3cu64) }
        0x102bfc3c => { vm.decode(0x45904211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc40u64) }
        0x102bfc40 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc44u64) }
        0x102bfc44 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc48u64) }
        0x102bfc48 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc4cu64) }
        0x102bfc4c => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc50u64) }
        0x102bfc50 => { vm.decode(0x1c014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc54u64) }
        0x102bfc54 => { vm.decode(0xe4012fb5u32); let d = 3i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc58u64) }
        0x102bfc58 => { vm.decode(0x60443835u32); let d = 1i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc5cu64) }
        0x102bfc5c => { vm.decode(0x31207f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc60u64) }
        0x102bfc60 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfc64u64) }
        0x102bfc64 => { vm.decode(0x45104a11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc68u64) }
        0x102bfc68 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc6cu64) }
        0x102bfc6c => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc70u64) }
        0x102bfc70 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc74u64) }
        0x102bfc74 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc78u64) }
        0x102bfc78 => { vm.decode(0x1a804d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc7cu64) }
        0x102bfc7c => { vm.decode(0x3357062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfc80u64) }
        0x102bfc80 => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfc84u64) }
        0x102bfc84 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfc88u64) }
        0x102bfc88 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfc8cu64) }
        0x102bfc8c => { vm.decode(0xd25673d9u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 12799i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfc90u64) }
        0x102bfc90 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfc94u64) }
        0x102bfc94 => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bfc98u64) }
        0x102bfc98 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bfc9cu64).wrapping_add((imm * 4) as u64) } else { 0x102bfc9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfc9cu64) }
        0x102bfc9c => { vm.decode(0x7100236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfca0u64) }
        0x102bfca8 => { vm.decode(0xa224735u32); let d = 1i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfcacu64) }
        0x102bfcac => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bfcb0u64) }
        0x102bfcb0 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfcb4u64) }
        0x102bfcb4 => { vm.decode(0x64314675u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfcb8u64) }
        0x102bfcb8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfcbcu64) }
        0x102bfcbc => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfcc0u64) }
        0x102bfcc0 => { vm.decode(0x92104f5u32); let d = 2i16; let s = 16i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bfcc4u64) }
        0x102bfcc4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfcc8u64) }
        0x102bfcc8 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfcccu64) }
        0x102bfccc => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bfcd0u64) }
        0x102bfcd0 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bfcd4u64).wrapping_add((imm * 4) as u64) } else { 0x102bfcd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfcd4u64) }
        0x102bfcd4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bfcd8u64) } }
        0x102bfcd8 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfcdcu64) }
        0x102bfcdc => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bfce0u64) }
        0x102bfce0 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfce4u64) }
        0x102bfce4 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfce8u64) }
        0x102bfce8 => { vm.decode(0x9b195530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfcecu64) }
        0x102bfcec => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfcf0u64) }
        0x102bfcf0 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfcf4u64) }
        0x102bfcf4 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfcf8u64) }
        0x102bfcf8 => { vm.decode(0xc000ad75u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bfcfcu64) }
        0x102bfcfc => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfd00u64) }
        0x102bfd00 => { vm.decode(0x216004f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bfd04u64) }
        0x102bfd04 => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd08u64) }
        0x102bfd08 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfd0cu64) }
        0x102bfd0c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfd10u64) }
        0x102bfd10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bfd14u64 + 4);
        vm.wr(37, 0x102bfd14u64 + 4);
        Ok(0x102bfd14u64) }
        0x102bfd14 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfd18u64) }
        0x102bfd18 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd1cu64) }
        0x102bfd1c => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd20u64) }
        0x102bfd20 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd24u64) }
        0x102bfd24 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bfd28u64) }
        0x102bfd28 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bfd2cu64).wrapping_add((imm * 4) as u64) } else { 0x102bfd2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfd2cu64) }
        0x102bfd2c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd30u64) }
        0x102bfd30 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd34u64) }
        0x102bfd34 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd38u64) }
        0x102bfd38 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bfd3cu64) }
        0x102bfd3c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bfd40u64) }
        0x102bfd40 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bfd44u64).wrapping_add((imm * 4) as u64) } else { 0x102bfd44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfd44u64) }
        0x102bfd44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bfd48u64) } }
        0x102bfd48 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd4cu64) }
        0x102bfd4c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bfd50u64) }
        0x102bfd50 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bfd54u64).wrapping_add((imm * 4) as u64) } else { 0x102bfd54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfd54u64) }
        0x102bfd54 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd58u64) }
        0x102bfd58 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfd5cu64) }
        0x102bfd5c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd60u64) }
        0x102bfd60 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bfd64u64) }
        0x102bfd64 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bfd68u64) }
        0x102bfd68 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bfd6cu64).wrapping_add((imm * 4) as u64) } else { 0x102bfd6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bfd6cu64) }
        0x102bfd6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bfd70u64) } }
        0x102bfd70 => { vm.decode(0xb4c501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 235816i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bfd74u64) }
        0x102bfd74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bfd78u64) } }
        0x102bfd78 => { vm.decode(0xf10c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2060i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd7cu64) }
        0x102bfd7c => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfd80u64) }
        0x102bfd80 => { vm.decode(0xf10c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd84u64) }
        0x102bfd84 => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfd88u64) }
        0x102bfd88 => { vm.decode(0xf104091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2052i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd8cu64) }
        0x102bfd8c => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfd90u64) }
        0x102bfd90 => { vm.decode(0xf104011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd94u64) }
        0x102bfd94 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfd98u64) }
        0x102bfd98 => { vm.decode(0xd9cead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2044i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfd9cu64) }
        0x102bfd9c => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfda0u64) }
        0x102bfda0 => { vm.decode(0xd9cea51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfda4u64) }
        0x102bfda4 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfda8u64) }
        0x102bfda8 => { vm.decode(0xd9c6ad1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2036i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdacu64) }
        0x102bfdac => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdb0u64) }
        0x102bfdb0 => { vm.decode(0xd9c6a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdb4u64) }
        0x102bfdb4 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdb8u64) }
        0x102bfdb8 => { vm.decode(0xd9ce2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdbcu64) }
        0x102bfdbc => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdc0u64) }
        0x102bfdc0 => { vm.decode(0xd9ce251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdc4u64) }
        0x102bfdc4 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdc8u64) }
        0x102bfdc8 => { vm.decode(0xd9c62d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdccu64) }
        0x102bfdcc => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdd0u64) }
        0x102bfdd0 => { vm.decode(0xd9c6251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdd4u64) }
        0x102bfdd4 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfdd8u64) }
        0x102bfdd8 => { vm.decode(0xd98ead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfddcu64) }
        0x102bfddc => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bfde0u64) }
        0x102bfde0 => { vm.decode(0x5598fb51u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfde4u64) }
        0x102bfde4 => { vm.decode(0x1d986ad1u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfde8u64) }
        0x102bfde8 => { vm.decode(0x55986b51u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfdecu64) }
        0x102bfdec => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfdf0u64) }
        0x102bfdf0 => { vm.decode(0x2884412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfdf4u64) }
        0x102bfdf4 => { vm.decode(0x12009119u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfdf8u64) }
        0x102bfdf8 => { vm.decode(0x9b215530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfdfcu64) }
        0x102bfdfc => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe00u64) }
        0x102bfe00 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe04u64) }
        0x102bfe04 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe08u64) }
        0x102bfe08 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe0cu64) }
        0x102bfe0c => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe10u64) }
        0x102bfe10 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe14u64) }
        0x102bfe14 => { vm.decode(0x488643afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe18u64) }
        0x102bfe18 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe1cu64) }
        0x102bfe1c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bfe20u64 + 4);
        vm.wr(37, 0x102bfe20u64 + 4);
        Ok(0x102bfe20u64) }
        0x102bfe20 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfe24u64) }
        0x102bfe24 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bfe28u64) }
        0x102bfe28 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe2cu64) }
        0x102bfe2c => { vm.decode(0x9b2155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe30u64) }
        0x102bfe30 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bfe34u64) }
        0x102bfe34 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe38u64) }
        0x102bfe38 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe3cu64) }
        0x102bfe3c => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bfe40u64) }
        0x102bfe40 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bfe44u64) }
        0x102bfe44 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe48u64) }
        0x102bfe48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bfe4cu64 + 4);
        vm.wr(37, 0x102bfe4cu64 + 4);
        Ok(0x102bfe4cu64) }
        0x102bfe4c => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bfe50u64) }
        0x102bfe50 => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe54u64) }
        0x102bfe54 => { vm.decode(0x45186911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe58u64) }
        0x102bfe58 => { vm.decode(0x41a835u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe5cu64) }
        0x102bfe5c => { vm.decode(0x60043835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe60u64) }
        0x102bfe60 => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe64u64) }
        0x102bfe64 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe68u64) }
        0x102bfe68 => { vm.decode(0xc4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe6cu64) }
        0x102bfe6c => { vm.decode(0x40290fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe70u64) }
        0x102bfe70 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe74u64) }
        0x102bfe74 => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe78u64) }
        0x102bfe78 => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe7cu64) }
        0x102bfe7c => { vm.decode(0x488835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe80u64) }
        0x102bfe80 => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe84u64) }
        0x102bfe84 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe88u64) }
        0x102bfe88 => { vm.decode(0xa0210fb5u32); let d = 2i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe8cu64) }
        0x102bfe8c => { vm.decode(0x24282fb5u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe90u64) }
        0x102bfe90 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe94u64) }
        0x102bfe94 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfe98u64) }
        0x102bfe98 => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfe9cu64) }
        0x102bfe9c => { vm.decode(0x204ca835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfea0u64) }
        0x102bfea0 => { vm.decode(0x4d14cb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfea4u64) }
        0x102bfea4 => { vm.decode(0x64a12fb5u32); let d = 3i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfea8u64) }
        0x102bfea8 => { vm.decode(0xf4a02fb5u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeacu64) }
        0x102bfeac => { vm.decode(0x60112fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeb0u64) }
        0x102bfeb0 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeb4u64) }
        0x102bfeb4 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeb8u64) }
        0x102bfeb8 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfebcu64) }
        0x102bfebc => { vm.decode(0x41010835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfec0u64) }
        0x102bfec0 => { vm.decode(0xd14c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfec4u64) }
        0x102bfec4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfec8u64) }
        0x102bfec8 => { vm.decode(0xd146911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeccu64) }
        0x102bfecc => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfed0u64) }
        0x102bfed0 => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfed4u64) }
        0x102bfed4 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfed8u64) }
        0x102bfed8 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfedcu64) }
        0x102bfedc => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfee0u64) }
        0x102bfee0 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfee4u64) }
        0x102bfee4 => { vm.decode(0x4590c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfee8u64) }
        0x102bfee8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfeecu64) }
        0x102bfeec => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bfef0u64) }
        0x102bfef0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfef4u64) }
        0x102bfef4 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfef8u64) }
        0x102bfef8 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfefcu64) }
        0x102bfefc => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff00u64) }
        0x102bff00 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff04u64) }
        0x102bff04 => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff08u64) }
        0x102bff08 => { vm.decode(0x44932fb5u32); let d = 5i16; let s = 5i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff0cu64) }
        0x102bff0c => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff10u64) }
        0x102bff10 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff14u64) }
        0x102bff14 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff18u64) }
        0x102bff18 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff1cu64) }
        0x102bff1c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff20u64) }
        0x102bff20 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff24u64) }
        0x102bff24 => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff28u64) }
        0x102bff28 => { vm.decode(0x45144b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff2cu64) }
        0x102bff2c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff30u64) }
        0x102bff30 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff34u64) }
        0x102bff34 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff38u64) }
        0x102bff38 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff3cu64) }
        0x102bff3c => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff40u64) }
        0x102bff40 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff44u64) }
        0x102bff44 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff48u64) }
        0x102bff48 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff4cu64) }
        0x102bff4c => { vm.decode(0x45104b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff50u64) }
        0x102bff50 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff54u64) }
        0x102bff54 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff58u64) }
        0x102bff58 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff5cu64) }
        0x102bff5c => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff60u64) }
        0x102bff60 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff64u64) }
        0x102bff64 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff68u64) }
        0x102bff68 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff6cu64) }
        0x102bff6c => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff70u64) }
        0x102bff70 => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff74u64) }
        0x102bff74 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff78u64) }
        0x102bff78 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff7cu64) }
        0x102bff7c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff80u64) }
        0x102bff80 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff84u64) }
        0x102bff84 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff88u64) }
        0x102bff88 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff8cu64) }
        0x102bff8c => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff90u64) }
        0x102bff90 => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bff94u64) }
        0x102bff94 => { vm.decode(0x4598e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bff98u64) }
        0x102bff98 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bff9cu64) }
        0x102bff9c => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bffa0u64) }
        0x102bffa0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffa4u64) }
        0x102bffa4 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffa8u64) }
        0x102bffa8 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffacu64) }
        0x102bffac => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffb0u64) }
        0x102bffb0 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bffb4u64) }
        0x102bffb4 => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bffb8u64) }
        0x102bffb8 => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bffbcu64) }
        0x102bffbc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffc0u64) }
        0x102bffc0 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bffc4u64) }
        0x102bffc4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffc8u64) }
        0x102bffc8 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffccu64) }
        0x102bffcc => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffd0u64) }
        0x102bffd0 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffd4u64) }
        0x102bffd4 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bffd8u64) }
        0x102bffd8 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bffdcu64) }
        0x102bffdc => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bffe0u64) }
        0x102bffe0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffe4u64) }
        0x102bffe4 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bffe8u64) }
        0x102bffe8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bffecu64) }
        0x102bffec => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfff0u64) }
        0x102bfff0 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfff4u64) }
        0x102bfff4 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bfff8u64) }
        0x102bfff8 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bfffcu64) }
        0x102bfffc => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0000u64) }
        0x102c0000 => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0004u64) }
        0x102c0004 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0008u64) }
        0x102c0008 => { vm.decode(0x4594e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c000cu64) }
        0x102c000c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0010u64) }
        0x102c0010 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0014u64) }
        0x102c0014 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0018u64) }
        0x102c0018 => { vm.decode(0x4860fb5u32); let d = 20i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c001cu64) }
        0x102c001c => { vm.decode(0x11a835u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0020u64) }
        0x102c0020 => { vm.decode(0x60042835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0024u64) }
        0x102c0024 => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0028u64) }
        0x102c0028 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c002cu64) }
        0x102c002c => { vm.decode(0xd186111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0030u64) }
        0x102c0030 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0034u64) }
        0x102c0034 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0038u64) }
        0x102c0038 => { vm.decode(0x1881cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c003cu64) }
        0x102c003c => { vm.decode(0xe4840fb5u32); let d = 19i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0040u64) }
        0x102c0040 => { vm.decode(0x2050b835u32); let d = 1i16; let s = 19i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0044u64) }
        0x102c0044 => { vm.decode(0x2000a835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0048u64) }
        0x102c0048 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c004cu64) }
        0x102c004c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0050u64) }
        0x102c0050 => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0054u64) }
        0x102c0054 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0058u64) }
        0x102c0058 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c005cu64) }
        0x102c005c => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0060u64) }
        0x102c0060 => { vm.decode(0xa4052fb5u32); let d = 18i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0064u64) }
        0x102c0064 => { vm.decode(0x40543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0068u64) }
        0x102c0068 => { vm.decode(0x2000b835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c006cu64) }
        0x102c006c => { vm.decode(0x45944211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0070u64) }
        0x102c0070 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0074u64) }
        0x102c0074 => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0078u64) }
        0x102c0078 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c007cu64) }
        0x102c007c => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0080u64) }
        0x102c0080 => { vm.decode(0x1a00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0084u64) }
        0x102c0084 => { vm.decode(0xc4002fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0088u64) }
        0x102c0088 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c008cu64) }
        0x102c008c => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0090u64) }
        0x102c0090 => { vm.decode(0x5600c219u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 12800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0094u64) }
        0x102c0094 => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0098u64) }
        0x102c0098 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c009cu64) }
        0x102c009c => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c00a0u64).wrapping_add((imm * 4) as u64) } else { 0x102c00a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c00a0u64) }
        0x102c00a0 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c00a4u64) }
        0x102c00ac => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c00b0u64) }
        0x102c00b0 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102c00b4u64) }
        0x102c00b4 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c00b8u64) }
        0x102c00b8 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c00bcu64) }
        0x102c00bc => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c00c0u64) }
        0x102c00c0 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c00c4u64) }
        0x102c00c4 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c00c8u64) }
        0x102c00c8 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c00ccu64) }
        0x102c00cc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c00d0u64) }
        0x102c00d0 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c00d4u64) }
        0x102c00d4 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c00d8u64) }
        0x102c00d8 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c00dcu64).wrapping_add((imm * 4) as u64) } else { 0x102c00dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c00dcu64) }
        0x102c00dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c00e0u64) } }
        0x102c00e0 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c00e4u64) }
        0x102c00e4 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c00e8u64) }
        0x102c00e8 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c00ecu64) }
        0x102c00ec => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c00f0u64) }
        0x102c00f0 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c00f4u64) }
        0x102c00f4 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c00f8u64) }
        0x102c00f8 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c00fcu64) }
        0x102c00fc => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0100u64) }
        0x102c0100 => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c0104u64) }
        0x102c0104 => { vm.decode(0x288043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0108u64) }
        0x102c0108 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c010cu64) }
        0x102c010c => { vm.decode(0x1c000011u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0110u64) }
        0x102c0110 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0114u64) }
        0x102c0114 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0118u64) }
        0x102c0118 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c011cu64 + 4);
        vm.wr(37, 0x102c011cu64 + 4);
        Ok(0x102c011cu64) }
        0x102c011c => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0120u64) }
        0x102c0120 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0124u64) }
        0x102c0124 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0128u64) }
        0x102c0128 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c012cu64) }
        0x102c012c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c0130u64) }
        0x102c0130 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c0134u64).wrapping_add((imm * 4) as u64) } else { 0x102c0134u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c0134u64) }
        0x102c0134 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c0138u64) }
        0x102c0138 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c013cu64) }
        0x102c013c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0140u64) }
        0x102c0140 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c0144u64) }
        0x102c0144 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c0148u64) }
        0x102c0148 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c014cu64).wrapping_add((imm * 4) as u64) } else { 0x102c014cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c014cu64) }
        0x102c014c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c0150u64) } }
        0x102c0150 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c0154u64) }
        0x102c0154 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c0158u64) }
        0x102c0158 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c015cu64).wrapping_add((imm * 4) as u64) } else { 0x102c015cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c015cu64) }
        0x102c015c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c0160u64) }
        0x102c0160 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c0164u64) }
        0x102c0164 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0168u64) }
        0x102c0168 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c016cu64) }
        0x102c016c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c0170u64) }
        0x102c0170 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c0174u64).wrapping_add((imm * 4) as u64) } else { 0x102c0174u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c0174u64) }
        0x102c0174 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c0178u64) } }
        0x102c0178 => { vm.decode(0x2f4c501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 236844i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c017cu64) }
        0x102c017c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c0180u64) } }
        0x102c0180 => { vm.decode(0x110229b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c0184u64) }
        0x102c0184 => { vm.decode(0x1d98f811u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0188u64) }
        0x102c0188 => { vm.decode(0x4388875u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c018cu64) }
        0x102c018c => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0190u64) }
        0x102c0190 => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0194u64) }
        0x102c0194 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0198u64) }
        0x102c0198 => { vm.decode(0x45906a51u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c019cu64) }
        0x102c019c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c01a0u64) }
        0x102c01a0 => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c01a4u64) }
        0x102c01a4 => { vm.decode(0x40190fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c01a8u64) }
        0x102c01a8 => { vm.decode(0x2a88c36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c01acu64) }
        0x102c01ac => { vm.decode(0x48320b75u32); let d = 20i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102c01b0u64) }
        0x102c01b0 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c01b4u64) }
        0x102c01b4 => { vm.decode(0xca0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c01b8u64) }
        0x102c01b8 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c01bcu64) }
        0x102c01bc => { vm.decode(0x8229b5u32); let d = 4i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c01c0u64) }
        0x102c01c0 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c01c4u64) }
        0x102c01c4 => { vm.decode(0x62255022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c01c8u64) }
        0x102c01c8 => { vm.decode(0x4d90eb51u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01ccu64) }
        0x102c01cc => { vm.decode(0x590fad1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1884i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01d0u64) }
        0x102c01d0 => { vm.decode(0xd947251u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01d4u64) }
        0x102c01d4 => { vm.decode(0xe114d35u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c01d8u64) }
        0x102c01d8 => { vm.decode(0x411129b5u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c01dcu64) }
        0x102c01dc => { vm.decode(0x45906bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1876i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01e0u64) }
        0x102c01e0 => { vm.decode(0xd18e2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1932i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01e4u64) }
        0x102c01e4 => { vm.decode(0x91230fb5u32); let d = 12i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c01e8u64) }
        0x102c01e8 => { vm.decode(0x4518e251u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01ecu64) }
        0x102c01ec => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01f0u64) }
        0x102c01f0 => { vm.decode(0x10a04875u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c01f4u64) }
        0x102c01f4 => { vm.decode(0x2112a835u32); let d = 5i16; let s = 4i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c01f8u64) }
        0x102c01f8 => { vm.decode(0x51863d1u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1924i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c01fcu64) }
        0x102c01fc => { vm.decode(0x4d9c7811u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0200u64) }
        0x102c0200 => { vm.decode(0x75112fb5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0204u64) }
        0x102c0204 => { vm.decode(0xd186351u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0208u64) }
        0x102c0208 => { vm.decode(0x65a06fb5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c020cu64) }
        0x102c020c => { vm.decode(0x4594ebd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0210u64) }
        0x102c0210 => { vm.decode(0x75292fb5u32); let d = 11i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0214u64) }
        0x102c0214 => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0218u64) }
        0x102c0218 => { vm.decode(0x55a24fb5u32); let d = 13i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c021cu64) }
        0x102c021c => { vm.decode(0x47604875u32); let d = 10i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0220u64) }
        0x102c0220 => { vm.decode(0x120f2835u32); let d = 11i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0224u64) }
        0x102c0224 => { vm.decode(0x4594fa51u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0228u64) }
        0x102c0228 => { vm.decode(0x459cf911u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c022cu64) }
        0x102c022c => { vm.decode(0x35a26fb5u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0230u64) }
        0x102c0230 => { vm.decode(0x4d947ad1u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1908i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0234u64) }
        0x102c0234 => { vm.decode(0x60b36fb5u32); let d = 7i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0238u64) }
        0x102c0238 => { vm.decode(0x409119b5u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c023cu64) }
        0x102c023c => { vm.decode(0x60ab2fb5u32); let d = 7i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0240u64) }
        0x102c0240 => { vm.decode(0x3835cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0244u64) }
        0x102c0244 => { vm.decode(0x75b30fb5u32); let d = 15i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0248u64) }
        0x102c0248 => { vm.decode(0x55e0d875u32); let d = 7i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c024cu64) }
        0x102c024c => { vm.decode(0x610fa835u32); let d = 7i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0250u64) }
        0x102c0250 => { vm.decode(0x45947b51u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0254u64) }
        0x102c0254 => { vm.decode(0x15105211u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0258u64) }
        0x102c0258 => { vm.decode(0xf4320fb5u32); let d = 7i16; let s = 7i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c025cu64) }
        0x102c025c => { vm.decode(0x1594f2d1u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1900i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0260u64) }
        0x102c0260 => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0264u64) }
        0x102c0264 => { vm.decode(0x411019b5u32); let d = 8i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c0268u64) }
        0x102c0268 => { vm.decode(0x648b6fb5u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c026cu64) }
        0x102c026c => { vm.decode(0x3e34cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0270u64) }
        0x102c0270 => { vm.decode(0x55b52fb5u32); let d = 25i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0274u64) }
        0x102c0274 => { vm.decode(0x45b08875u32); let d = 7i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0278u64) }
        0x102c0278 => { vm.decode(0x614fa835u32); let d = 7i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c027cu64) }
        0x102c027c => { vm.decode(0x1594e251u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0280u64) }
        0x102c0280 => { vm.decode(0x5105a11u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0284u64) }
        0x102c0284 => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0288u64) }
        0x102c0288 => { vm.decode(0x59472d1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1892i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c028cu64) }
        0x102c028c => { vm.decode(0x74330fb5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0290u64) }
        0x102c0290 => { vm.decode(0x511219b5u32); let d = 9i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c0294u64) }
        0x102c0294 => { vm.decode(0x60936fb5u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0298u64) }
        0x102c0298 => { vm.decode(0x3e354d35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c029cu64) }
        0x102c029c => { vm.decode(0x95330fb5u32); let d = 12i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02a0u64) }
        0x102c02a0 => { vm.decode(0x41a8c875u32); let d = 7i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c02a4u64) }
        0x102c02a4 => { vm.decode(0x611ea835u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c02a8u64) }
        0x102c02a8 => { vm.decode(0xd10fa91u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c02acu64) }
        0x102c02ac => { vm.decode(0x1d9c6011u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c02b0u64) }
        0x102c02b0 => { vm.decode(0xc4350fb5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02b4u64) }
        0x102c02b4 => { vm.decode(0x4d10eb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c02b8u64) }
        0x102c02b8 => { vm.decode(0x15080fb5u32); let d = 8i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02bcu64) }
        0x102c02bc => { vm.decode(0x1110a9b5u32); let d = 17i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c02c0u64) }
        0x102c02c0 => { vm.decode(0x15104fb5u32); let d = 8i16; let s = 8i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02c4u64) }
        0x102c02c4 => { vm.decode(0xc813d35u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02c8u64) }
        0x102c02c8 => { vm.decode(0x51824fb5u32); let d = 13i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02ccu64) }
        0x102c02cc => { vm.decode(0x6286875u32); let d = 8i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c02d0u64) }
        0x102c02d0 => { vm.decode(0x14032835u32); let d = 17i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c02d4u64) }
        0x102c02d4 => { vm.decode(0x5107a91u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c02d8u64) }
        0x102c02d8 => { vm.decode(0x5510ca11u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c02dcu64) }
        0x102c02dc => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02e0u64) }
        0x102c02e0 => { vm.decode(0x1199b5u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c02e4u64) }
        0x102c02e4 => { vm.decode(0x840c0fb5u32); let d = 16i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02e8u64) }
        0x102c02e8 => { vm.decode(0xe5324fb5u32); let d = 15i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02ecu64) }
        0x102c02ec => { vm.decode(0x3835fd35u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02f0u64) }
        0x102c02f0 => { vm.decode(0x75b34fb5u32); let d = 15i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c02f4u64) }
        0x102c02f4 => { vm.decode(0x11099b5u32); let d = 16i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c02f8u64) }
        0x102c02f8 => { vm.decode(0xc60f875u32); let d = 17i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c02fcu64) }
        0x102c02fc => { vm.decode(0x64033835u32); let d = 17i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0300u64) }
        0x102c0300 => { vm.decode(0x55144211u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0304u64) }
        0x102c0304 => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0308u64) }
        0x102c0308 => { vm.decode(0x950c0fb5u32); let d = 24i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c030cu64) }
        0x102c030c => { vm.decode(0x950c4fb5u32); let d = 24i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0310u64) }
        0x102c0310 => { vm.decode(0x8f00bd35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0314u64) }
        0x102c0314 => { vm.decode(0x118d6fb5u32); let d = 24i16; let s = 15i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0318u64) }
        0x102c0318 => { vm.decode(0x511299b5u32); let d = 25i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c031cu64) }
        0x102c031c => { vm.decode(0x8302875u32); let d = 16i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0320u64) }
        0x102c0320 => { vm.decode(0x24431835u32); let d = 16i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0324u64) }
        0x102c0324 => { vm.decode(0x800c2fb5u32); let d = 16i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0328u64) }
        0x102c0328 => { vm.decode(0x318a2fb5u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c032cu64) }
        0x102c032c => { vm.decode(0x11926fb5u32); let d = 12i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0330u64) }
        0x102c0330 => { vm.decode(0xe253d35u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0334u64) }
        0x102c0334 => { vm.decode(0x91124fb5u32); let d = 12i16; let s = 12i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0338u64) }
        0x102c0338 => { vm.decode(0x409129b5u32); let d = 14i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c033cu64) }
        0x102c033c => { vm.decode(0xb28e875u32); let d = 25i16; let s = 12i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0340u64) }
        0x102c0340 => { vm.decode(0x56123835u32); let d = 25i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0344u64) }
        0x102c0344 => { vm.decode(0x1514c211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0348u64) }
        0x102c0348 => { vm.decode(0xc50c4fb5u32); let d = 25i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c034cu64) }
        0x102c034c => { vm.decode(0x75096fb5u32); let d = 11i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0350u64) }
        0x102c0350 => { vm.decode(0x75116fb5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0354u64) }
        0x102c0354 => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0358u64) }
        0x102c0358 => { vm.decode(0x75a04fb5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c035cu64) }
        0x102c035c => { vm.decode(0x509019b5u32); let d = 13i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c0360u64) }
        0x102c0360 => { vm.decode(0x57e01875u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0364u64) }
        0x102c0364 => { vm.decode(0x130f8835u32); let d = 14i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0368u64) }
        0x102c0368 => { vm.decode(0x1d145a11u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c036cu64) }
        0x102c036c => { vm.decode(0xb1334fb5u32); let d = 14i16; let s = 14i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0370u64) }
        0x102c0370 => { vm.decode(0x31306fb5u32); let d = 10i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0374u64) }
        0x102c0374 => { vm.decode(0x35306fb5u32); let d = 10i16; let s = 10i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0378u64) }
        0x102c0378 => { vm.decode(0x2811bd35u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c037cu64) }
        0x102c037c => { vm.decode(0x31216fb5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0380u64) }
        0x102c0380 => { vm.decode(0x419309b5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c0384u64) }
        0x102c0384 => { vm.decode(0x13a89875u32); let d = 13i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0388u64) }
        0x102c0388 => { vm.decode(0x7342a835u32); let d = 13i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c038cu64) }
        0x102c038c => { vm.decode(0x4d14db11u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0390u64) }
        0x102c0390 => { vm.decode(0x55936fb5u32); let d = 13i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0394u64) }
        0x102c0394 => { vm.decode(0x24926fb5u32); let d = 6i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0398u64) }
        0x102c0398 => { vm.decode(0x208a6fb5u32); let d = 6i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c039cu64) }
        0x102c039c => { vm.decode(0x2e348d35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03a0u64) }
        0x102c03a0 => { vm.decode(0x30322fb5u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03a4u64) }
        0x102c03a4 => { vm.decode(0x509209b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c03a8u64) }
        0x102c03a8 => { vm.decode(0x53c05875u32); let d = 14i16; let s = 6i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c03acu64) }
        0x102c03ac => { vm.decode(0x134e8835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c03b0u64) }
        0x102c03b0 => { vm.decode(0x4d905311u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c03b4u64) }
        0x102c03b4 => { vm.decode(0x31b36fb5u32); let d = 14i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03b8u64) }
        0x102c03b8 => { vm.decode(0x40b34fb5u32); let d = 5i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03bcu64) }
        0x102c03bc => { vm.decode(0x40934fb5u32); let d = 5i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03c0u64) }
        0x102c03c0 => { vm.decode(0x1e254d35u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03c4u64) }
        0x102c03c4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03c8u64) }
        0x102c03c8 => { vm.decode(0x408219b5u32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c03ccu64) }
        0x102c03cc => { vm.decode(0x1788c875u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c03d0u64) }
        0x102c03d0 => { vm.decode(0x5341a835u32); let d = 13i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c03d4u64) }
        0x102c03d4 => { vm.decode(0x4590d311u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c03d8u64) }
        0x102c03d8 => { vm.decode(0x55926fb5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03dcu64) }
        0x102c03dc => { vm.decode(0x4924fb5u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03e0u64) }
        0x102c03e0 => { vm.decode(0x4a24fb5u32); let d = 4i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03e4u64) }
        0x102c03e4 => { vm.decode(0xca50d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03e8u64) }
        0x102c03e8 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03ecu64) }
        0x102c03ec => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03f0u64) }
        0x102c03f0 => { vm.decode(0x510119b5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c03f4u64) }
        0x102c03f4 => { vm.decode(0x13804875u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c03f8u64) }
        0x102c03f8 => { vm.decode(0x45804fb5u32); let d = 9i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c03fcu64) }
        0x102c03fc => { vm.decode(0x30d7062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c0400u64) }
        0x102c0400 => { vm.decode(0x3009a835u32); let d = 3i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0404u64) }
        0x102c0404 => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0408u64) }
        0x102c0408 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c040cu64) }
        0x102c040c => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0410u64) }
        0x102c0410 => { vm.decode(0x20206fb5u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0414u64) }
        0x102c0414 => { vm.decode(0x28118d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0418u64) }
        0x102c0418 => { vm.decode(0x20a00fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c041cu64) }
        0x102c041c => { vm.decode(0x41489875u32); let d = 3i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0420u64) }
        0x102c0420 => { vm.decode(0x31f3462u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c0424u64) }
        0x102c0424 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c0428u64) }
        0x102c0428 => { vm.decode(0x4322fb5u32); let d = 4i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c042cu64) }
        0x102c042c => { vm.decode(0x25d3062u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c0430u64) }
        0x102c0430 => { vm.decode(0x5944311u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c0434u64) }
        0x102c0434 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0438u64) }
        0x102c0438 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c043cu64) }
        0x102c043c => { vm.decode(0x40310fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0440u64) }
        0x102c0440 => { vm.decode(0x1e00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0444u64) }
        0x102c0444 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c0448u64) }
        0x102c0448 => { vm.decode(0x40014fb5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c044cu64) }
        0x102c044c => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c0450u64) }
        0x102c0450 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0454u64) }
        0x102c0454 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c0458u64) }
        0x102c0458 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c045cu64) }
        0x102c045c => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c0460u64) }
        0x102c0460 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0464u64) }
        0x102c0464 => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c0468u64) }
        0x102c0468 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c046cu64) }
        0x102c046c => { vm.decode(0x4056e3d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12797i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0470u64) }
        0x102c0470 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c0474u64) }
        0x102c0474 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c0478u64) }
        0x102c0478 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c047cu64) }
        0x102c047c => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c0480u64) }
        0x102c0480 => { vm.decode(0x22028000u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102c0484u64) }
        0x102c0484 => { vm.decode(0x40c82a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 185i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c0488u64).wrapping_add((imm * 4) as u64) } else { 0x102c0488u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c0488u64) }
        0x102c0488 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c048cu64) }
        0x102c048c => { vm.decode(0x8bec011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 242408i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c0490u64) }
        0x102c0490 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c0494u64) } }
        0x102c1668 => { vm.decode(0xb8027u32); let d = 0i16; let s = 20i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c166cu64).wrapping_add((imm * 4) as u64) } else { 0x102c166cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c166cu64) }
        0x102c166c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c1670u64) } }
        0x102c16b0 => { vm.decode(0x4a8063afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16b4u64) }
        0x102c16b4 => { vm.decode(0x2804402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16b8u64) }
        0x102c16b8 => { vm.decode(0x10408019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c16bcu64) }
        0x102c16bc => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16c0u64) }
        0x102c16c0 => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16c4u64) }
        0x102c16c4 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16c8u64) }
        0x102c16c8 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16ccu64) }
        0x102c16cc => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16d0u64) }
        0x102c16d0 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16d4u64) }
        0x102c16d4 => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16d8u64) }
        0x102c16d8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c16dcu64) }
        0x102c16dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c16e0u64 + 4);
        vm.wr(37, 0x102c16e0u64 + 4);
        Ok(0x102c16e0u64) }
        0x102c16e0 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c16e4u64) }
        0x102c16e4 => { vm.decode(0x2200406fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c16e8u64) }
        0x102c16e8 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c16ecu64) }
        0x102c16ec => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16f0u64) }
        0x102c16f0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c16f4u64) }
        0x102c16f4 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16f8u64) }
        0x102c16f8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c16fcu64) }
        0x102c16fc => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1700u64) }
        0x102c1700 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1704u64) }
        0x102c1704 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c1708u64 + 4);
        vm.wr(37, 0x102c1708u64 + 4);
        Ok(0x102c1708u64) }
        0x102c1708 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c170cu64) }
        0x102c170c => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1710u64) }
        0x102c1710 => { vm.decode(0xd400c219u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 12802i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1714u64) }
        0x102c1714 => { vm.decode(0x10101019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1718u64) }
        0x102c1718 => { vm.decode(0x740d9419u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c171cu64) }
        0x102c171c => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c1720u64) }
        0x102c1720 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c1724u64).wrapping_add((imm * 4) as u64) } else { 0x102c1724u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c1724u64) }
        0x102c1724 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c1728u64) }
        0x102c1730 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c1734u64) }
        0x102c1734 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102c1738u64) }
        0x102c1738 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c173cu64) }
        0x102c173c => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c1740u64) }
        0x102c1740 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1744u64) }
        0x102c1744 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1748u64) }
        0x102c1748 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c174cu64) }
        0x102c174c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c1750u64) }
        0x102c1750 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c1754u64) }
        0x102c1754 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1758u64) }
        0x102c1758 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c175cu64) }
        0x102c175c => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c1760u64).wrapping_add((imm * 4) as u64) } else { 0x102c1760u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c1760u64) }
        0x102c1760 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c1764u64) } }
        0x102c1764 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1768u64) }
        0x102c1768 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c176cu64) }
        0x102c176c => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1770u64) }
        0x102c1770 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1774u64) }
        0x102c1774 => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1778u64) }
        0x102c1778 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c177cu64) }
        0x102c177c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1780u64) }
        0x102c1780 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c1784u64) }
        0x102c1784 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1788u64) }
        0x102c1788 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c178cu64) }
        0x102c178c => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c1790u64) }
        0x102c1790 => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1794u64) }
        0x102c1794 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1798u64) }
        0x102c1798 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c179cu64) }
        0x102c179c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c17a0u64 + 4);
        vm.wr(37, 0x102c17a0u64 + 4);
        Ok(0x102c17a0u64) }
        0x102c17a0 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c17a4u64) }
        0x102c17a4 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c17a8u64) }
        0x102c17a8 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c17acu64) }
        0x102c17ac => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17b0u64) }
        0x102c17b0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c17b4u64) }
        0x102c17b4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c17b8u64).wrapping_add((imm * 4) as u64) } else { 0x102c17b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c17b8u64) }
        0x102c17b8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17bcu64) }
        0x102c17bc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17c0u64) }
        0x102c17c0 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c17c4u64) }
        0x102c17c4 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c17c8u64) }
        0x102c17c8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c17ccu64) }
        0x102c17cc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c17d0u64).wrapping_add((imm * 4) as u64) } else { 0x102c17d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c17d0u64) }
        0x102c17d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c17d4u64) } }
        0x102c17d4 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17d8u64) }
        0x102c17d8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c17dcu64) }
        0x102c17dc => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c17e0u64).wrapping_add((imm * 4) as u64) } else { 0x102c17e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c17e0u64) }
        0x102c17e0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17e4u64) }
        0x102c17e4 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c17e8u64) }
        0x102c17e8 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c17ecu64) }
        0x102c17ec => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c17f0u64) }
        0x102c17f0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c17f4u64) }
        0x102c17f4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c17f8u64).wrapping_add((imm * 4) as u64) } else { 0x102c17f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c17f8u64) }
        0x102c17f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c17fcu64) } }
        0x102c17fc => { vm.decode(0x83cc151du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 242608i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c1800u64) }
        0x102c1800 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c1804u64) } }
        0x102c1804 => { vm.decode(0xd10e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1808u64) }
        0x102c1808 => { vm.decode(0x45106a91u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c180cu64) }
        0x102c180c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1810u64) }
        0x102c1810 => { vm.decode(0x4510e051u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1814u64) }
        0x102c1814 => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1818u64) }
        0x102c1818 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c181cu64) }
        0x102c181c => { vm.decode(0x4d106851u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1820u64) }
        0x102c1820 => { vm.decode(0x510eb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1824u64) }
        0x102c1824 => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c1828u64) }
        0x102c1828 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c182cu64) }
        0x102c182c => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1830u64) }
        0x102c1830 => { vm.decode(0x31d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c1834u64) }
        0x102c1834 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1838u64) }
        0x102c1838 => { vm.decode(0x4d10ea91u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c183cu64) }
        0x102c183c => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c1840u64) }
        0x102c1840 => { vm.decode(0x31d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c1844u64) }
        0x102c1844 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1848u64) }
        0x102c1848 => { vm.decode(0x20200833u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c184cu64) }
        0x102c184c => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1850u64) }
        0x102c1850 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c1854u64) }
        0x102c1854 => { vm.decode(0x80220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1858u64) }
        0x102c1858 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c185cu64) }
        0x102c185c => { vm.decode(0xc2a08d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 786i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c1860u64) }
        0x102c1860 => { vm.decode(0x2208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1864u64) }
        0x102c1864 => { vm.decode(0x714132beu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 787i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c1868u64) }
        0x102c1868 => { vm.decode(0x802208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c186cu64) }
        0x102c186c => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1870u64) }
        0x102c1870 => { vm.decode(0x220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1874u64) }
        0x102c1874 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1878u64) }
        0x102c1878 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c187cu64) }
        0x102c187c => { vm.decode(0x80220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1880u64) }
        0x102c1880 => { vm.decode(0xc2a08da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 790i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c1884u64) }
        0x102c1884 => { vm.decode(0x220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1888u64) }
        0x102c1888 => { vm.decode(0x714132feu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 791i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c188cu64) }
        0x102c188c => { vm.decode(0x80220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1890u64) }
        0x102c1890 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c1894u64) }
        0x102c1894 => { vm.decode(0x220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c1898u64) }
        0x102c1898 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c189cu64) }
        0x102c189c => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c18a0u64) }
        0x102c18a0 => { vm.decode(0x80220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18a4u64) }
        0x102c18a4 => { vm.decode(0xc2a18d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 794i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c18a8u64) }
        0x102c18a8 => { vm.decode(0x2209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18acu64) }
        0x102c18ac => { vm.decode(0x71413abeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 795i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c18b0u64) }
        0x102c18b0 => { vm.decode(0x802209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18b4u64) }
        0x102c18b4 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c18b8u64) }
        0x102c18b8 => { vm.decode(0x220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18bcu64) }
        0x102c18bc => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c18c0u64) }
        0x102c18c0 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c18c4u64) }
        0x102c18c4 => { vm.decode(0x80220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18c8u64) }
        0x102c18c8 => { vm.decode(0xc2a18da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 798i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c18ccu64) }
        0x102c18cc => { vm.decode(0x220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18d0u64) }
        0x102c18d0 => { vm.decode(0x71413afeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 799i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c18d4u64) }
        0x102c18d4 => { vm.decode(0x80220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18d8u64) }
        0x102c18d8 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c18dcu64) }
        0x102c18dc => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c18e0u64) }
        0x102c18e0 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c18e4u64) }
        0x102c18e4 => { vm.decode(0x3cb84ea7u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 11518i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c18e8u64).wrapping_add((imm * 4) as u64) } else { 0x102c18e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c18e8u64) }
        0x102c18e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c18ecu64) } }
        0x102c18ec => { vm.decode(0x404024f5u32); let d = 1i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c18f0u64) }
        0x102c18f0 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c18f4u64) }
        0x102c18f4 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c18f8u64).wrapping_add((imm * 4) as u64) } else { 0x102c18f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c18f8u64) }
        0x102c18f8 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c18fcu64) }
        0x102c18fc => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c1900u64) }
        0x102c1900 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c1904u64) } }
        0x102c1bdc => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1be0u64) }
        0x102c1be0 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1be4u64) }
        0x102c1be4 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1be8u64) }
        0x102c1be8 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1becu64) }
        0x102c1bec => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c1bf0u64) }
        0x102c1bf0 => { vm.decode(0x4c645619u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = -16872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1bf4u64) }
        0x102c1bf4 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1bf8u64) }
        0x102c1bf8 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c1bfcu64) }
        0x102c1bfc => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1c00u64) }
        0x102c1c00 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c1c04u64) }
        0x102c1c04 => { vm.decode(0xa2028000u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102c1c08u64) }
        0x102c1c08 => { vm.decode(0x40c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 184i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c1c0cu64).wrapping_add((imm * 4) as u64) } else { 0x102c1c0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c1c0cu64) }
        0x102c1c0c => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c1c10u64) }
        0x102c1c10 => { vm.decode(0xbe8c01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 248424i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c1c14u64) }
        0x102c1c14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c1c18u64) } }
        0x102c1c18 => { vm.decode(0x110039b5u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c1c1cu64) }
        0x102c2de8 => { vm.decode(0x80b8027u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c2decu64).wrapping_add((imm * 4) as u64) } else { 0x102c2decu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2decu64) }
        0x102c2dec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c2df0u64) } }
        0x102c2df0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2df4u64) }
        0x102c2df4 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2df8u64) }
        0x102c2df8 => { vm.decode(0x89215db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2dfcu64) }
        0x102c2dfc => { vm.decode(0x28843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e00u64) }
        0x102c2e00 => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e04u64) }
        0x102c2e04 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e08u64) }
        0x102c2e08 => { vm.decode(0xd9c4011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2e0cu64) }
        0x102c2e0c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102c2e10u64) }
        0x102c2e10 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e14u64) }
        0x102c2e14 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c2e18u64) }
        0x102c2e18 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2e1cu64) }
        0x102c2e1c => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e20u64) }
        0x102c2e20 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e24u64) }
        0x102c2e24 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e28u64) }
        0x102c2e28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c2e2cu64 + 4);
        vm.wr(37, 0x102c2e2cu64 + 4);
        Ok(0x102c2e2cu64) }
        0x102c2e2c => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2e30u64) }
        0x102c2e30 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2e34u64) }
        0x102c2e34 => { vm.decode(0x9236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c2e38u64) }
        0x102c2e38 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2e3cu64) }
        0x102c2e3c => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102c2e40u64) }
        0x102c2e40 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c2e44u64).wrapping_add((imm * 4) as u64) } else { 0x102c2e44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2e44u64) }
        0x102c2e44 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102c2e48u64) }
        0x102c2e48 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2e4cu64) }
        0x102c2e54 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c2e58u64).wrapping_add((imm * 4) as u64) } else { 0x102c2e58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2e58u64) }
        0x102c2e58 => { vm.decode(0x12002019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2e5cu64) }
        0x102c2e5c => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c2e60u64) }
        0x102c2e60 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c2e64u64) }
        0x102c2e64 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102c2e68u64) }
        0x102c2e68 => { vm.decode(0xa0002040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102c2e6cu64) }
        0x102c2e6c => { vm.decode(0x840aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 142i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c2e70u64).wrapping_add((imm * 4) as u64) } else { 0x102c2e70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2e70u64) }
        0x102c2e70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c2e74u64) } }
        0x102c2e74 => { vm.decode(0x6a8863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e78u64) }
        0x102c2e78 => { vm.decode(0x2020416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e7cu64) }
        0x102c2e7c => { vm.decode(0x662d0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2e80u64) }
        0x102c2e80 => { vm.decode(0x9389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e84u64) }
        0x102c2e84 => { vm.decode(0x9211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e88u64) }
        0x102c2e88 => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e8cu64) }
        0x102c2e8c => { vm.decode(0x9395c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e90u64) }
        0x102c2e90 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2e94u64) }
        0x102c2e94 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102c2e98u64) }
        0x102c2e98 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2e9cu64) }
        0x102c2e9c => { vm.decode(0xc41019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2ea0u64) }
        0x102c2ea0 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c2ea4u64) }
        0x102c2ea4 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2ea8u64) }
        0x102c2ea8 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2eacu64) }
        0x102c2eac => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2eb0u64) }
        0x102c2eb0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2eb4u64) }
        0x102c2eb4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c2eb8u64 + 4);
        vm.wr(37, 0x102c2eb8u64 + 4);
        Ok(0x102c2eb8u64) }
        0x102c2eb8 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2ebcu64) }
        0x102c2ebc => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ec0u64) }
        0x102c2ec0 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ec4u64) }
        0x102c2ec4 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ec8u64) }
        0x102c2ec8 => { vm.decode(0x43351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2eccu64) }
        0x102c2ecc => { vm.decode(0x55106b91u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ed0u64) }
        0x102c2ed0 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ed4u64) }
        0x102c2ed4 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ed8u64) }
        0x102c2ed8 => { vm.decode(0xd90ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2edcu64) }
        0x102c2edc => { vm.decode(0x63251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ee0u64) }
        0x102c2ee0 => { vm.decode(0xd90ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ee4u64) }
        0x102c2ee4 => { vm.decode(0x30d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ee8u64) }
        0x102c2ee8 => { vm.decode(0xd906a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 852i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2eecu64) }
        0x102c2eec => { vm.decode(0x231d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ef0u64) }
        0x102c2ef0 => { vm.decode(0xd906a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2ef4u64) }
        0x102c2ef4 => { vm.decode(0x230d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2ef8u64) }
        0x102c2ef8 => { vm.decode(0xd90e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 844i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2efcu64) }
        0x102c2efc => { vm.decode(0x231d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f00u64) }
        0x102c2f00 => { vm.decode(0xd90e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f04u64) }
        0x102c2f04 => { vm.decode(0x30d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f08u64) }
        0x102c2f08 => { vm.decode(0xd906291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 836i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f0cu64) }
        0x102c2f0c => { vm.decode(0x431d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f10u64) }
        0x102c2f10 => { vm.decode(0xd906211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f14u64) }
        0x102c2f14 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f18u64) }
        0x102c2f18 => { vm.decode(0xd14ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 828i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f1cu64) }
        0x102c2f1c => { vm.decode(0x430d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f20u64) }
        0x102c2f20 => { vm.decode(0xd14ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f24u64) }
        0x102c2f24 => { vm.decode(0x630d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f28u64) }
        0x102c2f28 => { vm.decode(0xd146a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 820i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f2cu64) }
        0x102c2f2c => { vm.decode(0x431d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f30u64) }
        0x102c2f30 => { vm.decode(0xd146a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f34u64) }
        0x102c2f34 => { vm.decode(0x63351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f38u64) }
        0x102c2f38 => { vm.decode(0xd14e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 812i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f3cu64) }
        0x102c2f3c => { vm.decode(0x430d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f40u64) }
        0x102c2f40 => { vm.decode(0xd14e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f44u64) }
        0x102c2f44 => { vm.decode(0x230d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f48u64) }
        0x102c2f48 => { vm.decode(0xd146291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 804i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f4cu64) }
        0x102c2f4c => { vm.decode(0x63353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f50u64) }
        0x102c2f50 => { vm.decode(0xd146211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c2f54u64) }
        0x102c2f54 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2f58u64) }
        0x102c2f58 => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2f5cu64) }
        0x102c2f5c => { vm.decode(0x2824412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2f60u64) }
        0x102c2f60 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f64u64) }
        0x102c2f64 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2f68u64) }
        0x102c2f68 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f6cu64) }
        0x102c2f6c => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f70u64) }
        0x102c2f70 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f74u64) }
        0x102c2f74 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2f78u64) }
        0x102c2f78 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2f7cu64) }
        0x102c2f7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c2f80u64 + 4);
        vm.wr(37, 0x102c2f80u64 + 4);
        Ok(0x102c2f80u64) }
        0x102c2f80 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2f84u64) }
        0x102c2f84 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2f88u64) }
        0x102c2f88 => { vm.decode(0x2220416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2f8cu64) }
        0x102c2f8c => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f90u64) }
        0x102c2f90 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c2f94u64) }
        0x102c2f94 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f98u64) }
        0x102c2f98 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c2f9cu64) }
        0x102c2f9c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2fa0u64) }
        0x102c2fa0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2fa4u64) }
        0x102c2fa4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c2fa8u64 + 4);
        vm.wr(37, 0x102c2fa8u64 + 4);
        Ok(0x102c2fa8u64) }
        0x102c2fa8 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c2facu64) }
        0x102c2fac => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2fb0u64) }
        0x102c2fb0 => { vm.decode(0x5c64c619u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = -16872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2fb4u64) }
        0x102c2fb4 => { vm.decode(0x10101019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c2fb8u64) }
        0x102c2fb8 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c2fbcu64) }
        0x102c2fbc => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c2fc0u64).wrapping_add((imm * 4) as u64) } else { 0x102c2fc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2fc0u64) }
        0x102c2fc0 => { vm.decode(0x4102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c2fc4u64) }
        0x102c2fc4 => { vm.decode(0x748d41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 249136i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c2fc8u64) }
        0x102c2fcc => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c2fd0u64) }
        0x102c2fd0 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102c2fd4u64) }
        0x102c2fd4 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2fd8u64) }
        0x102c2fd8 => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c2fdcu64) }
        0x102c2fdc => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2fe0u64) }
        0x102c2fe0 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c2fe4u64) }
        0x102c2fe4 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c2fe8u64) }
        0x102c2fe8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c2fecu64) }
        0x102c2fec => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c2ff0u64) }
        0x102c2ff0 => { vm.decode(0x6a82636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c2ff4u64) }
        0x102c2ff4 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c2ff8u64) }
        0x102c2ff8 => { vm.decode(0x4848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c2ffcu64).wrapping_add((imm * 4) as u64) } else { 0x102c2ffcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c2ffcu64) }
        0x102c2ffc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c3000u64) } }
        0x102c3000 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3004u64) }
        0x102c3004 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c3008u64) }
        0x102c3008 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c300cu64) }
        0x102c300c => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3010u64) }
        0x102c3010 => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3014u64) }
        0x102c3014 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3018u64) }
        0x102c3018 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c301cu64) }
        0x102c301c => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3020u64) }
        0x102c3020 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3024u64) }
        0x102c3024 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c3028u64) }
        0x102c3028 => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c302cu64) }
        0x102c302c => { vm.decode(0x5c000111u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3030u64) }
        0x102c3030 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3034u64) }
        0x102c3034 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3038u64) }
        0x102c3038 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c303cu64 + 4);
        vm.wr(37, 0x102c303cu64 + 4);
        Ok(0x102c303cu64) }
        0x102c303c => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3040u64) }
        0x102c3040 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3044u64) }
        0x102c3044 => { vm.decode(0x288c8b35u32); let d = 4i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_sub(vm.ld(s)) as u32 as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3048u64) }
        0x102c3048 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c304cu64) }
        0x102c304c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c3050u64) }
        0x102c3050 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c3054u64).wrapping_add((imm * 4) as u64) } else { 0x102c3054u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c3054u64) }
        0x102c3054 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c3058u64) }
        0x102c3058 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c305cu64) }
        0x102c305c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3060u64) }
        0x102c3060 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c3064u64) }
        0x102c3064 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c3068u64) }
        0x102c3068 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c306cu64).wrapping_add((imm * 4) as u64) } else { 0x102c306cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c306cu64) }
        0x102c306c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c3070u64) } }
        0x102c3070 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c3074u64) }
        0x102c3074 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c3078u64) }
        0x102c3078 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c307cu64).wrapping_add((imm * 4) as u64) } else { 0x102c307cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c307cu64) }
        0x102c307c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c3080u64) }
        0x102c3080 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c3084u64) }
        0x102c3084 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3088u64) }
        0x102c3088 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c308cu64) }
        0x102c308c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c3090u64) }
        0x102c3090 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c3094u64).wrapping_add((imm * 4) as u64) } else { 0x102c3094u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c3094u64) }
        0x102c3094 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c3098u64) } }
        0x102c3098 => { vm.decode(0x2e68c01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 248908i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c309cu64) }
        0x102c309c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c30a0u64) } }
        0x102c30a0 => { vm.decode(0x8f4cc41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 253112i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c30a4u64) }
        0x102c30b0 => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30b4u64) }
        0x102c30b4 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30b8u64) }
        0x102c30b8 => { vm.decode(0xd906051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30bcu64) }
        0x102c30bc => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30c0u64) }
        0x102c30c0 => { vm.decode(0xd14e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1340i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30c4u64) }
        0x102c30c4 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30c8u64) }
        0x102c30c8 => { vm.decode(0xd14e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30ccu64) }
        0x102c30cc => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30d0u64) }
        0x102c30d0 => { vm.decode(0xd1468d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1332i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30d4u64) }
        0x102c30d4 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30d8u64) }
        0x102c30d8 => { vm.decode(0xd146851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30dcu64) }
        0x102c30dc => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30e0u64) }
        0x102c30e0 => { vm.decode(0xd14e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1324i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30e4u64) }
        0x102c30e4 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30e8u64) }
        0x102c30e8 => { vm.decode(0xd14e051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30ecu64) }
        0x102c30ec => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30f0u64) }
        0x102c30f0 => { vm.decode(0xd1460d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1316i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30f4u64) }
        0x102c30f4 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c30f8u64) }
        0x102c30f8 => { vm.decode(0xd146051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c30fcu64) }
        0x102c30fc => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c3100u64) }
        0x102c3100 => { vm.decode(0xd10e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3104u64) }
        0x102c3104 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c3108u64) }
        0x102c3108 => { vm.decode(0xd10e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c310cu64) }
        0x102c310c => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c3110u64) }
        0x102c3110 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3114u64) }
        0x102c3114 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c3118u64) }
        0x102c3118 => { vm.decode(0xd106851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c311cu64) }
        0x102c311c => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c3120u64) }
        0x102c3120 => { vm.decode(0x1510e1d1u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3124u64) }
        0x102c3124 => { vm.decode(0x5510e051u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3128u64) }
        0x102c3128 => { vm.decode(0x6a8a63afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c312cu64) }
        0x102c312c => { vm.decode(0x28a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3130u64) }
        0x102c3130 => { vm.decode(0x12008119u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3134u64) }
        0x102c3134 => { vm.decode(0x9b215430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3138u64) }
        0x102c3138 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c313cu64) }
        0x102c313c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3140u64) }
        0x102c3140 => { vm.decode(0x704b0419u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3144u64) }
        0x102c3144 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3148u64) }
        0x102c3148 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c314cu64) }
        0x102c314c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3150u64) }
        0x102c3150 => { vm.decode(0x488e4bafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3154u64) }
        0x102c3154 => { vm.decode(0xa30d875u32); let d = 25i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3158u64) }
        0x102c3158 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c315cu64 + 4);
        vm.wr(37, 0x102c315cu64 + 4);
        Ok(0x102c315cu64) }
        0x102c315c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3160u64) }
        0x102c3160 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3164u64) }
        0x102c3164 => { vm.decode(0x70498519u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c3168u64) }
        0x102c3168 => { vm.decode(0x22a0416fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c316cu64) }
        0x102c316c => { vm.decode(0x9b2154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3170u64) }
        0x102c3170 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c3174u64) }
        0x102c3174 => { vm.decode(0x9b311430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c3178u64) }
        0x102c3178 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c317cu64) }
        0x102c317c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c3180u64) }
        0x102c3180 => { vm.decode(0xa30d875u32); let d = 25i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3184u64) }
        0x102c3184 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c3188u64 + 4);
        vm.wr(37, 0x102c3188u64 + 4);
        Ok(0x102c3188u64) }
        0x102c3188 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c318cu64) }
        0x102c318c => { vm.decode(0xd98e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3190u64) }
        0x102c3190 => { vm.decode(0x5986111u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3194u64) }
        0x102c3194 => { vm.decode(0x2001a835u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3198u64) }
        0x102c3198 => { vm.decode(0x131207f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c319cu64) }
        0x102c319c => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31a0u64) }
        0x102c31a0 => { vm.decode(0x459c6011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c31a4u64) }
        0x102c31a4 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31a8u64) }
        0x102c31a8 => { vm.decode(0xc4002fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31acu64) }
        0x102c31ac => { vm.decode(0x45186811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c31b0u64) }
        0x102c31b0 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31b4u64) }
        0x102c31b4 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31b8u64) }
        0x102c31b8 => { vm.decode(0xc4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31bcu64) }
        0x102c31bc => { vm.decode(0x40598835u32); let d = 2i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31c0u64) }
        0x102c31c0 => { vm.decode(0x440017f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31c4u64) }
        0x102c31c4 => { vm.decode(0x200d0835u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31c8u64) }
        0x102c31c8 => { vm.decode(0x4d144a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c31ccu64) }
        0x102c31cc => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31d0u64) }
        0x102c31d0 => { vm.decode(0xa0a00fb5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let v = (vm.ld(imm as i16) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31d4u64) }
        0x102c31d4 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31d8u64) }
        0x102c31d8 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31dcu64) }
        0x102c31dc => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31e0u64) }
        0x102c31e0 => { vm.decode(0x200cb835u32); let d = 3i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31e4u64) }
        0x102c31e4 => { vm.decode(0x461207f5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31e8u64) }
        0x102c31e8 => { vm.decode(0x405d2835u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c31ecu64) }
        0x102c31ec => { vm.decode(0x510cb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c31f0u64) }
        0x102c31f0 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31f4u64) }
        0x102c31f4 => { vm.decode(0x514e111u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c31f8u64) }
        0x102c31f8 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c31fcu64) }
        0x102c31fc => { vm.decode(0x64112fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3200u64) }
        0x102c3200 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3204u64) }
        0x102c3204 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3208u64) }
        0x102c3208 => { vm.decode(0x41400835u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c320cu64) }
        0x102c320c => { vm.decode(0x1157f5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3210u64) }
        0x102c3210 => { vm.decode(0x21008835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3214u64) }
        0x102c3214 => { vm.decode(0xd904311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3218u64) }
        0x102c3218 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c321cu64) }
        0x102c321c => { vm.decode(0xd146911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3220u64) }
        0x102c3220 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3224u64) }
        0x102c3224 => { vm.decode(0xba0fb5u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 22i64;
        let v = (vm.ld(imm as i16) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3228u64) }
        0x102c3228 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c322cu64) }
        0x102c322c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3230u64) }
        0x102c3230 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3234u64) }
        0x102c3234 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3238u64) }
        0x102c3238 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c323cu64) }
        0x102c323c => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3240u64) }
        0x102c3240 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3244u64) }
        0x102c3244 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3248u64) }
        0x102c3248 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c324cu64) }
        0x102c324c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3250u64) }
        0x102c3250 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3254u64) }
        0x102c3254 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3258u64) }
        0x102c3258 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c325cu64) }
        0x102c325c => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3260u64) }
        0x102c3260 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3264u64) }
        0x102c3264 => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3268u64) }
        0x102c3268 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c326cu64) }
        0x102c326c => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3270u64) }
        0x102c3270 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3274u64) }
        0x102c3274 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3278u64) }
        0x102c3278 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c327cu64) }
        0x102c327c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3280u64) }
        0x102c3280 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3284u64) }
        0x102c3284 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3288u64) }
        0x102c3288 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c328cu64) }
        0x102c328c => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3290u64) }
        0x102c3290 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3294u64) }
        0x102c3294 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3298u64) }
        0x102c3298 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c329cu64) }
        0x102c329c => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32a0u64) }
        0x102c32a0 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32a4u64) }
        0x102c32a4 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32a8u64) }
        0x102c32a8 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32acu64) }
        0x102c32ac => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32b0u64) }
        0x102c32b0 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32b4u64) }
        0x102c32b4 => { vm.decode(0x4514cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c32b8u64) }
        0x102c32b8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32bcu64) }
        0x102c32bc => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c32c0u64) }
        0x102c32c0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32c4u64) }
        0x102c32c4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32c8u64) }
        0x102c32c8 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32ccu64) }
        0x102c32cc => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32d0u64) }
        0x102c32d0 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32d4u64) }
        0x102c32d4 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32d8u64) }
        0x102c32d8 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32dcu64) }
        0x102c32dc => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c32e0u64) }
        0x102c32e0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32e4u64) }
        0x102c32e4 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c32e8u64) }
        0x102c32e8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32ecu64) }
        0x102c32ec => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32f0u64) }
        0x102c32f0 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32f4u64) }
        0x102c32f4 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c32f8u64) }
        0x102c32f8 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c32fcu64) }
        0x102c32fc => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3300u64) }
        0x102c3300 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3304u64) }
        0x102c3304 => { vm.decode(0x459ce911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3308u64) }
        0x102c3308 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c330cu64) }
        0x102c330c => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3310u64) }
        0x102c3310 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3314u64) }
        0x102c3314 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3318u64) }
        0x102c3318 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c331cu64) }
        0x102c331c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3320u64) }
        0x102c3320 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3324u64) }
        0x102c3324 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3328u64) }
        0x102c3328 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c332cu64) }
        0x102c332c => { vm.decode(0x45104b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3330u64) }
        0x102c3330 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3334u64) }
        0x102c3334 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3338u64) }
        0x102c3338 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c333cu64) }
        0x102c333c => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3340u64) }
        0x102c3340 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3344u64) }
        0x102c3344 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3348u64) }
        0x102c3348 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c334cu64) }
        0x102c334c => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3350u64) }
        0x102c3350 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3354u64) }
        0x102c3354 => { vm.decode(0x4590c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3358u64) }
        0x102c3358 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c335cu64) }
        0x102c335c => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3360u64) }
        0x102c3360 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3364u64) }
        0x102c3364 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3368u64) }
        0x102c3368 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c336cu64) }
        0x102c336c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3370u64) }
        0x102c3370 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3374u64) }
        0x102c3374 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c3378u64) }
        0x102c3378 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c337cu64) }
        0x102c337c => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3380u64) }
        0x102c3380 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3384u64) }
        0x102c3384 => { vm.decode(0x4594e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c3388u64) }
        0x102c3388 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c338cu64) }
        0x102c338c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3390u64) }
        0x102c3390 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3394u64) }
        0x102c3394 => { vm.decode(0x24820fb5u32); let d = 6i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c3398u64) }
        0x102c3398 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c339cu64) }
        0x102c339c => { vm.decode(0x121207f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c33a0u64) }
        0x102c33a0 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c33a4u64) }
        0x102c33a4 => { vm.decode(0xd14c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c33a8u64) }
        0x102c33a8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c33acu64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
